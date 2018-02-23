use cairo;
use gtk;
use gdk;

use std::cmp::{max, min};
use basic_types::{convert, Color, GeneralRectangle, IsAColor, IsARectangle, IsARectangularArea, Rectangle};
use labyrinth::{Labyrinth, LabyrinthState};
use failure::Error;
use ndarray::{Ix2 as Dim, SliceInfo, SliceOrIndex};
use gtk::WidgetExt;

#[derive(Debug)]
pub struct EventHandler;

impl EventHandler {
    pub fn new() -> EventHandler { EventHandler {} }
    pub fn on_size_allocate(&mut self, state: &mut LabyrinthState, rect: &Rectangle) -> Result<(), Error> {
        if rect.width > 0 && rect.height > 0 {
            let width = convert(rect.width)?;
            let height = convert(rect.height)?;
            state.labyrinth = Some(Labyrinth::new(state.box_size, width, height));
        } else {
            state.labyrinth = None;
        }
        Ok(())
    }
    pub fn on_draw(&mut self, state: &mut LabyrinthState, cairo_context: &cairo::Context) -> Result<(), Error> {
        if let Some(labyrinth) = state.labyrinth.as_mut() {
            self.draw(labyrinth, cairo_context)
        } else {
            Ok(())
        }
    }
    pub fn on_button_press(&mut self,
                           drawing_area: &gtk::DrawingArea,
                           state: &mut LabyrinthState,
                           event: &gdk::EventButton)
                           -> Result<(), Error> {
        if let Some(ref mut labyrinth) = state.labyrinth {
            const LEFT_MOUSE_BUTTON: u32 = 1;
            const RIGHT_MOUSE_BUTTON: u32 = 3;
            match event.get_button() {
                LEFT_MOUSE_BUTTON => {
                    self.handle_mark_box(drawing_area, labyrinth, event.get_position(), false)?;
                }
                RIGHT_MOUSE_BUTTON => {
                    self.handle_mark_box(drawing_area, labyrinth, event.get_position(), true)?;
                }
                _ => (),
            }
        }
        Ok(())
    }
    pub fn on_motion_notify(&mut self,
                            drawing_area: &gtk::DrawingArea,
                            state: &mut LabyrinthState,
                            event: &gdk::EventMotion)
                            -> Result<(), Error> {
        if let Some(ref mut labyrinth) = state.labyrinth {
            if event.get_state() & gdk::ModifierType::BUTTON1_MASK != gdk::ModifierType::empty() {
                self.handle_mark_box(drawing_area, labyrinth, event.get_position(), false)?;
            } else if event.get_state() & gdk::ModifierType::BUTTON3_MASK != gdk::ModifierType::empty() {
                self.handle_mark_box(drawing_area, labyrinth, event.get_position(), true)?;
            }
        }
        Ok(())
    }
    fn draw(&mut self, labyrinth: &mut Labyrinth, cairo_context: &cairo::Context) -> Result<(), Error> {
        let (top_left_x, top_left_y, bottom_right_x, bottom_right_y) = cairo_context.clip_extents();
        let draw_area =
            Rectangle::approx_from(&(top_left_x, top_left_y, bottom_right_x - top_left_x, bottom_right_y - top_left_y))?;
        if let Some(intersection) = draw_area.intersect(&labyrinth.rectangle) {
            self.draw_axes(&intersection, labyrinth, cairo_context)?;
            self.draw_boxes(&intersection, labyrinth, cairo_context)?;
            // self.draw_legend(&intersection, labyrinth, cairo_context)?;
        }
        Ok(())
    }
    fn draw_axes(&self, draw_area: &Rectangle, labyrinth: &Labyrinth, cairo_context: &cairo::Context) -> Result<(), Error> {
        let color = Color::get_black();
        cairo_context.save();
        cairo_context.set_source_rgb(color.red(), color.green(), color.blue());

        self.draw_axes_x(draw_area, labyrinth, cairo_context)?;
        self.draw_axes_y(draw_area, labyrinth, cairo_context)?;

        cairo_context.stroke();
        cairo_context.restore();
        Ok(())
    }
    fn draw_axes_x(&self, draw_area: &Rectangle, labyrinth: &Labyrinth, cairo_context: &cairo::Context) -> Result<(), Error> {
        let start_x_cnt = (draw_area.top_left_x() - labyrinth.rectangle.x + labyrinth.box_size - 1) / labyrinth.box_size;
        let end_x_cnt = min(labyrinth.x_box_cnt + 1,
                            (draw_area.bottom_right_x() - labyrinth.rectangle.x + labyrinth.box_size - 1) / labyrinth.box_size);

        for x_cnt in start_x_cnt..end_x_cnt {
            let start_x = labyrinth.rectangle.x + labyrinth.box_size * x_cnt;
            let start_y = max(labyrinth.rectangle.y, draw_area.top_left_y());
            self.draw_line(Rectangle { x: start_x,
                                        y: start_y,
                                        width: 0,
                                        height: labyrinth.rectangle.height, },
                            draw_area,
                            cairo_context)?;
        }
        Ok(())
    }
    fn draw_axes_y(&self, draw_area: &Rectangle, labyrinth: &Labyrinth, cairo_context: &cairo::Context) -> Result<(), Error> {
        let start_y_cnt = (draw_area.top_left_y() - labyrinth.rectangle.y + labyrinth.box_size - 1) / labyrinth.box_size;
        let end_y_cnt = min(labyrinth.y_box_cnt + 1,
                            (draw_area.bottom_right_y() - labyrinth.rectangle.y + labyrinth.box_size - 1) / labyrinth.box_size);

        for y_cnt in start_y_cnt..end_y_cnt {
            let start_x = max(labyrinth.rectangle.x, draw_area.top_left_x());
            let start_y = labyrinth.rectangle.y + labyrinth.box_size * y_cnt;
            self.draw_line(Rectangle { x: start_x,
                                        y: start_y,
                                        width: labyrinth.rectangle.width,
                                        height: 0, },
                            draw_area,
                            cairo_context)?;
        }
        Ok(())
    }
    fn draw_line(&self, line: Rectangle, draw_area: &Rectangle, cairo_context: &cairo::Context) -> Result<(), Error> {
        match draw_area.intersect(&line)
                       .map(|x| x.approx_to::<f64, GeneralRectangle<f64>>())
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
    fn draw_boxes(&mut self,
                  drawing_area: &Rectangle,
                  labyrinth: &mut Labyrinth,
                  cairo_context: &cairo::Context)
                  -> Result<(), Error> {
        let blue = Color::get_blue();
        let white = Color::get_white();
        cairo_context.save();
        let (x_range, y_range) = labyrinth.pixel_rectangle_to_box_range(drawing_area)?;
        let slice_x = SliceOrIndex::from(x_range.clone());
        let slice_y = SliceOrIndex::from(y_range.clone());
        let slice_args = SliceInfo::<[SliceOrIndex; 2], Dim>::new([slice_x, slice_y])?;
        for ((x_box, y_box), marked) in labyrinth.marked.slice(&slice_args).indexed_iter() {
            let box_rectangle = labyrinth.box_to_pixel((x_box + x_range.start, y_box + y_range.start))?;
            if let Some(intersection) = box_rectangle.intersect(drawing_area) {
                if *marked {
                    cairo_context.set_source_rgb(blue.red(), blue.green(), blue.blue());
                } else {
                    cairo_context.set_source_rgb(white.red(), white.green(), white.blue());
                }
                let float_rectangle: GeneralRectangle<f64> = intersection.to()?;
                cairo_context.rectangle(float_rectangle.x(),
                                        float_rectangle.y(),
                                        float_rectangle.width(),
                                        float_rectangle.height());
                cairo_context.fill();
            }
        }
        cairo_context.restore();
        Ok(())
    }
    fn handle_mark_box(&mut self,
                       drawing_area: &gtk::DrawingArea,
                       labyrinth: &mut Labyrinth,
                       (x, y): (f64, f64),
                       unmark: bool)
                       -> Result<(), Error> {
        let clicked_box = labyrinth.pixel_to_box((x as u32, y as u32));
        if let Some(clicked_box) = clicked_box {
            if self.update_marked(labyrinth, clicked_box, unmark) {
                let rectangle = labyrinth.box_to_pixel::<i32, u32>(clicked_box)?;
                drawing_area.queue_draw_area(rectangle.x, rectangle.y, rectangle.width, rectangle.height);
            }
        }
        Ok(())
    }
    fn update_marked(&mut self, labyrinth: &mut Labyrinth, (x, y): (u32, u32), unmark: bool) -> bool {
        if let Some(marked) = labyrinth.marked.get_mut(Dim(x as usize, y as usize)) {
            if !unmark && !*marked {
                *marked = true;
                return true;
            } else if unmark && *marked {
                *marked = false;
                return true;
            }
        }
        false
    }
}
