use ploy::*;
use std::io::{Read, stdin, stdout};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut reader = Reader::from(stdin.bytes().map(Result::unwrap));
    let mut printer = Printer::from(stdout);

    loop {
        let obj = reader.read().expect("Reader error");
        printer.print(obj).expect("Printer error");
    }
}
