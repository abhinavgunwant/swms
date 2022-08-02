use num_derive::FromPrimitive;

/**
 * 0 - 19: Raw Images
 * 20 - 39: Images with Lossless compression
 * 40 - 59: Images with Lossy compression
 * 60 - 79: Images that can have both lossy and lossless compression
 * 80+: Vector graphics images
 */
enum Encoding {
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

    WEBP = 60

    EPS = 80,
    SVG = 81
}

struct Image {
    name: String; // name vs title in metadata?
    id: u32;
    encoding: Encoding;
    height: u16;
    width: u16;
    metadata_id: u32;
    slug: String;
}
