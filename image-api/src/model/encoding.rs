use serde::{ Serialize, Deserialize, Serializer };

/**
 * The integer values associated with enum variants are categorized according
 * the ranges below:
 * 0 - 19: Raw Images
 * 20 - 39: Images with Lossless compression
 * 40 - 59: Images with Lossy compression
 * 60 - 79: Images that can have both lossy and lossless compression
 * 80+: Vector graphics images
 */
#[derive(Copy, Clone, Deserialize)]
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

impl Encoding {
    pub fn extension(&self) -> String {
        use Encoding::*;

        match *self {
            TIF => String::from(".tif"),
            BMP => String::from(".bmp"),
            RAW => String::from(".raw"),
            CR2 => String::from(".cr2"),
            NEF => String::from(".nef"),
            ORF => String::from(".orf"),
            SR2 => String::from(".sr2"),
            PNG => String::from(".png"),
            GIF => String::from(".gif"),
            JPG => String::from(".jpg"),
            WEBP => String::from(".webp"),
            EPS => String::from(".eps"),
            SVG => String::from(".svg"),
        }
    }

    pub fn mime_type(&self) -> String {
        use Encoding::*;

        match *self {
            TIF => String::from("image/tif"),
            BMP => String::from("image/bmp"),
            RAW => String::from("image/raw"),
            CR2 => String::from("image/cr2"),
            NEF => String::from("image/nef"),
            ORF => String::from("image/orf"),
            SR2 => String::from("image/sr2"),
            PNG => String::from("image/png"),
            GIF => String::from("image/gif"),
            JPG => String::from("image/jpeg"),
            WEBP => String::from("image/webp"),
            EPS => String::from("image/eps"),
            SVG => String::from("image/svg"),
        }
    }
}

