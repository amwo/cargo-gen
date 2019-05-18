//use std::net::SocketAddr;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 {
        println!("Err::Arguments can get one.");
        std::process::exit(0);
    }

    let p = Path::new(&args[1]); //&std::path::Path
    let html = read_file(p);
    println!("{:?}", html.unwrap());
}

fn read_file(p: &std::path::Path) -> Result<String, Box<dyn std::error::Error + 'static>> {
    let html = fs::read_to_string(p)?;
    Ok(html)
}
//:RustRun test/files/index.html
