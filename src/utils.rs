use std::io::{self, Write};
use std::process::{Command, Stdio};

pub fn pbcopy(src: &str) -> io::Result<()> {
    let mut child = Command::new("pbcopy")
        .stdin(Stdio::piped())
        .spawn()?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(src.as_bytes())?;
    }
    child.wait()?;
    Ok(())
}
