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
    ConversionError  {
        value : String,
    }
} 

pub trait IsARectangle<T> {
    fn from_tuple(tuple : (T, T, T, T)) -> Self;
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn width(&self) -> T; 
    fn height(&self) -> T;
}

impl IsARectangle<i32> for gtk::Rectangle {
    fn from_tuple( (x,y,width,height) : (i32, i32, i32, i32) ) -> gtk::Rectangle {
        gtk::Rectangle { x, y, width, height }
    }
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height }
}

impl IsARectangle<i32> for cairo::RectangleInt {
    fn from_tuple( (x,y,width,height) : (i32, i32, i32, i32) ) -> cairo::RectangleInt {
        cairo::RectangleInt { x, y, width, height }
    }
    fn x(&self) -> i32 { self.x }
    fn y(&self) -> i32 { self.y }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height } 
}

impl<T> IsARectangle<T> for (T, T, T, T) where T : Copy {
    fn from_tuple( tuple : (T, T, T, T) ) -> (T, T, T, T) {
        tuple
    } 
    fn x(&self) -> T { self.0 }
    fn y(&self) -> T { self.1 }
    fn width(&self) -> T { self.2 }
    fn height(&self) -> T { self.3 }
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
} 

impl IsARectangle<u32> for Rectangle {
    fn from_tuple( (x,y,width,height) : (u32, u32, u32, u32) ) -> Rectangle {
        Rectangle { x,y,width,height }
    } 
    fn x(&self) -> u32 { self.x }
    fn y(&self) -> u32 { self.y }
    fn width(&self) -> u32 { self.width }
    fn height(&self) -> u32 { self.height }  
}

impl Rectangle {
    pub fn from<T, R>(rectangle: &R) -> Result<Rectangle, failure::Error> 
        where R : IsARectangle<T>, u32 : conv::ValueFrom<T>, T : Copy + std::fmt::Debug {
        let x = Rectangle::convert(rectangle.x())?;
        let y = Rectangle::convert(rectangle.y())?;
        let width = Rectangle::convert(rectangle.width())?;
        let height = Rectangle::convert(rectangle.height())?;
        Ok(Rectangle { x, y, width, height })
    }
    pub fn to<T, R>(&self) -> Result<R, failure::Error> 
        where R : IsARectangle<T>, T : conv::ValueFrom<u32> + Copy + std::fmt::Debug {
        let x = Rectangle::convert(self.x)?;
        let y = Rectangle::convert(self.y)?;
        let width = Rectangle::convert(self.width)?;
        let height = Rectangle::convert(self.height)?;
        Ok(R::from_tuple((x,y,width,height)))
    }
    pub fn call<F, R, S>(&self, f : F) -> Result<S, failure::Error> where F : FnMut(R, R, R, R) -> S {
        match self.to::<R, (R, R, R, R)>()  {
            Some((x,y,width,height)) => f(x,y,width,height),
            Err(error) => Err(error)
        }
    } 
    fn convert<T, S>(value : T) -> Result<S, failure::Error> 
        where S : conv::ValueFrom<T> + Copy + std::fmt::Debug, T : std::fmt::Debug + std::marker::Copy {
        use conv::ValueInto;
        if let Ok(value) = value.value_into() {
            Ok(value)
        } else {
            Err(LabyrinthError::ConversionError { value : format!("{:?}", value) }.into())
        }
    }
}

trait IsARectangularArea<T> {
    fn top_left_x(&self) -> T;
    fn top_left_y(&self) -> T;
    fn bottom_right_x(&self) -> T; 
    fn bottom_right_y(&self) -> T; 
}

impl<T, R> IsARectangularArea<T> for R 
    where T : std::ops::Add, R : IsARectangle<T>, T : std::ops::Add<Output=T> {
    fn top_left_x(&self) -> T { self.x() }
    fn top_left_y(&self) -> T { self.y() }
    fn bottom_right_x(&self) -> T { self.x() + self.width() } 
    fn bottom_right_y(&self) -> T { self.y() + self.height() }  
}

// #[cfg(test)]
// mod tests {
//     use cairo;
//     use std;
//     #[test]
//     fn from_cairo() {
//         let cairo = cairo::RectangleInt {
//             x : 0,
//             y : 0,
//             width : 0,
//             height : 0
//         };
//         let my : Rectangle = std::convert::From<&cairo::Rectangle>(&cairo);
//     }
// }
