mod jwt_auth;
mod password;
mod validator;

pub(crate) use jwt_auth::*;
pub(crate) use password::*;
pub(crate) use validator::*;


use chrono::Local;

pub fn current_timestamp() -> i64 {
    Local::now().timestamp()
}

pub fn i8_to_bool(v: i8) -> bool {
    match v {
        0 => false,
        1 => true,
        _ => panic!("Invalid bool in u8")
    }
}