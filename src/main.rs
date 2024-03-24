use std::env;

use codeforces_cli::utils::client::CFClient;

#[tokio::main]
async fn main() {
    let command = env::args().nth(1);
    let handle = env::args().nth(2);
    let commands = vec!["contestList", "ratingChange"];

    if let Some(first) = command {
        if commands.contains(&first.as_str()) {
            let client = CFClient::new();

            if let Some(user) = handle {
                if first == "ratingChange" {
                    client.fetch_rating_change(&user).await;
                }
            }
            else {
                client.fetch_contests().await;
            }
        } else {
            println!("{}", "Command not supported");
            println!("List of Supported Commands : {:?}", commands);
        }
    } else {
        println!("{}", "List of supported commands:");
        println!("{}", "1) codeforces-cli contestList");
        println!("{}", "2) codeforces-cli ratingChange <CF-Handle>");
    }
}
