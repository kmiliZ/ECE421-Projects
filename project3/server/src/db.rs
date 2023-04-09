// https://codevoweb.com/build-a-crud-api-with-rust-and-mongodb/
use crate::response::{GameData, GameListResponse, GameResponse, SingleGameResponse};
use crate::{error::Error::*, model::GameModel, schema::CreateGameSchema, Result};
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, IndexOptions, ReturnDocument};
use mongodb::{bson, options::ClientOptions, Client, Collection, IndexModel};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct DB {
    pub game_collection: Collection<GameModel>,
    pub collection: Collection<Document>,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mongodb_uri: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let database_name: String =
            std::env::var("MONGO_INITDB_DATABASE").expect("MONGO_INITDB_DATABASE must be set.");
        let mongodb_game_collection: String =
            std::env::var("MONGODB_GAME_COLLECTION").expect("MONGODB_GAME_COLLECTION must be set.");

        let mut client_options = ClientOptions::parse(mongodb_uri).await?;
        client_options.app_name = Some(database_name.to_string());

        let client = Client::with_options(client_options).unwrap();
        let database = client.database(database_name.as_str());

        let game_collection = database.collection(mongodb_game_collection.as_str());
        let collection = database.collection::<Document>(mongodb_game_collection.as_str());

        println!("✅ Database connected successfully");

        Ok(Self {
            game_collection,
            collection,
        })
    }

    fn doc_to_game(&self, game: &GameModel) -> Result<GameResponse> {
        let game_response = GameResponse {
            gameID: game.gameID.to_hex(),
            gameType: game.gameType.to_owned(),
            player1: game.player1.to_owned(),
            player2: game.player2.to_owned(),
            winner: game.winner.to_owned(),
            playedTime: game.playedTime,
        };

        Ok(game_response)
    }

    // find games at "page" page, "limit" games per page
    pub async fn fetch_games(&self, limit: i64, page: i64) -> Result<GameListResponse> {
        let find_options = FindOptions::builder()
            .limit(limit)
            .skip(u64::try_from((page - 1) * limit).unwrap())
            .build();

        let mut cursor = self
            .game_collection
            .find(None, find_options)
            .await
            .map_err(MongoQueryError)?;

        let mut json_result: Vec<GameResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_game(&doc.unwrap())?);
        }

        let json_game_list = GameListResponse {
            status: "success".to_string(),
            results: json_result.len(),
            games: json_result,
        };

        Ok(json_game_list)
    }

    pub async fn fetch_all_games(&self) -> Result<GameListResponse> {
        
        let mut cursor = self
            .game_collection
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut json_result: Vec<GameResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_game(&doc.unwrap())?);
        }

        let json_game_list = GameListResponse {
            status: "success".to_string(),
            results: json_result.len(),
            games: json_result,
        };

        Ok(json_game_list)
    }

    pub async fn create_game(&self, body: &CreateGameSchema) -> Result<Option<SingleGameResponse>> {
        let game_type = body.gameType.to_owned().unwrap_or("".to_string());
        let player1 = body.player1.to_owned().unwrap_or("".to_string());
        let player2 = body.player2.to_owned().unwrap_or("".to_string());
        let winner = body.winner.to_owned().unwrap_or("".to_string());
        let serialized_data = bson::to_bson(&body).map_err(MongoSerializeBsonError)?;
        let document = serialized_data.as_document().unwrap();
        let options = IndexOptions::builder().unique(true).build();

        let playedTime = Utc::now();

        let mut doc_with_dates = doc! {"playedTime": playedTime, "game_type": game_type, "player1": player1, "player2": player2, "winner": winner};
        doc_with_dates.extend(document.clone());

        let insert_result = self
            .collection
            .insert_one(&doc_with_dates, None)
            .await
            .map_err(|e| {
                if e.to_string()
                    .contains("E11000 duplicate key error collection")
                {
                    return MongoDuplicateError(e);
                }
                return MongoQueryError(e);
            })?;

        let new_id = insert_result
            .inserted_id
            .as_object_id()
            .expect("issue with new _id");

        let game_doc = self
            .game_collection
            .find_one(doc! {"_id":new_id }, None)
            .await
            .map_err(MongoQueryError)?;

        if game_doc.is_none() {
            return Ok(None);
        }

        let game_response = SingleGameResponse {
            status: "success".to_string(),
            game: GameData {
                game: self.doc_to_game(&game_doc.unwrap()).unwrap(),
            },
        };

        Ok(Some(game_response))
    }

    pub async fn get_game(&self, id: &str) -> Result<Option<SingleGameResponse>> {
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        let game_doc = self
            .game_collection
            .find_one(doc! {"_id":oid }, None)
            .await
            .map_err(MongoQueryError)?;

        if game_doc.is_none() {
            return Ok(None);
        }

        let game_response = SingleGameResponse {
            status: "success".to_string(),
            game: GameData {
                game: self.doc_to_game(&game_doc.unwrap()).unwrap(),
            },
        };

        Ok(Some(game_response))
    }

    pub async fn delete_game(&self, id: &str) -> Result<Option<()>> {
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        let result = self
            .collection
            .delete_one(doc! {"_id":oid }, None)
            .await
            .map_err(MongoQueryError)?;

        if result.deleted_count == 0 {
            return Ok(None);
        }

        Ok(Some(()))
    }
}
