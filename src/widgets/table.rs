use egui::{Button, Color32, RichText, Ui};
use egui_extras::{Column, TableBuilder};

use crate::tracker::{EntryType, EntryManager};

pub struct EntriesTableBuilder;

impl EntriesTableBuilder {
    pub fn new(ui: &mut Ui, entry_manager: &mut EntryManager) {
        //60 is subtracted since first column has fixed size of 30
        //and the extra 30 provides padding
        let available_width = ui.available_width() - 60.0;
        let available_height = ui.available_height();
        let mut builder = TableBuilder::new(ui)
            .column(Column::exact(30.0))
            .column(Column::exact(available_width * 0.5))
            .columns(Column::exact(available_width / 6.0), 3)
            .striped(true);
        let data = entry_manager.filtered_entries();
        static TABLE_ROW_HEIGHT: f32 = 30.0;
        let row_count = data.len() as f32;

        //add scrolling if height of table is larger than height of panel
        if (row_count * TABLE_ROW_HEIGHT) > available_height {
            builder = builder.vscroll(true).max_scroll_height(row_count * 30.0);
        }

        builder
            .header(30.0, |mut header| {
                header.col(|ui| {
                    ui.heading("+/-");
                });
                header.col(|ui| {
                    ui.heading("Description");
                });
                header.col(|ui| {
                    ui.heading("Amount");
                });
                header.col(|ui| {
                    ui.heading("Date");
                });
                header.col(|ui| {
                    ui.heading("Actions");
                });
            })
            .body(|body| {
                body.rows(TABLE_ROW_HEIGHT, row_count as usize, |ridx, mut row| {
                    let entry = data.get(ridx);
                    if let Some(entry) = entry {
                        row.col(|ui| {
                            let symbol = match entry.entry_type {
                                EntryType::Expense => RichText::new("-").color(Color32::RED),
                                EntryType::Income => RichText::new("+").color(Color32::GREEN),
                            }
                            .to_owned();
                            ui.centered_and_justified(|ui| {
                                ui.heading(symbol);
                            });
                        });
                        row.col(|ui| {
                            ui.horizontal_centered(|ui| {
                                ui.label(entry.description.clone());
                            });
                        });
                        row.col(|ui| {
                            ui.horizontal_centered(|ui| {
                                ui.label(format!("{} INR", entry.amount));
                            });
                        });
                        row.col(|ui| {
                            ui.horizontal_centered(|ui| {
                                ui.label(entry.date.format("%e/%b/%Y").to_string());
                            });
                        });
                        row.col(|ui| {
                            if ui
                                .add_sized(
                                    ui.available_size(),
                                    Button::new(RichText::new("remove").color(Color32::RED)),
                                )
                                .clicked()
                            {
                                entry_manager.remove_entry_by_id(entry.id.clone());
                            }
                        });
                    }
                });
            });
    }
}
