use clap::Clap;

use whitelists::repo;

#[derive(Clap)]
#[clap(version = "v0.1.0")]
/// DCCSCR Whitelist Tool
struct Opts {
    /// url of whiltelist repository
    #[clap(short, long, default_value = "https://repo1.dso.mil/dsop/dccscr-whitelists")]
    url: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let (repo, sha) = repo::latest(&opts.url).unwrap();
    println!("{}", sha)
}
