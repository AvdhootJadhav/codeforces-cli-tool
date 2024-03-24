use std::env;

use codeforces_cli::utils::client::CFClient;

#[tokio::main]
async fn main() {
    let command = env::args().nth(1);
    let handle = env::args().nth(2);
    let commands = vec!["contestList", "ratingChange", "userInfo"];

    if command.is_none() {
        println!("{}", "List of supported commands:");
        println!("{}", "1) codeforces-cli contestList");
        println!("{}", "2) codeforces-cli ratingChange <CF-Handle>");
        return;
    }

    let first = command.unwrap();

    if !commands.contains(&first.as_str()) {
        println!("{}", "Command not supported");
        println!("List of Supported Commands : {:?}", commands);
        return;
    }

    if first.eq("ratingChange") || first.eq("userInfo") {
        if handle.is_none() {
            println!("CF Handle is required");
            return;
        }
    }

    let client = CFClient::new();
    let user = handle.unwrap();

    match first.as_str() {
        "contestList" => {
            client.fetch_contests().await;
        }
        "ratingChange" => {
            client.fetch_rating_change(&user.as_str()).await;
        },
        "userInfo" => {
            client.fetch_user_info(&user.as_str().replace(",", ";")).await;
        },
        _ => println!("Something unexpected occurred"),
    }
}
