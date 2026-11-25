impl super::Shell {
    pub fn execute_command(&mut self, input: String) -> anyhow::Result<i32> {
        let mut splitted_input = input.split_whitespace();

        let name = splitted_input.next().map(String::from).unwrap_or_default();
        let args: Vec<String> = splitted_input.map(String::from).collect();

        if super::builtins::BUILTINS.contains(&name.as_str()) {
            return self.execute_builtin(name, &args);
        }

        let status = std::process::Command::new(name).args(args).status()?;

        Ok(status.code().unwrap_or(1))
    }
}
