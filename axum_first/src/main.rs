
use axum::{extract::{Path, Query}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ResponseUser {
    id: u64,
    name: Option<String>,
}


#[derive(Deserialize)]
struct User{
    name:Option<String>,
    age:Option<u16>
}


#[tokio::main]
async fn main(){

    let app: Router = Router::new()
        .route("/rest/path/{user_name}/{user_age}", get(path_user))
        .route("/rest/query", get(user))
        .route("/rest/new-user", post(new_user_add))
        .route("/rest", get(rest));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("server is running");
    axum::serve(listener, app).await.unwrap();

}

async fn rest()->String{
    let value =String::from("value");
    value
}

async fn path_user(Path((user_name,user_age)):Path<(String,u16)>)->String{

    format!("your name {} your age {}",user_name,user_age)

}

async fn user(Query(user):Query<User>) ->String{
    let name = user.name.unwrap_or(String::from("efe"));
    let age = user.age.unwrap_or(20);
    format!("hello {} your age {}",name,age)
}
async fn new_user_add(Json(user):Json<User>) ->Json<ResponseUser>{
    let new_user = ResponseUser{
        id:1,
        name:user.name.clone()
    };
    Json(new_user)

}