use super::system_prelude::*;
use deathframe::amethyst::ui::{UiText, UiTransform};
use std::collections::HashMap;

const UI_HIGHSCORE_DYNAMIC_ID: &str = "highscore";
const UI_HIGHSCORE_PROGRESSION_ID: &str = "highscore_progression";
const UI_HIGHSCORE_INFINITE_ID: &str = "highscore_infinite";

#[derive(Default)]
pub struct UpdateHighscoreUi;

impl<'a> System<'a> for UpdateHighscoreUi {
    type SystemData = (
        ReadExpect<'a, Savefile>,
        Read<'a, SelectedZone>,
        ReadStorage<'a, UiTransform>,
        WriteStorage<'a, UiText>,
        Read<'a, Option<ZoneProgressionMode>>,
    );

    fn run(
        &mut self,
        (
            savefile,
            selected_zone,
            ui_transform_store,
            mut ui_text_store,
            zone_progression_mode_opt,
        ): Self::SystemData,
    ) {
        let highs = {
            let mut highs = HashMap::new();
            if let Some(progression) = savefile
                .highscores
                .progression
                .as_ref()
                .map(|high| high.highscore)
            {
                highs.insert(HighType::Progression, progression);
                if let Some(ZoneProgressionMode::Progression) =
                    *zone_progression_mode_opt
                {
                    highs.insert(HighType::Dynamic, progression);
                }
            }
            if let Some(infinite) =
                selected_zone.0.as_ref().and_then(|selected| {
                    savefile
                        .highscores
                        .infinite
                        .get(&selected.1)
                        .map(|high| high.highscore)
                })
            {
                highs.insert(HighType::Infinite, infinite);
                if let Some(ZoneProgressionMode::Infinite) =
                    *zone_progression_mode_opt
                {
                    highs.insert(HighType::Dynamic, infinite);
                }
            }

            highs
        };

        for (high_type, ui_text) in (&ui_transform_store, &mut ui_text_store)
            .join()
            .filter_map(|(transform, text)| {
                if &transform.id == UI_HIGHSCORE_PROGRESSION_ID {
                    Some((HighType::Progression, text))
                } else if &transform.id == UI_HIGHSCORE_INFINITE_ID {
                    Some((HighType::Infinite, text))
                } else if &transform.id == UI_HIGHSCORE_DYNAMIC_ID {
                    Some((HighType::Dynamic, text))
                } else {
                    None
                }
            })
        {
            if let Some(&score) = highs.get(&high_type) {
                if score > 0 {
                    ui_text.text = format!("HIGH\n{}", score);
                } else {
                    ui_text.text = String::new();
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
enum HighType {
    Progression,
    Infinite,
    Dynamic,
}
