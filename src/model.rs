
#[allow(dead_code)]
pub struct User{
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
    pub email: String,
    pub age: u8,
}


#[allow(dead_code)]
impl User {
    pub fn say_hello(&self, name: &str){
        println!("Helo {}, my name is {}", name, self.first_name);
    }
}