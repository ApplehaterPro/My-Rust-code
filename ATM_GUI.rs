use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Bank Account",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

// ── Data ─────────────────────────────────────────────────────────────────────

#[derive(Debug)]
struct BankAccount {
    acount_holder: String,
    account_number: String,
    balance: f64,
}

impl BankAccount {
    fn show_balance(&self) -> String {
        format!("${:.2}", self.balance)
    }

    fn deposit(&mut self, amount: f64) -> String {
        if amount > 0.0 {
            self.balance += amount;
            format!("Deposited ${:.2} successfully.", amount)
        } else {
            String::from("Deposit amount must be greater than zero.")
        }
    }

    fn withdraw(&mut self, amount: f64) -> String {
        if amount <= 0.0 {
            String::from("Withdrawal amount must be greater than zero.")
        } else if amount > self.balance {
            String::from("Insufficient funds.")
        } else {
            self.balance -= amount;
            format!("Withdrew ${:.2} successfully.", amount)
        }
    }
}

// ── App State ─────────────────────────────────────────────────────────────────

struct MyApp {
    account: BankAccount,
    /// Amount the user typed in the input box
    amount_input: String,
    /// Last action message to display
    message: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            account: BankAccount {
                acount_holder: String::from("Jaivirsg"),
                account_number: String::from("1234567890"),
                balance: 1000.0,
            },
            amount_input: String::new(),
            message: String::from("Welcome! Enter an amount below."),
        }
    }
}

// ── GUI ───────────────────────────────────────────────────────────────────────

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Bank Account");
            ui.add_space(10.0);

            // Account info
            ui.label(format!("Holder:  {}", self.account.acount_holder));
            ui.label(format!("Account: {}", self.account.account_number));
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Balance:");
                ui.colored_label(egui::Color32::LIGHT_GREEN, self.account.show_balance());
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Amount input
            ui.horizontal(|ui| {
                ui.label("Amount: $");
                ui.text_edit_singleline(&mut self.amount_input);
            });

            ui.add_space(8.0);

            // Deposit / Withdraw buttons
            ui.horizontal(|ui| {
                if ui.button("Deposit").clicked() {
                    match self.amount_input.trim().parse::<f64>() {
                        Ok(n) => self.message = self.account.deposit(n),
                        Err(_) => self.message = String::from("Error: Invalid amount."),
                    }
                    self.amount_input.clear();
                }

                if ui.button("Withdraw").clicked() {
                    match self.amount_input.trim().parse::<f64>() {
                        Ok(n) => self.message = self.account.withdraw(n),
                        Err(_) => self.message = String::from("Error: Invalid amount."),
                    }
                    self.amount_input.clear();
                }
            });

            ui.add_space(15.0);

            // Status message
            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.colored_label(egui::Color32::LIGHT_BLUE, &self.message);
            });
        });
    }
}
