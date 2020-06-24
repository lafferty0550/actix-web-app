use mongodb::Collection;
use std::sync::Arc;

mod categories;
use categories::CategoriesService;

pub struct Services {
    pub categories: CategoriesService,
}

impl Services {
    pub fn new(categories: Arc<Collection>) -> Self {
        Self {
            categories: CategoriesService::new(categories),
        }
    }
}