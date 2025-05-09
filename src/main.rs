use gtk4::prelude::*;
use adw::prelude::*;
use adw::{glib, gio};  // Import glib through gtk4
use gtk4 as gtk;
use gio::SimpleAction;
use gtk::{Orientation, HeaderBar};
use adw::{Application as AdwApplication, ApplicationWindow, ActionRow};
use std::fs::File;
use std::error::Error;
use serde::Deserialize;
use csv::ReaderBuilder;

mod data;

#[derive(Debug, Deserialize)]
struct Record {
    system: String,
    name: String,
    object_class: String,
    atmospheric_pressure: String,
}

fn build_ui(app: &AdwApplication) -> Result<(), Box<dyn Error>> {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Speedwizard")
        .default_width(400)
        .default_height(600)
        .build();

    let header = HeaderBar::builder()
        .show_title_buttons(true)
        .css_classes(["flat"])
        .build();

    let list_box = gtk::ListBox::new();
    list_box.set_selection_mode(gtk::SelectionMode::None);

    let mut rdr = ReaderBuilder::new().from_reader(data::CSV_DATA.as_bytes());
    let mut records: Vec<Record> = rdr.deserialize().filter_map(Result::ok).collect();

    records.sort_by(|a, b| a.system.to_lowercase().cmp(&b.system.to_lowercase()));

    for record in records {
        let title = &record.name;
        let subtitle = format!("{} ({}) – {} atms", record.system, record.object_class, record.atmospheric_pressure);

        let row = ActionRow::builder()
            .title(title)
            .subtitle(&subtitle)
            .build();

        list_box.append(&row);
    }

    let scrolled = gtk::ScrolledWindow::builder()
        .child(&list_box)
        .vexpand(true)
        .min_content_height(600)
        .build();

    let content = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    content.append(&header);
    content.append(&scrolled);
    window.set_content(Some(&content));
    window.show();

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = AdwApplication::builder()
        .application_id("com.example.Speedwizard")
        .build();

    app.connect_activate(|app| {
        if let Err(err) = build_ui(app) {
            eprintln!("Error: {}", err);
        }
    });

    app.run();
    Ok(())
}
