struct User{
    id:u16,
    name:String,
    age:u16
}

trait UserController{
    fn save_user(id:u16,name:String,age:u16)->User;
    fn update_user(&mut self,user:User);
}

impl UserController for User  {
    fn save_user(id:u16,name:String,age:u16)->User{

        println!("User saved: {} (ID: {})", name, id);
        User { id, name, age }
    }
    fn update_user(&mut self,user:User){
        self.id = user.id;
        self.name = user.name;
        self.age = user.age;
        println!("user name {} user age {}",self.name,self.age)
    }

}

fn main() {

 let mut kullanıcı1 = User::save_user( 1, String::from("burak"), 21);
 kullanıcı1.update_user(User{id:1,name:String::from("efe"),age:19});

 println!("kullanıcı adı {}",kullanıcı1.name);

}
