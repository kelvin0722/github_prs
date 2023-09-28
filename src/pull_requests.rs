extern crate serde;
extern crate serde_json;

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequest {
    pub title: String,
    pub number: u32,
    pub labels: Vec<Label>,
    pub html_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullReviewers {
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub login: String,
    pub id: i64,
}

pub async fn fetch_pull_requests(
    client: &Client,
    github_token: &str,
    org: &str,
    repo: &str,
) -> Result<Vec<PullRequest>, reqwest::Error> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls?state=open",
        org, repo
    );

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", github_token))
        .header("User-Agent", "GitHub-PR-Notifier")
        .send()
        .await?;

    let pull_requests: Vec<PullRequest> = response.json().await?;

    Ok(pull_requests)
}

pub async fn fetch_requested_reviewers(
    client: &Client,
    github_token: &str,
    org: &str,
    repo: &str,
    pull_no: &u32,
) -> Result<PullReviewers, reqwest::Error> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}/requested_reviewers",
        org, repo, pull_no
    );

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", github_token))
        .header("User-Agent", "GitHub-PR-Notifier")
        .send()
        .await?;

    let requested_reviewers: PullReviewers = response.json().await?;

    Ok(requested_reviewers)
}
