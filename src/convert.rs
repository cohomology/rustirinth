use basic_types::LabyrinthError;
use conv::{ValueInto, ValueFrom, ApproxInto, ApproxFrom};
use failure::Error;

pub trait ConvertFrom<T> {
    fn convert_from(T) -> Result<Self, Error>;
} 

pub trait ConvertInto<T> {
    fn convert_into(&self) -> Result<T, Error>;
};

impl<T> ConvertFrom<T> for T {
    fn convert_from(T) {
        Ok(T)
    }
}

impl ConvertFrom<u32> for i32 {
    fn convert_from(value : T) {
        ValueFrom::from(value)
    }
}


