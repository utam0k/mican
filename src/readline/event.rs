use std::io;

use readline::editor::Complete;
use readline::context::{Context, Mode};

#[derive(Clone, Debug)]
pub enum Kind {
    Enter,
    Delete,
    Complete,
    CtrlL,
    ForwardChar,
    BackwardChar,
    PreviousHistory,
    NextHistory,
    BeginningOFLine,
    EndOfLine,
    Interrupt,
    // TODO
    Something,
}

pub type Handler = fn(&mut Context, Vec<u8>) -> io::Result<Option<String>>;

pub struct Event {
    // kind: Option<Kind>,
    pub handler: Handler,
}

impl Event {
    pub fn from_event_kind(k: &Option<Kind>) -> Self {
        let h: Handler = match *k {
            Some(Kind::Interrupt) => {
                |con, _| {
                    con.editor.completion_clear();
                    con.editor.reset();
                    con.editor.new_line();
                    con.history.reset();
                    Ok(None)
                }
            }
            Some(Kind::Complete) => {
                |con, _| {
                    if !con.editor.line.trim().len() == con.editor.line.len() {
                        // TODO
                        // editor.put("\t".into())?;
                        return Ok(None);
                    } else {
                        con.editor.complete();
                        con.editor.completion_next();
                        con.mode = Mode::Completion;
                    }
                    Ok(None)
                }
            }
            Some(Kind::Enter) => {
                |con, _| {
                    let result = con.editor.line.clone();
                    con.editor.completion_clear();
                    con.editor.reset();
                    con.editor.new_line();
                    con.history.push(result.clone());
                    con.history.reset();
                    Ok(Some(result))
                }
            }
            Some(Kind::CtrlL) => {
                |con, _| {
                    con.editor.clear_screen();
                    con.editor.write_line();
                    Ok(None)
                }
            }
            Some(Kind::Delete) => {
                |con, _| {
                    con.editor.completion_clear();
                    con.editor.delete(1);
                    Ok(None)
                }
            }
            Some(Kind::ForwardChar) => {
                |con, _| {
                    con.editor.move_right(1);
                    Ok(None)
                }
            }
            Some(Kind::BackwardChar) => {
                |con, _| {
                    con.editor.move_left(1);
                    Ok(None)
                }
            }
            Some(Kind::PreviousHistory) => {
                |Context {
                     editor,
                     history,
                     mode,
                 },
                 _| {
                    match mode {
                        Mode::Completion => {
                            editor.completion_prev();
                            Ok(None)
                        }
                        Mode::Normal => {
                            if history.is_started() {
                                history.set_first(editor.line.clone());
                            }
                            let history = match history.prev() {
                                Some(h) => h,
                                None => return Ok(None),
                            };
                            editor.replace(history);
                            editor.move_to_end();
                            Ok(None)
                        }
                    }
                }
            }
            Some(Kind::NextHistory) => {
                |Context {
                     editor,
                     history,
                     mode,
                 },
                 _| {
                    match mode {
                        Mode::Completion => {
                            editor.completion_next();
                            Ok(None)
                        }
                        Mode::Normal => {
                            let history = match history.next() {
                                Some(h) => h,
                                None => return Ok(None),
                            };
                            editor.replace(history);
                            editor.move_to_end();
                            Ok(None)
                        }
                    }
                }
            }
            Some(Kind::BeginningOFLine) => {
                |con, _| {
                    con.editor.move_to_first();
                    Ok(None)
                }
            }

            Some(Kind::EndOfLine) => {
                |con, _| {
                    con.editor.move_to_end();
                    Ok(None)
                }
            }
            Some(Kind::Something) => |_, _| Ok(None),
            _ => {
                |con, c: Vec<u8>| {
                    con.editor.completion_clear();
                    con.mode = Mode::Normal;

                    // con.editor.complete();
                    // con.editor.completion_next();
                    // con.editor.completion_disply();

                    con.editor.put(&String::from_utf8(c).unwrap());
                    con.history.reset_first();
                    Ok(None)
                }
            }
        };
        Self {
            // kind: k,
            handler: h,
        }
    }
}
