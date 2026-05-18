use gtk4::prelude::*;
use adw::prelude::*;
use adw::{glib, gio};
use gtk4 as gtk;
use gtk::{Orientation, HeaderBar, SearchBar, SearchEntry, ToggleButton};
use adw::{Application as AdwApplication, ApplicationWindow, ActionRow, ExpanderRow};

use std::error::Error;
use serde::Deserialize;
use csv::ReaderBuilder;
use gtk::DrawingArea;
use glib::clone;

mod data;

#[derive(Debug, Deserialize, Clone)]
struct Record {
    system: String,
    name: String,
    object_class: String,
    atmospheric_pressure: String,
}

#[derive(Debug, Clone)]
struct PlanetaryBody {
    record: Record,
    pressure: f64,
}

fn create_pressure_gauge(pressure: f64) -> gtk::Box {
    let box_container = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .vexpand(true)
        .hexpand(false)
        .build();

    let drawing_area = DrawingArea::builder()
        .content_width(120)
        .content_height(16)
        .vexpand(true)
        .hexpand(false)
        .halign(gtk::Align::End)
        .build();

    let max_pressure = 3.0;
    let normalized_pressure = (pressure / max_pressure).min(1.0).max(0.0);

    drawing_area.set_draw_func(move |_, cr, width, height| {
        let bar_height = height as f64 * 0.35;
        let y_offset = (height as f64 - bar_height) / 2.0;

        cr.set_operator(gtk::cairo::Operator::Clear);
        cr.paint().expect("Invalid cairo surface state");
        cr.set_operator(gtk::cairo::Operator::Over);

        let fill_width = width as f64 * (pressure / max_pressure).min(1.0);

        let radius = bar_height / 2.0;
        let x = 0.0;
        let y = y_offset;
        let w = width as f64;
        let h = bar_height;

        cr.new_sub_path();
        cr.arc(x + w - radius, y + radius, radius, -90.0 * std::f64::consts::PI / 180.0, 90.0 * std::f64::consts::PI / 180.0);
        cr.arc(x + w - radius, y + h - radius, radius, 0.0, 90.0 * std::f64::consts::PI / 180.0);
        cr.line_to(x + radius, y + h);
        cr.arc(x + radius, y + h - radius, radius, 90.0 * std::f64::consts::PI / 180.0, 180.0 * std::f64::consts::PI / 180.0);
        cr.arc(x + radius, y + radius, radius, 180.0 * std::f64::consts::PI / 180.0, 270.0 * std::f64::consts::PI / 180.0);
        cr.close_path();

        cr.set_source_rgba(0.8, 0.8, 0.8, 0.2);
        cr.fill().expect("Invalid cairo surface state");

        if fill_width > 0.0 {
            let red = 0.4 + (normalized_pressure * 0.5);
            let green = 0.4 - (normalized_pressure * 0.2);
            let blue = 0.8 - (normalized_pressure * 0.5);
            cr.set_source_rgb(red, green, blue);

            cr.save();

            cr.new_sub_path();
            cr.arc(x + w - radius, y + radius, radius, -90.0 * std::f64::consts::PI / 180.0, 90.0 * std::f64::consts::PI / 180.0);
            cr.arc(x + w - radius, y + h - radius, radius, 0.0, 90.0 * std::f64::consts::PI / 180.0);
            cr.line_to(x + radius, y + h);
            cr.arc(x + radius, y + h - radius, radius, 90.0 * std::f64::consts::PI / 180.0, 180.0 * std::f64::consts::PI / 180.0);
            cr.arc(x + radius, y + radius, radius, 180.0 * std::f64::consts::PI / 180.0, 270.0 * std::f64::consts::PI / 180.0);
            cr.close_path();

            cr.clip();

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
        .default_width(600)
        .default_height(600)
        .build();

    let header = HeaderBar::builder()
        .show_title_buttons(true)
        .css_classes(["flat"])
        .build();

    let search_entry = SearchEntry::new();
    let search_bar = SearchBar::builder()
        .search_mode_enabled(false)
        .child(&search_entry)
        .build();

    let search_button = ToggleButton::builder()
        .icon_name("system-search-symbolic")
        .tooltip_text("Search (Ctrl+F)")
        .build();

    let title_label = gtk::Label::new(Some("Speedwizard"));
    title_label.add_css_class("title");
    header.pack_start(&search_button);
    header.set_title_widget(Some(&title_label));

    let list_box = gtk::ListBox::new();
    list_box.set_selection_mode(gtk::SelectionMode::Single);
    list_box.add_css_class("boxed-list");

    let mut rdr = ReaderBuilder::new().from_reader(data::CSV_DATA.as_bytes());
    let mut records: Vec<Record> = rdr.deserialize().filter_map(Result::ok).collect();

    records.sort_by(|a, b| {
        let system_cmp = a.system.to_lowercase().cmp(&b.system.to_lowercase());
        if system_cmp != std::cmp::Ordering::Equal {
            return system_cmp;
        }
        let class_order = |r: &Record| match r.object_class.as_str() {
            "Planet" => 0,
            "Protoplanet" => 1,
            "Moon" => 2,
            _ => 3,
        };
        class_order(a).cmp(&class_order(b))
    });

    let mut systems: std::collections::HashMap<String, Vec<PlanetaryBody>> = std::collections::HashMap::new();

    for record in records {
        let pressure = record.atmospheric_pressure.trim_end_matches(" atms")
            .parse::<f64>().unwrap_or(0.0);
        systems.entry(record.system.clone())
            .or_insert_with(Vec::new)
            .push(PlanetaryBody { record, pressure });
    }

    let mut system_names: Vec<String> = systems.keys().cloned().collect();
    system_names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    let mut all_rows: Vec<gtk::ListBoxRow> = Vec::new();

    for system_name in system_names {
        let bodies = systems.get(&system_name).unwrap();

        let expander_row = ExpanderRow::builder()
            .title(&system_name)
            .expanded(true)
            .selectable(false)
            .build();

        let mut planet_moons: std::collections::HashMap<String, Vec<PlanetaryBody>> = std::collections::HashMap::new();
        let mut current_planet: Option<String> = None;

        for body in bodies {
            match body.record.object_class.as_str() {
                "Planet" | "Protoplanet" => {
                    current_planet = Some(body.record.name.clone());
                    planet_moons.entry(body.record.name.clone())
                        .or_insert_with(Vec::new)
                        .push(body.clone());
                }
                "Moon" => {
                    if let Some(ref planet) = current_planet {
                        planet_moons.entry(planet.clone())
                            .or_insert_with(Vec::new)
                            .push(body.clone());
                    }
                }
                _ => {}
            }
        }

        let planet_names: Vec<String> = planet_moons.keys().cloned().collect();
        for planet_name in &planet_names {
            if let Some(bodies) = planet_moons.get(planet_name) {
                for body in bodies {
                    let is_moon = body.record.object_class == "Moon";
                    let is_planet = body.record.object_class == "Planet";
                    let title = &body.record.name;

                    let is_breathable = is_planet && body.pressure >= 0.8 && body.pressure <= 1.2;
                    let title_with_indicator = if is_breathable {
                        format!("{} 🌬️", title)
                    } else {
                        title.to_string()
                    };

                    let subtitle = if is_moon {
                        format!("Moon of {} – {} atms", planet_name, body.record.atmospheric_pressure)
                    } else {
                        format!("{} ({}) – {} atms", body.record.system, body.record.object_class, body.record.atmospheric_pressure)
                    };

                    let h_box = gtk::Box::builder()
                        .orientation(Orientation::Horizontal)
                        .spacing(6)
                        .margin_start(0)
                        .margin_end(12)
                        .margin_top(6)
                        .margin_bottom(6)
                        .focusable(true)
                        .build();

                    let row = ActionRow::builder()
                        .title(&title_with_indicator)
                        .subtitle(&subtitle)
                        .hexpand(true)
                        .build();

                    let gauge = create_pressure_gauge(body.pressure);

                    h_box.append(&row);
                    h_box.append(&gauge);

                    let list_row = gtk::ListBoxRow::new();
                    list_row.set_child(Some(&h_box));
                    list_row.set_activatable(true);
                    list_row.set_visible(true);

                    let gesture = gtk::GestureClick::new();
                    let title_for_popover = body.record.name.clone();
                    let is_breathable_for_popover = is_breathable;
                    let list_row_for_popover = list_row.clone();
                    gesture.connect_pressed(move |_, _, x, y| {
                        let popover = gtk::Popover::new();
                        popover.set_parent(&list_row_for_popover);

                        let popover_box = gtk::Box::builder()
                            .orientation(Orientation::Vertical)
                            .spacing(12)
                            .margin_start(12)
                            .margin_end(12)
                            .margin_top(12)
                            .margin_bottom(12)
                            .width_request(300)
                            .build();

                        if let Some(info) = data::PLANET_INFO.get(&title_for_popover) {
                            let header_box = gtk::Box::builder()
                                .orientation(Orientation::Horizontal)
                                .spacing(8)
                                .build();

                            let difficulty_label = gtk::Label::builder()
                                .label(&format!("Difficulty: {}", info.difficulty))
                                .halign(gtk::Align::Start)
                                .build();
                            difficulty_label.add_css_class("heading");
                            header_box.append(&difficulty_label);

                            if is_breathable_for_popover {
                                let breathable_label = gtk::Label::builder()
                                    .label("(Breathable Atmosphere)")
                                    .halign(gtk::Align::End)
                                    .hexpand(true)
                                    .build();
                                header_box.append(&breathable_label);
                            }

                            popover_box.append(&header_box);

                            let scrolled_window = gtk::ScrolledWindow::builder()
                                .hscrollbar_policy(gtk::PolicyType::Never)
                                .vscrollbar_policy(gtk::PolicyType::Automatic)
                                .height_request(150)
                                .hexpand(true)
                                .vexpand(true)
                                .build();

                            let description_label = gtk::Label::builder()
                                .label(&info.description)
                                .wrap(true)
                                .wrap_mode(gtk::pango::WrapMode::WordChar)
                                .xalign(0.0)
                                .build();

                            scrolled_window.set_child(Some(&description_label));
                            popover_box.append(&scrolled_window);
                        }

                        popover.set_child(Some(&popover_box));
                        popover.popup();
                        let _ = x;
                        let _ = y;
                    });
                    list_row.add_controller(gesture);

                    let row_clone = list_row.clone();
                    let name_for_filter = body.record.name.clone();
                    let system_for_filter = body.record.system.clone();
                    search_entry.connect_search_changed(clone!(@weak row_clone => move |entry| {
                        let search_text = entry.text().to_lowercase();
                        let name = name_for_filter.to_lowercase();
                        let system = system_for_filter.to_lowercase();
                        let visible = search_text.is_empty() || name.contains(&search_text) || system.contains(&search_text);
                        row_clone.set_visible(visible);
                    }));

                    expander_row.add_row(&list_row);
                    all_rows.push(list_row);
                }
            }
        }

        list_box.append(&expander_row);
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
    content.append(&search_bar);
    content.append(&scrolled);

    search_button.connect_toggled(clone!(@weak search_bar, @weak search_entry => move |button| {
        search_bar.set_search_mode(button.is_active());
        if button.is_active() {
            search_entry.grab_focus();
        }
    }));

    search_bar.bind_property("search-mode-enabled", &search_button, "active")
        .flags(glib::BindingFlags::BIDIRECTIONAL)
        .build();

    app.set_accels_for_action("win.search", &["<Control>f"]);

    let search_action = gio::SimpleAction::new("search", None);
    search_action.connect_activate(glib::clone!(@weak search_button, @weak search_bar, @weak search_entry => move |_, _| {
        let is_active = search_button.is_active();
        search_button.set_active(!is_active);
        search_bar.set_search_mode(!is_active);
        if !is_active {
            search_entry.grab_focus();
        }
    }));
    window.add_action(&search_action);

    let quit_action = gio::SimpleAction::new("quit", None);
    quit_action.connect_activate(glib::clone!(@weak window => move |_, _| {
        window.close();
    }));
    window.add_action(&quit_action);

    app.set_accels_for_action("win.quit", &["<Control>q"]);

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