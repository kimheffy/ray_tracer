use crate::color::Color;

struct Canvas {
    width: usize,
    height: usize,
    data: Vec<Vec<Color>>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        let data = vec![vec![Color::new(0.0, 0.0, 0.0); height]; width];
        Self {
            width,
            height,
            data,
        }
    }

    fn to_ppm(&self) -> String {
        format!("\nP3\n{} {}\n255", self.width, self.height)
    }

    fn write_pixel(&mut self, width: usize, height: usize, color: Color) {
        for (row_index, row) in self.data.iter_mut().enumerate() {
            for (col_index, element) in row.iter_mut().enumerate() {
                if width == row_index && height == col_index {
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
}
