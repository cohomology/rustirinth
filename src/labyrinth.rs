use ndarray;

pub const BOX_SIZE: i32 = 64;

#[derive(Debug)]
pub struct Labyrinth {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub x_box_cnt: i32,
    pub y_box_cnt: i32,
    pub marked: ndarray::ArrayD<bool>,
}

pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Labyrinth {
    pub fn new(total_width: i32, total_height: i32) -> Labyrinth {
        const MARGIN_FACTOR: i32 = 32;
        let left_margin = total_width / MARGIN_FACTOR;
        let top_margin = total_height / MARGIN_FACTOR;
        let width = (total_width - 2 * left_margin) / BOX_SIZE * BOX_SIZE;
        let height = (total_height - 2 * top_margin) / BOX_SIZE * BOX_SIZE;
        let x_box_cnt = width / BOX_SIZE;
        let y_box_cnt = height / BOX_SIZE;
        Labyrinth {
            x: total_width / 2 - width / 2,
            y: total_height / 2 - height / 2,
            width: width,
            height: height,
            x_box_cnt: x_box_cnt,
            y_box_cnt: y_box_cnt,
            marked: ndarray::ArrayD::<bool>::default(ndarray::IxDyn(&[
                x_box_cnt as usize,
                y_box_cnt as usize,
            ])),
        }
    }
    pub fn pixel_to_box(&self, (x, y): (f64, f64)) -> Option<(i32, i32)> {
        if x <= self.x as f64 || x >= (self.x + self.width) as f64 || y <= self.y as f64
            || y >= (self.y + self.height) as f64
        {
            None
        } else {
            Some((
                ((x - self.x as f64) / BOX_SIZE as f64) as i32,
                ((y - self.y as f64) / BOX_SIZE as f64) as i32,
            ))
        }
    }
    pub fn box_to_pixel(&self, (x_box, y_box): (i32, i32)) -> Option<Rectangle> {
        if x_box >= self.x_box_cnt || y_box >= self.y_box_cnt {
            None
        } else {
            Some(Rectangle {
                x: self.x + BOX_SIZE * x_box,
                y: self.y + BOX_SIZE * y_box,
                width: BOX_SIZE,
                height: BOX_SIZE,
            })
        }
    }
}
