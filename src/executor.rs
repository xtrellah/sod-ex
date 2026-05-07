use std::process::Command;

pub fn run_script(path: &str) -> std::io::Result<()> {
    Command::new("sh").arg(path).status()?;
    Ok(())
}

pub fn run_script_with_args(path: &str, args: &[String]) -> std::io::Result<()> {
    Command::new("sh").arg(path).args(args).status()?;
    Ok(())
}
