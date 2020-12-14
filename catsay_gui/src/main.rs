#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unused_assignments)]

extern crate gio;
extern crate gtk;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Image, Label, Orientation};
use std::env;

/*fn gui() {
    let app = Application::new(
        Option::from("com.john.catsay-gui"),
        gio::ApplicationFlags::empty()
    ).expect("Failed to initialize GTK.");

    app.connect_startup(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Catsay");
        window.set_default_size(350, 70);
        window.connect_delete_event(|win, _| unsafe {
            win.destroy();
            Inhibit(false) // Don't prevent the default behavior (i.e close)
        });
        window.show_all();
    });

    app.connect_activate(|_| {});
    app.run(&env::args().collect::<Vec<_>>());
}*/

fn main() {
    let app = Application::new(
        Option::from("com.john.catsay-gui"),
        gio::ApplicationFlags::empty()
    ).expect("Failed to initialize GTK.");

    app.connect_startup(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Catsay");
        window.set_default_size(350, 70);
        window.connect_delete_event(|win, _| unsafe {
            win.destroy();
            Inhibit(false) // Don't prevent the default behavior (i.e close)
        });
        let layout_box = Box::new(Orientation::Vertical, 0);
        let label = Label::new(Option::from("Meow!\n      \\\n      \\"));
        layout_box.add(&label);

        let cat_image = Image::from_file("./images/cat.jpg");
        layout_box.add(&cat_image);

        window.add(&layout_box);
        window.show_all();
    });

    app.connect_activate(|_| {});
    app.run(&env::args().collect::<Vec<_>>());
}
