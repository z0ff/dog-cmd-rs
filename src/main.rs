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

    let mut f = match File::create(args.file) {
        Ok(file) => file,
        Err(e) => panic!("Error: {:?}", e),
    };

    let mut buf: String;

    loop {
        buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(n) => {
                if n == 0 { break; }
                else {
                    match write!(f, "{}", buf) {
                        Ok(_) => (),
                        Err(e) => panic!("Error!: {:?}", e),
                    }
                }
            }
            Err(e) => panic!("Error: {}", e),
        }
    }
}
