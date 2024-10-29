use clap::Parser;
use ziweidoushu::calc::{birthday_from_arg, build_palais};
use ziweidoushu::interactive::birthday_from_prompt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    /// YYYYMMDDHHG (G: 0 - female; !0 - male)
    #[arg(long, short)]
    date: Option<u64>,
}

fn main() {
    let args = Args::parse();
    match args.date {
        Some(d) => build_palais(birthday_from_arg(d)),
        None => build_palais(birthday_from_prompt()),
    }
}
