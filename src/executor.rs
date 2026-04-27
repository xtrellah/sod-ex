use std::process::Command;

pub fn run_script(path: &str) -> std::io::Result<()> {
    Command::new("sh").arg(path).status()?;
    Ok(())
}
