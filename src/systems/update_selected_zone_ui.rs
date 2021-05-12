use super::system_prelude::*;
use deathframe::amethyst::ui::{UiText, UiTransform};

const UI_SELECTED_ZONE_ID: &str = "selected_zone";

#[derive(Default)]
pub struct UpdateSelectedZoneUi;

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
        if let Some(zone) = selected_zone.0.as_ref().map(|selected| &selected.1)
        {
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
                ui_text.text = zone.clone();
            }
        }
    }
}
