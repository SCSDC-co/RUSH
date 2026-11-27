use crate::shell::Shell;
use std::{env, fs, path::PathBuf};

pub fn pwd(shell: &mut Shell, _args: &[String]) -> anyhow::Result<i32> {
    println!("{}", shell.cwd.display());

    Ok(0)
}

pub fn echo(_shell: &mut Shell, args: &[String]) -> anyhow::Result<i32> {
    println!("{}", args.join(" "));

    Ok(0)
}

pub fn cd(shell: &mut Shell, args: &[String]) -> anyhow::Result<i32> {
    let path = if args.is_empty() || args[0] == "~" {
        &shell.home_dir
    } else {
        &PathBuf::from(&args[0])
    };

    let path = std::fs::canonicalize(path)?;

    env::set_current_dir(&path)?;
    shell.cwd = path;

    Ok(0)
}

pub fn exit(shell: &mut Shell, _args: &[String]) -> anyhow::Result<i32> {
    println!("Bye!");
    shell.quit = true;

    Ok(0)
}

pub fn help(_shell: &mut Shell, _args: &[String]) -> anyhow::Result<i32> {
    println!("RUSH {}", env!("CARGO_PKG_VERSION"));

    for builtin in super::BUILTINS {
        println!("  - {builtin}")
    }

    Ok(0)
}

pub fn history(shell: &mut Shell, args: &[String]) -> anyhow::Result<i32> {
    let history_file = fs::read_to_string(&shell.history_file)?;

    let entries: Option<usize> = if args.is_empty() {
        None
    } else {
        Some(args[0].parse()?)
    };

    let mut lines = history_file.lines();

    lines.next();

    let mut lines: Vec<&str> = lines.collect();

    if let Some(n) = entries {
        lines = lines.into_iter().take(n).collect();
    }

    for line in lines {
        println!("{line}");
    }

    Ok(0)
}
