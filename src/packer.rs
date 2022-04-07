use serde::{Deserialize, Serialize};
use interoptopus::{ffi_type};
use interoptopus::patterns::string::AsciiPointer;


#[derive(Deserialize, Serialize)]
#[repr(C)]
pub struct LogOn {
    pub nickname: String,
}

#[derive(Deserialize, Serialize, Default,Debug)]
#[repr(C)]
pub struct LogOnRes {
    pub success: bool,
    pub msg: String,
}

#[derive(Deserialize, Serialize, Clone,Default)]
pub struct User {
    pub nickname: String,
    pub sessionid: i64,
}

#[ffi_type(name="User")]
#[repr(C)]
pub struct CUser<'a>{
    pub nickname: AsciiPointer<'a>,
    pub session_id: i64,
}

impl<'a> From<(&'a str,&i64)> for CUser<'a> {
    fn from((nickname,session_id): (&'a str, &i64)) -> Self {
       Self{
           nickname: AsciiPointer::from_slice_with_nul(nickname.as_bytes()).unwrap(),
           session_id:*session_id
       }
    }
}