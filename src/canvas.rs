#![allow(dead_code, unused_variables)]
use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![vec![Color::new(0.0, 0.0, 0.0); width]; height];
        Self {
            width,
            height,
            data,
        }
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm: String = String::new();

        // ppm header
        ppm.push_str("P3\n");
        ppm.push_str(&format!("{} {}\n", self.width, self.height));
        ppm.push_str("255\n");

        // pixel data
        for (row_index, row) in self.data.iter().enumerate() {
            let mut line = String::new();

            for color in row.clone() {
                let scaled = color * 255;
                let red = scaled.red.ceil().clamp(0.0, 255.0);
                let pixel = format!("{} ", red);
                if line.chars().count() + pixel.chars().count() > 70 {
                    ppm.push_str(&line.trim_end());
                    ppm.push('\n');
                    line.clear();
                }
                line.push_str(&pixel);

                let green = scaled.green.ceil().clamp(0.0, 255.0);
                let pixel = format!("{} ", green);
                if line.chars().count() + pixel.chars().count() > 70 {
                    ppm.push_str(&line.trim_end());
                    ppm.push('\n');
                    line.clear();
                }
                line.push_str(&pixel);

                let blue = scaled.blue.ceil().clamp(0.0, 255.0);
                let pixel = format!("{} ", blue);
                if line.chars().count() + pixel.chars().count() > 70 {
                    ppm.push_str(&line.trim_end());
                    ppm.push('\n');
                    line.clear();
                }
                line.push_str(&pixel);
            }

            if !line.is_empty() {
                ppm.push_str(&line.trim_end());
                ppm.push('\n')
            }
        }

        ppm
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        for (row_index, row) in self.data.iter_mut().enumerate() {
            for (col_index, element) in row.iter_mut().enumerate() {
                if y == row_index && x == col_index {
                    *element = color.clone();
                }
            }
        }
    }

    fn fill_every_with(&mut self, default_color: Color) {
        for (row_index, row) in self.data.iter_mut().enumerate() {
            for (col_index, element) in row.iter_mut().enumerate() {
                *element = default_color.clone();
            }
        }
    }
}

#[test]
fn creating_canvas() {
    let c = Canvas::new(10, 20);

    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);

    for (row_index, row) in c.data.iter().enumerate() {
        for (col_index, element) in row.iter().enumerate() {
            assert_eq!(*element, Color::new(0.0, 0.0, 0.0));
        }
    }
}

#[ignore = "no need to create ppm header"]
#[test]
fn constructing_ppm_header() {
    let c = Canvas::new(5, 3);
    let ppm = c.to_ppm();

    assert_eq!(ppm, format!("P3\n5 3\n255"));
}

#[ignore]
#[test]
fn constructing_ppm_pixel_data() {
    let mut c = Canvas::new(5, 3);

    let c1 = Color::new(1.5, 0.0, 0.0);
    let c2 = Color::new(0.0, 0.5, 0.0);
    let c3 = Color::new(-0.5, 0.0, 1.0);

    c.write_pixel(0, 0, c1);
    c.write_pixel(2, 1, c2);
    c.write_pixel(4, 2, c3);

    let ppm = c.to_ppm();

    assert_eq!(
        String::from(
            "
P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
"
        ),
        ppm
    );
}

#[test]
fn splitting_long_lines_ppm_file() {
    let mut c = Canvas::new(10, 2);

    c.fill_every_with(Color::new(1.0, 0.8, 0.6));

    let ppm = c.to_ppm();

    assert_eq!(
        String::from(
            "
P3
10 2
255
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
"
        ),
        ppm
    );
}
