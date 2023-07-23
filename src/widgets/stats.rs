use egui::{Widget, RichText, Color32};

use crate::tracker::EntryManager;

pub struct StatisticsWidget {
    total: f64,
    total_expenses: f64,
    total_income: f64
}
impl StatisticsWidget {
    pub fn new(data: &EntryManager) -> Self {
        Self {
            total:data.total(),
            total_expenses: data.total_expenses(),
            total_income: data.total_expenses(),
        }
    }
}
impl Widget for StatisticsWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.heading("Statstics");
            ui.label(format!("total income: {}", self.total_income));
            ui.label(format!("total expenses: {}", self.total_expenses));
            ui.horizontal(|ui| {
                ui.label("total:");
                let total = self.total;
                let text = RichText::new(total.to_string());
                ui.label(if total > 0.0 {
                    text.color(Color32::GREEN)
                } else {
                    text.color(Color32::RED)
                });
            });                
        }).response
    }
}