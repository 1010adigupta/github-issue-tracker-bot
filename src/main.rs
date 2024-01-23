use std::os::unix::process;

use reqwest::{ self, Error };
use serde::{ Deserialize, Serialize };
use tokio::time::{ sleep, Duration };
use serde_json::Value;

const GITHUB_API_URL: &str = "https://api.github.com/repos/:owner/:repo/issues";
const TELEGRAM_API_URL: &str = "https://api.telegram.org/bot6838813835:AAGPtWnnJgAZO1uSN4cSK4vFW-716lsIydI/getUpdates";
const TELEGRAM_CHAT_ID: &str = "5807632964";
#[tokio::main]
async fn main() {
    loop {
    let issues = fetch_github_issues().await;
    let processed_issues = process_issues(issues.unwrap());
    send_telegram_notification(processed_issues).await;
    sleep(Duration::from_secs(10)).await;
    }
}

async fn fetch_github_issues() -> Result<Vec<Issue>, reqwest::Error> {
    let url = "https://api.github.com/repos/1010adigupta/portfolio/issues";
    let client = reqwest::Client::new();
    let response = client.get(url)
                         .header("User-Agent", "request")
                         .send()
                         .await?;

    // Deserialize the JSON content of the response
    let issues: Vec<Issue> = response.json().await?;

 Ok(issues)
}

fn process_issues(issues: Vec<Issue>) -> Vec<Issue> {
    let mut processed_issues: Vec<Issue> = Vec::new();
    for issue in issues {
        if issue.labels.iter().any(|label| label.name == "good first issue") {
            processed_issues.push(issue);
        }
    }

    processed_issues
}

async fn send_telegram_notification(issues: Vec<Issue>) {
    let instance = rustelebot::create_instance("6838813835:AAGPtWnnJgAZO1uSN4cSK4vFW-716lsIydI", "5807632964");
    for issue in issues {
        let message = format!("{} {} ", issue.title, issue.html_url);
        if let Err(_) = rustelebot::send_message(&instance, &message, None) {
            print!("Error")
        }
    }
}

#[derive(Deserialize, Debug)]
struct Issue {
    id: u32,
    title: String,
    html_url: String,
    labels: Vec<Label>,
    // Add other fields as needed
}


#[derive(Deserialize, Debug)]
struct Label {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RepositoryInfo {
    last_issue_id: u32,
    repo_name: String,
}