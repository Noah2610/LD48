use amethyst::ecs::{World, WorldExt};
use amethyst::renderer::rendy::wsi::winit::Window;
use deathframe::amethyst;

pub fn enter_fullscreen(world: &World) {
    let window = world.read_resource::<Window>();
    let monitor_id = window.get_current_monitor();
    window.set_fullscreen(Some(monitor_id));
}

pub fn leave_fullscreen(world: &World) {
    let window = world.read_resource::<Window>();
    window.set_fullscreen(None);
}

pub fn toggle_fullscreen(world: &World) {
    let window = world.read_resource::<Window>();
    let is_fullscreen = window.get_fullscreen().is_some();
    if is_fullscreen {
        window.set_fullscreen(None);
    } else {
        window.set_fullscreen(Some(window.get_current_monitor()));
    }
}
