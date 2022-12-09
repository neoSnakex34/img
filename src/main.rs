use image::{self, GenericImageView, Rgba};

// fn open_image(path: &str) -> ImageBuffer{
//     Reader::open("maze.png")?.decode()?
// }

fn main() {
    
    let img = image::open("src/maze.png").unwrap();
    // println!("{:?}", img.get_pixel(10, 10));
     
    img
        .into_rgba8()
        .enumerate_pixels()
        .for_each(|(x, y, p)| {
            if *p == Rgba([255, 0, 0, 255]){
                println!("{:?}", (x, y));
            }
        })


}
