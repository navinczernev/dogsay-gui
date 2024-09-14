use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Image, Label, Orientation};

fn main() {
    let app = Application::new(
        Some("com.navinczernev.dogsay-gui"),
        Default::default()
    );

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Dogsay");
        window.set_default_size(500, 590);

        let layout_box = GtkBox::new(Orientation::Vertical, 0);

        let label = Label::new(
            Some("Woof!\n     \\\n       \\")
        );
        layout_box.add(&label);
        let dog_image = Image::from_file(
            "./images/dog.png"
        );
        layout_box.add(&dog_image);

        window.add(&layout_box);
        window.show_all();
    });

    app.run();
}
