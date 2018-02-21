use cairo;
use gtk;
use gdk;
use failure;
use ndarray;

use basic_types;
use labyrinth;

#[derive(Debug)]
pub struct EventHandler;

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {}
    }
    pub fn on_size_allocate(&mut self, state: &mut labyrinth::LabyrinthState, rect: &basic_types::Rectangle) {
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
    pub fn on_draw(&mut self, state: &mut labyrinth::LabyrinthState, cairo_context: &cairo::Context) -> Result<(), failure::Error> {
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
    fn draw(&mut self, labyrinth: &mut labyrinth::Labyrinth, cairo_context: &cairo::Context) -> Result<(), failure::Error> {
        let (top_left_x, top_left_y, bottom_right_x, bottom_right_y) = cairo_context.clip_extents();
        let draw_area = basic_types::Rectangle::approx_from(&(
            top_left_x,
            top_left_y,
            bottom_right_x - top_left_x,
            bottom_right_y - top_left_y,
        ))?;
        if let Some(intersection) = draw_area.intersect(&labyrinth.rectangle) {
            self.draw_axes(&intersection, labyrinth, cairo_context)?;
            self.draw_boxes(&intersection, labyrinth, cairo_context)?;
        }
        Ok(())
    }
    fn draw_axes(
        &self,
        draw_area: &basic_types::Rectangle,
        labyrinth: &labyrinth::Labyrinth,
        cairo_context: &cairo::Context,
    ) -> Result<(), failure::Error> {
        use basic_types::IsAColor;
        let color = basic_types::Color::get_black();
        cairo_context.save();
        cairo_context.set_source_rgb(color.red(), color.green(), color.blue());

        self.draw_axes_x(draw_area, labyrinth, cairo_context)?;
        self.draw_axes_y(draw_area, labyrinth, cairo_context)?;

        cairo_context.stroke();
        cairo_context.restore();
        Ok(())
    }
    fn draw_axes_x(
        &self,
        draw_area: &basic_types::Rectangle,
        labyrinth: &labyrinth::Labyrinth,
        cairo_context: &cairo::Context,
    ) -> Result<(), failure::Error> {
        use std::cmp::{max, min};
        use basic_types::IsARectangularArea;

        let start_x_cnt = (draw_area.top_left_x() - labyrinth.rectangle.x + labyrinth.box_size - 1) / labyrinth.box_size;
        let end_x_cnt = min(
            labyrinth.x_box_cnt + 1,
            (draw_area.bottom_right_x() - labyrinth.rectangle.x + labyrinth.box_size - 1) / labyrinth.box_size,
        );

        for x_cnt in start_x_cnt..end_x_cnt {
            let start_x = labyrinth.rectangle.x + labyrinth.box_size * x_cnt;
            let start_y = max(labyrinth.rectangle.y, draw_area.top_left_y());
            self.draw_line(
                basic_types::Rectangle {
                    x: start_x,
                    y: start_y,
                    width: 0,
                    height: labyrinth.rectangle.height,
                },
                draw_area,
                cairo_context,
            )?;
        }
        Ok(())
    }
    fn draw_axes_y(
        &self,
        draw_area: &basic_types::Rectangle,
        labyrinth: &labyrinth::Labyrinth,
        cairo_context: &cairo::Context,
    ) -> Result<(), failure::Error> {
        use std::cmp::{max, min};
        use basic_types::IsARectangularArea;

        let start_y_cnt = (draw_area.top_left_y() - labyrinth.rectangle.y + labyrinth.box_size - 1) / labyrinth.box_size;
        let end_y_cnt = min(
            labyrinth.y_box_cnt + 1,
            (draw_area.bottom_right_y() - labyrinth.rectangle.y + labyrinth.box_size - 1) / labyrinth.box_size,
        );

        for y_cnt in start_y_cnt..end_y_cnt {
            let start_x = max(labyrinth.rectangle.x, draw_area.top_left_x());
            let start_y = labyrinth.rectangle.y + labyrinth.box_size * y_cnt;
            self.draw_line(
                basic_types::Rectangle {
                    x: start_x,
                    y: start_y,
                    width: labyrinth.rectangle.width,
                    height: 0,
                },
                draw_area,
                cairo_context,
            )?;
        }
        Ok(())
    }
    fn draw_line(
        &self,
        line: basic_types::Rectangle,
        draw_area: &basic_types::Rectangle,
        cairo_context: &cairo::Context,
    ) -> Result<(), failure::Error> {
        use basic_types::IsARectangularArea;
        match draw_area
            .intersect(&line)
            .map(|x| x.approx_to::<f64, basic_types::GeneralRectangle<f64>>())
        {
            Some(Ok(intersection)) => {
                cairo_context.move_to(intersection.top_left_x(), intersection.top_left_y());
                cairo_context.line_to(intersection.bottom_right_x(), intersection.bottom_right_y());
                Ok(())
            }
            Some(Err(err)) => Err(err),
            None => Ok(()),
        }
    }
    fn draw_boxes(
        &mut self,
        drawing_area: &basic_types::Rectangle,
        labyrinth: &mut labyrinth::Labyrinth,
        cairo_context: &cairo::Context,
    ) -> Result<(), failure::Error> {
        use basic_types::{IsAColor, IsARectangle};
        let color = basic_types::Color::get_blue();
        cairo_context.save();
        cairo_context.set_source_rgb(color.red(), color.green(), color.blue());
        for (index, _) in labyrinth.marked.indexed_iter().filter(|&(_, &elem)| elem) {
            let (x_box, y_box) = (index[0] as u32, index[1] as u32);
            let box_rectangle = labyrinth.box_to_pixel((x_box, y_box))?;
            if let Some(intersection) = box_rectangle.intersect(drawing_area) {
                let float_rectangle: basic_types::GeneralRectangle<f64> = intersection.to()?;
                cairo_context.rectangle(
                    float_rectangle.x(),
                    float_rectangle.y(),
                    float_rectangle.width(),
                    float_rectangle.height(),
                );
                cairo_context.fill();
            }
        }
        cairo_context.restore();
        Ok(())
    }
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
                drawing_area.queue_draw_area(rectangle.x, rectangle.y, rectangle.width, rectangle.height);
            }
        }
        Ok(())
    }
    fn update_marked(&mut self, labyrinth: &mut labyrinth::Labyrinth, clicked_box: (u32, u32)) -> bool {
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
