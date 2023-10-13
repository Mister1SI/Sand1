use std::env;

fn main() {
    let mut args = env::args().into_iter();
    args.next();
    if let Some(a) = args.next() {
        sand::client(a.trim().into());
    } else {
        sand::help();
    }
}