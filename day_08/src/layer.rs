use crate::color::Color;
use crate::{IMAGE_HEIGHT, IMAGE_WIDTH};
use std::fmt;

#[derive(Clone)]
pub struct Layer {
    pub matrix: [[Color; IMAGE_WIDTH]; IMAGE_HEIGHT],
}

impl Layer {
    pub fn new(matrix: [[Color; IMAGE_WIDTH]; IMAGE_HEIGHT]) -> Self {
        Self { matrix }
    }

    pub fn how_many(&self, color: Color) -> usize {
        let mut counter = 0;

        for row in &self.matrix {
            for pixel in row {
                if pixel == &color {
                    counter += 1;
                }
            }
        }

        counter
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.matrix {
            for pixel in row {
                write!(
                    formatter,
                    "{}",
                    match pixel {
                        Color::Black => " ",
                        Color::White => "#",
                        Color::Transparent => " ",
                    }
                )?;
            }
            writeln!(formatter)?;
        }

        Ok(())
    }
}
