use std::{env, fs::OpenOptions, io, path::PathBuf};

use anyhow::Context;
use rustyline::{CompletionType, Editor, config::Configurer, error::ReadlineError};

pub mod builtins;
pub mod execute_command;
pub mod helper;

/// This is the shell, with all of his commands, and built-ins
#[derive(Debug)]
pub struct Shell {
    pub quit: bool,
    pub cwd: PathBuf,
    pub home_dir: PathBuf,
    pub history_file: PathBuf,
}

impl Shell {
    /// Creates a new shell
    pub fn new() -> io::Result<Self> {
        let home_dir = dirs::home_dir()
            .with_context(|| "Could not determine home directory")
            .map_err(io::Error::other)?;

        Ok(Shell {
            quit: false,
            cwd: env::current_dir()?,
            home_dir: home_dir.clone(),
            history_file: home_dir.join(".rush_history"),
        })
    }

    /// Runs the actual shell
    pub fn run(&mut self) -> anyhow::Result<()> {
        let mut last_status = 0;

        let mut rl = Editor::<helper::RushHelper, _>::default()?;

        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.history_file)
        {
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e.into()),
        }

        rl.load_history(&self.history_file)?;

        rl.set_completion_type(CompletionType::List);

        while !self.quit {
            let readline = rl.readline(&format!("{} | {last_status} > ", self.cwd.display()));

            match readline {
                Ok(i) => {
                    rl.add_history_entry(&i)?;
                    last_status = self.execute_command(i)?;
                }
                Err(ReadlineError::Interrupted) => {}
                Err(ReadlineError::Eof) => {
                    builtins::implementation::exit(self, &[])?;
                }
                Err(e) => {
                    eprintln!("Failed to read the line: {e}");
                    last_status = 1;
                }
            }
        }

        rl.append_history(&self.history_file)?;

        Ok(())
    }
}
