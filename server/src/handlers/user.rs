use actix_web::{post, web::{Data, Json}, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use store::user::{User};

use crate::{utils::{generate_jwt_token, get_hashed_password, verify_password}, GlobalState};

#[derive(Deserialize, Serialize)]
pub struct SigninInput {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppResponse{
    pub data:String
}

#[post("/signup")]
async fn signup(data:Data<GlobalState>, input: Json<User>) -> impl Responder {

    let state_result = data.overall_state.lock();

    if state_result.is_err(){
        return HttpResponse::InternalServerError().json(AppResponse{data:String::from("Internal Error")});
    }

    let mut state = state_result.unwrap();
    let users = &mut state.users;

    let hashed_password_res = get_hashed_password(&input.password);

    if hashed_password_res.is_err(){
        println!("error while hashing password");
        return HttpResponse::InternalServerError().json(AppResponse{data:String::from("Internal Error")});
    }

    let user = User { 
        email: input.email.clone(), 
        name: input.name.clone(), 
        password:hashed_password_res.unwrap(), 
    };

    let res = store::user::User::add_user(users, &user);
    
    match res {
        Ok(val) => HttpResponse::Ok().json(AppResponse{data:val}),
        Err(e) => HttpResponse::BadRequest().json(AppResponse{data:e}),
    }

}

#[post("/signin")]
async fn signin(data: Data<GlobalState>, input:Json<SigninInput>) -> impl Responder {

    let state_result = data.overall_state.lock();

    if state_result.is_err(){
        return HttpResponse::InternalServerError().json(AppResponse{data:String::from("Internal Error")});
    }

    let mut state = state_result.unwrap();
    let users = &mut state.users;

    let res = store::user::User::get_user(users, &input.email);

    if res.is_none() {
        return HttpResponse::BadRequest().json(AppResponse{data:String::from("Signup first")});
    }

    let user = res.unwrap();

    let verify_res = verify_password(&user.password, &input.password);

    if verify_res == false{
        return HttpResponse::BadRequest().json(AppResponse{data:String::from("Enter valid Password")});
    }

    let token_res = generate_jwt_token(input.email.clone());

    if token_res.is_err(){
        return HttpResponse::InternalServerError().json(AppResponse{data:String::from("Internal Error")});
    }
    
    let token = token_res.unwrap();

    HttpResponse::Ok().json(AppResponse{data:token})

}


#[cfg(test)]
mod tests{
    use actix_web::test::{self, TestRequest};
    use store::user::User;

    use crate::{handlers::user::{AppResponse, SigninInput}, init_app, prepare_global_state};


    #[actix_web::test]
    pub async fn should_signup(){
        let state = prepare_global_state();
        let app = init_app!(state);
        let app = test::init_service(app).await;

        let input = User{
            email:"vk1@gmail.com".to_string(),
            name:"VK".to_string(),
            password:"Random123".to_string(),
        };

        let res = TestRequest::post().uri("/user/signup").set_json(input).send_request(&app).await;
        let res: AppResponse = actix_web::test::read_body_json(res).await;
        assert_eq!(res.data, String::from("User created Successfully"));
    }   

    #[actix_web::test]
    pub async fn should_signin(){
        let state = prepare_global_state();
        let app = init_app!(state);
        let app = test::init_service(app).await;

        let input = User{
            email:"vk2@gmail.com".to_string(),
            name:"VK".to_string(),
            password:"Random1234".to_string(),
        };

        let res = TestRequest::post().uri("/user/signup").set_json(input).send_request(&app).await;
        let res: AppResponse = actix_web::test::read_body_json(res).await;

        assert_eq!(res.data, String::from("User created Successfully"));

        let input = SigninInput{
            email:"vk2@gmail.com".to_string(),
            password:"Random1234".to_string(),
        };

        let res = TestRequest::post().uri("/user/signin").set_json(input).send_request(&app).await;
        assert!(res.status().is_success());

    }

    #[actix_web::test]
    pub async fn should_not_signin(){
        let state = prepare_global_state();
        let app = init_app!(state);
        let app = test::init_service(app).await;

        let input = User{
            email:"vk3@gmail.com".to_string(),
            name:"VK".to_string(),
            password:"Random1234".to_string(),
        };

        let res = TestRequest::post().uri("/user/signup").set_json(input).send_request(&app).await;
        let res: AppResponse = actix_web::test::read_body_json(res).await;

        assert_eq!(res.data, String::from("User created Successfully"));

        let input = SigninInput{
            email:"vk3@gmail.com".to_string(),
            password:"INVALID_PASSWORD".to_string(),
        };

        let res = TestRequest::post().uri("/user/signin").set_json(input).send_request(&app).await;
        assert!(!res.status().is_success());


        let res: AppResponse = actix_web::test::read_body_json(res).await;
        assert_eq!(res.data, String::from("Enter valid Password"));
    }


}