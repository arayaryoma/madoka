use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "madoka.conf.yaml")]
    config: String,
}

fn main() {
    let args = Args::parse();
    println!("config file: {}", args.config)
}
