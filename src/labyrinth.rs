use std::option::Option;
use std::fmt::Debug;
use std::ops::{Add, Sub};
use ndarray::{Array2 as Array, SliceOrIndex, SliceInfo, Ix2 as Dim};
use basic_types::{convert, GeneralRectangle, IsARectangularArea, LabyrinthError, Rectangle, TwoDimensionalRange, Color};
use failure::Error;
use conv::ValueFrom;

#[derive(Debug,Copy,Clone)] 
pub enum BoxState {
    Empty,
    Labyrinth,
    StartPoint,
    EndPoint
}

impl Default for BoxState {
    fn default() -> BoxState {
        BoxState::Empty
    }
}

impl BoxState {
    fn to_color() -> Color {
        match BoxState {
            BoxState::Empty => 

        }
    }

};

#[derive(Debug,Copy,Clone,Default)]
pub struct LabyrinthEntry {
    pub state : BoxState,
    pub color : Color,
}

#[derive(Debug)]
pub struct Labyrinth {
    pub rectangle: Rectangle,
    pub x_box_cnt: u32,
    pub y_box_cnt: u32,
    pub marked: Array<LabyrinthEntry>,
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
        Labyrinth { rectangle: Rectangle { x: total_width / 2 - width / 2,
                                           y: total_height / 2 - height / 2,
                                           width: width + 1,
                                           height: height + 1, },
                    x_box_cnt: x_box_cnt,
                    y_box_cnt: y_box_cnt,
                    marked: Array::<bool>::default(Dim(x_box_cnt as usize, y_box_cnt as usize)),
                    box_size: box_size, }
    }
    pub fn set_box_state<F>(&self, pointer : (f64, f64), state : BoxState, call_success : F) -> Result<(), Error>
        where F : FnOnce(&Rectangle) -> Result<(), Error> { 
        let clicked_box = self.pixel_to_box((x as u32, y as u32));
        if let Some(clicked_box) = clicked_box {
            if self.update_marked(clicked_box, state) {
                call_success(clicked_box)?;
            }
        } 
        Ok(())
    }
    fn update_marked(&mut self, (x, y): (u32, u32), state : BoxState) -> bool {
        if let Some(box) = labyrinth.marked.get_mut(Dim(x as usize, y as usize)) {
            if box.state != state {
                
               return true;
            }
            
            if !unmark && !*marked {
                *marked = true;
                return true;
            } else if unmark && *marked {
                *marked = false;
                return true;
            }
        }
        false
    } 
    pub fn call_for_every_box<F>(&self, area : &Rectangle, function : F) -> Result<(), Error>
        where F : FnOnce(&Rectangle, &LabyrinthEntry) -> Result<(), Error> {
        let (x_range, y_range) = self.pixel_rectangle_to_box_range(area)?;
        let slice_x = SliceOrIndex::from(x_range.clone());
        let slice_y = SliceOrIndex::from(y_range.clone());
        let slice_args = SliceInfo::<[SliceOrIndex; 2], Dim>::new([slice_x, slice_y])?;
        for ((x_box, y_box), entry) in self.marked.slice(&slice_args).indexed_iter() {
            let box_rectangle = self.box_to_pixel((x_box + x_range.start, y_box + y_range.start))?;
            if let Some(intersection) = box_rectangle.intersect(area) {
                function(intersection, entry)?; 
            }
        }  
        Ok(())
    }
    pub fn pixel_to_box(&self, (x, y): (u32, u32)) -> Option<(u32, u32)> {
        if x <= self.rectangle.x || x >= self.rectangle.x + self.rectangle.width || y <= self.rectangle.y
           || y >= self.rectangle.y + self.rectangle.height
        {
            None
        } else {
            // with border => this is for user input
            Some((((x - self.rectangle.x) / self.box_size), ((y - self.rectangle.y) / self.box_size)))
        }
    }
    pub fn pixel_rectangle_to_box_range(&self, rectangle: &Rectangle) -> Result<TwoDimensionalRange, Error> {
        let ((top_left_x, top_left_y), (bottom_right_x, bottom_right_y)) = self.coerce_rectangle_into_labyrinth(rectangle);
        let (box_top_left_x, box_top_left_y) =
            Labyrinth::check_valid_tuple::<u32, usize>(self.pixel_to_box((top_left_x, top_left_y)))?;
        let (mut box_bottom_right_x, mut box_bottom_right_y) =
            Labyrinth::check_valid_tuple::<u32, usize>(self.pixel_to_box((bottom_right_x, bottom_right_y)))?;
        box_bottom_right_x = if box_bottom_right_x + 1 >= self.x_box_cnt as usize {
            self.x_box_cnt as usize
        } else {
            box_bottom_right_x + 1
        };
        box_bottom_right_y = if box_bottom_right_y + 1 >= self.y_box_cnt as usize {
            self.y_box_cnt as usize
        } else {
            box_bottom_right_y + 1
        };
        Ok(((box_top_left_x..box_bottom_right_x), (box_top_left_y..box_bottom_right_y)))
    }
    fn coerce_rectangle_into_labyrinth(&self, rectangle: &Rectangle) -> ((u32, u32), (u32, u32)) {
        (self.coerce_point_into_labyrinth((rectangle.top_left_x(), rectangle.top_left_y())),
        self.coerce_point_into_labyrinth((rectangle.bottom_right_x(), rectangle.bottom_right_y())))
    }
    fn coerce_point_into_labyrinth(&self, (x, y): (u32, u32)) -> (u32, u32) {
        (Labyrinth::coerce_coordinate_into_labyrinth(x, self.rectangle.x, self.rectangle.width),
        Labyrinth::coerce_coordinate_into_labyrinth(y, self.rectangle.y, self.rectangle.height))
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
    fn check_valid_tuple<T, S>(value: Option<(T, T)>) -> Result<(S, S), Error>
        where S: Copy + Debug + ValueFrom<T>,
              T: Copy + Debug
    {
        match value {
            Some((x, y)) => {
                let x = convert(x)?;
                let y = convert(y)?;
                Ok((x, y))
            }
            _ => Err(LabyrinthError::InternalError.into()),
        }
    }
    pub fn box_to_pixel<T, S>(&self, (x_box, y_box): (S, S)) -> Result<GeneralRectangle<T>, Error>
        where T: Copy + Default + PartialOrd + Debug + Add<Output = T> + Sub<Output = T>,
              T: ValueFrom<u32>,
              u32: ValueFrom<S>,
              S: Copy + Default + PartialOrd + Debug + Add<Output = S> + Sub<Output = S>
    {
        let x_box: u32 = convert(x_box)?;
        let y_box: u32 = convert(y_box)?;
        if x_box >= self.x_box_cnt || y_box >= self.y_box_cnt {
            Err(LabyrinthError::InternalError.into())
        } else {
            // without border for this function; this is just for drawing
            GeneralRectangle::<T>::from::<u32, Rectangle>(&Rectangle { x: self.rectangle.x + self.box_size * x_box + 1,
                                                                       y: self.rectangle.y + self.box_size * y_box + 1,
                                                                       width: self.box_size - 2,
                                                                       height: self.box_size - 2, })
        }
    }
}

#[derive(Debug)]
pub struct LabyrinthState {
    pub box_size: u32,
    pub labyrinth: Option<Labyrinth>,
}

impl LabyrinthState {
    pub fn new(box_size: u32) -> LabyrinthState {
        LabyrinthState { box_size: box_size,
                         labyrinth: None, }
    }
}
