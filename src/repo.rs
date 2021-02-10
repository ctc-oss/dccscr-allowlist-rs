use git2::Repository;
use std::path::Path;

const REPODIR: &str = "/tmp/dccscr";

pub fn latest(url: &str) -> Result<(String, String), String> {
    if !Path::new(REPODIR).join(".git").exists() {
        clone(url)
    } else {
        pull()
    }
}

/// (path,sha) on ok, message on fail
fn clone(url: &str) -> Result<(String, String), String> {
    match Repository::clone(url, REPODIR) {
        Ok(r) => {
            let sha = r.revparse_single("HEAD").unwrap().id().to_string();
            Ok((String::from(REPODIR), sha))
        }
        Err(e) => Err(format!("failed to clone: {}", e)),
    }
}

/// (path,sha) on ok, mesage on fail
fn pull() -> Result<(String, String), String> {
    match Repository::open(REPODIR) {
        Ok(r) => {
            let sha = r.revparse_single("HEAD").unwrap().id().to_string();
            Ok((String::from(REPODIR), sha))
        }
        Err(e) => Err(format!("failed to pull: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        match latest("https://repo1.dso.mil/dsop/dccscr-whitelists") {
            Ok((path, sha)) => println!("{}", sha),
            Err(e) => println!("error: {}", e),
        }
    }
}
