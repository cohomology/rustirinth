#![allow(dead_code)]

use std::ops::{Add, Sub}; 
use rectangle::{GeneralRectangle};

pub struct GeneralScreen<T> {
    width : T,
    height : T
}

type Screen = GeneralScreen<u32>;

impl<T> From<(T, T)> for GeneralScreen<T>
{
    fn from((width, height): (T, T)) -> Self {
        GeneralScreen {
            width,
            height,
        }
    }
} 

pub struct GeneralGrid<T> 
    where T : Copy + Clone + PartialOrd + Add<Output=T> + Sub<Output=T> {
    pub screen : GeneralScreen<T>,
    pub area : GeneralRectangle<T>,
    pub box_size : T,
}

type Grid = GeneralGrid<u32>;

pub trait IsAPoint<T> {
    fn x(&self) -> T;
    fn y(&self) -> T;
}

pub struct GeneralPoint<T> {
    pub x : T,
    pub y : T 
}

pub type Point = GeneralPoint<u32>;

type PointInScreenCoordinates = Point<T>; 
type PointInGridCoordinates = Point<T>;

pub type PointInScreenCoordinates = GeneralPointInScreenCoordinates<u32>; 
pub type PointInGridCoordinates = GeneralPointInGridCoordinates<u32>;

pub struct GeneralGridCoordinate<T> 
    where T : Copy + Clone + PartialOrd + Add<Output=T> + Sub<Output=T> {
    pub grid : GeneralGrid<T>,
    pub point : GeneralPointInScreenCoordinates<T>, 
}

