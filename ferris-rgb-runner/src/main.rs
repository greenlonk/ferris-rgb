use ferris_rgb_core::application::display_service::DisplayService;

fn main() {
    let display = DisplayService::new();
    display.show_clock();
}
