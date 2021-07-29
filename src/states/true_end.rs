use crate::states::{states_util::load_font, StartGameState};
use amethyst::{
    core::ecs::{Builder, Entity, World, WorldExt},
    input::{InputEvent, VirtualKeyCode},
    ui::{Anchor, Interactable, LineMode, UiEventType, UiText, UiTransform},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};

///State for when the user has finished all levels
#[derive(Default)]
pub struct TrueEnd {
    ///Stores the Entity for the Button as an option for easier initialisation
    btn: Option<Entity>,
}

impl SimpleState for TrueEnd {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.btn = Some(get_true_end_txt(world));
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let mut back_to_mm = false;
        match event {
            StateEvent::Input(event) => {
                if let InputEvent::KeyPressed { key_code, .. } = event {
                    if key_code == VirtualKeyCode::Return || key_code == VirtualKeyCode::Space {
                        back_to_mm = true;
                    }
                }
            }
            StateEvent::Ui(event) => {
                let is_target = event.target == self.btn.unwrap(); //TODO: Better than unwrap
                let mut txts = data.world.write_storage::<UiText>();
                let txt = txts.get_mut(event.target);

                if let Some(txt) = txt {
                    match event.event_type {
                        UiEventType::ClickStart => txt.color = [0.8, 0.8, 0.9, 1.0],
                        UiEventType::ClickStop => {
                            if is_target {
                                txt.color = [1.0, 1.0, 1.0, 0.5];
                                back_to_mm = true;
                            }
                        }
                        UiEventType::HoverStart => txt.color = [1.0, 1.0, 1.0, 0.5],
                        UiEventType::HoverStop => txt.color = [1.0; 4],
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        if back_to_mm {
            Trans::Switch(Box::new(StartGameState::default()))
        } else {
            Trans::None
        }
    }
}

///Instantiates text with end text detailing how to get back to the main menu
///
///Returns the entity of that text, for checking when it was clicked
pub fn get_true_end_txt(world: &mut World) -> Entity {
    let trans = UiTransform::new(
        "end_txt".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        0.0,
        0.5,
        1000.0,
        1000.0,
    );
    let txt = UiText::new(
        load_font(world, "ZxSpectrum"),
        "Well, I never thought we'd get here... Click here, or press [Space] or [Enter] to go back to the Main Menu. Congrats!".to_string(),
        [1.0; 4],
        45.0,
        LineMode::Wrap,
        Anchor::Middle,
    );
    world
        .create_entity()
        .with(trans)
        .with(txt)
        .with(Interactable)
        .build()
}
