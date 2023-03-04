use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "fwp", about = "Fun with pictures", author = "nkirschall")]
pub struct Args {
    #[arg(short, long)]
    pub pic: Option<String>,
    #[arg(short, long)]
    pub out: Option<String>,
    #[arg(short, long, default_value_t = false)]
    pub server: bool,
}
