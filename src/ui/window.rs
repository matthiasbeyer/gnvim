use gtk::prelude::*;

use ui::grid::Grid;

pub struct Window {
    fixed: gtk::Fixed,
    frame: gtk::Frame,

    pub x: u64,
    pub y: u64,

    /// Currently shown's grid id.
    pub grid_id: u64,
    pub id: u64,
}

impl Window {
    pub fn new(id: u64, fixed: gtk::Fixed, grid: &Grid) -> Self {
        let frame = gtk::Frame::new(None);
        fixed.put(&frame, 0, 0);

        gtk::WidgetExt::set_name(&frame, &format!("Window #{}", id));

        let widget = grid.widget();
        frame.add(&widget);

        Self {
            fixed,
            frame,
            grid_id: grid.id,
            id,
            x: 0,
            y: 0,
        }
    }

    pub fn set_position(&mut self, x: u64, y: u64, w: u64, h: u64) {
        self.x = x;
        self.y = y;
        self.fixed.move_(&self.frame, x as i32, y as i32);

        self.frame.set_size_request(w as i32, h as i32);
    }

    pub fn show(&mut self) {
        //if let Some(child) = self.frame.get_child() {
            //self.frame.remove(&child);
        //}

        self.frame.show_all();
    }

    pub fn hide(&self) {
        self.frame.hide();
    }
}
