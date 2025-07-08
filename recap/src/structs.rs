pub struct SiteUser {
    pub UserName: String,
    pub UserAge: u16,
}

impl SiteUser {
    pub fn new(UserName: String, UserAge: u16) -> Self {
        Self {
            UserName,
            UserAge,
        }
    }

    pub fn set_user_name(&mut self, new_name: String) {
        self.UserName = new_name;
    }

    pub fn get_user_name(&self) -> &String {
        &self.UserName
    }
    pub fn get_user_age(&self) -> &u16{
        &self.UserAge
    }
}
