use std::env;

enum CLI_Options {
    List = '-l',
    Help = '-h',
    New = '-n',
    Explicit = '-x',
    Update = '-u',
}

fn main() {
    let args: Vec<String> = env::args().collect();
}

