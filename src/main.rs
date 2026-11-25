fn main() -> anyhow::Result<()> {
    let mut shell = rush::shell::Shell::new()?;

    shell.run()
}
