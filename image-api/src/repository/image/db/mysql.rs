use crate::repository::item::Item;
use crate::repository::Repository;
use crate::repository::image::{ Image, Encoding };

pub struct ImageRepositoryMySQL {}

// TODO: implement all for MySQL
impl Repository for ImageRepositoryMySQL {
    fn get(&self, id: u32) -> dyn Item {
        println!("Getting image with id: {}", id);

        Image {
            name: "test".to_string(),
            id: 0,
            encoding: Encoding::JPG,
            height: 16,
            width: 16,
            metadata_id: 0,
            slug: "test".to_string()
        }
    }

    fn get_all(&self) -> Vec::<dyn Item> {
        println!("Getting all images.");

        let img = Image {
            name: "test".to_string(),
            id: 0,
            encoding: Encoding::JPG,
            height: 16,
            width: 16,
            metadata_id: 0,
            slug: "test".to_string()
        };

        let mut vec = Vec::new();
        vec.push(img);

        return vec;
    }

    fn get_all(&self, page: u32, page_length: u32) -> Vec::<Item> {
        println!("Getting all images, page: {}, page_length: {}.", page, page_length);

        return self.get_all();
    }

    fn add(&self, item: dyn Item) {
        println!("Adding image item");
    }

    fn update(&self, item: dyn Item) {
        println!("Updating image item");
    }

    fn remove(&self, item: dyn Item) {
        println!("Removing image item");
    }

    fn remove(&self, id: u32) {
        println!("Removing image with id: {}", id);
    }
}
