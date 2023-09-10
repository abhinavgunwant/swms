use std::slice::Iter;
use serde::{ Serialize, Deserialize };
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref RE: Regex = Regex::new(
        r"\.(png|gif|jpg|jpeg|webp|tif|bmp|raw|cr2|nef|orf|sr2|eps|svg)$"
    ).unwrap();
}

/// The integer values associated with enum variants are categorized according
/// the ranges below:
/// 0 - 19: Raw Images
/// 20 - 39: Images with Lossless compression
/// 40 - 59: Images with Lossy compression
/// 60 - 79: Images that can have both lossy and lossless compression
/// 80+: Vector graphics images
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Encoding {
    NONE = 0,
    TIF = 1,
    BMP = 2,
    RAW = 3,
    CR2 = 4,
    NEF = 5,
    ORF = 6,
    SR2 = 7,

    PNG = 20,
    GIF = 21,

    JPG = 40,

    WEBP = 60,

    EPS = 80,
    SVG = 81
}

impl Encoding {
    /// Inits Encoding instance from the input text
    ///
    /// ### Example:
    /// ```rust
    /// assert(Encoding::from("example.jpg"), Encoding::JPG);
    /// ```
    pub fn from(text: &str) -> Self {
        if text.is_empty() { return Encoding::NONE; }

        match RE.captures(text) {
            Some(captures) => {
                match captures.get(0) {
                    Some(encoding) => {
                        match encoding.as_str() {
                            ".tif" => Encoding::TIF,
                            ".bmp" => Encoding::BMP,
                            ".raw" => Encoding::RAW,
                            ".cr2" => Encoding::CR2,
                            ".nef" => Encoding::NEF,
                            ".orf" => Encoding::ORF,
                            ".sr2" => Encoding::SR2,
                            ".png" => Encoding::PNG,
                            ".gif" => Encoding::GIF,
                            ".jpg" => Encoding::JPG,
                            ".webp" => Encoding::WEBP,
                            ".eps" => Encoding::EPS,
                            ".svg" => Encoding::SVG,
                            &_ => Encoding::NONE,
                        }
                    }

                    None => { return Encoding::NONE; }
                }
            }

            None => { return Encoding::NONE; }
        }
    }

    /// Returns `true` if the supplied text contains a valid extension.
    /// Returns `false' otherwise.
    pub fn match_extension(text: &str) -> bool {
        if text.is_empty() { return false; }

        RE.is_match(text)
    }

    /// Returns an iterator to the enum variants.
    pub fn iter() -> Iter<'static, Encoding> {
        static ENCODINGS: [Encoding; 14] = [
            Encoding::TIF,
            Encoding::BMP,
            Encoding::RAW,
            Encoding::CR2,
            Encoding::NEF,
            Encoding::ORF,
            Encoding::SR2,
            Encoding::PNG,
            Encoding::GIF,
            Encoding::JPG,
            Encoding::WEBP,
            Encoding::EPS,
            Encoding::SVG,
            Encoding::NONE,
        ];

        ENCODINGS.iter()
    }

    /// Returns the extension for a particular image encoding.
    pub fn extension(&self) -> String {
        use Encoding::*;

        match *self {
            NONE => String::new(),
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

    /// Returns the mime type for an Encoding.
    pub fn mime_type(&self) -> String {
        use Encoding::*;

        match *self {
            NONE => String::new(),
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

