extern crate piston;

fn main() {
    let mut window: GlutinWindow = WindowSettings::new("Main Window", (0,0))
        .fullscreen(true)
        .vsync(true)
        .build();
    println!("Hello, world!");
}
