
pub struct GlobalState {
    pub sitename : String,
}

impl GlobalState {
    // pub fn new() -> GlobalState {
    //     GlobalState::default()
    // }
    pub fn new() -> GlobalState {
        return GlobalState {
            sitename: "Woppleblox".to_string()
        }
    }
}
