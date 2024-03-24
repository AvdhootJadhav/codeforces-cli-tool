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