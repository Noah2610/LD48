use super::system_prelude::*;

#[derive(Default)]
pub struct HandleZoneSelect;

impl<'a> System<'a> for HandleZoneSelect {
    type SystemData = (
        ReadExpect<'a, InputManager<MenuBindings>>,
        ReadExpect<'a, ZonesSettings>,
        Write<'a, SelectedZone>,
        ReadExpect<'a, Savefile>,
    );

    fn run(
        &mut self,
        (
            input_manager,
            settings,
            mut selected_zone,
            savefile,
        ): Self::SystemData,
    ) {
        let select_dir_opt = if input_manager.is_down(MenuAction::Next) {
            Some(SelectDir::Next)
        } else if input_manager.is_down(MenuAction::Prev) {
            Some(SelectDir::Prev)
        } else {
            None
        };
        let zone_idx = *selected_zone
            .0
            .as_ref()
            .map(|selected| selected.0)
            .get_or_insert(0);
        if let Some(select_dir) = select_dir_opt {
            let zones_len = settings.config.zone_order.len();
            let next_zone_idx = match select_dir {
                SelectDir::Next => {
                    (zone_idx + 1).min(zones_len.checked_sub(1).unwrap_or(0))
                }
                SelectDir::Prev => zone_idx.checked_sub(1).unwrap_or(0),
            };
            if zone_idx != next_zone_idx {
                if let Some(next_zone) =
                    settings.config.zone_order.get(next_zone_idx)
                {
                    let has_unlocked_next_zone =
                        savefile.unlocked.contains(next_zone);
                    if has_unlocked_next_zone {
                        selected_zone.0 =
                            Some((next_zone_idx, next_zone.clone()));
                    }
                }
            }
        }
    }
}

enum SelectDir {
    Next,
    Prev,
}
