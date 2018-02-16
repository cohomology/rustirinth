use std;
use ndarray;

#[derive(Debug)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct Labyrinth {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub x_box_cnt: u32,
    pub y_box_cnt: u32,
    pub marked: ndarray::ArrayD<bool>,
    pub box_size: u32
}

impl Labyrinth {
    pub fn new(box_size : u32, total_width: u32, total_height: u32) -> Labyrinth {
        const MARGIN_FACTOR: u32 = 32;
        let left_margin = total_width / MARGIN_FACTOR;
        let top_margin = total_height / MARGIN_FACTOR;
        let width = (total_width - 2 * left_margin) / box_size * box_size;
        let height = (total_height - 2 * top_margin) / box_size * box_size;
        let x_box_cnt = width / box_size;
        let y_box_cnt = height / box_size;
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
            box_size : box_size
        }
    }
    pub fn pixel_to_box(&self, (x, y): (u32, u32)) -> Option<(u32, u32)> {
        if x <= self.x || x >= self.x + self.width || y <= self.y || y >= self.y + self.height {
            None
        } else {
            Some((((x - self.x) / self.box_size), ((y - self.y) / self.box_size)))
        }
    }
    pub fn box_to_pixel(&self, (x_box, y_box): (u32, u32)) -> Option<Rectangle> {
        if x_box >= self.x_box_cnt || y_box >= self.y_box_cnt {
            None
        } else {
            Some(Rectangle {
                x: self.x + self.box_size * x_box,
                y: self.y + self.box_size * y_box,
                width: self.box_size,
                height: self.box_size,
            })
        }
    }
}

#[derive(Debug)]
pub struct LabyrinthState {
    pub width: u32,
    pub height: u32,
    pub box_size: u32,
    pub labyrinth: std::option::Option<Labyrinth>,
}

impl LabyrinthState {
    pub fn new(box_size : u32, (width, height): (u32, u32)) -> LabyrinthState {
        LabyrinthState {
            width: width,
            height: height,
            box_size : box_size,
            labyrinth: None,
        }
    }
}
