//use std::fs::File;
//use std::path::{Path, PathBuf};
use std::fs;
use std::net::SocketAddr;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 {
        println!("Err::Arguments can get one.");
        std::process::exit(0);
    }
    //let a: () = args[1];
    let p = &args[1];
    println!("{:?}", p);
    println!("{:?}", read_file(p));
}

fn read_file(path: &String) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let f: SocketAddr = fs::read_to_string(path)?.parse()?;
    Ok(())
}
