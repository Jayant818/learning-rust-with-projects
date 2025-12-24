use async_trait::async_trait;

use crate::{application::AppResult, domain::ShortUrl};

#[async_trait]
pub trait UrlRepository: Send + Sync{
    async fn save(&self, short_url:ShortUrl)->AppResult<()>;
    async fn find_by_code(&self,code:&str)->AppResult<ShortUrl>;
}

#[async_trait]
pub trait UrlCache: Send + Sync {
    async fn get(&self,code:&str)->Option<String>;
    async fn set(&self,code:&str,url:&str,ttl_seconds:usize);
}