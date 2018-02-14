pub struct LabyrinthState {
    width: i32,
    height: i32,
    labyrinth : std::option::Option<labyrinth::Labyrinth>
}

impl LabyrinthState {
    pub fn new(width : i32, height : i32) -> LabyrinthState {
        LabyrinthState {
            width: width,
            height: height,
            labyrinth : None, 
        }
    }   
};    
