/**
 * Repository trait for all repositories
 */

trait Repository {
    get(id: u32) -> Item;
    add(item: Item);
    update(item: Item);
    remove(id: Item);
    remove(id: u32);
}

