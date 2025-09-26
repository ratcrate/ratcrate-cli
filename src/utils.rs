//! Helper functions (parsers and other small utilities)


use regex::Regex;

/// Parse a github repository URL and extract (owner, repo)
///
/// Accepts common forms:
/// - https://github.com/owner/repo
/// - https://www.github.com/owner/repo
/// - https://github.com/owner/repo.git
/// - git@github.com:owner/repo.git
///
/// Returns None if it doesn't look like a Github Repo URL

pub fn parse_github_repo(url: &str) -> Option<String, String> {
    //case insensitive matcg to handle "github.com" or "GitHub.com"

    // "#" -  for raw string literal in rust helps to write strings withouur escape
    // [:/] - Match either ":" or "/"
    // (?P<owner>[^/]) - This is the capture groups
    // [^/]+ - Matches one more char which are not "/"
    // "/" - Matches the literal "/"
    // [^/.]+ - Matches one more char which are not "/" or "."
    // unwrap() - Pancking is okay here becuse pattern is hardcoded always
    let re = Regex::new(r#"(?i)github\.com[:/](?P<owner)[^/]+)/(?P<repo>[^/.]+)"#).unwrap(); 

    // syntax - re.captures(url).and_then(|caps| {...}))
    // captures - Helps to find match for `url` pattern and returns Option<Capture> - Some(caps) 
    // and_then - Method on Option enum which handles Option and Result types
    // and_then - Takes closure args and execute on Sum and do not run when None
    re.captures(url).and_then(|caps| {
        let owner = caps.name("owner")?.as_str().to_string();
        let repo = caps.name("repo")?.as_str().to_string();
        Some((owner, repo))
    })
}

// unit testing
// cfg(test) - Only compiles when running tests
// run using `cargo test`

#[cfg(test)]

// new module for better organization
mod tests {
    // to bring parent function in scope
    use super::parse_github_repo;

    // helps to mark this as a test function
    #[test]
    fn http_url() {
        let url = "https://github.com/owner/repo";
        let (o, r) = parse_github_repo(url).unwrap();
        assert_eq!(o, "owner");
        assert_eq!(r, "repo");
    }


    #[test]
    fn git_at_url() {
        let url = "git@github.com:owner/repo.git";
        let (o, r) = parse_github_repo(url).unwrap();
        assert_eq!(o, "owner");
        assert_eq!(r, "repo");
    }

    #[test]
    fn bad_url() {
        assert!(parse_github_repo("https://example.com/foo").is_none());
    }

}
