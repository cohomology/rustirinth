use ndarray; 

pub struct Labyrinth {
    x : f64,
    y : f64,
    width : f64,
    height : f64,
    marked : ndarray::ArrayD<bool>,  
} 

impl Labyrinth {
    pub fn new(total_width : i32, total_height : i32) -> Labyrinth {
        const MARGIN_FACTOR : i32 = 16;
        let left_margin = total_width / MARGIN_FACTOR;
        let top_margin = total_height / MARGIN_FACTOR;
        let box_size : i32 = 64;
        let width = ( total_width - 2 * left_margin ) / box_size * box_size;
        let height = ( total_height - 2 * top_margin ) / box_size * box_size;
        Labyrinth {
            x : ( total_width / 2 - width / 2 ) as f64,
            y : ( total_height / 2 - height / 2 ) as f64,
            width : width as f64, 
            height : height as f64,
            marked : ndarray::ArrayD::<bool>::default(ndarray::IxDyn(&[(width / box_size) as usize, (height / box_size) as usize]))
        }
    }
}
