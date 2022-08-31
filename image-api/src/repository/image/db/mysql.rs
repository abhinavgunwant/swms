use crate::repository::item::Item;
use crate::repository::Repository;
use crate::repository::image::{ Image, Encoding };

use chrono::Utc;

pub struct ImageRepositoryMySQL {}

// TODO: implement all for MySQL
impl Repository for ImageRepositoryMySQL {
    fn get(&self, id: u32) -> Box::<dyn Item> {
        println!("Getting image with id: {}", id);

        Box::new(Image {
            name: "test".to_string(),
            id: 0,
            encoding: Encoding::JPG,
            height: 16,
            width: 16,
            metadata_id: 0,
            slug: "test".to_string(),
            created_by: 0,
            modified_by: 0,
            created_on: Utc::now(),
            modified_on: Utc::now()
        })
    }

    fn get_all(&self) -> Vec::<Box<dyn Item>> {
        println!("Getting all images.");

        let img = Box::new(Image {
            name: "test".to_string(),
            id: 0,
            encoding: Encoding::JPG,
            height: 16,
            width: 16,
            metadata_id: 0,
            slug: "test".to_string(),
            created_by: 0,
            modified_by: 0,
            created_on: Utc::now(),
            modified_on: Utc::now()
        });

        let mut vec = Vec::<Box<dyn Item>>::new();
        vec.push(img);

        return vec;
    }

    fn get_all_paged(&self, page: u32, page_length: u32) -> Vec::<Box::<dyn Item>> {
        println!("Getting all images, page: {}, page_length: {}.", page, page_length);

        return self.get_all();
    }

    fn add(&self, item: Box::<dyn Item>) {
        println!("Adding image item");
    }

    fn update(&self, item: Box::<dyn Item>) {
        println!("Updating image item");
    }

    fn remove_item(&self, item: Box::<dyn Item>) {
        println!("Removing image item");
    }

    fn remove(&self, id: u32) {
        println!("Removing image with id: {}", id);
    }
}
