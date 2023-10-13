use std::env;

fn main() {
    let mut args = env::args().into_iter();
    args.next();
    if let Some(a) = args.next() {
        sand::server(a.trim().into());
    } else {
        sand::help();
    }
}