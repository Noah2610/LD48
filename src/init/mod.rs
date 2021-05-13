use crate::resource;
use crate::settings::Settings;
use crate::states::aliases::GameData;
use crate::states::prelude::Startup;
use amethyst::core::frame_limiter::FrameRateLimitConfig;
use amethyst::utils::app_root_dir::application_root_dir;
use amethyst::window::EventLoop;
use amethyst::ApplicationBuilder;
use deathframe::amethyst;

mod init_game_data;

pub fn run() -> amethyst::Result<()> {
    start_logger();

    let settings = Settings::load()?;
    let event_loop = EventLoop::new();
    let game_data = init_game_data::build_game_data(&settings, &event_loop)?;

    let Settings {
        camera: camera_settings,
        player: player_settings,
        objects: objects_settings,
        lanes: lanes_settings,
        zones: zones_settings,
        audio: audio_settings,
        savefile: savefile_settings,
    } = settings;

    let mut game: amethyst::CoreApplication<GameData> =
        ApplicationBuilder::new(application_root_dir()?, Startup::default())?
            .with_frame_limit_config(frame_rate_limit_config()?)
            .with_resource(camera_settings)
            .with_resource(player_settings)
            .with_resource(objects_settings)
            .with_resource(lanes_settings)
            .with_resource(zones_settings)
            .with_resource(audio_settings)
            .with_resource(savefile_settings)
            .build(game_data)?;

    game.run_winit_loop(event_loop);

    Ok(())
}

fn start_logger() {
    use amethyst::{LogLevelFilter, LoggerConfig};
    amethyst::start_logger(LoggerConfig {
        level_filter: LogLevelFilter::Error,
        ..Default::default()
    });
}

fn frame_rate_limit_config() -> amethyst::Result<FrameRateLimitConfig> {
    use std::fs::File;
    Ok(ron::de::from_reader(File::open(resource(
        "config/frame_limiter.ron",
    ))?)?)
}
