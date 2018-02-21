use std;
use ndarray;
use basic_types;
use failure;
use conv;

#[derive(Debug)]
pub struct Labyrinth {
    pub rectangle: basic_types::Rectangle,
    pub x_box_cnt: u32,
    pub y_box_cnt: u32,
    pub marked: ndarray::Array2<bool>,
    pub box_size: u32,
}

impl Labyrinth {
    pub fn new(box_size: u32, total_width: u32, total_height: u32) -> Labyrinth {
        const MARGIN_FACTOR: u32 = 32;
        let left_margin = total_width / MARGIN_FACTOR;
        let top_margin = total_height / MARGIN_FACTOR;
        let width = (total_width - 2 * left_margin) / box_size * box_size;
        let height = (total_height - 2 * top_margin) / box_size * box_size;
        let x_box_cnt = width / box_size;
        let y_box_cnt = height / box_size;
        Labyrinth {
            rectangle: basic_types::Rectangle {
                x: total_width / 2 - width / 2,
                y: total_height / 2 - height / 2,
                width: width + 1,
                height: height + 1,
            },
            x_box_cnt: x_box_cnt,
            y_box_cnt: y_box_cnt,
            marked: ndarray::Array2::<bool>::default(ndarray::Ix2(x_box_cnt as usize, y_box_cnt as usize)),
            box_size: box_size,
        }
    }
    pub fn pixel_to_box(&self, (x, y): (u32, u32)) -> Option<(u32, u32)> {
        if x <= self.rectangle.x || x >= self.rectangle.x + self.rectangle.width || y <= self.rectangle.y
            || y >= self.rectangle.y + self.rectangle.height
        {
            None
        } else {
            // with border => this is for user input
            Some((
                ((x - self.rectangle.x) / self.box_size),
                ((y - self.rectangle.y) / self.box_size),
            ))
        }
    }
    pub fn pixel_rectangle_to_box_slice_index<T>(
        &self,
        rectangle: &basic_types::Rectangle,
    ) -> Result<((T, T), (T, T)), failure::Error>
    where
        T: conv::ValueFrom<u32> + Copy + std::fmt::Debug,
    {
        let ((top_left_x, top_left_y), (bottom_right_x, bottom_right_y)) = self.coerce_rectangle_into_labyrinth(rectangle);
        let box_top_left = self.pixel_to_box((top_left_x, top_left_y));
        let box_bottom_right = self.pixel_to_box((bottom_right_x, bottom_right_y));
        // failure::Failure can't be used with Option's ok_or() right now!
        let box_top_left = Labyrinth::check_valid_tuple(box_top_left)?;
        let box_bottom_right = Labyrinth::check_valid_tuple(box_bottom_right)?;
        Ok((box_top_left, box_bottom_right))
    }
    fn coerce_rectangle_into_labyrinth(&self, rectangle: &basic_types::Rectangle) -> ((u32, u32), (u32, u32)) {
        use basic_types::IsARectangularArea;
        (
            self.coerce_point_into_labyrinth((rectangle.top_left_x(), rectangle.top_left_y())),
            self.coerce_point_into_labyrinth((rectangle.bottom_right_x(), rectangle.bottom_right_y())),
        )
    }
    fn coerce_point_into_labyrinth(&self, (x, y): (u32, u32)) -> (u32, u32) {
        (
            Labyrinth::coerce_coordinate_into_labyrinth(x, self.rectangle.x, self.rectangle.width),
            Labyrinth::coerce_coordinate_into_labyrinth(y, self.rectangle.y, self.rectangle.height),
        )
    }
    fn coerce_coordinate_into_labyrinth(coordinate: u32, start: u32, dimension: u32) -> u32 {
        if coordinate <= start {
            start + 1
        } else if coordinate >= start + dimension {
            start + dimension - 1
        } else {
            coordinate
        }
    }
    fn check_valid_tuple<T, S>(value: Option<(T, T)>) -> Result<(S, S), failure::Error>
    where
        S: conv::ValueFrom<T> + Copy + std::fmt::Debug,
        T: std::fmt::Debug + Copy,
    {
        match value {
            Some((x, y)) => {
                let x = basic_types::convert(x)?;
                let y = basic_types::convert(y)?;
                Ok((x, y))
            }
            _ => Err(basic_types::LabyrinthError::InternalError.into()),
        }
    }
    pub fn box_to_pixel<T, S>(&self, (x_box, y_box): (S, S)) -> Result<basic_types::GeneralRectangle<T>, failure::Error>
    where
        T: Copy + Default + PartialOrd + std::fmt::Debug + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
        T: conv::ValueFrom<u32>,
        u32: conv::ValueFrom<S>,
        S: Copy + Default + std::fmt::Debug + std::ops::Add<Output = S> + std::ops::Sub<Output = S> + PartialOrd,
    {
        use basic_types::{GeneralRectangle, Rectangle};
        let x_box: u32 = basic_types::convert(x_box)?;
        let y_box: u32 = basic_types::convert(y_box)?;
        if x_box >= self.x_box_cnt || y_box >= self.y_box_cnt {
            Err(basic_types::LabyrinthError::InternalError.into())
        } else {
            // without border for this function; this is just for drawing
            GeneralRectangle::<T>::from::<u32, Rectangle>(&Rectangle {
                x: self.rectangle.x + self.box_size * x_box + 1,
                y: self.rectangle.y + self.box_size * y_box + 1,
                width: self.box_size - 2,
                height: self.box_size - 2,
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
    pub fn new(box_size: u32, (width, height): (u32, u32)) -> LabyrinthState {
        LabyrinthState {
            width: width,
            height: height,
            box_size: box_size,
            labyrinth: None,
        }
    }
}
