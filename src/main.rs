use std::env;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() >= 4 {
        println!("Err::Arguments can get one.");
        std::process::exit(0);
    }

    let p = Path::new(&args[2]); //&std::path::Path
    let html = read_file(&p);
    println!("{:?}", html);

    //println!("{:?}", html.unwrap());

    write();
}

fn write() {
    let b = b"test";
    let mut f = BufWriter::new(fs::File::create("test.html").unwrap());
    for _ in 0..4 {
        f.write(b).unwrap();
    }
}

fn read_file(p: &std::path::Path) -> Result<String, Box<dyn std::error::Error + 'static>> {
    let current_dir = env::current_dir()?;
    let p = current_dir.join(p);
    println!("{:?}", p);
    let html = fs::read_to_string(p)?;
    Ok(html)
}

//:RustRun test/files/index.html
