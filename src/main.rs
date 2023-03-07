use clap::Parser;
use fwp::{args, ascii, open, scale, server};
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Args::parse();
    if args.server {
        println!("Starting server");
        server::start_server()?;
        return Ok(());
    }
    match args.pic {
        Some(pic) => {
            let image = open::open_img(&pic)?;
            let thumb1 = scale::scale(100, image);
            match args.out {
                Some(out) => {
                    let f = &mut std::fs::File::create(out)?;
                    f.write_all(ascii::to_ascii(thumb1).as_bytes())?;
                }
                None => {
                    println!("{}", ascii::to_ascii(thumb1));
                    return Ok(());
                }
            }
        }
        None => println!("See -h for help"),
    }
    Ok(())
}
