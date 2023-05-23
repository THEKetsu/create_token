use actix_web::{web, App, Error, HttpResponse, HttpServer,Responder, HttpRequest, post, http::header,get};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    admin: String,
    login: String,
    exp: usize,
}


#[derive(Deserialize)]
pub struct User {
    user_name: String,
    password: String,
}
#[get("/create_token")]
pub async fn create_token(_user:web::Form<User>) -> impl Responder {
    let claims = Claims {
        admin: "true".to_owned(),
        login: _user.user_name.to_owned(),
        exp: 10000000000,
    };
    let token = encode(&Header::default(),&claims, &EncodingKey::from_secret("SECRET_KEY".as_ref())).unwrap();
    return HttpResponse::Ok()
    .insert_header(("Authorization",token))
    .body("Token created");



}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_token)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}