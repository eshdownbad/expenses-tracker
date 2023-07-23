use crate::app::AppState;

pub mod filter_selector;
pub mod new_entry;
pub mod stats;
pub mod table;

pub trait WidgetConstructor<'a> {
    fn new(state: &'a mut AppState) -> Self;
}
