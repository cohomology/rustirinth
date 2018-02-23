#![allow(dead_code)]
use gtk;
use cairo;

use std::fmt::Debug;
use std::marker::Copy;
use std::ops::{Add, Range, Sub};
use failure::Error;
use conv::{ApproxFrom, ApproxInto, ValueFrom, ValueInto};

#[derive(Debug, Fail)]
pub enum LabyrinthError {
    #[fail(display = "Could not get default screen")]
    CouldNotGetDefaultScreen,
    #[fail(display = "Conversion error or overflow while converting \"{}\"", value)]
    ConversionError { value: String },
    #[fail(display = "An internal error occurred")]
    InternalError,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Point {
    x: u32,
    y: u32,
}

impl From<(u32, u32)> for Point {
    fn from((x, y): (u32, u32)) -> Point {
        Point { x, y }
    }
}

impl From<Point> for (u32, u32) {
    fn from(p: Point) -> (u32, u32) {
        (p.x, p.y)
    }
}

pub trait IsARectangle<T> {
    fn from_tuple(tuple: (T, T, T, T)) -> Self;
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn width(&self) -> T;
    fn height(&self) -> T;
}

impl IsARectangle<i32> for gtk::Rectangle {
    fn from_tuple((x, y, width, height): (i32, i32, i32, i32)) -> gtk::Rectangle {
        gtk::Rectangle { x,
                         y,
                         width,
                         height, }
    }
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
    fn width(&self) -> i32 {
        self.width
    }
    fn height(&self) -> i32 {
        self.height
    }
}

impl IsARectangle<i32> for cairo::RectangleInt {
    fn from_tuple((x, y, width, height): (i32, i32, i32, i32)) -> cairo::RectangleInt {
        cairo::RectangleInt { x,
                              y,
                              width,
                              height, }
    }
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
    fn width(&self) -> i32 {
        self.width
    }
    fn height(&self) -> i32 {
        self.height
    }
}

impl<T> IsARectangle<T> for (T, T, T, T)
    where T: Copy
{
    fn from_tuple(tuple: (T, T, T, T)) -> (T, T, T, T) {
        tuple
    }
    fn x(&self) -> T {
        self.0
    }
    fn y(&self) -> T {
        self.1
    }
    fn width(&self) -> T {
        self.2
    }
    fn height(&self) -> T {
        self.3
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, PartialOrd)]
pub struct GeneralRectangle<T>
    where T: Copy + Clone + Default + Debug
{
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

pub type Rectangle = GeneralRectangle<u32>;

impl<T> IsARectangle<T> for GeneralRectangle<T>
    where T: Copy + Clone + Default + Debug
{
    fn from_tuple((x, y, width, height): (T, T, T, T)) -> GeneralRectangle<T> {
        GeneralRectangle::<T> { x,
                                y,
                                width,
                                height, }
    }
    fn x(&self) -> T {
        self.x
    }
    fn y(&self) -> T {
        self.y
    }
    fn width(&self) -> T {
        self.width
    }
    fn height(&self) -> T {
        self.height
    }
}

fn partial_max<U: PartialOrd>(a: U, b: U) -> U {
    if b > a {
        b
    } else {
        a
    }
}

fn partial_min<U: PartialOrd>(a: U, b: U) -> U {
    if b < a {
        b
    } else {
        a
    }
}

fn raise_error<T>(value: T) -> Error
    where T: Debug
{
    LabyrinthError::ConversionError { value: format!("{:?}", value), }.into()
}

pub fn convert<T, S>(value: T) -> Result<S, Error>
    where S: ValueFrom<T> + Copy + Debug,
          T: Debug + Copy
{
    ValueInto::value_into(value).map_err(|_| raise_error(value))
}

pub fn approx_convert<T, S>(value: T) -> Result<S, Error>
    where S: ApproxFrom<T> + Copy + Debug,
          T: Debug + Copy
{
    ApproxInto::approx_into(value).map_err(|_| raise_error(value))
}

impl<U> GeneralRectangle<U>
    where U: Copy + Clone + Default + PartialOrd + Debug + Add<Output = U> + Sub<Output = U>
{
    pub fn from<T, R>(rectangle: &R) -> Result<GeneralRectangle<U>, Error>
        where R: IsARectangle<T>,
              U: ValueFrom<T>,
              T: Copy + Debug
    {
        let x = convert(rectangle.x())?;
        let y = convert(rectangle.y())?;
        let width = convert(rectangle.width())?;
        let height = convert(rectangle.height())?;
        Ok(GeneralRectangle::<U> { x,
                                   y,
                                   width,
                                   height, })
    }
    pub fn approx_from<T, R>(rectangle: &R) -> Result<GeneralRectangle<U>, Error>
        where R: IsARectangle<T>,
              U: ApproxFrom<T>,
              T: Copy + Debug
    {
        let x = approx_convert(rectangle.x())?;
        let y = approx_convert(rectangle.y())?;
        let width = approx_convert(rectangle.width())?;
        let height = approx_convert(rectangle.height())?;
        Ok(GeneralRectangle::<U> { x,
                                   y,
                                   width,
                                   height, })
    }
    pub fn to<T, R>(&self) -> Result<R, Error>
        where R: IsARectangle<T>,
              T: ValueFrom<U> + Copy + Debug
    {
        let x = convert(self.x)?;
        let y = convert(self.y)?;
        let width = convert(self.width)?;
        let height = convert(self.height)?;
        Ok(R::from_tuple((x, y, width, height)))
    }
    pub fn approx_to<T, R>(&self) -> Result<R, Error>
        where R: IsARectangle<T>,
              T: ApproxFrom<U> + Copy + Debug
    {
        let x = approx_convert(self.x)?;
        let y = approx_convert(self.y)?;
        let width = approx_convert(self.width)?;
        let height = approx_convert(self.height)?;
        Ok(R::from_tuple((x, y, width, height)))
    }
    pub fn intersect(&self, other: &GeneralRectangle<U>) -> Option<GeneralRectangle<U>> {
        if self.inside_bounds(other) {
            let top_left_x = partial_max(self.x, other.x);
            let top_left_y = partial_max(self.y, other.y);
            let bottom_right_x = partial_min(self.bottom_right_x(), other.bottom_right_x());
            let bottom_right_y = partial_min(self.bottom_right_y(), other.bottom_right_y());
            Some(GeneralRectangle::<U> { x: top_left_x,
                                         y: top_left_y,
                                         width: bottom_right_x - top_left_x,
                                         height: bottom_right_y - top_left_y, })
        } else {
            None
        }
    }
    fn inside_bounds(&self, other: &GeneralRectangle<U>) -> bool {
        other.bottom_right_x() >= self.x && other.x <= self.x + self.width && other.y + other.height >= self.y
        && other.y <= self.y + self.height
    }
}

pub trait IsARectangularArea<T> {
    fn top_left_x(&self) -> T;
    fn top_left_y(&self) -> T;
    fn bottom_right_x(&self) -> T;
    fn bottom_right_y(&self) -> T;
}

impl<T, R> IsARectangularArea<T> for R
    where T: Add,
          R: IsARectangle<T>,
          T: Add<Output = T>
{
    fn top_left_x(&self) -> T {
        self.x()
    }
    fn top_left_y(&self) -> T {
        self.y()
    }
    fn bottom_right_x(&self) -> T {
        self.x() + self.width()
    }
    fn bottom_right_y(&self) -> T {
        self.y() + self.height()
    }
}

pub type TwoDimensionalRange = (Range<usize>, Range<usize>);

pub trait IsAColor<T>
    where T: Copy
{
    fn from_tuple(tuple: (T, T, T)) -> Self;
    fn to_tuple(&self) -> (T, T, T) {
        (self.red(), self.green(), self.blue())
    }
    fn to_float_tuple(&self) -> (f64, f64, f64);
    fn red(&self) -> T;
    fn green(&self) -> T;
    fn blue(&self) -> T;

    fn get_white() -> Self;
    fn get_black() -> Self;
    fn get_blue() -> Self;
}

pub struct GeneralColor<T>
    where f64: From<T>,
          T: From<u32> + Copy
{
    red: T,
    green: T,
    blue: T,
}

impl<T> IsAColor<T> for GeneralColor<T>
    where f64: From<T>,
          T: From<u32> + Copy
{
    fn from_tuple((red, green, blue): (T, T, T)) -> GeneralColor<T> {
        GeneralColor::<T> { red, green, blue }
    }
    fn to_float_tuple(&self) -> (f64, f64, f64) {
        (self.red.into(), self.green.into(), self.blue.into())
    }
    fn red(&self) -> T {
        self.red
    }
    fn blue(&self) -> T {
        self.blue
    }
    fn green(&self) -> T {
        self.green
    }

    fn get_black() -> GeneralColor<T> {
        GeneralColor::<T>::from_tuple((0.into(), 0.into(), 0.into()))
    }

    fn get_white() -> GeneralColor<T> {
        GeneralColor::<T>::from_tuple((255.into(), 255.into(), 255.into()))
    }

    fn get_blue() -> GeneralColor<T> {
        GeneralColor::<T>::from_tuple((0.into(), 0.into(), 255.into()))
    }
}

pub type Color = GeneralColor<f64>;

#[cfg(test)]
mod tests {

    use cairo;
    use gtk;
    use super::*;

    #[test]
    fn from_cairo_ok() {
        let cairo_rectangle = cairo::RectangleInt { x: 1,
                                                    y: 2,
                                                    width: 3,
                                                    height: 4, };
        let rectangle = Rectangle::from(&cairo_rectangle).unwrap();
        assert_eq!(rectangle,
                   Rectangle { x: 1,
                               y: 2,
                               width: 3,
                               height: 4, });
    }

    #[test]
    fn from_cairo_err() {
        let cairo_rectangle = cairo::RectangleInt { x: -1,
                                                    y: 2,
                                                    width: 3,
                                                    height: 4, };
        let rectangle = Rectangle::from(&cairo_rectangle);
        assert!(rectangle.is_err());
        let error = rectangle.err().unwrap();
        let error_string = format!("{}", error);
        assert_eq!(error_string,
                   "Conversion error or overflow while converting \"-1\"");
    }

    #[test]
    fn from_gtk_ok() {
        let gtk_rectangle = gtk::Rectangle { x: 1,
                                             y: 2,
                                             width: 3,
                                             height: 4, };
        let rectangle = Rectangle::from(&gtk_rectangle).unwrap();
        assert_eq!(rectangle,
                   Rectangle { x: 1,
                               y: 2,
                               width: 3,
                               height: 4, });
    }

    #[test]
    fn to_float_tuple() {
        type Ftuple = (f64, f64, f64, f64);
        let rectangle = Rectangle { x: 1,
                                    y: 2,
                                    width: 3,
                                    height: 4, };
        let float_tuple: Ftuple = rectangle.to().unwrap();
        assert_eq!(float_tuple, (1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn general_rectangle_overflow() {
        let big_rectangle = GeneralRectangle::<u64> { x: 1,
                                                      y: 4_294_967_296,
                                                      width: 3,
                                                      height: 4, };
        let rectangle = big_rectangle.to::<u32, Rectangle>();
        assert!(rectangle.is_err());
        let error = rectangle.err().unwrap();
        let error_string = format!("{}", error);
        assert_eq!(error_string,
                   "Conversion error or overflow while converting \"4294967296\"");
    }

    #[test]
    fn float_overflow() {
        let big_rectangle = (1.0, 4294967296.0, 3.0, 4.0);
        let rectangle = Rectangle::approx_from(&big_rectangle);
        assert!(rectangle.is_err());
        let error = rectangle.err().unwrap();
        let error_string = format!("{}", error);
        assert_eq!(error_string,
                   "Conversion error or overflow while converting \"4294967296.0\"");
    }

    #[test]
    fn to_float() {
        let rectangle = Rectangle::from::<u32, (u32, u32, u32, u32)>(&(1, 4294967295, 3, 4)).unwrap();
        let float_tuple = rectangle.approx_to::<f64, (f64, f64, f64, f64)>().unwrap();
        assert_eq!(float_tuple, ((1.0, 4294967295.0, 3.0, 4.0)));
    }
}
