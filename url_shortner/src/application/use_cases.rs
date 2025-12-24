use std::sync::Arc;

use nanoid::nanoid;

use crate::{application::{AppError, AppResult, UrlCache, UrlRepository}, domain::ShortUrl};

pub struct UrlService{
    repo: Arc<dyn UrlRepository>,
    cache: Arc<dyn UrlCache>
}

impl UrlService{
    pub fn new(repo:Arc<dyn UrlRepository>,cache:Arc<dyn UrlCache>)->Self{
        Self{
            repo,cache
        }
    }

    pub async fn create(&self,original_url:String)->AppResult<ShortUrl>{
        if !original_url.starts_with("http"){
            return Err(AppError::InvalidUrl);
        }

        let code = nanoid!(8);
        let short_url = ShortUrl::new(code, original_url);

        self.repo.save(short_url.clone()).await?;

        Ok(short_url)
    }

    pub async fn resolve(&self,code:&str)->AppResult<String>{
        // 1) Try Cache
        if let Some(data) = self.cache.get(code).await {
            return Ok(data)
        }

        // 2) Fallback to the DB
        let short_url = self.repo.find_by_code(code).await?;

        // Save to the cache 
        self.cache.set(code, &short_url.original_url, 3600).await;

        Ok(short_url.original_url)
    }
}