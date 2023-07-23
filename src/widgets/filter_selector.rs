use egui::Widget;

use crate::tracker::Filter;

pub struct FilterSelectorWidget<'a>( &'a mut Filter);

impl Widget for FilterSelectorWidget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.heading("Select Filter");
            ui.selectable_value(  self.0,  Filter::NoFilter, "None");
            ui.selectable_value(self.0, Filter::ThisMonth, "Current month");
            ui.selectable_value(self.0, Filter::ThisYear, "Current year");
            ui.selectable_value( self.0, Filter::LastMonth, "last month");
            ui.selectable_value( self.0,  Filter::LastYear, "last year");
        })
        .response
    }
}

impl<'a> FilterSelectorWidget<'a> {
    pub fn new( value: &'a mut Filter) -> Self {
        return Self(value);
    }

}