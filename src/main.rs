use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let builder = gtk::Builder::from_file("./layout/dog.xml");

    if let Some(window) = builder.object::<gtk::Window>("applicationWindow1") {
        window.set_application(Some(app));
        window.show_all();
    } else {
        eprintln!("Failed to get window with an id of 'applicationWindow1'");
        std::process::exit(1);
    }
}

fn main() {
    let app = gtk::Application::new(
        Some("com.navinczernev.dogsay-gui"),
        Default::default()
    );

    app.connect_activate(build_ui);

    app.run();
}
