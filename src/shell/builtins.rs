use std::{env, path::PathBuf};

pub const BUILTINS: &[&str] = &["pwd", "echo", "exit", "cd"];

impl super::Shell {
    pub fn execute_builtin(&mut self, name: String, args: &[String]) -> anyhow::Result<i32> {
        match name.as_str() {
            "pwd" => {
                println!("{}", self.cwd.display())
            }
            "echo" => {
                println!("{}", args.join(" "))
            }
            "cd" => {
                let path = if args.is_empty() || args[0] == "~" {
                    PathBuf::from(env::var("HOME")?)
                } else {
                    PathBuf::from(&args[0])
                };

                let path = std::fs::canonicalize(path)?;

                env::set_current_dir(&path)?;
                self.cwd = path;
            }
            "exit" | "quit" => {
                println!("Bye!");
                self.quit = true
            }
            _ => {}
        }

        Ok(0)
    }
}
