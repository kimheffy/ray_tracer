use crate::color::Color;

struct Canvas {
    width: usize,
    height: usize,
    data: Vec<Vec<Color>>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        let data = vec![vec![Color::new(0.0, 0.0, 0.0); width]; height];
        Self {
            width,
            height,
            data,
        }
    }

    fn to_ppm(&self) -> String {
        let mut s = format!("\nP3\n{} {}\n255\n", self.width, self.height);

        for (row_index, row) in self.data.iter().enumerate() {
            for (col_index, element) in row.iter().enumerate() {
                let scaled = element.clone() * 255;

                let red = (scaled.red.ceil()).clamp(0.0, 255.0);
                let green = (scaled.green.ceil()).clamp(0.0, 255.0);
                let blue = (scaled.blue).ceil().clamp(0.0, 255.0);

                s += &format!("{} {} {}", red, green, blue);

                if col_index < row.len() - 1 {
                    s += " ";
                }
            }
            s += "\n";
        }

        s
    }

    fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        for (row_index, row) in self.data.iter_mut().enumerate() {
            for (col_index, element) in row.iter_mut().enumerate() {
                if y == row_index && x == col_index {
                    *element = color.clone();
                }
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

    assert_eq!(ppm, format!("\nP3\n5 3\n255"));
}

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
