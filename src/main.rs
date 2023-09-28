extern crate serde;
extern crate serde_json;

use dotenv::dotenv;
use mac_notification_sys::{send_notification, MainButton, Notification};
use reqwest::Client;
use std::error::Error;
use tokio::time::{sleep, Duration};

mod notification_helper;
mod pull_requests;

pub use self::pull_requests::{
    fetch_pull_requests, fetch_requested_reviewers, PullRequest, PullReviewers,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let github_token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let username = std::env::var("GITHUB_USERNAME").expect("GITHUB_USERNAME not set");
    let org = std::env::var("GITHUB_ORG").expect("GITHUB_ORG not set");
    let repo = std::env::var("GITHUB_REPO").expect("GITHUB_REPO not set");

    let action1 = "Review PR";
    let action2 = "Cancel";

    let client = Client::new();

    let pull_requests = fetch_pull_requests(&client, &github_token, &org, &repo).await?;

    loop {
        for pr in &pull_requests {
            for label in &pr.labels {
                if label.name == "READY FOR REVIEW" {
                    // Check if you are requested for review
                    let requested_reviewers =
                        fetch_requested_reviewers(&client, &github_token, &org, &repo, &pr.number)
                            .await?;

                    match &requested_reviewers
                        .users
                        .iter()
                        .find(|&x| x.login == username)
                    {
                        Some(_) => {
                            let title = &format!("PR #{} READY FOR REVIEW ", &pr.number);
                            let message = &format!("{}", &pr.html_url);
                            let response = send_notification(
                                &title,
                                Some(&format!("{}", &pr.title)),
                                &message,
                                Some(
                                    Notification::new()
                                        .main_button(MainButton::DropdownActions(
                                            "Dropdown",
                                            &[action1, action2],
                                        ))
                                        .sound("Blow"),
                                ),
                            )
                            .unwrap();

                            notification_helper::handle_response(
                                &response,
                                &action1,
                                &action2,
                                &pr.html_url,
                            );
                        }
                        None => {
                            println!("You have not been requested to review a PR");
                        }
                    }
                } else {
                    println!("No PRs ready for review");
                }
            }
        }

        // Sleep for a minute before checking again
        sleep(Duration::from_secs(60)).await;
    }
}
