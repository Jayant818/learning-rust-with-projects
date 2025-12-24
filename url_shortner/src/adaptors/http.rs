use std::collections::HashMap;

use actix_web::{HttpResponse, Responder, web};
use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{app_state::AppState, application::{AppError, AppResult, UrlRepository}, domain::ShortUrl};

// temporary In-memory adaptor 
pub struct InMemoryRepo{
    store: RwLock<HashMap<String,ShortUrl>>
}

impl InMemoryRepo {
    pub fn new()->Self{
        Self { 
            store: RwLock::new(HashMap::new())
        }
    }
}

#[async_trait]
impl UrlRepository for InMemoryRepo{
    async fn save(&self, short_url:ShortUrl)->AppResult<()>{
        let data = self.
        store.
        write().
        await.
        insert(short_url.code.clone(), short_url);

        Ok(())
    }

    async fn find_by_code(&self,code:&str)->AppResult<ShortUrl>{
        self
            .store
            .read()
            .await
        .get(code)
        .cloned()
        .ok_or(AppError::NotFound) 
    }   
}

pub async fn create(state:web::Data<AppState>,body:String)->impl Responder{
    match state.url_service.create(body).await {
        Ok(short)=>{
            HttpResponse::Ok().body(short.code)
        },
        Err(_)=>HttpResponse::BadRequest().finish()
    }
}

pub async fn resolve(state:web::Data<AppState>,path:web::Path<String>)->impl Responder{
    println!("Hello");
    match state.url_service.resolve(&path).await {
        Ok(url)=> HttpResponse::Found()
            .append_header(("Location",url))
            .finish(),
        Err(_)=>HttpResponse::NotFound().finish()
    }
}