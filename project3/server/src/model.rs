use chrono::prelude::*;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameModel {
    #[serde(rename = "_id")]
    pub gameID: ObjectId,
    pub gameType: String,
    pub player1: String,
    pub player2: String,
    pub winner: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub playedTime: DateTime<Utc>,
}