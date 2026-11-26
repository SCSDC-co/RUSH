use std::fmt::Display;

pub mod implementation;

type BuiltinFn = fn(&mut super::Shell, &[String]) -> anyhow::Result<i32>;

pub struct Builtin {
    pub function: BuiltinFn,
    pub description: &'static str,
    pub aliases: &'static [&'static str],
}

impl Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.aliases.join(", "), self.description)
    }
}

static BUILTINS: &[Builtin] = &[
    Builtin {
        function: implementation::pwd,
        description: "Print working directory",
        aliases: &["pwd"],
    },
    Builtin {
        function: implementation::echo,
        description: "Prints its arguments",
        aliases: &["echo"],
    },
    Builtin {
        function: implementation::cd,
        description: "Change the current directory",
        aliases: &["cd"],
    },
    Builtin {
        function: implementation::exit,
        description: "Exits the shell",
        aliases: &["exit", "quit"],
    },
    Builtin {
        function: implementation::help,
        description: "Prints this help message",
        aliases: &["help", "?"],
    },
];

impl super::Shell {
    pub fn execute_builtin(&mut self, name: &str, args: &[String]) -> anyhow::Result<Option<i32>> {
        if let Some(builtin) = BUILTINS.iter().find(|b| b.aliases.contains(&name)) {
            return Ok(Some((builtin.function)(self, args)?));
        }

        Ok(None)
    }
}
