use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;
use tokio::time::sleep;
use serde_json::*;


#[derive(Debug, Serialize, Deserialize)] struct Crates {
    id: String,
    name: String,
    description: Option<String>,
    homepage: Option<String>,
    documentation: Option<String>,
    repository: Option<String>,
    downloads: u64,
    max_stable_version: Option<String>,
    yanked : bool,
}


// crates.io API response
#[derive(Debug, Deserialize)]
struct CratesResponse {
    crates: Vec<Crates>,
    meta: Meta,
}



#[derive(Debug, Deserialize)]
struct Meta {
    total: u32,
    next_page: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CrateVersionResponse {
    version: Vec<Version>,
}

#[derive(Debug, Deserialize)]
struct Version {
    num: String,
    yanked: bool,
}

//github api response

struct GithubContent {
    name: String,
    download_url: Option<String>,
    // #[serde(rename = "type")]
    content_type: String,
}

//final output structure 
struct RatatuiCrate {
    id: String,
    name: String,
    description: Option<String>,
    homepage: Option<String>,
    documentation: Option<String>,
    repository: Option<String>,
    max_stable_version: Option<String>,
    downloads: u64,
    latest_version_yanked: Option<bool>,
    github_repo: Option<String>,
    cargo_toml_utl: Option<String>,
    uses_ratatui: bool,
    ratatui_dependencies_info: Option<String>,
}


#[tokio::main]

async fn main() -> Result<()> {

    println!("Starting crates extraction");
    
    let client = Client::builder()
                .timeout(Duration::from_secs(30))
                .user_agent("ratatui-test/1.0")
                .build()?;

    // println!("{:?}", client);
    let url = "https://crates.io/api/v1/crates?per_page=20";
    println!("Fetching 20 crates from {}", url);

    let response = client.get(url).send().await?;

    // let text = response.text().await?;
    // println!("text: {}", text);

    //prints status 
    println!("Status: {}", response.status());

    //json
    let crates_response = response.json::<CratesResponse>().await?;
    println!("Found {} crates", crates_response.meta.total);
    // println!("{:?}", crates_response.crates);
    println!("{:?}", crates_response.crates.len());
    // println!("{:?}", crates_response.crates[0]);

    //filter the crates

    let github_crates: Vec<Crates> = crates_response.crates
                    .into_iter()
                    .filter(|c|
                        if let Some(repo) = &c.repository {
                            repo.contains("github.com")
                        }        
                        else {
                                false
                            }
        )
        .collect();

    // some of the repo will have repository as blank
    // so need to find a different way or just skip it for now
    // IDEA, TASK: I can automate to find email address and send a note to the author
    
    println!("Found {} crates with github repository", github_crates.len());

    //process each crate 

    let mut raratui_crates: Vec<RatatuiCrate> = Vec::new();
    
    for (i, crate_info) in github_crates.iter().enumerate() {
        if !crate_info.yanked {
            println!("\n --- processing crates {}/{}: {} --  {:?}", i+1, 
                    github_crates.len(), crate_info.name, crate_info.yanked,
                    );
            
        }
    }

    //match gihub repo
    let github_repo = match extract_github_repo(&crate_info.repository) {
       Some(repo) => repo,
        None => {
            println!("{} has no github repo", crate_info.name);
            continue;
        }
    }
    println!("Github Repo {}", github_repo);

    Ok(())
}


fn extract_github_repo(repository_url: &Option<String>) -> Result<Option<String>> {

    let url = match repository_url {
        Some(url) => url,
        Ok(None) => return Ok(None),
    };




    // Ok(None)
} 














