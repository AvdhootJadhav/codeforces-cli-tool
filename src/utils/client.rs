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
        let mut response = reqwest::get(url)
            .await
            .unwrap()
            .json::<HttpResponse<ContestData>>()
            .await
            .unwrap();

        if response.result.len() > 0 {
            println!("List of Upcoming Contests:");
            let mut index = 0;
            response.result.reverse();
            for contest in response.result {
                if contest.phase == "BEFORE" {
                    index += 1;
                    println!("{}) {} - {}", index, contest.name, contest.contest_type);
                }
            }
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

        let mut data = response.result;

        if data.len() > 0 {
            println!("Rating Changes for {}", user);
            let mut index = 0;

            for change in data {
                index += 1;
                if change.old_rating < change.new_rating {
                    println!("{}) {}[{}] {} ==> {}", index, change.contest_name, change.contest_id, change.old_rating, format!("{}",change.new_rating).color("green").bold());
                } else if change.old_rating == change.new_rating {
                    println!("{}) {}[{}] {} ==> {}", index, change.contest_name, change.contest_id, change.old_rating, format!("{}",change.new_rating).color("cyan").bold());
                } else {
                    println!("{}) {}[{}] {} ==> {}", index, change.contest_name, change.contest_id, change.old_rating, format!("{}",change.new_rating).color("red").bold());
                }
            }
        }
    }

}
