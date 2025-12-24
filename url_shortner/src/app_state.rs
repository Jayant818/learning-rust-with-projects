use std::sync::{Arc, atomic::AtomicU64};

use crate::application::UrlService;

// This struct repersent All shared state of the application
// Anything that needs to be accessed by multiple requests live here
pub struct AppState{
    pub url_service:Arc<UrlService>
}

