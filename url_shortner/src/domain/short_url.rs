use std::time::SystemTime;

use sqlx::types::time::OffsetDateTime;

#[derive(Clone)]
pub struct ShortUrl{
    pub code: String,
    pub original_url: String,
    pub created_at: OffsetDateTime,
}

impl ShortUrl{
    pub fn new(code:String,original_url:String)->Self{
        Self { 
            code, 
            original_url, 
            created_at: OffsetDateTime::now_utc(), 
        }
    }
}