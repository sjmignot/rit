use anyhow::Context;
use std::fs;

pub fn init() -> anyhow::Result<()> {
    fs::create_dir(".git").context("failed to create .git directory")?;
    fs::create_dir(".git/objects").context("Failed to created .git/objects directory")?;
    fs::create_dir(".git/refs").context("Failed to create .git/refs directory")?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n").context("Failed to write to .git/HEAD")?;
    println!("Initialized git directory");
    Ok(())
}
