extern crate image;

use image::GenericImageView;
fn main() {
    let img = image::open("puppy.jpeg").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());
    img.rotate90().save("puppy.jpeg").unwrap();
}
