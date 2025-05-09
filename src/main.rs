use adw::prelude::*;
use gtk4 as gtk;
use gtk::{Orientation, HeaderBar};
use adw::{Application as AdwApplication, ApplicationWindow, ActionRow};
use std::fs::File;
use std::error::Error;
use serde::Deserialize;
use csv::Reader;

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

    // Load and sort CSV
    let file = File::open("planetary_atmospheres_normalized.csv")?;
    let mut rdr = Reader::from_reader(file);
    let mut records: Vec<Record> = rdr.deserialize().filter_map(Result::ok).collect();

    records.sort_by(|a, b| a.system.to_lowercase().cmp(&b.system.to_lowercase()));

    for record in records {
        let title = &record.name;
        let subtitle = format!("{} ({}) – {}", record.system, record.object_class, record.atmospheric_pressure);

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
        .application_id("com.example.PlanetaryList")
        .build();

    app.connect_activate(|app| {
        if let Err(err) = build_ui(app) {
            eprintln!("Error: {}", err);
        }
    });

    app.run();
    Ok(())
}
