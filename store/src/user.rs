use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User{
    pub email: String,
    pub name: String,
    pub password:String,
}

impl User {

    pub fn get_user(users: &Vec<User> , email: &String) -> Option<User>{
        let user_exist = users.iter().find(|u| u.email == *email);

        if user_exist.is_none(){
            return None;
        }

        let user = user_exist.unwrap().clone();

        Some(user)
    }

    pub fn add_user(users: &mut Vec<User>, user:&User) -> Result<String, String>{

        let existing_user_res = User::get_user(users, &user.email);

        match existing_user_res {
            Some(_val) => {
                return Err(String::from("User exists already"));
            },
            None => {
                users.push(user.clone());
                return Ok(String::from("User created Successfully"));
            }
        }

    }
}