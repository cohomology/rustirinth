use std;
use labyrinth;

#[derive(Debug)]
pub struct LabyrinthState {
    pub width: i32,
    pub height: i32,
    pub labyrinth: std::option::Option<labyrinth::Labyrinth>,
}

impl LabyrinthState {
    pub fn new((width, height): (i32, i32)) -> LabyrinthState {
        LabyrinthState {
            width: width,
            height: height,
            labyrinth: None,
        }
    }
}
