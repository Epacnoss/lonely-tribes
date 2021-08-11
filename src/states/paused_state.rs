use crate::components::data_holder::EntityHolder;
use amethyst::{
    core::{
        ecs::{Entity, World, WorldExt},
        Hidden,
    },
    input::{InputEvent, VirtualKeyCode},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent,
};
use std::collections::HashMap;
use amethyst::ui::{UiEvent, UiEventType, UiText, UiTransform, Anchor, LineMode, Interactable};
use crate::HOVER_COLOUR;
use crate::systems::move_player::MovementType;
use crate::states::states_util::load_font;
use amethyst::core::ecs::Builder;

///Resource to optionally disable movement - unless it is true, we assume false as the default is false
pub struct MovementDisabler {
    pub enabled: bool,
}
impl Default for MovementDisabler {
    fn default() -> Self {
        Self { enabled: false }
    }
}

///Enum which contains different actions for buttons and whatnot
#[derive(Copy, Clone, Hash)]
pub enum PausedStateMenuAction {
    ///Option to toggle the movement
    ToggleMovement,
}

///State for when the game is paused
#[derive(Default)]
pub struct PausedState {
    ///All of the toggle-ale buttons
    buttons: HashMap<Entity, PausedStateMenuAction>,
    ///The title entity
    title: Option<Entity>
}

impl SimpleState for PausedState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.insert(MovementDisabler { enabled: true });

        let entities = world.read_resource::<EntityHolder>().get_all_entities();
        hide_entities(world, entities);

        let (buttons, top) = get_pause_buttons(world);
        self.buttons = buttons;
        self.title = Some(top);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let mut t = SimpleTrans::None;
        match event {
            StateEvent::Input(InputEvent::KeyPressed { key_code, .. }) => match key_code {
                VirtualKeyCode::Escape => {
                    t = SimpleTrans::Pop;

                    let world = data.world;
                    world.insert(MovementDisabler::default());
                    for (k, _) in self.buttons.clone().into_iter() {
                        world.delete_entity(k);
                    }
                    if let Some(t) = self.title {
                        world.delete_entity(t);
                    }

                    let entities = world.read_resource::<EntityHolder>().get_all_entities();
                    show_entities(world, entities);
                },
                _ => {}
            },
            StateEvent::Ui(UiEvent {event_type, target}) => {
                let action = {
                    let mut res = None;
                    'working_out_target: for (k, v) in &self.buttons {
                        if &target == k {
                            res = Some(*v);
                            break 'working_out_target;
                        }
                    }
                    res
                };

                if let Some(action) = action {
                    let world = data.world;
                    let mut texts = world.write_storage::<UiText>();

                    if let Some(txt) = texts.get_mut(target) {
                        match event_type {
                            UiEventType::ClickStop => match action {
                                PausedStateMenuAction::ToggleMovement => {
                                    let mut current_state = world.write_resource::<MovementType>();

                                    let stepped_movement = current_state.can_move.is_some();
                                    if stepped_movement {
                                        current_state.can_move = None;
                                        current_state.movement_timer = Some((0.0, 0.05));
                                        txt.text = "Toggle Movement type to Stepped.".to_string();
                                    } else {
                                        current_state.can_move = Some(true);
                                        current_state.movement_timer = None;
                                        txt.text = "Toggle Movement type to Held.".to_string();
                                    }

                                }
                            },
                            UiEventType::HoverStart => txt.color = HOVER_COLOUR,
                            UiEventType::HoverStop => txt.color = HOVER_COLOUR,
                            _ => {}
                        }
                    }
                }
            },
            _ => {}
        }

        t
    }
}

///Function to hide a given list of entities, by adding Hidden components to all of them
pub fn hide_entities(world: &mut World, entities: Vec<Entity>) {
    let mut hiddens = world.write_storage::<Hidden>();

    entities.into_iter().for_each(|ent| {
        hiddens.insert(ent, Hidden).unwrap_or_else(|err| {
            log::warn!("Unable to hide entity: {}", err);
            None
        });
    });
}

///Function to show a given list of entities, by removing Hidden components from all of them
///
///If they don't have a Hidden component, then it does a log::warn
pub fn show_entities(world: &mut World, entities: Vec<Entity>) {
    let mut hiddens = world.write_storage::<Hidden>();

    entities.into_iter().for_each(|ent| {
        hiddens.remove(ent).unwrap_or_else(|| {
            log::warn!("Unable to show entity: {:?}", ent);
            Hidden
        });
    });
}

///Inserts the Pause Menu Buttons and the title
///
/// Returns a HashMap of all the buttons, and the title
pub fn get_pause_buttons (world: &mut World) -> (HashMap<Entity, PausedStateMenuAction>, Entity) {
    let mut map = HashMap::new();

    let bold_font_handle = load_font(world, "ZxSpectrumBold");
    let font_handle = load_font(world, "ZxSpectrum");

    let welcome_trans = UiTransform::new(
        String::from("welcome_txt"),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        100.0,
        0.0,
        1000.0,
        250.0,
    );
    let welcome_txt = UiText::new(
        bold_font_handle,
        String::from("The Game is currently Paused."),
        [1.0, 1.0, 1.0, 0.5],
        75.0,
        LineMode::Wrap,
        Anchor::Middle,
    );
    let top = world
        .create_entity()
        .with(welcome_trans)
        .with(welcome_txt)
        .build();

    let toggle = {
        let toggle_btn_trans = UiTransform::new(
            String::from("toggle_btn"),
            Anchor::Middle,
            Anchor::Middle,
            0.0,
            -85.0,
            0.0,
            1500.0,
            40.0,
        );

        let actual_txt =
            if world.read_resource::<MovementType>().can_move.is_some() {
                "Toggle Movement type to Stepped.".to_string()
            } else {
                "Toggle Movement type to Held.".to_string()
            };


        let toggle_btn_txt = UiText::new(
            font_handle.clone(),
            actual_txt,
            [1.0; 4],
            25.0,
            LineMode::Single,
            Anchor::Middle,
        );
        world
            .create_entity()
            .with(toggle_btn_trans)
            .with(toggle_btn_txt)
            .with(Interactable)
            .build()
    };
    map.insert(toggle, PausedStateMenuAction::ToggleMovement);

    /*
    let help_btn_trans = UiTransform::new(
        String::from("help_btn"),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        -145.0,
        0.0,
        1000.0,
        40.0,
    );
    let help_btn_txt = UiText::new(
        font_handle.clone(),
        String::from("Click here to get Help."),
        [1.0; 4],
        50.0,
        LineMode::Single,
        Anchor::Middle,
    );
    let help = world
        .create_entity()
        .with(help_btn_trans)
        .with(help_btn_txt)
        .with(Interactable)
        .build();
        */



    (map, top)
}