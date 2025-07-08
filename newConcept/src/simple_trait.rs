trait UserApplications{
    fn save_user(&self) ->String;
}

struct User{
    user_name:String,
    user_age:u16
}

impl UserApplications for User {
    fn save_user(&self) ->String {
        format!("user name {} user age {}",self.user_name,self.user_age)
    }
}

fn main(){
    let kullanici = User{
        user_name:String::from("burak"),
        user_age:20,
    };
    println!("{}",kullanici.save_user());
}

