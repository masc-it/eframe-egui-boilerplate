

#[derive(Clone)]
pub struct AppState {
    pub output: String,
    pub refresh: bool,
}
impl AppState {
    pub fn new() -> Self {
        AppState {
            output: "".into(),
            refresh: false,
        }
    }
}