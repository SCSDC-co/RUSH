use std::io::ErrorKind;

impl super::Shell {
    pub fn execute_command(&mut self, input: String) -> anyhow::Result<i32> {
        let mut splitted_input = input.split_whitespace();

        let name = splitted_input.next().map(String::from).unwrap_or_default();
        let args: Vec<String> = splitted_input.map(String::from).collect();

        if let Some(status) = self.execute_builtin(&name, &args)? {
            return Ok(status);
        }

        let status = std::process::Command::new(&name).args(args).status();

        match status {
            Ok(s) => Ok(s.code().unwrap_or(1)),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    eprintln!("Command not found: {name}");

                    Ok(127)
                }
                _ => Err(e.into()),
            },
        }
    }
}
