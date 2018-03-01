#![allow(dead_code)]

use std::ops::{Add, Sub};
use conv::{ApproxFrom, ApproxInto, DefaultApprox};
use failure::Error;

pub trait IsARectangle<T> {
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn width(&self) -> T;
    fn height(&self) -> T;
}

pub trait IsARectangularArea<T> {
    fn top_left_x(&self) -> T;
    fn top_left_y(&self) -> T;
    fn bottom_right_x(&self) -> T;
    fn bottom_right_y(&self) -> T;
}

#[derive(Debug, Eq, PartialEq)]
pub struct GeneralRectangle<T>
where
    T: Copy + Clone + PartialOrd + Add<Output = T> + Sub<Output = T>,
{
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

pub type Rectangle = GeneralRectangle<u32>;

impl<T> IsARectangle<T> for GeneralRectangle<T>
where
    T: Copy + Clone + PartialOrd + Add<Output = T> + Sub<Output = T>,
{
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

impl<T, S> IsARectangularArea<T> for S
where
    S: IsARectangle<T>,
    T: Copy + Clone + PartialOrd + Add<Output = T> + Sub<Output = T>,
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

#[derive(Debug, Eq, PartialEq)]
pub struct GeneralRectangularArea<T>
where
    T: Copy + Clone + PartialOrd + Sub<Output = T>,
{
    pub top_left_x: T,
    pub top_left_y: T,
    pub bottom_right_x: T,
    pub bottom_right_y: T,
}

pub type RectangularArea = GeneralRectangularArea<u32>;

impl<T> From<(T, T, T, T)> for GeneralRectangle<T>
where
    T: Copy + Clone + PartialOrd + Add<Output = T> + Sub<Output = T>,
{
    fn from((x, y, width, height): (T, T, T, T)) -> Self {
        GeneralRectangle {
            x,
            y,
            width,
            height,
        }
    }
}

impl<T> From<GeneralRectangle<T>> for GeneralRectangularArea<T>
where
    T: Copy + Clone + PartialOrd + Add<Output = T> + Sub<Output = T>,
{
    fn from(
        GeneralRectangle {
            x,
            y,
            width,
            height,
        }: GeneralRectangle<T>,
    ) -> Self {
        GeneralRectangularArea {
            top_left_x: x,
            top_left_y: y,
            bottom_right_x: x + width,
            bottom_right_y: y + height,
        }
    }
}

impl<T> From<GeneralRectangularArea<T>> for GeneralRectangle<T>
where
    T: Copy + Clone + PartialOrd + Add<Output = T> + Sub<Output = T>,
    T: From<u32>,
{
    fn from(
        GeneralRectangularArea {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
        }: GeneralRectangularArea<T>,
    ) -> Self {
        GeneralRectangle {
            x: top_left_x,
            y: top_left_y,
            width: if bottom_right_x >= top_left_x {
                bottom_right_x - top_left_x
            } else {
                0.into()
            },
            height: if bottom_right_y >= top_left_y {
                bottom_right_y - top_left_y
            } else {
                0.into()
            },
        }
    }
}

fn partial_max<T: PartialOrd>(a: T, b: T) -> T {
    if b > a {
        b
    } else {
        a
    }
}

fn partial_min<T: PartialOrd>(a: T, b: T) -> T {
    if b < a {
        b
    } else {
        a
    }
}

impl<T> GeneralRectangle<T>
where
    T: Copy + Clone + PartialOrd + Add<Output = T> + Sub<Output = T>,
{
    pub fn from<S>(rectangle: &GeneralRectangle<S>) -> Result<GeneralRectangle<T>, Error>
    where
        T: ApproxFrom<S, DefaultApprox>,
        S: Copy + Clone + PartialOrd + Add<Output = S> + Sub<Output = S>,
        <T as ApproxFrom<S>>::Err: Send + Sync + 'static,
    {
        let x = rectangle.x.approx_into()?;
        let y = rectangle.y.approx_into()?;
        let width = rectangle.width.approx_into()?;
        let height = rectangle.height.approx_into()?;
        Ok(GeneralRectangle::<T> {
            x,
            y,
            width,
            height,
        })
    }
    pub fn to<S>(&self) -> Result<GeneralRectangle<S>, Error>
    where
        S: ApproxFrom<T, DefaultApprox>,
        S: Copy + Clone + PartialOrd + Add<Output = S> + Sub<Output = S>,
        <S as ApproxFrom<T>>::Err: Send + Sync + 'static,
    {
        let x = self.x.approx_into()?;
        let y = self.y.approx_into()?;
        let width = self.width.approx_into()?;
        let height = self.height.approx_into()?;
        Ok(GeneralRectangle {
            x,
            y,
            width,
            height,
        })
    }
    pub fn intersect(&self, other: &GeneralRectangle<T>) -> Option<GeneralRectangle<T>> {
        if self.inside_bounds(other) {
            let top_left_x = partial_max(self.x, other.x);
            let top_left_y = partial_max(self.y, other.y);
            let bottom_right_x = partial_min(self.bottom_right_x(), other.bottom_right_x());
            let bottom_right_y = partial_min(self.bottom_right_y(), other.bottom_right_y());
            Some(GeneralRectangle {
                x: top_left_x,
                y: top_left_y,
                width: bottom_right_x - top_left_x,
                height: bottom_right_y - top_left_y,
            })
        } else {
            None
        }
    }
    fn inside_bounds(&self, other: &GeneralRectangle<T>) -> bool {
        other.bottom_right_x() >= self.x && other.x <= self.x + self.width && other.y + other.height >= self.y
            && other.y <= self.y + self.height
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn general_rectangle_overflow() {
        let big_rectangle = GeneralRectangle::<u64> {
            x: 1,
            y: 4_294_967_296,
            width: 3,
            height: 4,
        };
        let rectangle = big_rectangle.to::<u32>();
        assert!(rectangle.is_err());
    }

    #[test]
    fn float_overflow() {
        let big_rectangle = (1.0, 4294967296.0, 3.0, 4.0);
        let rectangle = Rectangle::from(big_rectangle.into());
        assert!(rectangle.is_err());
        let error = rectangle.err().unwrap();
    }
}
