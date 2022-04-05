mod controller;
pub mod error;
mod interface_server;
mod packer;
use crate::error::NetXFFIError;
use std::sync::Arc;

use crate::controller::ClientController;
use crate::interface_server::{IServer, ___impl_IServer_call};
use crate::packer::{LogOn,CUser};
use anyhow::Result;
use interoptopus::patterns::string::AsciiPointer;
use interoptopus::{
    callback, ffi_function, ffi_service, ffi_service_ctor, ffi_type,ffi_service_method, function, pattern, Inventory,
    InventoryBuilder,
};
use interoptopus::patterns::api_guard::APIVersion;
use interoptopus::patterns::slice::FFISlice;
use log::LevelFilter;
use netxclient::client::{DefaultSessionStore, NetXClient, ServerOption};
use netxclient::prelude::*;
use tokio::runtime::Runtime;


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

callback!(LogOnCallBack(success: bool, msg: AsciiPointer)->bool);
callback!(GetUsersCallBack(users:FFISlice<CUser>));

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
        let res:Result<bool>=self.runtime.block_on(async move {
            let server: Box<dyn IServer> = impl_interface!(self.client=>IServer);
            let res = server
                .login(LogOn {
                    nickname: nickname.as_str()?.to_string(),
                })
                .await?;
            Ok(callback.call(
                res.success,
                AsciiPointer::from_slice_with_nul(cstr!(res.msg).as_bytes())?,
            ))
        });
        match res{
            Ok(v)=>v,
            Err(err)=>{
                log::error!("error:{}",err);
                false
            }
        }
    }

    /// get all online users
    pub fn get_users(&self,callback:GetUsersCallBack)->Result<()>{
        self.runtime.block_on(async move {
            let server: Box<dyn IServer> = impl_interface!(self.client=>IServer);
            let res = server
                .get_users()
                .await?;
            let users=res.into_iter().map(|p|{
                (cstr!(p.nickname),p.sessionid)
            }).collect::<Vec<_>>();
            let c_user= users.iter().map(|(nickname,session_id)|{
                (nickname.as_str(),session_id).into()
            }).collect::<Vec<_>>();
            callback.call(FFISlice::from_slice(&c_user));
            Ok(())
        })
    }


    //----------------------------
}


pub fn inventory() -> Inventory {
    InventoryBuilder::new()
        .register(function!(my_api_guard))
        .register(pattern!(MessageClient))
        .inventory()
}
