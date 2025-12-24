use async_trait::async_trait;
use sqlx::{PgPool, types::time::OffsetDateTime};

use crate::{application::{AppError, AppResult, UrlRepository}, domain::ShortUrl};

pub struct PostgresUrlRepository{
    pool:PgPool
}

impl PostgresUrlRepository{
    pub fn new(pool:PgPool)->Self{
        Self{pool}
    }
}


#[async_trait]
impl UrlRepository for PostgresUrlRepository{
    async fn save(&self, short_url:ShortUrl)->AppResult<()>{

        sqlx::query!(
            r#"
            INSERT INTO short_urls (code,original_url,created_at)
            VALUES ($1,$2,$3)
            "#,
            short_url.code,
            short_url.original_url,
            short_url.created_at
        ).execute(&self.pool)
        .await
        .map_err(|_| AppError::Internal)?;

        Ok(())
    }

    async fn find_by_code(&self,code:&str)->AppResult<ShortUrl>{
        let record = sqlx::query!(
            r#"
                SELECT * FROM short_urls WHERE code = $1
            "#,
            code
        ).fetch_optional(&self.pool)
        .await
        .map_err(|_| AppError::NotFound)?;
    
        match record{
            Some(r)=>{
                return Ok(
                    ShortUrl{
                        code: r.code,
                        original_url: r.original_url,
                        created_at: r.created_at.into()
                    }
                )
            },
            None=>Err(AppError::NotFound),
        }
    }
}