use eframe::egui;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize, Clone)]
struct CountrySummary {
    #[serde(rename = "Origin")]
    origin: String,
    #[serde(rename = "Visa Free")]
    visa_free: u32,
    #[serde(rename = "Visa Required")]
    #[allow(dead_code)]
    visa_required: u32,
}

#[derive(Debug, Deserialize, Clone)]
struct VisaDetail {
    #[serde(rename = "Origin")]
    origin: String,
    #[serde(rename = "Destination")]
    destination: String,
    #[serde(rename = "Requirement")]
    visa_requirement: String,
}

#[derive(PartialEq)]
enum Tab {
    Ranking,
    Search,
}

struct AppData {
    summaries: Vec<CountrySummary>,
    details: HashMap<String, Vec<VisaDetail>>,
    selected_country: Option<String>,
    destination_filter: Option<String>,
    origin_search_text: String,
    destination_search_text: String,
    ranking_search: String,
    tab: Tab,
    origin_combo_open: bool,
    dest_combo_open: bool,
}

impl AppData {
    fn load() -> Result<Self, Box<dyn Error>> {
        let mut summary_reader =
            csv::Reader::from_path("henley-passport-index-count-2025-10-17.csv")?;
        let mut summaries: Vec<CountrySummary> =
            summary_reader.deserialize().collect::<Result<_, _>>()?;
        summaries.sort_by(|a, b| b.visa_free.cmp(&a.visa_free).then(a.origin.cmp(&b.origin)));

        let mut detail_reader = csv::Reader::from_path("henley-passport-index-2025-10-17.csv")?;
        let mut details_map: HashMap<String, Vec<VisaDetail>> = HashMap::new();
        for result in detail_reader.deserialize::<VisaDetail>() {
            let rec = result?;
            details_map.entry(rec.origin.clone()).or_default().push(rec);
        }

        Ok(Self {
            summaries,
            details: details_map,
            selected_country: None,
            destination_filter: None,
            origin_search_text: String::new(),
            destination_search_text: String::new(),
            ranking_search: String::new(),
            tab: Tab::Ranking,
            origin_combo_open: false,
            dest_combo_open: false,
        })
    }
}

impl eframe::App for AppData {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        use egui::{Color32, Margin, RichText, Rounding, Stroke, Vec2};

        // Force light mode - ignore system theme
        ctx.set_visuals(egui::Visuals::light());

        // Enhanced styling
        let mut style = (*ctx.style()).clone();
        style.spacing.button_padding = Vec2::new(12.0, 8.0);
        style.spacing.item_spacing = Vec2::new(12.0, 8.0);
        style.visuals.window_rounding = Rounding::same(12.0);
        style.visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
        style.visuals.widgets.inactive.rounding = Rounding::same(8.0);
        style.visuals.widgets.hovered.rounding = Rounding::same(8.0);
        style.visuals.widgets.active.rounding = Rounding::same(8.0);

        // Enhanced shadow using correct fields
        style.visuals.window_shadow.blur = 12.0;
        style.visuals.window_shadow.spread = 2.0;
        style.visuals.popup_shadow.blur = 8.0;
        style.visuals.popup_shadow.spread = 1.0;

        ctx.set_style(style);

        // Enhanced header with gradient-like effect
        egui::TopBottomPanel::top("header")
            .frame(
                egui::Frame::none()
                    .fill(Color32::from_rgb(41, 98, 255))
                    .inner_margin(Margin::symmetric(20.0, 16.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // App title
                    ui.label(
                        RichText::new("🌍 Henley Passport Index")
                            .size(22.0)
                            .color(Color32::WHITE)
                            .strong(),
                    );

                    ui.add_space(30.0);

                    // Tab buttons with enhanced styling
                    let ranking_sel = self.tab == Tab::Ranking;
                    let search_sel = self.tab == Tab::Search;

                    let ranking_button =
                        egui::Button::new(RichText::new("🏆 Ranking").size(15.0).color(
                            if ranking_sel {
                                Color32::from_rgb(41, 98, 255)
                            } else {
                                Color32::DARK_GRAY
                            },
                        ))
                        .fill(if ranking_sel {
                            Color32::WHITE
                        } else {
                            Color32::from_rgba_premultiplied(255, 255, 255, 30)
                        })
                        .stroke(Stroke::NONE)
                        .rounding(Rounding::same(8.0));

                    if ui
                        .add(ranking_button)
                        .on_hover_text("See the world passport power rankings")
                        .clicked()
                    {
                        self.tab = Tab::Ranking;
                    }

                    ui.add_space(8.0);

                    let search_button = egui::Button::new(
                        RichText::new("🔍 Search").size(15.0).color(if search_sel {
                            Color32::from_rgb(41, 98, 255)
                        } else {
                            Color32::DARK_GRAY
                        }),
                    )
                    .fill(if search_sel {
                        Color32::WHITE
                    } else {
                        Color32::from_rgba_premultiplied(255, 255, 255, 30)
                    })
                    .stroke(Stroke::NONE)
                    .rounding(Rounding::same(8.0));

                    if ui
                        .add(search_button)
                        .on_hover_text("Search visa access for a passport")
                        .clicked()
                    {
                        self.tab = Tab::Search;
                    }
                });
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(Color32::from_rgb(248, 249, 252))
                    .inner_margin(Margin::same(24.0)),
            )
            .show(ctx, |ui| {
                match self.tab {
                    Tab::Ranking => {
                        egui::Frame::none()
                            .fill(Color32::WHITE)
                            .inner_margin(Margin::same(24.0))
                            .rounding(Rounding::same(12.0))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(226, 232, 240)))
                            .show(ui, |ui| {
                                ui.heading(RichText::new("🌐 Global Passport Power Rankings").size(24.0).strong());
                                ui.add_space(16.0);

                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("🔎 Filter by country:").size(15.0).color(Color32::from_rgb(100, 116, 139)));

                                    egui::Frame::none()
                                        .stroke(Stroke::new(1.5, Color32::from_rgb(203, 213, 225)))
                                        .rounding(Rounding::same(8.0))
                                        .inner_margin(Margin::symmetric(12.0, 8.0))
                                        .show(ui, |ui| {
                                            ui.add(
                                                egui::TextEdit::singleline(&mut self.ranking_search)
                                                    .hint_text("Search countries...")
                                                    .desired_width(250.0)
                                                    .frame(false)
                                            );
                                        });
                                });

                                ui.add_space(20.0);

                                let mut ranks: Vec<(usize, &CountrySummary)> =
                                    Vec::with_capacity(self.summaries.len());
                                let mut prev_score: Option<u32> = None;
                                let mut rank = 1;
                                for c in self.summaries.iter() {
                                    if Some(c.visa_free) != prev_score {
                                        if !ranks.is_empty() {
                                            rank += 1;
                                        }
                                        prev_score = Some(c.visa_free);
                                    }
                                    ranks.push((rank, c));
                                }

                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    egui::Grid::new("ranking_grid")
                                        .striped(true)
                                        .min_col_width(180.0)
                                        .spacing([20.0, 12.0])
                                        .show(ui, |ui| {
                                            ui.label(RichText::new("Rank").size(16.0).strong().color(Color32::from_rgb(71, 85, 105)));
                                            ui.label(RichText::new("Country").size(16.0).strong().color(Color32::from_rgb(71, 85, 105)));
                                            ui.label(RichText::new("Visa-Free Access").size(16.0).strong().color(Color32::from_rgb(71, 85, 105)));
                                            ui.end_row();

                                            let search = self.ranking_search.to_lowercase();
                                            for (rank, country) in ranks.iter().filter(|(_, c)| {
                                                search.is_empty()
                                                    || c.origin.to_lowercase().contains(&search)
                                            }) {
                                                // Rank with colored badges for top 3
                                                match rank {
                                                    1 => {
                                                        egui::Frame::none()
                                                            .fill(Color32::from_rgb(255, 215, 0))
                                                            .inner_margin(Margin::symmetric(8.0, 4.0))
                                                            .rounding(Rounding::same(6.0))
                                                            .show(ui, |ui| {
                                                                ui.label(RichText::new("1st").size(14.0).strong().color(Color32::from_rgb(139, 69, 19)));
                                                            });
                                                    }
                                                    2 => {
                                                        egui::Frame::none()
                                                            .fill(Color32::from_rgb(192, 192, 192))
                                                            .inner_margin(Margin::symmetric(8.0, 4.0))
                                                            .rounding(Rounding::same(6.0))
                                                            .show(ui, |ui| {
                                                                ui.label(RichText::new("2nd").size(14.0).strong().color(Color32::from_rgb(64, 64, 64)));
                                                            });
                                                    }
                                                    3 => {
                                                        egui::Frame::none()
                                                            .fill(Color32::from_rgb(205, 127, 50))
                                                            .inner_margin(Margin::symmetric(8.0, 4.0))
                                                            .rounding(Rounding::same(6.0))
                                                            .show(ui, |ui| {
                                                                ui.label(RichText::new("3rd").size(14.0).strong().color(Color32::WHITE));
                                                            });
                                                    }
                                                    _ => {
                                                        ui.label(RichText::new(format!("{}", rank)).size(15.0).color(Color32::from_rgb(100, 116, 139)));
                                                    }
                                                }

                                                ui.label(RichText::new(&country.origin).size(15.0));

                                                ui.label(
                                                    RichText::new(format!("{}", country.visa_free))
                                                        .size(15.0)
                                                        .strong()
                                                        .color(Color32::from_rgb(34, 197, 94))
                                                );
                                                ui.end_row();
                                            }
                                        });
                                });
                            });
                    }
                    Tab::Search => {
                        egui::Frame::none()
                            .fill(Color32::WHITE)
                            .inner_margin(Margin::same(24.0))
                            .rounding(Rounding::same(12.0))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(226, 232, 240)))
                            .show(ui, |ui| {
                                ui.heading(RichText::new("🔍 Visa Requirements Search").size(24.0).strong());
                                ui.add_space(20.0);

                                ui.vertical(|ui| {
                                    // Origin Country Selection
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("📍 Origin Country:").size(16.0).strong().color(Color32::from_rgb(71, 85, 105)));
                                        ui.add_space(10.0);

                                        let selected_text = self
                                            .selected_country
                                            .as_deref()
                                            .unwrap_or("Select your passport country...");

                                        egui::ComboBox::from_id_source("origin_combo")
                                            .selected_text(RichText::new(selected_text).size(15.0))
                                            .width(350.0)
                                            .show_ui(ui, |ui| {
                                                let search_response = ui
                                                    .horizontal(|ui| {
                                                        ui.label(RichText::new("🔍").size(16.0));
                                                        let response = ui.add(
                                                            egui::TextEdit::singleline(&mut self.origin_search_text)
                                                                .hint_text("Type to search...")
                                                                .desired_width(280.0)
                                                        );
                                                        if response.gained_focus() {
                                                            self.origin_combo_open = true;
                                                        }
                                                        response
                                                    })
                                                    .inner;

                                                if !self.origin_combo_open {
                                                    self.origin_combo_open = true;
                                                    search_response.request_focus();
                                                }

                                                ui.separator();

                                                let search_lower = self.origin_search_text.to_lowercase();
                                                let filtered: Vec<_> = self
                                                    .summaries
                                                    .iter()
                                                    .filter(|country| {
                                                        search_lower.is_empty()
                                                            || country.origin.to_lowercase().contains(&search_lower)
                                                    })
                                                    .collect();

                                                egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                                                    for country in filtered {
                                                        let label = format!(
                                                            "{} (🌍 {} countries)",
                                                            country.origin, country.visa_free
                                                        );
                                                        if ui
                                                            .selectable_label(
                                                                self.selected_country.as_deref() == Some(&country.origin),
                                                                RichText::new(label).size(14.0),
                                                            )
                                                            .clicked()
                                                        {
                                                            self.selected_country = Some(country.origin.clone());
                                                            self.destination_filter = None;
                                                            self.origin_search_text.clear();
                                                            self.origin_combo_open = false;
                                                        }
                                                    }
                                                });
                                            });

                                        if ui.input(|i| i.pointer.any_click()) {
                                            self.origin_combo_open = false;
                                        }
                                    });

                                    ui.add_space(16.0);

                                    // Destination Country Selection
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("🎯 Destination:").size(16.0).strong().color(Color32::from_rgb(71, 85, 105)));
                                        ui.add_space(10.0);

                                        let dest_text = self
                                            .destination_filter
                                            .as_deref()
                                            .unwrap_or("All countries");

                                        egui::ComboBox::from_id_source("dest_combo")
                                            .selected_text(RichText::new(dest_text).size(15.0))
                                            .width(350.0)
                                            .show_ui(ui, |ui| {
                                                let search_response = ui
                                                    .horizontal(|ui| {
                                                        ui.label(RichText::new("🔍").size(16.0));
                                                        let response = ui.add(
                                                            egui::TextEdit::singleline(&mut self.destination_search_text)
                                                                .hint_text("Type to search...")
                                                                .desired_width(280.0)
                                                        );
                                                        if response.gained_focus() {
                                                            self.dest_combo_open = true;
                                                        }
                                                        response
                                                    })
                                                    .inner;

                                                if !self.dest_combo_open {
                                                    self.dest_combo_open = true;
                                                    search_response.request_focus();
                                                }

                                                ui.separator();

                                                if ui
                                                    .selectable_label(
                                                        self.destination_filter.is_none(),
                                                        RichText::new("🌎 All countries").size(14.0),
                                                    )
                                                    .clicked()
                                                {
                                                    self.destination_filter = None;
                                                    self.destination_search_text.clear();
                                                    self.dest_combo_open = false;
                                                }

                                                let search_lower = self.destination_search_text.to_lowercase();
                                                let filtered: Vec<_> = self
                                                    .summaries
                                                    .iter()
                                                    .filter(|country| {
                                                        if let Some(origin) = &self.selected_country {
                                                            if &country.origin == origin {
                                                                return false;
                                                            }
                                                        }
                                                        search_lower.is_empty()
                                                            || country.origin.to_lowercase().contains(&search_lower)
                                                    })
                                                    .collect();

                                                egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                                                    for country in filtered {
                                                        if ui
                                                            .selectable_label(
                                                                self.destination_filter.as_deref() == Some(&country.origin),
                                                                RichText::new(&country.origin).size(14.0),
                                                            )
                                                            .clicked()
                                                        {
                                                            self.destination_filter = Some(country.origin.clone());
                                                            self.destination_search_text.clear();
                                                            self.dest_combo_open = false;
                                                        }
                                                    }
                                                });
                                            });

                                        if ui.input(|i| i.pointer.any_click()) {
                                            self.dest_combo_open = false;
                                        }
                                    });

                                    ui.add_space(24.0);
                                    ui.separator();
                                    ui.add_space(20.0);

                                    // Results display
                                    if let Some(origin) = &self.selected_country {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new("Visa Requirements for").size(18.0).color(Color32::from_rgb(100, 116, 139)));
                                            ui.label(RichText::new(origin).size(20.0).strong().color(Color32::from_rgb(41, 98, 255)));
                                        });
                                        ui.add_space(16.0);

                                        if let Some(details) = self.details.get(origin) {
                                            let mut requirement_map: HashMap<&str, &str> = HashMap::new();
                                            for detail in details {
                                                requirement_map.insert(
                                                    detail.destination.as_str(),
                                                    detail.visa_requirement.as_str(),
                                                );
                                            }

                                            egui::ScrollArea::vertical().max_height(520.0).show(ui, |ui| {
                                                egui::Grid::new("visa_detail_grid")
                                                    .striped(true)
                                                    .min_col_width(200.0)
                                                    .spacing([20.0, 12.0])
                                                    .show(ui, |ui| {
                                                        ui.label(RichText::new("Destination").size(16.0).strong().color(Color32::from_rgb(71, 85, 105)));
                                                        ui.label(RichText::new("Visa Requirement").size(16.0).strong().color(Color32::from_rgb(71, 85, 105)));
                                                        ui.end_row();

                                                        for destination in self.summaries.iter().filter(|c| {
                                                            if &c.origin == origin {
                                                                return false;
                                                            }
                                                            if let Some(filter) = &self.destination_filter {
                                                                &c.origin == filter
                                                            } else {
                                                                true
                                                            }
                                                        }) {
                                                            let req = requirement_map
                                                                .get(destination.origin.as_str())
                                                                .copied()
                                                                .unwrap_or("unknown");

                                                            let (display_text, bg_color, text_color) = match req.to_lowercase().as_str() {
                                                                "visa free" => ("✓ Visa Free", Color32::from_rgb(220, 252, 231), Color32::from_rgb(22, 101, 52)),
                                                                "visa required" => ("✗ Visa Required", Color32::from_rgb(254, 226, 226), Color32::from_rgb(153, 27, 27)),
                                                                "visa on arrival" => ("◉ Visa on Arrival", Color32::from_rgb(254, 243, 199), Color32::from_rgb(146, 64, 14)),
                                                                "e-visa" => ("◎ E-Visa", Color32::from_rgb(209, 250, 229), Color32::from_rgb(6, 95, 70)),
                                                                _ => ("? Unknown", Color32::from_rgb(243, 244, 246), Color32::from_rgb(107, 114, 128)),
                                                            };

                                                            ui.label(RichText::new(&destination.origin).size(15.0));

                                                            egui::Frame::none()
                                                                .fill(bg_color)
                                                                .inner_margin(Margin::symmetric(10.0, 6.0))
                                                                .rounding(Rounding::same(6.0))
                                                                .show(ui, |ui| {
                                                                    ui.label(RichText::new(display_text).size(14.0).color(text_color).strong());
                                                                });

                                                            ui.end_row();
                                                        }
                                                    });
                                            });
                                        } else {
                                            ui.vertical_centered(|ui| {
                                                ui.add_space(40.0);
                                                ui.label(RichText::new("⚠ No visa information available").size(16.0).color(Color32::from_rgb(239, 68, 68)));
                                            });
                                        }
                                    } else {
                                        ui.vertical_centered(|ui| {
                                            ui.add_space(80.0);
                                            ui.label(RichText::new("👆").size(48.0));
                                            ui.add_space(12.0);
                                            ui.label(RichText::new("Select an origin country to see visa requirements").size(18.0).color(Color32::from_rgb(148, 163, 184)));
                                        });
                                    }
                                });
                            });
                    }
                }
            });
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = AppData::load()?;
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([900.0, 600.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Henley Passport Index",
        native_options,
        Box::new(|_cc| Box::new(app)),
    );
    Ok(())
}
