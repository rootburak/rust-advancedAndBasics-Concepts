
use user::{ActiveModel as UserModel,Entity as users};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ConnectionTrait, Database, EntityTrait, ModelTrait, Statement};

mod user;

#[tokio::main]
async fn main() {
    let connection = Database::connect("sqlite://db.sqlite?mode=rwc").await.unwrap();
    let db_backend = connection.get_database_backend();
    let stat = Statement::from_string(db_backend, "
    Create Table if not exists users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT
    )
    ");

    connection.execute(stat).await.unwrap();
    let user1 = UserModel{
        name:Set("burak".to_string()),
        ..Default::default()
    };
    user1.insert(&connection).await.unwrap();    
    let user2 = UserModel{
        name:Set("efe".to_string()),
        ..Default::default()
    };
    user2.insert(&connection).await.unwrap();    

    let all_users = users::find().all(&connection).await.unwrap();

    println!("all users {:?}",all_users);
    println!();

    let user_by_id = users::find_by_id(1).one(&connection).await.unwrap();
    println!("user by id {:?}",user_by_id);
    let mut user_by_id2:UserModel = users::find_by_id(2).one(&connection).await.unwrap().unwrap().into();
    user_by_id2.name=Set("ateş".to_string());
    user_by_id2.update(&connection).await.unwrap();
    
    let all_users: Vec<user::Model> = users::find().all(&connection).await.unwrap();
    println!("all users {:?}",all_users);
    println!();

    user_by_id.unwrap().delete(&connection).await.unwrap();
    
    let all_users: Vec<user::Model> = users::find().all(&connection).await.unwrap();
    println!("all users {:?}",all_users);
    println!();

}
