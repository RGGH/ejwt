use actix_web::{
  dev::Payload,
  error::InternalError,
  http::header,
  FromRequest, HttpRequest, HttpResponse,
};
use serde_json::json;
use std::future::{ready, Ready};

use ejwt::{decode_jwt, User};

pub struct Auth(pub User);

impl FromRequest for Auth {
  type Error = InternalError<String>;
  type Future = Ready<Result<Self, Self::Error>>;

  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    let token = req
      .headers()
      .get(header::AUTHORIZATION)
      .and_then(|value| value.to_str().ok())
      .and_then(|str| str.split(" ").nth(1));

    match token {
      Some(token) => match decode_jwt(token) {
        Ok(user) => ready(Ok(Auth(user))),
        Err(e) => ready(Err(unauthorized(e))),
      },
      None => ready(Err(unauthorized("No token provided".into()))),
    }
  }
}

fn unauthorized(msg: String) -> InternalError<String> {
  InternalError::from_response(
    msg.clone(),
    HttpResponse::Unauthorized().json(json!({
      "success": false,
      "data": { "message": msg }
    })),
  )
}

