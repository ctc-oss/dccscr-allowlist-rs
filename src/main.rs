use clap::Clap;

use whitelists::allow;
use whitelists::repo;

#[derive(Clap)]
#[clap(version = "v0.1.0")]
/// DCCSCR Whitelist Tool
struct Opts {
    /// url of whiltelist repository
    #[clap(
        short,
        long,
        default_value = "https://repo1.dso.mil/dsop/dccscr-whitelists"
    )]
    url: String,

    /// image name excluding tag
    image: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let (repo, sha) = repo::latest(&opts.url).unwrap();
    match allow::greylisted(&opts.image, repo.as_str()) {
        Ok(list) => {
            for i in list {
                println!("{}", i.id);
                eprintln!("{} ({})", i.id, i.by);
            }
        }
        Err(e) => println!("Failure {}", e),
    }
    eprintln!("sha: {}", sha)
}
