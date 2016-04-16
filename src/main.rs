extern crate babble;
use babble::babble::Babble;

extern crate getopts;
use getopts::Options;
use std::env;

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut b = Babble::new();

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "output this help text");
    opts.optopt("e", "evaluate", "evaluate a string provided as an argument",
                "CODE");
    let matches = opts.parse(&args[1..]).unwrap();

    if matches.opt_present("h") {
        let usage = format!("usage: {} [options...] [FILENAME]", args[0]);
        print!("{}", opts.usage(&usage));
    } else if let Some(code) = matches.opt_str("e") {
        b.run(code);
    } else {
        let mut stream: Box<Read> = if matches.free.is_empty() {
            Box::new(::std::io::stdin())
        } else {
            Box::new(File::open(matches.free[0].clone()).unwrap())
        };
        let mut code = String::new();
        stream.read_to_string(&mut code).unwrap();
        b.run(code);
    }
}
