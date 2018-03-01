#![allow(dead_code)]

use std::ops::{Add, Deref, Sub};
use rectangle::{GeneralRectangle, IsARectangle, IsARectangularArea, Rectangle};
use failure::Error;
use basic_types::{LabyrinthError, TwoDimensionalRange};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct GeneralPoint<T>
where
    T: Copy + Clone,
{
    pub coordinate: (T, T),
}

pub type Point = GeneralPoint<u32>;

impl<T> GeneralPoint<T>
where
    T: Copy + Clone,
{
    pub fn x(&self) -> T {
        self.coordinate.0
    }
    pub fn y(&self) -> T {
        self.coordinate.1
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct GeneralPointInScreenCoordinates<T>(GeneralPoint<T>)
where
    T: Copy + Clone;
pub type PointInScreenCoordinates = GeneralPointInScreenCoordinates<u32>;

impl<T> GeneralPointInScreenCoordinates<T>
where
    T: Copy + Clone + PartialOrd + Add<Output = T> + Sub<Output = T>,
{
    fn top_left_of<S>(area: &S) -> GeneralPointInScreenCoordinates<T>
    where
        S: IsARectangularArea<T>,
    {
        GeneralPointInScreenCoordinates(GeneralPoint {
            coordinate: (area.top_left_x(), area.top_left_y()),
        })
    }
    fn bottom_right_of<S>(area: &S) -> GeneralPointInScreenCoordinates<T>
    where
        S: IsARectangularArea<T>,
    {
        GeneralPointInScreenCoordinates(GeneralPoint {
            coordinate: (area.bottom_right_x(), area.bottom_right_y()),
        })
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct GeneralPointInGridCoordinates<T>(GeneralPoint<T>)
where
    T: Copy + Clone;
pub type PointInGridCoordinates = GeneralPointInGridCoordinates<u32>;

#[derive(Debug, Eq, PartialEq)]
pub struct GeneralRectangleInScreenCoordinates<T>(GeneralRectangle<T>)
where
    T: Copy + Clone + PartialOrd + Add<Output = T> + Sub<Output = T>;
pub type RectangleInScreenCoordinates = GeneralRectangleInScreenCoordinates<u32>;

#[derive(Debug, Eq, PartialEq)]
pub struct GeneralRectangleInGridCoordinates<T>(GeneralRectangle<T>)
where
    T: Copy + Clone + PartialOrd + Add<Output = T> + Sub<Output = T>;
pub type RectangleInGridCoordinates = GeneralRectangleInGridCoordinates<u32>;

macro_rules! implement_deref {
    ( $tp:ty, $inner:ty ) => (
        impl<T> Deref for $tp
          where T: Copy + Clone + PartialOrd + Add<Output=T> + Sub<Output=T>  {
            type Target = $inner;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    )
}

macro_rules! implement_rectangle {
    ( $tp:ty ) => (
        impl<T> IsARectangle<T> for $tp
          where T: Copy + Clone + PartialOrd + Add<Output=T> + Sub<Output=T>  {
            fn x(&self) -> T {
                self.0.x()
            }
            fn y(&self) -> T {
                self.0.y()
            }
            fn width(&self) -> T {
                self.0.width()
            }
            fn height(&self) -> T {
                self.0.height()
            }
        }
    )
}

implement_deref!(GeneralPointInScreenCoordinates<T>, GeneralPoint<T>);
implement_deref!(GeneralPointInGridCoordinates<T>, GeneralPoint<T>);
implement_deref!(GeneralRectangleInScreenCoordinates<T>, GeneralRectangle<T>);
implement_deref!(GeneralRectangleInGridCoordinates<T>, GeneralRectangle<T>);
implement_rectangle!(GeneralRectangleInScreenCoordinates<T>);
implement_rectangle!(GeneralRectangleInGridCoordinates<T>);

#[derive(Debug, Eq, PartialEq)]
pub struct GeneralScreen<T> {
    width: T,
    height: T,
}

type Screen = GeneralScreen<u32>;

impl<T> From<(T, T)> for GeneralScreen<T> {
    fn from((width, height): (T, T)) -> Self {
        GeneralScreen { width, height }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Grid {
    pub screen: Screen,
    pub area: RectangleInScreenCoordinates,
    pub box_size: u32,
    pub x_box_cnt: u32,
    pub y_box_cnt: u32,
}

impl Grid {
    pub fn is_on_axis(&self, point: PointInScreenCoordinates) -> bool {
        self.is_inside(point)
            && ((point.x() - self.area.top_left_x()) % self.box_size == 0 || (point.y() - self.area.top_left_y()) % self.box_size == 0)
    }
    pub fn is_inside(&self, point: PointInScreenCoordinates) -> bool {
        point.x() >= self.area.top_left_x() && point.x() <= self.area.bottom_right_x() && point.y() >= self.area.top_left_y()
            && point.y() <= self.area.bottom_right_y()
    }
    pub fn box_to_pixel(&self, point: &PointInGridCoordinates) -> Result<RectangleInScreenCoordinates, Error> {
        const BORDER_SIZE: u32 = 1;
        if point.x() >= self.x_box_cnt || point.y() >= self.y_box_cnt {
            Err(LabyrinthError::InternalError.into())
        } else {
            Ok(GeneralRectangleInScreenCoordinates(GeneralRectangle {
                x: self.area.x + self.box_size * point.x() + BORDER_SIZE,
                y: self.area.y + self.box_size * point.y() + BORDER_SIZE,
                width: self.box_size - 2 * BORDER_SIZE,
                height: self.box_size - 2 * BORDER_SIZE,
            }))
        }
    }
    pub fn pixel_to_box(&self, point: PointInScreenCoordinates) -> Option<PointInGridCoordinates> {
        use std::cmp::min;
        if self.is_inside(point) {
            Some(GeneralPointInGridCoordinates(GeneralPoint {
                coordinate: (
                    min(
                        (point.x() - self.area.x()) / self.box_size,
                        self.x_box_cnt - 1,
                    ),
                    min(
                        (point.y() - self.area.y()) / self.box_size,
                        self.y_box_cnt - 1,
                    ),
                ),
            }))
        } else {
            None
        }
    }
    pub fn pixel_area_to_box_area(&self, rectangle: &RectangleInScreenCoordinates) -> Option<RectangleInGridCoordinates> {
        rectangle.intersect(&*self.area).map(|intersection| {
            let top_left = self.pixel_to_box(PointInScreenCoordinates::top_left_of(&intersection)) 
                .unwrap();
            let bottom_right = self.pixel_to_box(PointInScreenCoordinates::bottom_right_of(&intersection))
                .unwrap();
            GeneralRectangleInGridCoordinates::<u32>(Rectangle {
                x: top_left.x(),
                y: top_left.y(),
                width: bottom_right.x() - top_left.x(),
                height: bottom_right.y() - top_left.y(),
            })
        })
    }
    pub fn pixel_area_to_box_range(&self, rectangle: &RectangleInScreenCoordinates) -> Option<TwoDimensionalRange> {
        self.pixel_area_to_box_area(rectangle).map( |area| {
          (
            ((area.top_left_x() as usize)..(area.bottom_right_x() as usize + 1)),
            ((area.top_left_y() as usize)..(area.bottom_right_y() as usize + 1))
          )
        })
    }
}
