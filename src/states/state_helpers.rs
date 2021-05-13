use amethyst::renderer::rendy::wsi::winit::Window;
use deathframe::amethyst;

pub fn enter_fullscreen(window: &Window) {
    let monitor_id = window.get_current_monitor();
    window.set_fullscreen(Some(monitor_id));
    window.hide_cursor(true);
}

pub fn leave_fullscreen(window: &Window) {
    window.set_fullscreen(None);
    window.hide_cursor(false);
}

pub fn toggle_fullscreen(window: &Window) {
    let is_fullscreen = window.get_fullscreen().is_some();
    if is_fullscreen {
        leave_fullscreen(window);
    } else {
        enter_fullscreen(window);
    }
}
