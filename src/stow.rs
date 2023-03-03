use std::{fmt, process::Command};

pub struct Error {
    pub error: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

pub fn stow(target: &str, path: &str, unstow: Option<bool>) -> Result<(), Error> {
    let mut args = vec![format!("--target={}", target), format!("{}/", path)];

    if unstow.unwrap_or(false) {
        args.insert(0, "-D".to_string());
        println!("Unstowing {} from {}", path, target)
    } else {
        println!("Stowing {} to {}", path, target)
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
