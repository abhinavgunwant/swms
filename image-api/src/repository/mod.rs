pub mod image;
pub mod item;
mod metadata;
mod rendition;
mod user;

use item::Item;
use crate::db::DBImpl;
use image::db::mysql;

/**
 * Repository trait for all repositories
 */
pub trait Repository {
    // fn get(&self, id: u32) -> dyn Item; // TODO: change Item to Box<Item> ??
    fn get(&self, id: u32) -> Box::<dyn Item>; // TODO: change Item to Box<Item> ??
    fn get_all(&self) -> Vec::<Box::<dyn Item>>;
    fn get_all_paged(&self, page: u32, page_length: u32) -> Vec::<Box::<dyn Item>>;
    fn add(&self, item: Box::<dyn Item>);
    fn update(&self, item: Box::<dyn Item>);
    fn remove_item(&self, item: Box::<dyn Item>);
    fn remove(&self, id: u32);
}

pub fn get_image_repository () -> impl Repository {
    // TODO: Read config here to get the configured DB.

    let mut db = DBImpl::MYSQL;

    match db {
        DBImpl::MYSQL => mysql::ImageRepositoryMySQL {}
    }
}

impl<T: ?Sized> Repository for Box<T> where T: Repository {
    fn get(&self, id: u32) -> Box::<dyn Item> {
        (**self).get(id)
    }

    fn get_all(&self) -> Vec::<Box<dyn Item>> {
        (**self).get_all()
    }

    fn get_all_paged(&self, page: u32, page_length: u32) -> Vec::<Box::<dyn Item>> {
        (**self).get_all_paged(page, page_length)
    }

    fn add(&self, item: Box::<dyn Item>) {
        (**self).add(item)
    }

    fn update(&self, item: Box::<dyn Item>) {
        (**self).update(item)
    }

    fn remove_item(&self, item: Box::<dyn Item>) {
        (**self).remove_item(item)
    }

    fn remove(&self, id: u32) {
        (**self).remove(id)
    }
}

