use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8
}
fn main() {
    // println!("Hello, world!");

    let args = Args::parse();

    for _ in 0..args.count {
        println!("hello, {}", args.name)
    }
}
