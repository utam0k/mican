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
    // TODO
    Something,
}

// use std::io;
//
// use readline::editor::{Editor, Complete};
// use readline::history::History;
//
// pub type Handler = FnMut(Editor, Vec<u8>) -> io::Result<Option<String>>;
//
// pub struct EventHandler<'a> {
//     pub handler: &'a mut Handler,
//     pub event: Event,
// }
//
// impl<'a> EventHandler<'a> {
//     pub fn new(e: Event, h: &'a mut Handler) -> Self {
//         Self {
//             handler: h,
//             event: e,
//         }
//     }
// }
//
// pub fn hoge<'a>(e: Option<Event>, history: History) -> EventHandler<'a> {
//     match e {
//         Some(Event::Complete) => {
//             EventHandler::new(e.unwrap(), &mut move |mut ed, _| {
//                 if !ed.line.trim().len() == ed.line.len() {
//                     // TODO
//                     // ed.put("\t".into())?;
//                     return Ok(None);
//                 } else {
//                     ed.complete();
//                     ed.completion_next();
//                     ed.completion_disply();
//                 }
//                 Ok(None)
//             })
//         }
//         Some(Event::Enter) => {
//             EventHandler::new(e.unwrap(), &mut |ed: Editor, _| {
//                 let result = ed.line.clone();
//                 ed.completion_clear();
//                 ed.reset();
//                 ed.new_line();
//                 history.push(result.clone());
//                 history.reset();
//                 Ok(Some(result))
//             })
//         }
//         Some(Event::CtrlL) => {
//             EventHandler::new(e.unwrap(), &mut |ed, _| {
//                 ed.clear_screen();
//                 ed.write_line();
//                 Ok(None)
//             })
//         }
//         Some(Event::Delete) => {
//             EventHandler::new(e.unwrap(), &mut |ed, _| {
//                 ed.completion_clear();
//                 ed.delete(1);
//                 Ok(None)
//             })
//         }
//         Some(Event::ForwardChar) => {
//             EventHandler::new(e.unwrap(), &mut |ed, _| {
//                 ed.move_right(1);
//                 Ok(None)
//             })
//         }
//         Some(Event::BackwardChar) => {
//             EventHandler::new(e.unwrap(), &mut |ed, _| {
//                 ed.move_left(1);
//                 Ok(None)
//             })
//         }
//         Some(Event::PreviousHistory) => {
//             EventHandler::new(e.unwrap(), &mut |ed, _| {
//                 ed.completion_clear();
//                 if history.is_started() {
//                     history.set_first(ed.line.clone());
//                 }
//                 let history = match history.prev() {
//                     Some(h) => h,
//                     None => return Ok(None),
//                 };
//                 ed.replace(history);
//                 ed.move_to_end();
//                 Ok(None)
//             })
//         }
//         Some(Event::NextHistory) => {
//             EventHandler::new(e.unwrap(), &mut |ed, _| {
//                 ed.completion_clear();
//                 let history = match history.next() {
//                     Some(h) => h,
//                     None => return Ok(None),
//                 };
//                 ed.replace(history);
//                 ed.move_to_end();
//                 Ok(None)
//             })
//         }
//         Some(Event::BeginningOFLine) => {
//             EventHandler::new(e.unwrap(), &mut |ed, _| {
//                 ed.move_to_first();
//                 Ok(None)
//             })
//         }
//
//         Some(Event::EndOfLine) => {
//             EventHandler::new(e.unwrap(), &mut |ed, _| {
//                 ed.move_to_end();
//                 Ok(None)
//             })
//         }
//         Some(Event::Something) => EventHandler::new(e.unwrap(), &mut |_, _| Ok(None)),
//         None => {
//             EventHandler::new(e.unwrap(), &mut |ed, c| {
//                 ed.completion_clear();
//
//                 ed.put(&String::from_utf8(c).unwrap());
//                 history.reset_first();
//                 Ok(None)
//             })
//         }
//     }
// }
