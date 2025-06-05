use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Todo{
    pub id: u32,
    pub title: String,
    pub done: bool,
    pub user_email:String,
}


impl Todo {
    pub fn add_todo(title:String, done: bool, email: String, todos: &mut Vec<Todo>) -> Todo{

        let id = (todos.len() + 1) as u32;
        let todo = Todo{
            id,
            done,
            title,
            user_email: email,
        };

        todos.push(todo.clone());
        todo
    }

    pub fn get_user_todos(email:String, todos: &mut Vec<Todo>) -> Vec<Todo>{
        let mut user_todos = vec![];

        for todo in todos{
            if todo.user_email == email {
                user_todos.push(todo.clone());
            }
        }

        user_todos
    }

    pub fn get_todo(id:u32, todos: &Vec<Todo>) -> Option<Todo>{
        let res = todos.iter().find(|t|t.id == id);
        match res {
            Some(val) => Some(val.clone()),
            None => None,
        }
    }

    pub fn update_todo(id:u32, email:String, title:String, done:bool, todos:&mut Vec<Todo>) -> Result<String, String>{
        let existing_todo = Todo::get_todo(id, todos);

        if existing_todo.is_none(){
            return Err(String::from("Enter Valid todo id"));
        }

        let existing_todo = existing_todo.unwrap();

        if existing_todo.user_email != email {
            return Err(String::from("UNAUTHORISED"));
        }
        
        for todo in todos {
            if todo.id == id {
                todo.done = done;
                todo.title = title.clone();
            }
        }

        Ok(String::from("Updated Successfully"))


    }
    
}