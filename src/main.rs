use clap::Clap;
use log::info;

use whitelists::allow;
use whitelists::repo;

const DCCSCR_REPO: &str = "https://repo1.dso.mil/dsop/dccscr-whitelists";

#[derive(Clap)]
#[clap(version = "v0.1.0")]
/// DCCSCR Greylist Tool
struct Opts {
    /// url of greylist repository
    #[clap(short, long, default_value = DCCSCR_REPO)]
    url: String,

    /// debug mode shows source of allow
    #[clap(long, conflicts_with = "quiet")]
    debug: bool,

    /// quiet mode does not write to stdout
    #[clap(short, long, conflicts_with = "debug")]
    quiet: bool,

    /// destination file writes cve list to file
    #[clap(short, long)]
    outfile: Option<String>,

    /// image name excluding tag
    image: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let (repo, sha) = repo::latest(&opts.url).unwrap();
    match allow::greylisted(&opts.image, repo.as_str()) {
        Ok(list) => {
            for i in list {
                if !opts.quiet {
                    if opts.debug {
                        println!("{} ({})", i.id, i.by);
                    } else {
                        println!("{}", i.id);
                    }
                }
                if let Some(_f) = opts.outfile.as_ref() {
                    // todo;; write file
                }
            }
        }
        Err(e) => eprintln!("Failure {}", e),
    }
    info!("sha: {}", sha)
}
