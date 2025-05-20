use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::auth::Auth;
use ejwt::{get_jwt, User};

pub fn configure_app(cfg: &mut web::ServiceConfig) {
  cfg
    .route("/public-view", web::get().to(public_view_handler))
    .route("/get-token", web::post().to(get_token_handler))
    .route("/secret-view", web::get().to(secret_view_handler));
}

async fn public_view_handler() -> HttpResponse {
  HttpResponse::Ok().json(json!({
    "success": true,
    "data": {
      "message": "This data is visible to all users"
    }
  }))
}

async fn get_token_handler(web::Json(user): web::Json<User>) -> HttpResponse {
  match get_jwt(user) {
    Ok(token) => HttpResponse::Ok().json(json!({
      "success": true,
      "data": { "token": token }
    })),
    Err(error) => HttpResponse::BadRequest().json(json!({
      "success": false,
      "data": { "message": error }
    })),
  }
}

async fn secret_view_handler(Auth(user): Auth) -> HttpResponse {
  HttpResponse::Ok().json(json!({
    "success": true,
    "data": user
  }))
}

