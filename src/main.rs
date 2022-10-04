use clap::Parser;

#[derive(Parser, Debug)]
#[command(author="Jan Vanbuel", version="0.1.0", about="CLI to create Personal Access Tokens in Azure DevOps", long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
