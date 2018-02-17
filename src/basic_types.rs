use std;
use gtk;
use cairo;
use failure;
use conv;

#[derive(Debug, Fail)]
pub enum LabyrinthError {
    #[fail(display = "Could not get default screen")]
    CouldNotGetDefaultScreen,
} 

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
} 

impl conv::TryFrom<Rectangle> for gtk::Rectangle {
    fn try_into(rectangle: &gtk::Rectangle) -> Result<Rectangle, failure::Error> {
        Rectangle { x : rectangle.x as u32,
                    y: rectangle.y as u32,
                    width : rectangle.width as u32,
                    height : rectangle.height as u32 }
    }
}

impl<'a> std::convert::From<&'a cairo::RectangleInt> for Rectangle {
    fn from(rectangle: &'a cairo::RectangleInt) -> Rectangle {
        Rectangle { x : rectangle.x as u32,
                    y: rectangle.y as u32,
                    width : rectangle.width as u32,
                    height : rectangle.height as u32 }
    }
} 

#[cfg(test)]
mod tests {
    use cairo;
    use std;
    #[test]
    fn from_cairo() {
        let cairo = cairo::RectangleInt {
            x : 0,
            y : 0,
            width : 0,
            height : 0
        };
        let my : Rectangle = std::convert::From<&cairo::Rectangle>(&cairo);
    }
}

