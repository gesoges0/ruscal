fn main() {
    if let Err(e) = ruscal::get_args().and_then(ruscal::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
