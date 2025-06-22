fn github_repo_to_raw_url(repo_url: &str, branch: &str, file_path: &str) -> Option<String> {
    // Handle both https and git URLs
    let cleaned_url = repo_url
        .trim_end_matches(".git")
        .replace("git@github.com:", "https://github.com/");

    // Extract owner and repo from GitHub URL
    if let Some(github_part) = cleaned_url.strip_prefix("https://github.com/") {
        let parts: Vec<&str> = github_part.split('/').collect();
        if parts.len() >= 2 {
            let owner = parts[0];
            let repo = parts[1];
            return Some(format!(
                "https://raw.githubusercontent.com/{}/{}/{}/{}",
                owner, repo, branch, file_path
            ));
        }
    }
    None
}

pub fn fetch_package_files(repo_url: &str, package: &str) -> (String, String) {
    let client = reqwest::blocking::Client::new();
    let build_lua_url =
        github_repo_to_raw_url(repo_url, "refs/heads/master", &format!("packages/{}/build.lua", package))
            .ok_or("Invalid GitHub URL")
            .unwrap();
    let info_json_url =
        github_repo_to_raw_url(repo_url, "refs/heads/master", &format!("packages/{}/info.json", package))
            .ok_or("Invalid GitHub URL")
            .unwrap();

    println!("{}",build_lua_url);
    println!("{}",info_json_url);

    let build_response = client.get(&build_lua_url).send().unwrap();
    let info_response = client.get(&info_json_url).send().unwrap();

    (
        build_response.text().unwrap(),
        info_response.text().unwrap(),
    )
}
