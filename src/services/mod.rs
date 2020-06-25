use mongodb::Collection;
use std::sync::Arc;

mod categories;
use categories::CategoriesService;

mod images;
use images::ImagesService;

pub struct Services {
    pub categories: CategoriesService,
    pub images: ImagesService,
}

impl Services {
    pub fn new(categories: Arc<Collection>, images: Arc<Collection>) -> Self {
        Self {
            categories: CategoriesService::new(categories),
            images: ImagesService::new(images),
        }
    }
}