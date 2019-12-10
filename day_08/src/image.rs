use crate::color::Color;
use crate::layer::Layer;
use crate::{IMAGE_HEIGHT, IMAGE_WIDTH};

#[derive(Clone)]
pub struct Image {
    pub layers: Vec<Layer>,
}

impl Image {
    pub fn render(&self) -> Layer {
        let mut render = [[Color::Transparent; IMAGE_WIDTH]; IMAGE_HEIGHT];

        for (y, row) in render.iter_mut().enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                *pixel = self.render_pixel(0, x, y);
            }
        }

        Layer::new(render)
    }

    fn render_pixel(&self, layer: usize, x: usize, y: usize) -> Color {
        match self.layers[layer].matrix[y][x] {
            Color::Black => Color::Black,
            Color::White => Color::White,
            Color::Transparent => {
                if layer + 1 == self.layers.len() {
                    return Color::Transparent;
                }

                self.render_pixel(layer + 1, x, y)
            }
        }
    }
}

impl IntoIterator for Image {
    type Item = Layer;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.layers.into_iter()
    }
}

impl From<String> for Image {
    fn from(string: String) -> Self {
        let num_of_layers = string.len() / (IMAGE_WIDTH * IMAGE_HEIGHT);
        let mut layers: Vec<Layer> = Vec::with_capacity(num_of_layers);
        let mut remaining = string;

        for _ in 0..num_of_layers {
            let mut layer = [[Color::Transparent; IMAGE_WIDTH]; IMAGE_HEIGHT];

            for row in layer.iter_mut().take(IMAGE_HEIGHT) {
                let temporary_row: Vec<Color> = remaining
                    .split_off(remaining.len() - IMAGE_WIDTH)
                    .chars()
                    .map(|character| match character {
                        '0' => Color::Black,
                        '1' => Color::White,
                        '2' => Color::Transparent,
                        _ => panic!("invalid pixel value: {}", character),
                    })
                    .collect();

                row.copy_from_slice(&temporary_row[..IMAGE_WIDTH]);
            }

            layer.reverse();
            layers.push(Layer::new(layer));
        }

        layers.reverse();

        Self { layers }
    }
}
