pub mod model {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct HttpResponse<T> {
        pub status: String,
        pub result: Vec<T>
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct ContestData {
        pub id: i64,
        pub name: String,
        #[serde(alias = "type")]
        pub contest_type: ContestType,
        pub phase: ContestPhase,
        pub frozen: bool,
        #[serde(alias = "durationSeconds")]
        pub duration_seconds: i64,
        #[serde(alias = "startTimeSeconds")]
        pub start_time_seconds: i128,
        #[serde(alias = "relativeTimeSeconds")]
        pub relative_time_seconds: i128
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct RatingChangeData {
        #[serde(alias = "contestId")]
        pub contest_id: i64,
        #[serde(alias = "contestName")]
        pub contest_name: String,
        pub handle: String,
        pub rank: i64,
        #[serde(alias = "ratingUpdateTimeSeconds")]
        pub rating_update_time_seconds: i128,
        #[serde(alias = "oldRating")]
        pub old_rating: i64,
        #[serde(alias = "newRating")]
        pub new_rating: i64
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct UserInfo {
        pub handle: String,
        pub email: Option<String>,
        #[serde(alias = "vkId")]
        pub vk_id: Option<String>,
        #[serde(alias = "openId")]
        pub open_id: Option<String>,
        #[serde(alias = "firstName")]
        pub first_name: Option<String>,
        #[serde(alias = "lastName")]
        pub last_name: Option<String>,
        pub country: Option<String>,
        pub city: Option<String>,
        pub organization: Option<String>,
        pub contribution: i64,
        pub rank: String,
        pub rating: i64,
        #[serde(alias = "maxRank")]
        pub max_rank: String,
        #[serde(alias = "maxRating")]
        pub max_rating: i64,
        #[serde(alias = "lastOnlineTimeSeconds")]
        pub last_online_time_seconds: i128,
        #[serde(alias = "registrationTimeSeconds")]
        pub registration_time_seconds: i128,
        #[serde(alias = "friendOfCount")]
        pub friend_of_count: i64,
        pub avatar: String,
        #[serde(alias = "titlePhoto")]
        pub title_photo: String
    }

    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
    pub enum ContestPhase {
        #[serde(alias = "BEFORE")]
        Before,
        #[serde(alias = "CODING")]
        Coding,
        #[serde(alias = "PENDING_SYSTEM_TEST")]
        PendingSystemTest,
        #[serde(alias = "SYSTEM_TEST")]
        SystemTest,
        #[serde(alias = "FINISHED")]
        Finished
    }

    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
    pub enum ContestType {
        CF,
        IOI,
        ICPC
    }

}