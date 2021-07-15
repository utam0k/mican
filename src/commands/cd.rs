use token::CommandData;

use std::env;

// TODO

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn run(cmd: CommandData) -> Result<(), String> {
    if cmd.options.is_empty() {
        #[allow(deprecated)]
        env::set_current_dir(&env::home_dir().unwrap()).unwrap();
        return Ok(());
    }

    let mut current_path_buf = env::current_dir().unwrap();
    current_path_buf.push(&cmd.options[0]);
    if env::set_current_dir(current_path_buf.as_path()).is_err() {
        return Err(format!("{} not found", cmd.options[0]));
    };
    Ok(())
}
