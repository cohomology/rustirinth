#![allow(dead_code)]
use std;
use gtk;
use cairo;
use failure;
use conv;

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
        gtk::Rectangle {
            x,
            y,
            width,
            height,
        }
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
        cairo::RectangleInt {
            x,
            y,
            width,
            height,
        }
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
where
    T: Copy,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct GeneralRectangle<T>
where
    T: Copy + Clone + Default + std::fmt::Debug,
{
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

pub type Rectangle = GeneralRectangle<u32>;

impl<T> IsARectangle<T> for GeneralRectangle<T>
where
    T: Copy + Clone + Default + std::fmt::Debug
{
    fn from_tuple((x, y, width, height): (T, T, T, T)) -> GeneralRectangle<T> {
        GeneralRectangle::<T> {
            x,
            y,
            width,
            height,
        }
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

impl<U> GeneralRectangle<U>
where
    U: Copy + Clone + Default + std::fmt::Debug,
{
    pub fn from<T, R>(rectangle: &R) -> Result<GeneralRectangle<U>, failure::Error>
    where
        R: IsARectangle<T>,
        U: conv::ValueFrom<T>,
        T: Copy + std::fmt::Debug,
    {
        let x = GeneralRectangle::<U>::convert(rectangle.x())?;
        let y = GeneralRectangle::<U>::convert(rectangle.y())?;
        let width = GeneralRectangle::<U>::convert(rectangle.width())?;
        let height = GeneralRectangle::<U>::convert(rectangle.height())?;
        Ok(GeneralRectangle::<U> {
            x,
            y,
            width,
            height,
        })
    }
    pub fn approx_from<T, R>(rectangle: &R) -> Result<GeneralRectangle<U>, failure::Error>
    where
        R: IsARectangle<T>,
        U: conv::ApproxFrom<T>,
        T: Copy + std::fmt::Debug,
    {
        let x = GeneralRectangle::<U>::approx_convert(rectangle.x())?;
        let y = GeneralRectangle::<U>::approx_convert(rectangle.y())?;
        let width = GeneralRectangle::<U>::approx_convert(rectangle.width())?;
        let height = GeneralRectangle::<U>::approx_convert(rectangle.height())?;
        Ok(GeneralRectangle::<U> {
            x,
            y,
            width,
            height,
        })
    }
    pub fn to<T, R>(&self) -> Result<R, failure::Error>
    where
        R: IsARectangle<T>,
        T: conv::ValueFrom<U> + Copy + std::fmt::Debug,
    {
        let x = Rectangle::convert(self.x)?;
        let y = Rectangle::convert(self.y)?;
        let width = Rectangle::convert(self.width)?;
        let height = Rectangle::convert(self.height)?;
        Ok(R::from_tuple((x, y, width, height)))
    }
    pub fn approx_to<T, R>(&self) -> Result<R, failure::Error>
    where
        R: IsARectangle<T>,
        T: conv::ApproxFrom<U> + Copy + std::fmt::Debug,
    {
        let x = Rectangle::approx_convert(self.x)?;
        let y = Rectangle::approx_convert(self.y)?;
        let width = Rectangle::approx_convert(self.width)?;
        let height = Rectangle::approx_convert(self.height)?;
        Ok(R::from_tuple((x, y, width, height)))
    }
    fn raise_error<T>(value: T) -> failure::Error
    where
        T: std::fmt::Debug,
    {
        LabyrinthError::ConversionError {
            value: format!("{:?}", value),
        }.into()
    }
    fn convert<T, S>(value: T) -> Result<S, failure::Error>
    where
        S: conv::ValueFrom<T> + Copy + std::fmt::Debug,
        T: std::fmt::Debug + std::marker::Copy,
    {
        conv::ValueInto::value_into(value).map_err(|_| GeneralRectangle::<U>::raise_error(value))
    }
    fn approx_convert<T, S>(value: T) -> Result<S, failure::Error>
    where
        S: conv::ApproxFrom<T> + Copy + std::fmt::Debug,
        T: std::fmt::Debug + std::marker::Copy,
    {
        conv::ApproxInto::approx_into(value).map_err(|_| GeneralRectangle::<U>::raise_error(value))
    }
}

pub trait IsARectangularArea<T> {
    fn top_left_x(&self) -> T;
    fn top_left_y(&self) -> T;
    fn bottom_right_x(&self) -> T;
    fn bottom_right_y(&self) -> T;
}

impl<T, R> IsARectangularArea<T> for R
where T: std::ops::Add, R: IsARectangle<T>, T: std::ops::Add<Output = T> {
    fn top_left_x(&self) -> T { self.x() }
    fn top_left_y(&self) -> T { self.y() }
    fn bottom_right_x(&self) -> T { self.x() + self.width() }
    fn bottom_right_y(&self) -> T { self.y() + self.height() }
}

pub trait IsAColor<T>
where
    T: Copy,
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
}

pub struct GeneralColor<T>
where
    f64: From<T>,
    T: From<u32> + Copy,
{
    red: T,
    green: T,
    blue: T,
}

impl<T> IsAColor<T> for GeneralColor<T>
where
    f64: From<T>,
    T: From<u32> + Copy,
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
}

pub type Color = GeneralColor<f64>;

#[cfg(test)]
mod tests {

    use cairo;
    use gtk;
    use super::*;

    #[test]
    fn from_cairo_ok() {
        let cairo_rectangle = cairo::RectangleInt {
            x: 1,
            y: 2,
            width: 3,
            height: 4,
        };
        let rectangle = Rectangle::from(&cairo_rectangle).unwrap();
        assert_eq!(
            rectangle,
            Rectangle {
                x: 1,
                y: 2,
                width: 3,
                height: 4,
            }
        );
    }

    #[test]
    fn from_cairo_err() {
        let cairo_rectangle = cairo::RectangleInt {
            x: -1,
            y: 2,
            width: 3,
            height: 4,
        };
        let rectangle = Rectangle::from(&cairo_rectangle);
        assert!(rectangle.is_err());
        let error = rectangle.err().unwrap();
        let error_string = format!("{}", error);
        assert_eq!(
            error_string,
            "Conversion error or overflow while converting \"-1\""
        );
    }

    #[test]
    fn from_gtk_ok() {
        let gtk_rectangle = gtk::Rectangle {
            x: 1,
            y: 2,
            width: 3,
            height: 4,
        };
        let rectangle = Rectangle::from(&gtk_rectangle).unwrap();
        assert_eq!(
            rectangle,
            Rectangle {
                x: 1,
                y: 2,
                width: 3,
                height: 4,
            }
        );
    }

    #[test]
    fn to_float_tuple() {
        type Ftuple = (f64, f64, f64, f64);
        let rectangle = Rectangle {
            x: 1,
            y: 2,
            width: 3,
            height: 4,
        };
        let float_tuple: Ftuple = rectangle.to().unwrap();
        assert_eq!(float_tuple, (1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn general_rectangle_overflow() {
        let big_rectangle = GeneralRectangle::<u64> {
            x: 1,
            y: 4_294_967_296,
            width: 3,
            height: 4,
        };
        let rectangle = big_rectangle.to::<u32, Rectangle>();
        assert!(rectangle.is_err());
        let error = rectangle.err().unwrap();
        let error_string = format!("{}", error);
        assert_eq!(
            error_string,
            "Conversion error or overflow while converting \"4294967296\""
        );
    }

    #[test]
    fn float_overflow() {
        let big_rectangle = (1.0, 4294967296.0, 3.0, 4.0);
        let rectangle = Rectangle::approx_from(&big_rectangle);
        assert!(rectangle.is_err());
        let error = rectangle.err().unwrap();
        let error_string = format!("{}", error);
        assert_eq!(
            error_string,
            "Conversion error or overflow while converting \"4294967296.0\""
        );
    }

    #[test]
    fn to_float() {
        let rectangle =
            Rectangle::from::<u32, (u32, u32, u32, u32)>(&(1, 4294967295, 3, 4)).unwrap();
        let float_tuple = rectangle.approx_to::<f64, (f64, f64, f64, f64)>().unwrap();
        assert_eq!(float_tuple, ((1.0, 4294967295.0, 3.0, 4.0)));
    }
}
