use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware::Next, Error, HttpMessage};

use crate::{errors::AppError, utils::decode_token};

pub async fn middleware(req:ServiceRequest, next:Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error>{

    let headers = req.headers();

    let header = headers.get("Authorization");

    if header.is_none(){
        return Err(AppError::TokenNotFound.into());
    }

    let token_res = header.unwrap().to_str();

    if token_res.is_err(){
        return Err(AppError::InternalError.into());
    }

    let token = token_res.unwrap();


    let decoded = decode_token(token);

    if decoded.is_err(){
        return Err(AppError::InternalError.into());
    }

    let email = decoded.unwrap();

    req.extensions_mut().insert(email);

    next.call(req).await

}