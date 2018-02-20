use cairo;
use gtk;
use gdk;
use failure;
use ndarray;

use basic_types;
use labyrinth;

use basic_types::{IsARectangle, IsARectangularArea, IsAColor}; 

#[derive(Debug)]
pub struct EventHandler;

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {}
    }
    pub fn on_size_allocate(
        &mut self,
        state: &mut labyrinth::LabyrinthState,
        rect: &basic_types::Rectangle,
    ) {
        if rect.width > 0 && rect.height > 0 {
            state.width = rect.width as u32;
            state.height = rect.height as u32;
            state.labyrinth = Some(labyrinth::Labyrinth::new(
                state.box_size,
                state.width,
                state.height,
            ));
        } else {
            state.labyrinth = None;
            state.width = 0;
            state.height = 0;
        }
    }
    pub fn on_draw(
        &mut self,
        state: &mut labyrinth::LabyrinthState,
        cairo_context: &cairo::Context,
    ) -> Result<(), failure::Error> {
        if let Some(labyrinth) = state.labyrinth.as_mut() {
            self.draw(labyrinth, cairo_context)
        } else {
            Ok(())
        }
    }
    pub fn on_button_press(
        &mut self,
        drawing_area: &gtk::DrawingArea,
        state: &mut labyrinth::LabyrinthState,
        event: &gdk::EventButton,
    ) -> Result<(), failure::Error> {
        if let Some(ref mut labyrinth) = state.labyrinth {
            self.handle_mark_box(drawing_area, labyrinth, event.get_position())?;
        }
        Ok(())
    }
    pub fn on_motion_notify(
        &mut self,
        drawing_area: &gtk::DrawingArea,
        state: &mut labyrinth::LabyrinthState,
        event: &gdk::EventMotion,
    ) -> Result<(), failure::Error> {
        if let Some(ref mut labyrinth) = state.labyrinth {
            if event.get_state() & gdk::ModifierType::BUTTON1_MASK != gdk::ModifierType::empty() {
                self.handle_mark_box(drawing_area, labyrinth, event.get_position())?;
            }
        }
        Ok(())
    }
    fn draw(
        &mut self,
        labyrinth: &mut labyrinth::Labyrinth,
        cairo_context: &cairo::Context,
    ) -> Result<(), failure::Error> {
        let (top_left_x, top_left_y, bottom_right_x, bottom_right_y) = cairo_context.clip_extents();
        let rectangle = basic_types::Rectangle::approx_from(&(top_left_x, top_left_y, bottom_right_x - top_left_x,
                                                              bottom_right_y - top_left_y))?;
        self.draw_axes(rectangle, labyrinth, cairo_context);
        Ok(())
    }
    fn draw_axes(
        &self,
        rectangle: basic_types::Rectangle,
        labyrinth: &labyrinth::Labyrinth,
        cairo_context: &cairo::Context,
    ) {
        use basic_types::IsAColor;
        let color = basic_types::Color::get_black();
        cairo_context.save();
        cairo_context.set_source_rgb(color.red(), color.green(), color.blue());

        if self.is_inside_bounds(rectangle, labyrinth) {
            self.draw_axes_x(rectangle, labyrinth, cairo_context);
            // self.draw_axes_y(rectangle, labyrinth, cairo_context);

            // for y_cnt in 0..(labyrinth.y_box_cnt + 1) {
            //     let y_pos = labyrinth.y + labyrinth.box_size * y_cnt;
            //     cairo_context.move_to(f64::from(labyrinth.x), f64::from(y_pos));
            //     cairo_context.line_to(f64::from(labyrinth.x + labyrinth.width), f64::from(y_pos));
            // }
        }
        cairo_context.stroke();
        cairo_context.restore();
    }
    fn is_inside_bounds(&self, rectangle: basic_types::Rectangle, labyrinth: &labyrinth::Labyrinth) -> bool {
        rectangle.bottom_right_x() >= labyrinth.x && 
        rectangle.top_left_x() <= labyrinth.x + labyrinth.width &&
        rectangle.bottom_right_y() >= labyrinth.y &&
        rectangle.top_left_y() <= labyrinth.y + labyrinth.height 
    } 
    fn draw_axes_x(&self, rectangle: basic_types::Rectangle, labyrinth: &labyrinth::Labyrinth, cairo_context: &cairo::Context) {   
        use std::cmp::{min,max};
        let start_x_cnt = if rectangle.top_left_x() > labyrinth.x { ( rectangle.top_left_x() - labyrinth.x + labyrinth.box_size - 1 ) / labyrinth.box_size } else { 0 };
        let end_x_cnt = min(labyrinth.x_box_cnt + 1, ( rectangle.bottom_right_x() - labyrinth.x + labyrinth.box_size - 1 ) / labyrinth.box_size);

        for x_cnt in start_x_cnt..end_x_cnt {
            let x_pos = labyrinth.x + labyrinth.box_size * x_cnt;
            cairo_context.move_to(f64::from(x_pos), f64::from(max(labyrinth.y, rectangle.top_left_y())));
            cairo_context.line_to(f64::from(x_pos), f64::from(min(labyrinth.y + labyrinth.height, rectangle.bottom_right_y())));
        } 
        println!("{} -> {}", start_x_cnt, end_x_cnt); 
    }
    // fn draw_marked_boxes(
    //     &mut self,
    //     labyrinth: &mut labyrinth::Labyrinth,
    //     cairo_context: &cairo::Context,
    // ) -> Result<(), failure::Error> {
    //     cairo_context.save();
    //     for (index, _) in labyrinth.marked.indexed_iter().filter(|&(_, &elem)| elem) {
    //         let (x_box, y_box) = (index[0] as u32, index[1] as u32);
    //         if let Some(rectangle) = labyrinth.box_to_pixel((x_box, y_box)) {
    //             let float_rectangle: basic_types::GeneralRectangle<f64> = rectangle.to()?;
    //             cairo_context.rectangle(
    //                 float_rectangle.x(),
    //                 float_rectangle.y(),
    //                 float_rectangle.width(),
    //                 float_rectangle.height(),
    //             );
    //             cairo_context.fill();
    //         }
    //     }
    //     cairo_context.restore();
    //     Ok(())
    // }
    // fn repaint_box(&self, cairo_context: &cairo::Context) -> Result<(), failure::Error> {
    //     let color = self.to_be_painted.color;
    //     let rectangle = self.to_be_painted.rectangle;
    //     let float_rectangle: basic_types::GeneralRectangle<f64> = rectangle.to()?;
    //     cairo_context.save();
    //     cairo_context.set_source_rgb(color.0, color.1, color.2);
    //     cairo_context.rectangle(
    //         float_rectangle.x(),
    //         float_rectangle.y(),
    //         float_rectangle.width(),
    //         float_rectangle.height(),
    //     );
    //     cairo_context.fill();
    //     cairo_context.restore();
    //     Ok(())
    // }
    fn handle_mark_box(
        &mut self,
        drawing_area: &gtk::DrawingArea,
        labyrinth: &mut labyrinth::Labyrinth,
        position: (f64, f64),
    ) -> Result<(), failure::Error> {
        use gtk::WidgetExt;
        let clicked_box = labyrinth.pixel_to_box((position.0 as u32, position.1 as u32));
        if let Some(clicked_box) = clicked_box {
            if self.update_marked(labyrinth, clicked_box) {
                let rectangle = labyrinth.box_to_pixel::<i32>(clicked_box)?;
                drawing_area.queue_draw_area(rectangle.x, rectangle.y,
                                             rectangle.width, rectangle.height);  
            }
        }
        Ok(())
    }
    fn update_marked(
        &mut self,
        labyrinth: &mut labyrinth::Labyrinth,
        clicked_box: (u32, u32),
    ) -> bool {
        if let Some(marked) = labyrinth.marked.get_mut(ndarray::IxDyn(&[
            clicked_box.0 as usize,
            clicked_box.1 as usize,
        ])) {
            if !*marked {
                *marked = true;
                return true;
            }
        }
        false
    }
}
