//use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(name = "FILE")]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut f = File::create(args.file).expect("file open error");

    let mut buf: String;

    loop {
        buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(n) => {
                if n == 0 { break; }
                else { write!(f, "{}", buf).expect("write error"); }
            }
            Err(e) => panic!("Error: {}", e),
        }
    }
}
