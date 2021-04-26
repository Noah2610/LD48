use super::system_prelude::*;
use deathframe::amethyst::ui::{UiText, UiTransform};

const UI_SCORE_ID: &str = "score";

#[derive(Default)]
pub struct UpdateScoreUi {
    coins_cache: Option<usize>,
}

impl<'a> System<'a> for UpdateScoreUi {
    type SystemData = (
        ReadExpect<'a, Score>,
        ReadStorage<'a, UiTransform>,
        WriteStorage<'a, UiText>,
    );

    fn run(
        &mut self,
        (score, ui_transform_store, mut ui_text_store): Self::SystemData,
    ) {
        let should_update = self
            .coins_cache
            .map(|cached| cached != score.coins)
            .unwrap_or(true);

        if should_update {
            self.coins_cache = Some(score.coins);
            if let Some(ui_text) = (&ui_transform_store, &mut ui_text_store)
                .join()
                .find_map(|(transform, text)| {
                    if &transform.id == UI_SCORE_ID {
                        Some(text)
                    } else {
                        None
                    }
                })
            {
                ui_text.text = format!("SCORE\n{}", score.coins);
            }
        }
    }
}
