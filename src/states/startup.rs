use super::state_prelude::*;
use std::path::PathBuf;

#[derive(Default)]
pub struct Startup;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Startup {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        #[cfg(not(feature = "debug"))]
        enter_fullscreen(&mut data.world);

        setup(&mut data.world);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update_core(data.world);
        Trans::Push(Box::new(Cutscene::default()))
    }

    fn shadow_fixed_update(&mut self, data: StateData<GameData<'a, 'b>>) {
        use crate::input::prelude::{MenuAction, MenuBindings};

        let input_manager =
            data.world.read_resource::<InputManager<MenuBindings>>();
        if input_manager.is_down(MenuAction::ToggleFullscreen) {
            toggle_fullscreen(data.world);
        }
    }
}

fn setup(world: &mut World) {
    use crate::components::prelude::{
        BelongsToSegment,
        Coin,
        Cutscene,
        Object,
        Obstacle,
        Portal,
        Segment,
        Tile,
        Turret,
    };

    world.register::<Tile>();
    world.register::<Object>();
    world.register::<BelongsToSegment>();
    world.register::<Segment>();
    world.register::<Portal>();
    world.register::<Obstacle>();
    world.register::<Coin>();
    world.register::<Turret>();
    world.register::<Cutscene>();

    let sprite_sheet_handles = SpriteSheetHandles::<PathBuf>::default();
    world.insert(sprite_sheet_handles);

    world.insert(load_savefile(&world));

    let (songs, sounds) = {
        let audio_settings = world.read_resource::<AudioSettings>();

        let mut songs = Songs::<SongKey>::default();
        for (song_key, song) in audio_settings.bgm.iter() {
            match songs.load_audio(
                song_key.clone(),
                resource(format!("audio/bgm/{}", song.file)),
                true,
                world,
            ) {
                Ok(_) => {
                    if let Some(song) = songs.get_mut(song_key) {
                        song.set_volume(audio_settings.volume);
                    }
                }
                Err(e) => eprintln!(
                    "[WARNING]\n    Error loading song {}, skipping.\n{:#?}",
                    &song.file, e
                ),
            }
        }

        let mut sounds = Sounds::<SoundKey>::default();
        for (sound_key, sound) in audio_settings.sfx.iter() {
            if let Err(e) = sounds.load_audio(
                sound_key.clone(),
                resource(format!("audio/sfx/{}", sound.file)),
                world,
            ) {
                eprintln!(
                    "[WARNING]\n    Error loading sound {}, skipping.\n{:#?}",
                    &sound.file, e
                );
            }
        }

        (songs, sounds)
    };

    world.insert(songs);
    world.insert(sounds);

    world
        .insert(crate::components::prelude::SoundPlayer::<SoundKey>::default());
}

fn load_savefile(world: &World) -> Savefile {
    let savefile =
        match world.read_resource::<SavefileSettings>().savefile_path() {
            Ok(savefile_path) => match Savefile::load(savefile_path) {
                Ok(savefile) => Some(savefile),
                Err(e) => {
                    eprintln!("[WARNING]\n    {}", e);
                    None
                }
            },
            Err(e) => {
                eprintln!("[WARNING]\n    {}", e);
                None
            }
        };
    savefile.unwrap_or_default()
}
