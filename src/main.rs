use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let builder = gtk::Builder::from_file("./layout/dog.glade");

    let window: gtk::Window = builder.object("applicationWindow1")
                                        .expect("Failed to get window with an id of 'applicationWindow1'");
    window.set_application(Some(app));

    let message_input: gtk::Entry = builder.object("inputMessage1")
                                            .expect("Failed to get label with an id of 'inputMessage1'");

    let is_dead_switch: gtk::Switch = builder.object("isDeadSwitch1")
                                                .expect("Failed to get label with an id of 'isDeadSwitch1'");

    let generate_button: gtk::Button = builder.object("generateButton1")
                                                .expect("Failed to get button with an id of 'generateButton1'");

    let message_output: gtk::Label = builder.object("outputMessageLabel1")
                                                .expect("Failed to get label with an id of 'outputMessageLabel1'");

    let image_output: gtk::Image = builder.object("outputImage1")
                                            .expect("Failed to get label with an id of 'outputImage1'");

    let message_output_clone = message_output.clone();
    let image_output_clone = image_output.clone();

    generate_button.connect_clicked(move |_| {
        message_output_clone.set_text(
            &format!("{}\n   \\\n     \\", message_input.text().as_str())
        );
        message_output_clone.show();
        let is_dead = is_dead_switch.is_active();
        if is_dead {
            image_output_clone.set_from_file(
                Some("./images/dog-dead.png")
            )
        } else {
            image_output_clone.set_from_file(
                Some("./images/dog.png")
            )
        }
        image_output_clone.show();
    });

    window.show_all();
    message_output.hide();
    image_output.hide();
}

fn main() {
    let app = gtk::Application::new(
        Some("com.navinczernev.dogsay-gui"),
        Default::default()
    );

    app.connect_activate(build_ui);

    app.run();
}
