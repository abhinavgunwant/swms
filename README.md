# swms: Simple Web Multimedia Server

A digital asset management server for storing and scaling images for the web.

swms allows you to create different "Renditions" in which an image can be
scaled down based on the target device screen size.

## How are images accessed

Images can publicly be accessed by any web application using this simple web API:

Here is an example URL: /api/image/product-images/books/ebooks/ebook-cover1.jpg

Here:
- `product-images` is a project slug
- `/books/ebooks` is the image path
- `ebook-cover1` is the rendition slug
- `jpg` is the image encoding

## Building and executing

This project is still in it's early phase, in order to use it you need to:

`cargo run` to run a development build
`cargo watch -x 'run'` to run app in "watch mode" (auto reloading).

### Setting up
1. Create `image-rendition-cache` and `image-uploads` folders.
2. Run SQL scripts in the following order:
    1. ddl.sql
    2. dml.sql
    3. con.sql

