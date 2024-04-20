mod bottom_bar;
mod bottom_panel;
mod close_confirmation_dialog;
mod frames;
mod process_info;
mod source_view;
mod threads;
mod top_bar;
mod variables;

pub use bottom_bar::add as bottom_bar;
pub use bottom_panel::add as console_tabs;
pub use close_confirmation_dialog::add as close_confirmation;
pub use frames::add as frames;
pub use process_info::add as process_info;
pub use source_view::add as source_view;
pub use threads::add as threads;
pub use top_bar::add as top_bar;
pub use variables::add as variables;
