use crate::packer::{LogOn, LogOnRes, User};
use anyhow::Result;
use netxclient::prelude::*;

//服务器接口,调用服务器需要使用它
//server interface,it is required to call the server
#[build]
pub trait IServer {
    #[tag(1000)]
    async fn login(&self, msg: LogOn) -> Result<LogOnRes>;
    #[tag(1001)]
    async fn get_users(&self) -> Result<Vec<User>>;
    #[tag(1002)]
    async fn talk(&self, msg: &str) -> Result<()>;
    #[tag(1003)]
    async fn to(&self, target: &str, msg: &str) -> Result<()>;
    #[tag(1004)]
    async fn ping(&self, target: &str, time: i64) -> Result<i64>;
}
