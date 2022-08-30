pub mod image;
mod item;
mod metadata;
mod rendition;
mod user;

use item::Item;

/**
 * Repository trait for all repositories
 */
trait Repository {
    fn get(&self, id: u32) -> dyn Item; // TODO: change Item to Box<Item> ??
    fn get_all(&self) -> Vec<dyn Item>;
    fn get_all_paged(&self, page: u32, page_length: u32) -> Vec::<dyn Item>;
    fn add(&self, item: dyn Item);
    fn update(&self, item: dyn Item);
    fn remove_item(&self, item: dyn Item);
    fn remove(&self, id: u32);
}

// TODO: move it to `db` mod.
enum DBImpl {
    MYSQL
}
