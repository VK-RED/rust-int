use actix_web::{get, post, put, web::{Data, Json, Path}, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{GlobalState};

#[derive(Deserialize, Serialize)]
pub struct CreateTodo{
    title: String,
    done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message{
    pub message:String
}

#[post("/todo")]
pub async fn create_todo(req:HttpRequest, data:Data<GlobalState>, input:Json<CreateTodo>) -> impl Responder{

    let email_ext = req.extensions().get::<String>().cloned();
    
    if email_ext.is_none(){
        return  HttpResponse::Unauthorized().json(String::from("UNAUTHORIZEDD"));
    }

    let email = email_ext.unwrap();

    let state_result = data.overall_state.lock();

    if state_result.is_err(){
        return HttpResponse::InternalServerError().json(String::from("Internal Server Error"));
    }

    let mut state = state_result.unwrap();
    let todos = &mut state.todos;

    let res = store::todo::Todo::add_todo(input.title.clone(), input.done, email, todos);

    HttpResponse::Ok().json(res)
}


#[put("/todo/{id}")]
pub async fn update_todo(req:HttpRequest, data:Data<GlobalState>, input:Json<CreateTodo>, path:Path<u32>) -> impl Responder {

    let email_ext = req.extensions().get::<String>().cloned();
    
    if email_ext.is_none(){
        return  HttpResponse::Unauthorized().json(String::from("UNAUTHORIZEDD"));
    }

    let email = email_ext.unwrap();

    let state_result = data.overall_state.lock();

    if state_result.is_err(){
        return HttpResponse::InternalServerError().json(String::from("Internal Server Error"));
    }

    let mut state = state_result.unwrap();
    let todos = &mut state.todos;

    let id = path.into_inner();

    let res = store::todo::Todo::update_todo(id, email, input.title.clone(), input.done, todos);

    match res {
        Ok(val) => HttpResponse::Ok().json(Message{message:val}),
        Err(e) => HttpResponse::BadRequest().json(Message{message:e})
    }

}

#[get("/todos")]
pub async fn get_todos(req:HttpRequest, data:Data<GlobalState>) -> impl Responder{

    let email_ext = req.extensions().get::<String>().cloned();
    
    if email_ext.is_none(){
        return  HttpResponse::Unauthorized().json(String::from("UNAUTHORIZEDD"));
    }

    let email = email_ext.unwrap();

    let state_result = data.overall_state.lock();

    if state_result.is_err(){
        return HttpResponse::InternalServerError().json(String::from("Internal Server Error"));
    }

    let mut state = state_result.unwrap();
    let todos = &mut state.todos;

    let res = store::todo::Todo::get_user_todos(email, todos);

    HttpResponse::Ok().json(res)
}


#[cfg(test)]
mod tests{
    use actix_web::{test::{self, TestRequest}};
    use store::{todo::Todo, user::User};

    use crate::{handlers::{todo::{CreateTodo, Message}, user::{AppResponse, SigninInput}}, init_app, prepare_global_state};

    #[actix_web::test]
    pub async fn should_create_todo(){
        let state = prepare_global_state();
        let app = init_app!(state);
        let app = test::init_service(app).await;

        let input = User{
            email:"vk4@gmail.com".to_string(),
            name:"VK".to_string(),
            password:"Random1234".to_string(),
        };

        let res = TestRequest::post().uri("/user/signup").set_json(input).send_request(&app).await;
        let res: AppResponse = actix_web::test::read_body_json(res).await;

        assert_eq!(res.data, String::from("User created Successfully"));

        let input = SigninInput{
            email:"vk4@gmail.com".to_string(),
            password:"Random1234".to_string(),
        };

        let res = TestRequest::post().uri("/user/signin").set_json(input).send_request(&app).await;
        assert!(res.status().is_success());

        let res: AppResponse = actix_web::test::read_body_json(res).await;
        let token = res.data;

        let todo = CreateTodo{
            title:"Go to Gym".to_string(),
            done:false,
        };

        let res = TestRequest::post()
        .uri("/authed/todo").set_json(todo)
        .append_header(("Authorization", token))
        .send_request(&app).await;

        let res : Todo = actix_web::test::read_body_json(res).await;
        assert_eq!(res.title, "Go to Gym");
        assert_eq!(res.done, false);

    }

    #[actix_web::test]
    pub async fn should_not_create_todo(){
        let state = prepare_global_state();
        let app = init_app!(state);
        let app = test::init_service(app).await;
        let todo = CreateTodo{
            title:"Go to Gym".to_string(),
            done:false,
        };

        let req = TestRequest::post()
        .uri("/authed/todo")
        .set_json(todo)
        .to_request();

        let res = test::try_call_service(&app, req).await;
        println!("{:?}", res);
        match res {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e.to_string(), String::from("Token Not found")),
        }
    }

    #[actix_web::test]
    pub async fn should_update_todo(){
        let state = prepare_global_state();
        let app = init_app!(state);
        let app = test::init_service(app).await;

        let input = User{
            email:"vk5@gmail.com".to_string(),
            name:"VK".to_string(),
            password:"Random1234".to_string(),
        };

        let res = TestRequest::post().uri("/user/signup").set_json(input).send_request(&app).await;
        let res: AppResponse = actix_web::test::read_body_json(res).await;

        assert_eq!(res.data, String::from("User created Successfully"));

        let input = SigninInput{
            email:"vk5@gmail.com".to_string(),
            password:"Random1234".to_string(),
        };

        let res = TestRequest::post().uri("/user/signin").set_json(input).send_request(&app).await;
        assert!(res.status().is_success());

        let res: AppResponse = actix_web::test::read_body_json(res).await;
        let token = res.data;

        let todo = CreateTodo{
            title:"Go to Gym".to_string(),
            done:false,
        };

        let res = TestRequest::post()
        .uri("/authed/todo").set_json(todo)
        .append_header(("Authorization", token.clone()))
        .send_request(&app).await;

        let data : Todo = actix_web::test::read_body_json(res).await;
        assert_eq!(data.title, "Go to Gym");
        assert_eq!(data.done, false);

        // Update the existing todo
        let todo =  CreateTodo{
            done:true,
            title:"Go to Gym".to_string(),
        };

        let uri = format!("/authed/todo/{}", data.id);

        let res = TestRequest::put()
        .uri(&uri).set_json(todo)
        .append_header(("Authorization", token))
        .send_request(&app).await;

        let res: Message = actix_web::test::read_body_json(res).await;
        assert_eq!(res.message, String::from("Updated Successfully"));

    }

    #[actix_web::test]
    pub async fn should_get_todos(){
        let state = prepare_global_state();
        let app = init_app!(state);
        let app = test::init_service(app).await;

        let input = User{
            email:"vk6@gmail.com".to_string(),
            name:"VK".to_string(),
            password:"Random1234".to_string(),
        };

        let res = TestRequest::post().uri("/user/signup").set_json(input).send_request(&app).await;
        let res: AppResponse = actix_web::test::read_body_json(res).await;

        assert_eq!(res.data, String::from("User created Successfully"));

        let input = SigninInput{
            email:"vk6@gmail.com".to_string(),
            password:"Random1234".to_string(),
        };

        let res = TestRequest::post().uri("/user/signin").set_json(input).send_request(&app).await;
        assert!(res.status().is_success());

        let res: AppResponse = actix_web::test::read_body_json(res).await;
        let token = res.data;

        let todo = CreateTodo{
            title:"Go to Gym".to_string(),
            done:false,
        };
        let todo2 = CreateTodo{
            title:"Go to Movie".to_string(),
            done:false,
        };

        TestRequest::post()
        .uri("/authed/todo").set_json(todo)
        .append_header(("Authorization", token.clone()))
        .send_request(&app).await;

        TestRequest::post()
        .uri("/authed/todo").set_json(todo2)
        .append_header(("Authorization", token.clone()))
        .send_request(&app).await;

        let res = TestRequest::get()
        .uri("/authed/todos")
        .append_header(("Authorization", token))
        .send_request(&app).await;

        let res: Vec<Todo> = actix_web::test::read_body_json(res).await;
        assert_eq!(res.len(), 2);
        
        assert_eq!(res[0].title, String::from("Go to Gym"));
        assert_eq!(res[0].done, false);
        assert_eq!(res[1].title, String::from("Go to Movie"));
        assert_eq!(res[1].done, false);

    }

}