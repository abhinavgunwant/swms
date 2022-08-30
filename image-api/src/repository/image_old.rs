
// use num_derive::FromPrimitive;
// mod repostory::rendition::Rendition;

// /**
//  * The integer values associated with enum variants are categorized according
//  * the ranges below:
//  * 0 - 19: Raw Images
//  * 20 - 39: Images with Lossless compression
//  * 40 - 59: Images with Lossy compression
//  * 60 - 79: Images that can have both lossy and lossless compression
//  * 80+: Vector graphics images
//  */
// // moved to repository/image/encoding.rs
// enum Encoding {
//     TIF = 0,
//     BMP = 1,
//     RAW = 2,
//     CR2 = 3,
//     NEF = 4,
//     ORF = 5,
//     SR2 = 6,

//     PNG = 20,
//     GIF = 21,

//     JPG = 40,

//     WEBP = 60

//     EPS = 80,
//     SVG = 81
// }

// struct Image {
//     name: String, // name vs title in metadata?
//     id: u32,
//     encoding: Encoding,
//     height: u16,
//     width: u16,
//     metadata_id: u32,
//     slug: String
// }

// trait ImageItem : Item {
//     fn renditions(&self) -> Vec<rendition::Rendition>;
//     fn renditions(&self, device: String) -> Vec<rendition::Rendition>;
//     fn rendition(&self, width: u32) -> rendition::Rendition;
//     fn rendition(&self, name: String) -> rendition::Rendition;
//     fn metadata(&self) -> Metadata;
// }

// impl ImageItem for Image {
//     fn renditions(&self) -> Vec<rendition::Rendition> {
//         let mut renditions = Vec::new();

//         renditions.push(Rendition { 0, 0, 0, 0, "Rendition 1".to_string()});
//         renditions.push(Rendition { 1, 0, 0, 0, "Rendition 2".to_string()});
//         renditions.push(Rendition { 2, 0, 0, 0, "Rendition 3".to_string()});
//         renditions.push(Rendition { 3, 0, 0, 0, "Rendition 4".to_string()});

//         return renditions;
//     }

//     fn renditions(&self, device: String) -> Vec<rendition::Rendition> {
//         return self.get_renditions();
//     }

//     fn rendition(&self, width: u32) -> rendition::Rendition {
//         return self.get_renditions();
//     }

//     fn rendition(&self, name: String) -> rendition::Rendition {
//         return self.get_renditions();
//     }

//     fn metadata(&self) -> Metadata {
//         return Metadata {
//             0, 0, "Test Image!", "This is test!", "Test Author!"
//         };
//     }
// }

// impl Item for Image {
//     fn id(&self) -> u32 {
//         return self.id;
//     }

//     fn slug(&self) -> String {
//         return self.slug;
//     }

//     fn created_on(&self) -> DateTime {
//         return self.createdOn;
//     }

//     fn created_by(&self) -> u16 {
//         return self.createdOn;
//     }

//     fn modified_on(&self) -> DateTime {
//         return self.createdOn;
//     }

//     fn modified_by(&self) -> u16 {
//         return self.createdOn;
//     }
// }

// struct ImageRepository {
//     cache: Vec<Image>;

//     // Stores rank of each cached item in nth position. If rank is above max
//     // threshold, item is dropped from the cache.
//     cache_rank: Vec<u8>;
// }

// impl Repository for ImageRepository {
//     /**
//      * Gets the image corresponding to the id supplied.
//      */
//     fn get(id: u32) -> Item {
//         // TODO: Implement this with database code.
        
//         return Image {
//             name: "Image 1",
//             id: 0,
//             encoding: Encoding::JPG,
//             height: 100,
//             width: 100,
//             metadata_id: 0,
//             slug: "image-1"
//         };
//     }

//     /**
//      * Returns a vector of all the images.
//      */
//     fn get_all(&self) -> Vec<Item> {
//         let mut images = Vec::new();

//         images.push(self.get(0));

//         return images;
//     }

//     /**
//      * Returns paginated results of images as vectors.
//      */
//     fn get_all(&self, page: u32, page_length: u32) -> Vec<Item> {
//         let mut images = Vec::new();

//         images.push(self.get(0));

//         return images;
//     }

//     /**
//      * Adds image to the database.
//      */
//     fn add(&self, item: Item) {
//     }

//     /**
//      * Update the image in database.
//      */
//     fn update(&self, item: Item) {

//     }

//     /**
//      * Removes the image from the database.
//      */
//     fn remove(&self, item: Item) {

//     }

//     /**
//      * Removes the image from the database.
//      */
//     fn remove(&self, id: u32) {

//     }
// }
