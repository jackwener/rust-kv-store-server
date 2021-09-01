struct Opt {
    command: Command
}

enum Command {
    Get,
    Set,
    Remove,
}


fn main() {
    let opt = Opt;
    if let Err(e) = run(opt) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(opt: Opt) -> Result<> {
    match opt.command {
        Command::Get => {}
        Command::Set => {}
        Command::Remove => {}
    }
    Ok(())
}