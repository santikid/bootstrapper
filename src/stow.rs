use std::{fmt, process::Command};

pub struct Error {
    pub error: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

pub fn stow(target: &str, path: &str, unstow: bool) -> Result<(), Error> {
    let mut args = vec![format!("--target={}", target), format!("{}/", path)];

    if unstow {
        args.insert(0, "-D".to_string());
    }
    let cmd = Command::new("stow")
        .args(args)
        .output()
        .expect("Failed to execute stow; is it installed?");
    if !cmd.status.success() {
        return Err(Error {
            error: String::from_utf8(cmd.stderr).unwrap(),
        });
    }
    Ok(())
}
