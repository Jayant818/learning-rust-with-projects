use async_trait::async_trait;
use redis::{AsyncCommands, Client};

use crate::application::UrlCache;

pub struct RedisRepository{
    client:Client,
}

impl RedisRepository{
    pub fn new (url:&str)->Self{
        let client = redis::Client::open(url).expect("Invalid Redis Url");
        Self { client }
    }
}

#[async_trait]
impl UrlCache for RedisRepository{
    async fn get(&self,code:&str)->Option<String>{
        let mut conn = match self.client.get_multiplexed_async_connection().await {
            Ok(conn)=>{
                conn
            }
            Err(err)=>{
                // return Err("Error Getting Connection")
                return None;
            }
        };
        let data:Option<String> = conn.get(code).await.ok();
        return data;
    }

    async fn set(&self,code:&str,url:&str,ttl_seconds:usize){
        let mut conn = match self.client.get_multiplexed_async_connection().await{
            Ok(conn)=>{
                conn
            }
            Err(e)=>{
                return;
            }
        };
        let _:() = conn.set(code, url).await.unwrap();
    }

}
