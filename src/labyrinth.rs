use ndarray;

const BOX_SIZE: i32 = 64;  

#[derive(Debug)]
pub struct Labyrinth {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    marked: ndarray::ArrayD<bool>,
}
                                                                
impl Labyrinth {
    pub fn new(total_width: i32, total_height: i32) -> Labyrinth {
        const MARGIN_FACTOR : i32 = 32;
        let left_margin = total_width / MARGIN_FACTOR;
        let top_margin = total_height / MARGIN_FACTOR;
        let width = (total_width - 2 * left_margin) / BOX_SIZE * BOX_SIZE;
        let height = (total_height - 2 * top_margin) / BOX_SIZE * BOX_SIZE;
        Labyrinth {
            x: total_width / 2 - width / 2,
            y: total_height / 2 - height / 2,
            width: width,
            height: height,
            marked: ndarray::ArrayD::<bool>::default(ndarray::IxDyn(&[
                (width / BOX_SIZE) as usize,
                (height / BOX_SIZE) as usize,
            ])),
        }
    }
    // pub fn pixel_to_box(&self, (x,y) : (i32, i32)) -> Option<(i32, i32)> {
    //     if x < self.x || x >= self.x + self.width ||
    //        y < self.y || y >= self.y + self.height {
    //            None
    //        } else {
    //            Some( ( ( x - self.x ) % BOX_SIZE, ( y - self.y ) % BOX_SIZE ) )
    //        }
    // }
}
