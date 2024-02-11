use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String,
}

fn main() {
    let args = Cli::from_args();
    println!(
        "Country: {}, Country Code: {}",
        args.city, args.country_code
    );
}
