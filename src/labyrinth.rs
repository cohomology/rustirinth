use ndarray; 

struct Labyrinth {
    x : f64,
    y : f64,
    width : f64,
    height : f64,
    // state : ndarray::ArrayD<bool>,  
}; 

impl Labyrinth {
    fn new(width : i32, height : i32) -> Labyrinth {
        const MARGIN_FACTOR : i32 = 16;
        let left_margin = width / MARGIN_FACTOR;
        let top_margin = height / MARGIN_FACTOR;
        let box_size : i32 = 64;
        Labyrinth {
            x : left_margin,
            y : top_margin,
            width : (width - 2 * left_margin) / box_size * box_size,
            height : (height - 2 * top_margin) / box_size * box_size
        }
    }
}
