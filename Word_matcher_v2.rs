use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Word Matcher",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    /// The fixed word list (same as the original array)
    word_list: [&'static str; 5],
    /// Text the user is typing
    input: String,
    /// Words that matched and were stored
    storage: Vec<String>,
    /// Feedback after each submit
    message: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            word_list: ["book", "hotel", "sweet", "crakers", "holy", ],
            input: String::new(),
            storage: Vec::new(),
            message: String::from("Type a sentence and press Submit."),
        }
    }
}

// ── GUI ───────────────────────────────────────────────────────────────────────

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dewali Booker");
            ui.add_space(10.0);

            // Show the word list for reference
            ui.label(format!("Booking options: {:?}", self.word_list));
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Input field
            ui.horizontal(|ui| {
                ui.label("Activities:");
                ui.text_edit_singleline(&mut self.input);
            });

            ui.add_space(8.0);

            // Buttons
            ui.horizontal(|ui| {
                if ui.button("Submit").clicked() {
                    let mut match_found = false;

                    // Clone to avoid borrow conflict
                    let words: Vec<String> = self.input
                        .to_lowercase()
                        .split_whitespace()
                        .map(|w| w.to_string())
                        .collect();

                    for word in &words {
                        if self.word_list.contains(&word.as_str()) {
                            self.storage.push(word.clone());
                            match_found = true;
                        }
                    }

                    self.message = if match_found {
                        format!("Match found and stored!")
                    } else {
                        String::from("No match was found.")
                    };

                    self.input.clear();
                }

                if ui.button("Clear Storage").clicked() {
                    self.storage.clear();
                    self.message = String::from("Storage cleared.");
                }
            });

            ui.add_space(15.0);

            // Status message
            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.colored_label(egui::Color32::LIGHT_GREEN, &self.message);
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(5.0);

            // Storage display
            ui.label("Stored matches:");
            if self.storage.is_empty() {
                ui.colored_label(egui::Color32::GRAY, "(none yet)");
            } else {
                for item in &self.storage {
                    ui.label(format!("• {}", item));
                }
            }
        });
    }
}
