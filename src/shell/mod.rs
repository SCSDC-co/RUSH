use std::{
    env,
    io::{self, Write},
    path::PathBuf,
};

pub mod builtins;
pub mod execute_command;

/// This is the shell, with all of his commands, and built-ins
#[derive(Debug)]
pub struct Shell {
    pub quit: bool,
    pub cwd: PathBuf,
}

impl Shell {
    /// Creates a new shell
    pub fn new() -> io::Result<Self> {
        Ok(Shell {
            quit: false,
            cwd: env::current_dir()?,
        })
    }

    /// Runs the actual shell
    pub fn run(&mut self) -> anyhow::Result<()> {
        let mut last_status = 0;

        while !self.quit {
            let mut input = String::new();

            print!("{} | {last_status} > ", self.cwd.display());
            io::stdout().flush()?;

            io::stdin().read_line(&mut input)?;

            last_status = self.execute_command(input)?;
        }

        Ok(())
    }
}
