use crate::{
    tracker::{EntryManager, NewEntry},
    widgets::{
        filter_selector::FilterSelectorWidget, new_entry::NewEntryWidget, stats::StatisticsWidget,
        table::EntriesTableBuilder, WidgetConstructor, 
    },
};
use egui::Color32;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct App {
   pub tracker: EntryManager,
    #[serde(skip)]
   pub new_entry: NewEntry,
    #[serde(skip)]
   pub entry_insert_error: String,
}
pub type AppState = App;
impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut state: App = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            state.tracker.sort();
            return state;
        }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    //auto save data every min
    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::new(60, 0)
    }
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let window_width = frame.info().window_info.size.x;
        let window_height = frame.info().window_info.size.y;
        egui::TopBottomPanel::top("add_panel")
            .exact_height(window_height * (8.0 / 100.0))
            .show(ctx, |ui| {
                ui.add(NewEntryWidget::new( self))
            });
        egui::SidePanel::left("stats")
            .exact_width(window_width * 0.15)
            .resizable(false)
            .show(ctx, |ui| {
                ui.add(StatisticsWidget::new(&self.tracker));
                ui.separator();
                ui.add(FilterSelectorWidget::new(&mut self.tracker.filter));
                ui.separator();
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    let styles = ui.style_mut();
                    styles.visuals.extreme_bg_color = Color32::BLUE;
                    egui::warn_if_debug_build(ui);
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            let entries = self.tracker.filtered_entries();
            if entries.len() == 0 {
                ui.centered_and_justified(|ui| {
                    ui.heading("No entries. Try changing filters or adding new entries.");
                });
            } else {
                EntriesTableBuilder::new(ui, &mut self.tracker)
            }
        });
    }
}
