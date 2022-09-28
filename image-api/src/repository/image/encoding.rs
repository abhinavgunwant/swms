use serde::ser::{ Serialize, Serializer };

/**
 * The integer values associated with enum variants are categorized according
 * the ranges below:
 * 0 - 19: Raw Images
 * 20 - 39: Images with Lossless compression
 * 40 - 59: Images with Lossy compression
 * 60 - 79: Images that can have both lossy and lossless compression
 * 80+: Vector graphics images
 */
#[derive(Copy, Clone)]
pub enum Encoding {
    TIF = 0,
    BMP = 1,
    RAW = 2,
    CR2 = 3,
    NEF = 4,
    ORF = 5,
    SR2 = 6,

    PNG = 20,
    GIF = 21,

    JPG = 40,

    WEBP = 60,

    EPS = 80,
    SVG = 81
}

impl Serialize for Encoding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {

        let t = *self;
        serializer.serialize_u8(t as u8)
    }
}
