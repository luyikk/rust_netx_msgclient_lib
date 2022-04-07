mod controller;
pub mod error;
mod interface_server;
mod packer;
use crate::error::NetXFFIError;
use std::sync::Arc;

use crate::controller::ClientController;
use crate::interface_server::{IServer, ___impl_IServer_call};
use crate::packer::{LogOn, CUser};
use anyhow::Result;
use interoptopus::patterns::string::AsciiPointer;
use interoptopus::{
    callback, ffi_function, ffi_service, ffi_service_ctor, ffi_type,ffi_service_method, function, pattern, Inventory,
    InventoryBuilder,
};
use interoptopus::patterns::api_guard::APIVersion;
use interoptopus::patterns::slice::FFISlice;
use log::{debug, LevelFilter};
use netxclient::client::{DefaultSessionStore, NetXClient, ServerOption};
use netxclient::prelude::*;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;


#[ffi_function]
#[no_mangle]
pub extern "C" fn my_api_guard() -> APIVersion {
    inventory().into()
}
/// netx message lib
#[ffi_type(opaque)]
pub struct MessageClient {
    runtime: Runtime,
    client: NetxClientArc<DefaultSessionStore>,
}

#[macro_export]
macro_rules! cstr {
    ($str:expr) => {
        format!("{}\0", $str)
    };
}

callback!(LogOnCallBack(success: u8, msg: AsciiPointer)->bool);
callback!(GetUsersCallBack(users:FFISlice<CUser>));
callback!(PingCallBack(target: AsciiPointer,time:i64));

#[ffi_service(error = "NetXFFIError")]
impl MessageClient {
    /// new MessageClient obj
    /// config is json ServerOption
    #[ffi_service_ctor]
    pub fn new_by_config(config: AsciiPointer) -> Result<Self> {
        env_logger::Builder::default()
            .filter_level(LevelFilter::Debug)
            .init();
        let config = config.as_str()?;
        let server_config = serde_json::from_str::<ServerOption>(config)?;
        let runtime=tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;

        let client= runtime.block_on(async move{
            NetXClient::new(server_config, DefaultSessionStore::default())
        });

        Ok(Self {
            runtime,
            client,
        })
    }

    /// init
    pub fn init(&self) -> Result<()> {
        // let client= self.client.clone();
        self.runtime.block_on(async move {
            self.client
                .init(ClientController::new(Arc::downgrade(&self.client)))
                .await
        })
    }

    /// test connect
    pub fn connect_test(&self) -> std::result::Result<(), NetXFFIError> {
        match self
            .runtime
            .block_on(async move { self.client.connect_network().await })
        {
            Ok(_) => Ok(()),
            Err(_) => Err(NetXFFIError::NotConnect),
        }
    }

    //----------------------------
    /// login
    /// callback args:
    ///     success:bool
    ///     msg:string
    ///     ret:bool
    #[ffi_service_method(on_panic = "return_default")]
    pub fn login(&self, nickname: AsciiPointer, callback: LogOnCallBack) -> bool {
        let res=self.runtime.block_on(async move {
            let server: Box<dyn IServer> = impl_interface!(self.client=>IServer);
            server
                .login(LogOn {
                    nickname: nickname.as_str()?.to_string(),
                })
                .await
        });
        match res{
            Ok(v)=>{
                debug!("LogOn res:{:?}",v);
                callback.call(
                    if v.success{1}else{0},
                    AsciiPointer::from_slice_with_nul(cstr!(v.msg).as_bytes()).unwrap(),
                )
            },
            Err(err)=>{
                log::error!("error:{}",err);
                false
            }
        }
    }

    /// get all online users
    pub fn get_users(&self,callback:GetUsersCallBack)->Result<()>{
        let users= self.runtime.block_on(async move {
            let server: Box<dyn IServer> = impl_interface!(self.client=>IServer);
            server
                .get_users()
                .await
        })?;
        let users=users.into_iter().map(|p|{
            (cstr!(p.nickname),p.sessionid)
        }).collect::<Vec<_>>();
        let c_user= users.iter().map(|(nickname,session_id)|{
            (nickname.as_str(),session_id).into()
        }).collect::<Vec<_>>();
        callback.call(FFISlice::from_slice(&c_user));
        Ok(())
    }

    /// message to all online users
    pub fn talk(&self,msg:AsciiPointer)->Result<()>{
        self.runtime.block_on(async move{
            let server: Box<dyn IServer> = impl_interface!(self.client=>IServer);
            server.talk(msg.as_str()?).await
        })
    }

    /// message to target user
    pub fn to(&self,target: AsciiPointer, msg: AsciiPointer)->Result<()>{
        self.runtime.block_on(async move{
            let server: Box<dyn IServer> = impl_interface!(self.client=>IServer);
            server.to(target.as_str()?,msg.as_str()?).await
        })
    }

    /// ping
    pub fn ping(&self,target:AsciiPointer,time:i64,callback:PingCallBack)->Result<()> {
        let target = target.as_str()?.to_string();
        let client = self.client.clone();
        let _: JoinHandle<Result<()>> = self.runtime.spawn(async move {
            let server = impl_struct!(client=>IServer);
            let time = server.ping(&target, time).await?;
            callback.call(AsciiPointer::from_slice_with_nul(cstr!(target).as_bytes()).unwrap(), time);
            Ok(())
        });
        Ok(())

        // self.runtime.block_on(async move {
        //     let target = target.as_str()?;
        //     let server = impl_struct!(self.client=>IServer);
        //     let time = server.ping(&target, time).await?;
        //     callback.call(AsciiPointer::from_slice_with_nul(cstr!(target).as_bytes()).unwrap(), time);
        //     Ok(())
        // })
    }


    //----------------------------
}


pub fn inventory() -> Inventory {
    InventoryBuilder::new()
        .register(function!(my_api_guard))
        .register(pattern!(MessageClient))
        .inventory()
}
