use amethyst::window::winit::window::{Fullscreen, Window};
use deathframe::amethyst;

pub fn enter_fullscreen(window: &Window) {
    let monitor_id = window.current_monitor();
    window.set_fullscreen(Some(Fullscreen::Borderless(monitor_id)));
    window.set_cursor_visible(false);
}

pub fn leave_fullscreen(window: &Window) {
    window.set_fullscreen(None);
    window.set_cursor_visible(true);
}

pub fn toggle_fullscreen(window: &Window) {
    let is_fullscreen = window.fullscreen().is_some();
    if is_fullscreen {
        leave_fullscreen(window);
    } else {
        enter_fullscreen(window);
    }
}
