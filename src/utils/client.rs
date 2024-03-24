use std::cmp::Ordering;

use crate::utils::model::model::{HttpResponse, RatingChangeData};

use super::model::model::ContestData;
use colored::Colorize;

pub struct CFClient {
    pub base_path: String,
}

impl CFClient {
    pub fn new() -> CFClient {
        CFClient {
            base_path: String::from("https://codeforces.com/api/"),
        }
    }

    pub async fn fetch_contests(&self) {
        let url = self.base_path.to_string() + "contest.list?gym=false";
        let response = reqwest::get(url)
            .await
            .unwrap()
            .json::<HttpResponse<ContestData>>()
            .await
            .unwrap();

        if !response.result.is_empty() {
            response
                .result
                .iter()
                .rev()
                .filter(|&data| data.phase == "BEFORE")
                .enumerate()
                .for_each(|(index, contest)| {
                    println!("{}) {} - {}", index + 1, contest.name, contest.contest_type)
                });
        }
    }

    pub async fn fetch_rating_change(&self, user: &str) {
        let url = self.base_path.to_string() + "user.rating?handle=" + user;
        let response = reqwest::get(url)
            .await
            .unwrap()
            .json::<HttpResponse<RatingChangeData>>()
            .await
            .unwrap();

        let data = response.result;

        if !data.is_empty() {
            println!("Rating Changes for {}", user);

            data.iter().enumerate().for_each(|(index, change)| {
                let color = match change.old_rating.cmp(&change.new_rating) {
                    Ordering::Less => "green",
                    Ordering::Greater => "red",
                    Ordering::Equal => "cyan",
                };
                println!(
                    "{}) {}[{}] {} ==> {}",
                    index + 1,
                    change.contest_name,
                    change.contest_id,
                    change.old_rating,
                    format!("{}", change.new_rating).color(color).bold()
                );
            })
        }
    }
}
