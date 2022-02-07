use std::sync::Mutex;


pub struct AppState {
    pub health_check_response: String,
    pub vis_count: Mutex<u32>,
}
