fn main() {
    let mut args = std::env::args();

    match args.len() {
        1 => dynamix::repl(),
        2 => dynamix::run_file(&args.nth(1).unwrap()),
        3 => dynamix::run_file(&args.nth(1).unwrap()),
        _ => dynamix::print_usage(),
    }
}
