use crate::shell::Shell;
use std::{env, path::PathBuf};

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
