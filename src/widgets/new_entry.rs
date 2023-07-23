use egui::Widget;

use crate::{
    app::AppState,
    tracker::{Entry, EntryManager, EntryType, NewEntry},
};

use super::WidgetConstructor;

pub struct NewEntryWidget<'a> {
    fields: &'a mut NewEntry,
    entry_manager: &'a mut EntryManager,
}

impl Widget for NewEntryWidget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        return ui
            .vertical_centered_justified(|ui| {
                ui.horizontal(|ui| {
                    ui.label("description:");
                    ui.text_edit_singleline(&mut self.fields.description);
                });
                ui.horizontal(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut self.fields.amount)
                            .clamp_range(1..=u64::MAX)
                            .suffix("INR")
                            .prefix("Amount:"),
                    );
                    ui.label("type:");
                    ui.selectable_value(&mut self.fields.entry_type, EntryType::Income, "Income");
                    ui.selectable_value(&mut self.fields.entry_type, EntryType::Expense, "Expense");
                    ui.add(egui_extras::DatePickerButton::new(&mut self.fields.date));
                    if ui.button("Add").clicked() {
                        let entry_data = self.fields.clone();
                        //TODO add validation and error msgs
                        let entry = Entry::from(entry_data);
                        self.entry_manager.add_entry(entry);
                        self.fields.reset();
                    }
                });
            })
            .response;
    }
}
impl<'a> WidgetConstructor<'a> for NewEntryWidget<'a> {
    fn new(state: &'a mut AppState) -> Self {
        Self {
            fields: &mut state.new_entry,
            entry_manager: &mut state.tracker,
        }
    }
}
