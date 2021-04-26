use super::state_prelude::*;
use std::path::PathBuf;

#[derive(Default)]
pub struct Startup;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Startup {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        setup(&mut data.world);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update_core(data.world);
        Trans::Switch(Box::new(MainMenu::default()))
    }
}

fn setup(world: &mut World) {
    use crate::components::prelude::{
        BelongsToSegment,
        Coin,
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

    let sprite_sheet_handles = SpriteSheetHandles::<PathBuf>::default();
    world.insert(sprite_sheet_handles);

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
