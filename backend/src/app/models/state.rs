use strum::{Display, EnumIter, FromRepr};

#[derive(Default)]
pub struct App {
    pub state: AppState,
    pub selected_tab: SelectedTab,
}   

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Running,
    Quitting,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Home")]
    Home,
    #[strum(to_string = "About")]
    About,
    #[strum(to_string = "Projects")]
    Projects,
    #[strum(to_string = "Contact")]
    Contact,
}