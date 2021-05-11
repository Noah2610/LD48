use super::system_prelude::*;
use deathframe::amethyst::ui::{UiText, UiTransform};

const UI_SELECTED_ZONE_ID: &str = "selected_zone";

#[derive(Default)]
pub struct UpdateSelectedZoneUi {
    last_selected_zone_idx: Option<usize>,
}

impl<'a> System<'a> for UpdateSelectedZoneUi {
    type SystemData = (
        ReadExpect<'a, SelectedZone>,
        ReadStorage<'a, UiTransform>,
        WriteStorage<'a, UiText>,
    );

    fn run(
        &mut self,
        (
            selected_zone,
            ui_transform_store,
            mut ui_text_store,
        ): Self::SystemData,
    ) {
        if let Some((selected_zone, selected_zone_idx)) =
            selected_zone.0.as_ref()
        {
            if self
                .last_selected_zone_idx
                .as_ref()
                .map(|last_idx| last_idx != selected_zone_idx)
                .unwrap_or(true)
            {
                self.last_selected_zone_idx = Some(*selected_zone_idx);
                for ui_text in (&ui_transform_store, &mut ui_text_store)
                    .join()
                    .filter_map(|(transform, text)| {
                        if &transform.id == UI_SELECTED_ZONE_ID {
                            Some(text)
                        } else {
                            None
                        }
                    })
                {
                    ui_text.text = selected_zone.to_string();
                }
            }
        }
    }
}
