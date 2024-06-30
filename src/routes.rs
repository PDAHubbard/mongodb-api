use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use rocket::{
    delete, futures::TryStreamExt, get, http::Status, post, put, response::status,
    serde::json::Json,
};
use rocket_db_pools::Connection;
use serde_json::{json, Value};

use crate::db;
use crate::models;

#[post("/recipes", data = "<data>", format = "json")] 
pub async fn create_recipe( 
    db: Connection<MainDatabase>, 
    data: Json<Recipe>, 
) -> status::Custom<Json<Value>> { 
    if let Ok(res) = db 
        .database("bread") 
        .collection::<Recipe>("recipes") 
        .insert_one(data.into_inner(), None) 
        .await 
    { 
        if let Some(id) = res.inserted_id.as_object_id() { 
            return status::Custom( 
                Status::Created, 
                Json( 
                    json!({"status": "success", "message": format!("Recipe ({}) created successfully", id.to_string())}), 
                ), 
            ); 
        } 
    } 

    status::Custom( 
        Status::BadRequest, 
        Json(json!({"status": "error", "message":"Recipe could not be created"})), 
    ) 
}

#[get("/recipes", format = "json")] 
pub async fn get_recipes(db: Connection<MainDatabase>) -> Json<Vec<Recipe>> { 
    let recipes = db 
        .database("bread") 
        .collection("recipes") 
        .find(None, None) 
        .await; 

    if let Ok(r) = recipes { 
        if let Ok(collected) = r.try_collect::<Vec<Recipe>>().await { 
            return Json(collected); 
        } 
    } 

    return Json(vec![]); 
}