/*
  user
    POST /v1/user/signup  { username: <String>, passwd: <String> }
    POST /v1/user/login   { username <String>, passwd: <String> }
*/

use actix_web::{HttpRequest, Responder};
use super::super::db::user;

pub fn greet(req: HttpRequest) -> impl Responder {
    format!("Hello!")
}

pub fn signin(req: HttpRequest) -> impl Responder {
    if let (Some(username), Some(passwd)) = (req.match_info().get("username"), req.match_info().get("passwd")) {
        match user::verification(username, passwd) {
            Ok(_) => {
                format!("success")
            },
            Err(e) => {
                format!("username or passwd error")
            }
        }
    } else {
        format!("param error need 'username' 'passwd'")
    }
}