pub const IMAGE_WIDTH: usize = 25;
pub const IMAGE_HEIGHT: usize = 6;

mod color;
mod image;
mod layer;

use color::Color;
use common::input;
use image::Image;

fn main() {
    let image: Image = input::read_line().into();

    let mut current = usize::max_value();
    let mut index = 0;

    for (i, layer) in image.clone().into_iter().enumerate() {
        let zeros = layer.how_many(Color::Black);

        if zeros < current {
            current = zeros;
            index = i;
        }
    }

    println!(
        "[PART ONE] number of white pixels multiplied by number of transparent pixels: {}",
        image.layers[index].how_many(Color::White)
            * image.layers[index].how_many(Color::Transparent)
    );

    let rendered = image.render();
    println!("[PART TWO] rendered image");
    print!("{}", rendered);
}
