use std::cmp::Ordering;

use crate::utils::model::model::{HttpResponse, Ranks, RatingChangeData};

use super::model::model::{ContestData, ContestPhase, UserInfo};
use colored::{Colorize, CustomColor};

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
                .filter(|&data| data.phase == ContestPhase::Before)
                .enumerate()
                .for_each(|(index, contest)| {
                    println!("{}) {} - {:?}", index + 1, contest.name, contest.contest_type)
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

    pub async fn fetch_user_info(&self, user: &str) {
        let url = self.base_path.to_string() + "user.info?handles=" + user;
        let response = reqwest::get(url)
            .await
            .unwrap()
            .json::<HttpResponse<UserInfo>>()
            .await
            .unwrap();

        println!("User Info:\n");
        response.result
            .iter()
            .enumerate()
            .for_each(|(index, data)| {
                println!("{}) Handle - {}", index+1, data.handle);

                let rank_color = get_color(&data.rank);
                let max_rank_color = get_color(&data.max_rank);

                if data.first_name.is_some() && data.last_name.is_some() {
                    println!("Name - {} {}", data.first_name.as_ref().unwrap(), data.last_name.as_ref().unwrap());
                }
                if data.country.is_some() {
                    println!("Country - {}", data.country.as_ref().unwrap());
                }
                println!("Rating - {}", format!("{}", data.rating).custom_color(rank_color).bold());
                println!("Rank - {:?}", data.rank);
                println!("Contribution - {}", data.contribution);
                println!("Max Rating - {}", format!("{}", data.max_rating).custom_color(max_rank_color).bold());
                println!("Max Rank - {:?}", data.max_rank);
                println!("Friend Count - {}", data.friend_of_count);
                if data.email.is_some() {
                    println!("Email - {}", data.email.as_ref().unwrap());
                }
                println!("Avatar : {}", data.avatar);
                println!("Title Photo : {}\n", data.title_photo);
            })
    }

    
}

fn get_color(rank: &Ranks) -> CustomColor {
    match rank {
        Ranks::Newbie => return CustomColor::new(128, 128, 128),
        Ranks::Pupil => return CustomColor::new(0, 128, 0),
        Ranks::Specialist => return CustomColor::new(3, 168, 158),
        Ranks::Expert => return CustomColor::new(0, 0, 255),
        Ranks::CandidateMaster => return CustomColor::new(170, 0, 170),
        Ranks::Master => return CustomColor::new(255, 140, 0),
        Ranks::InternationalMaster => return CustomColor::new(255, 140, 0),
        Ranks::GrandMaster => return CustomColor::new(255, 0, 0),
        Ranks::InternationalGrandMaster => return CustomColor::new(255, 0, 0),
        Ranks::LegendaryGrandMaster => return CustomColor::new(255, 0, 0)
    };
}