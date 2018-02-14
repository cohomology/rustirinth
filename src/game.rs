struct LabyrinthGame {
    main_window: std::rc::Rc<LabyrinthMainWindow>,
}

impl LabyrinthGame {
    fn run() -> Result<(), failure::Error> {
        gtk::init()?;
        let game = LabyrinthGame::initialize_screen()?;
        gtk::main();
        Ok(game)
    }
    fn initialize_screen() -> Result<LabyrinthGame, failure::Error> {
        match gdk::Screen::get_default() {
            Some(screen) => Ok(LabyrinthGame::initialize_window(&screen)),
            None => Err(LabyrinthError::CouldNotGetDefaultScreen.into()),
        }
    }
    fn initialize_window(screen: &gdk::Screen) -> LabyrinthGame {
        LabyrinthGame {
            main_window: std::rc::Rc::new(LabyrinthMainWindow::new(screen)),
        }.connect_delete_event()
         .connect_key_press_event()
         .connect_button_press_event()
         .connect_on_size_allocate_event()
         .connect_on_draw_event()
         .show_all()
    }
    fn connect_delete_event(self) -> Self {
        gtk::WidgetExt::connect_delete_event(self.main_window.window, |_, _| {
            gtk::main_quit();
            gtk::Inhibit(true)
        });
        self
    }
    fn connect_key_press_event(self) -> Self {
        use gtk::WidgetExt;
        self.main_window.window.connect_key_press_event(|_, key| {
            if key.get_keyval() == gdk::enums::key::Escape {
                gtk::main_quit();
            }
            gtk::Inhibit(true)
        });
        self
    }
    fn connect_button_press_event(self) -> Self {
        use gtk::WidgetExt;
        let window_instance = self.main_window.clone();
        self.main_window
            .drawing_area
            .connect_button_press_event(move |_, event| {
                window_instance.state.borrow_mut().on_button_press(event);
                gtk::Inhibit(true)
            });
        self
    }
    fn connect_on_size_allocate_event(self) -> Self {
        use gtk::WidgetExt;
        let window_instance = self.main_window.clone();
        self.main_window
            .drawing_area
            .connect_size_allocate(move |_, rect| {
                window_instance.state.borrow_mut().on_size_allocate(rect);
            });
        self
    }
    fn connect_on_draw_event(self) -> Self {
        use gtk::WidgetExt;
        let window_instance = self.main_window.clone();
        self.main_window
            .drawing_area
            .connect_draw(move |_, cairo_context| {
                window_instance.state.borrow_mut().on_draw(cairo_context);
                gtk::Inhibit(true)
            });
        self
    }
    fn show_all(self) -> Self {
        use gtk::WidgetExt;
        self.main_window.window.show_all();
        self
    }
} 
