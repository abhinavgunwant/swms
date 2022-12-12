# dam

This is a prototype "Digital Asset Manager".

Contains `dam-fe` a.k.a the "Dam Frontend" and `image-api` a.k.a the "Dam Backend" folders.

## Building and executing

`cargo run` to run a development build
`cargo watch -x 'run'` to run app in "watch mode" (auto reloading).


WORK IN PROGRESS!

example file path:

/product-images/books/ebooks/ebook-cover1.jpg

Here:
- `product-images` is a project slug
- `/books/ebooks` is the image path
- `ebook-cover1` is the image slug
- `jpg` is the desired image encoding (This is an optional feature)

## Setting up
1. Create `image-rendition-cache` and `image-uploads` folders.
2. Run ddl and dml SQL scripts.

