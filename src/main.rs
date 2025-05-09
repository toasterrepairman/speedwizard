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
use gtk::{DrawingArea, CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION};
use glib::clone;

mod data;

#[derive(Debug, Deserialize)]
struct Record {
    system: String,
    name: String,
    object_class: String,
    atmospheric_pressure: String,
}

fn create_pressure_gauge(pressure: f64) -> gtk::Box {
    let box_container = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .vexpand(true)
        .hexpand(true)
        .build();

    let drawing_area = DrawingArea::builder()
        .content_width(120)
        .content_height(16)
        .vexpand(true)
        .hexpand(true)
        .halign(gtk::Align::Fill)
        .build();

    // Calculate color based on pressure (0-3 atms)
    // Normalize pressure to 0-1 range for color calculation
    let max_pressure = 3.0;
    let normalized_pressure = (pressure / max_pressure).min(1.0).max(0.0);

    drawing_area.set_draw_func(move |_, cr, width, height| {
        // Make the bar thinner
        let bar_height = height as f64 * 0.35; // Thinner bar (50% of container height)
        let y_offset = (height as f64 - bar_height) / 2.0; // Center vertically

        // Set transparent background
        cr.set_operator(gtk::cairo::Operator::Clear);
        cr.paint().expect("Invalid cairo surface state");
        cr.set_operator(gtk::cairo::Operator::Over);

        // Calculate fill width based on pressure
        let fill_width = width as f64 * (pressure / max_pressure).min(1.0);

        // Draw the background track with rounded corners
        let radius = bar_height / 2.0;
        let x = 0.0;
        let y = y_offset;
        let w = width as f64;
        let h = bar_height;

        // Background track (very subtle)
        cr.new_sub_path();
        cr.arc(x + w - radius, y + radius, radius, -90.0 * std::f64::consts::PI / 180.0, 90.0 * std::f64::consts::PI / 180.0);
        cr.arc(x + w - radius, y + h - radius, radius, 0.0, 90.0 * std::f64::consts::PI / 180.0);
        cr.line_to(x + radius, y + h);
        cr.arc(x + radius, y + h - radius, radius, 90.0 * std::f64::consts::PI / 180.0, 180.0 * std::f64::consts::PI / 180.0);
        cr.arc(x + radius, y + radius, radius, 180.0 * std::f64::consts::PI / 180.0, 270.0 * std::f64::consts::PI / 180.0);
        cr.close_path();

        cr.set_source_rgba(0.8, 0.8, 0.8, 0.2);
        cr.fill().expect("Invalid cairo surface state");

        // Only draw fill if we have some pressure
        if fill_width > 0.0 {
            // Calculate color based on normalized pressure - desaturated blue to red gradient
            let red = 0.4 + (normalized_pressure * 0.5);  // 0.4-0.9 range
            let green = 0.4 - (normalized_pressure * 0.2); // 0.4-0.2 range
            let blue = 0.8 - (normalized_pressure * 0.5);  // 0.7-0.2 range
            cr.set_source_rgb(red, green, blue);

            // Create a clipping path with rounded corners for the fill area
            cr.save();

            // Same rounded rectangle path as background
            cr.new_sub_path();
            cr.arc(x + w - radius, y + radius, radius, -90.0 * std::f64::consts::PI / 180.0, 90.0 * std::f64::consts::PI / 180.0);
            cr.arc(x + w - radius, y + h - radius, radius, 0.0, 90.0 * std::f64::consts::PI / 180.0);
            cr.line_to(x + radius, y + h);
            cr.arc(x + radius, y + h - radius, radius, 90.0 * std::f64::consts::PI / 180.0, 180.0 * std::f64::consts::PI / 180.0);
            cr.arc(x + radius, y + radius, radius, 180.0 * std::f64::consts::PI / 180.0, 270.0 * std::f64::consts::PI / 180.0);
            cr.close_path();

            // Create the clip path
            cr.clip();

            // Draw the fill rectangle (will be clipped by the rounded corners)
            cr.rectangle(x, y, fill_width, h);
            cr.fill().expect("Invalid cairo surface state");

            cr.restore();
        }
    });

    box_container.append(&drawing_area);
    box_container
}

fn build_ui(app: &AdwApplication) -> Result<(), Box<dyn Error>> {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Speedwizard")
        .default_width(600)  // Increased width to accommodate gauge
        .default_height(600)
        .build();

    let header = HeaderBar::builder()
        .show_title_buttons(true)
        .css_classes(["flat"])
        .build();

    let list_box = gtk::ListBox::new();
    list_box.set_selection_mode(gtk::SelectionMode::None);
    list_box.add_css_class("boxed-list");

    let mut rdr = ReaderBuilder::new().from_reader(data::CSV_DATA.as_bytes());
    let mut records: Vec<Record> = rdr.deserialize().filter_map(Result::ok).collect();

    records.sort_by(|a, b| a.system.to_lowercase().cmp(&b.system.to_lowercase()));

    for record in records {
        let title = &record.name;
        let subtitle = format!("{} ({}) – {} atms", record.system, record.object_class, record.atmospheric_pressure);

        // Parse the atmospheric pressure
        let pressure = record.atmospheric_pressure.trim_end_matches(" atms")
            .parse::<f64>().unwrap_or(0.0);

        // Create the horizontal box to hold both the row and gauge
        let h_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(6)
            .margin_start(0)
            .margin_end(12)
            .margin_top(6)
            .margin_bottom(6)
            .focusable(true)
            .build();

        // Create the row with title and subtitle
        let row = ActionRow::builder()
            .title(title)
            .subtitle(&subtitle)
            .hexpand(false)
            .width_request(0)  // Set minimum width to 150 pixels
            .build();

        // Create pressure gauge widget
        let gauge = create_pressure_gauge(pressure);

        // Add both to the horizontal box
        h_box.append(&row);
        h_box.append(&gauge);

        // Create a list box row to contain our horizontal box
        let list_row = gtk::ListBoxRow::new();
        list_row.set_child(Some(&h_box));
        list_row.set_activatable(false);
        list_row.set_selectable(false);

        list_box.append(&list_row);
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
