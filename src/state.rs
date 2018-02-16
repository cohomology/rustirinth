use std;
use labyrinth;

#[derive(Debug)]
pub struct LabyrinthState {
    pub width: u32,
    pub height: u32,
    pub labyrinth: std::option::Option<labyrinth::Labyrinth>,
}

impl LabyrinthState {
    pub fn new((width, height): (u32, u32)) -> LabyrinthState {
        LabyrinthState {
            width: width,
            height: height,
            labyrinth: None,
        }
    }
}
