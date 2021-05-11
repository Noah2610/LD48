use super::system_prelude::*;
use deathframe::amethyst::ui::{UiText, UiTransform};
use std::collections::HashMap;

const UI_HIGHSCORE_PROGRESSION_ID: &str = "highscore_progression";
const UI_HIGHSCORE_INFINITE_ID: &str = "highscore_infinite";

#[derive(Default)]
pub struct UpdateHighscoreUi;

impl<'a> System<'a> for UpdateHighscoreUi {
    type SystemData = (
        ReadExpect<'a, Savefile>,
        ReadStorage<'a, UiTransform>,
        WriteStorage<'a, UiText>,
    );

    fn run(
        &mut self,
        (savefile, ui_transform_store, mut ui_text_store): Self::SystemData,
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
            };
            // TODO: INFINITE SCORE
            highs
        };

        for (high_type, ui_text) in (&ui_transform_store, &mut ui_text_store)
            .join()
            .filter_map(|(transform, text)| {
                if &transform.id == UI_HIGHSCORE_PROGRESSION_ID {
                    Some((HighType::Progression, text))
                } else if &transform.id == UI_HIGHSCORE_INFINITE_ID {
                    Some((HighType::Infinite, text))
                } else {
                    None
                }
            })
        {
            if let Some(score) = highs.get(&high_type) {
                ui_text.text = format!("HIGH\n{}", score);
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
enum HighType {
    Progression,
    Infinite,
}
