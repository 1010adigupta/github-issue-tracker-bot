use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use reqwest::{self, Error};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

#[derive(Deserialize, Debug, Clone)]
struct RepoInput {
    owner: String,
    repo: String,
    label: String,
}

#[derive(Deserialize, Debug)]
struct Issue {
    id: u32,
    title: String,
    html_url: String,
    labels: Vec<Label>,
}

#[derive(Deserialize, Debug)]
struct Label {
    name: String,
}

#[derive(Debug)]
struct AppState {
    repo_inputs: Mutex<Vec<RepoInput>>,
    last_checked_issue_ids: Mutex<HashMap<String, u32>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_data = Arc::new(AppState {
        repo_inputs: Mutex::new(Vec::new()),
        last_checked_issue_ids: Mutex::new(HashMap::new()),
    });

    let data_clone = shared_data.clone();
    tokio::spawn(async move {
        loop {
            let repo_inputs = {
                let lock = data_clone.repo_inputs.lock().unwrap();
                lock.clone() // Clone the data to release the lock early
            };
        
            for repo_input in &repo_inputs {
                // Get the last checked issue id for the current repository
                let last_checked_issue_id = {
                    let mut last_checked = data_clone.last_checked_issue_ids.lock().unwrap();
                    *last_checked.entry(repo_input.repo.clone()).or_insert(0)
                };
        
                let issues = fetch_github_issues(&repo_input.owner, &repo_input.repo).await.unwrap();
                let mut new_last_checked_issue_id = last_checked_issue_id; 
                for issue in &issues {
                    if issue.id > last_checked_issue_id && issue.labels.iter().any(|label| label.name == repo_input.label) {
                        send_telegram_notification(issue).await;
                        new_last_checked_issue_id = new_last_checked_issue_id.max(issue.id);
                    }
                }
        
                // Update the last checked issue id for the current repository
                let mut last_checked = data_clone.last_checked_issue_ids.lock().unwrap();
                last_checked.insert(repo_input.repo.clone(), new_last_checked_issue_id);
            }
        
            sleep(Duration::from_secs(60*10)).await; // Check every 10 minutes
        }     
    });

    // Start Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_data.clone()))
            .route("/input", web::post().to(handle_input))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}

async fn fetch_github_issues(owner: &str, repo: &str) -> Result<Vec<Issue>, reqwest::Error> {
    let url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);
    let client = reqwest::Client::new();
    let response = client.get(url)
                         .header("User-Agent", "request")
                         .send()
                         .await?;

    let issues: Vec<Issue> = response.json().await?;
    Ok(issues)
}

async fn handle_input(input: web::Json<RepoInput>, data: web::Data<Arc<AppState>>) -> impl Responder {
    let mut repo_inputs = data.repo_inputs.lock().unwrap();
    repo_inputs.push(input.into_inner());
    HttpResponse::Ok().body("Input received")
}

//Add you bot token and chat id
async fn send_telegram_notification(issue: &Issue) {
    let instance = rustelebot::create_instance("bot token", "chat id");
        let message = format!("{} {}", issue.title, issue.html_url);
        if let Err(_) = rustelebot::send_message(&instance, &message, None) {
            print!("Error");
    }
}