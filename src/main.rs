use crate::{
    states::welcome_state::StartGameState,
    systems::{
        colliders_list_system::ListSystem, fps_counter::FpsPrinterSystem,
        move_player::MovePlayerSystem, powerup_system::PowerUpSystem,
        txt_wobble_system::TextWobbleSystem, update_score::ScoreUpdaterSystem,
        update_tile_transforms::UpdateTileTransforms, win_system::EndOfGameSystem,
    },
};
use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        palette::Srgba,
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterSystem},
};
use log::LevelFilter;
use structopt::StructOpt;
// use steamworks::{Client, FriendFlags};

#[macro_use]
extern crate lazy_static;

mod components;
mod high_scores;
mod level;
mod level_editor;
mod quick_save_load;
mod states;
mod systems;
mod tag;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 36;
pub const ARENA_WIDTH: u32 = 8 * WIDTH;
pub const ARENA_HEIGHT: u32 = 8 * HEIGHT; //each sprite is 8px wide, so arena will be 16 sprites by 9 sprites

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let opts = Flags::from_args();

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = app_root.join("config/display.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(app_root.join("config/bindings.ron"))?;

    let mut game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear(get_colours(34.0, 35.0, 35.0)),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(UpdateTileTransforms, "update_tile_transforms", &[])
        .with(ListSystem, "collider_list", &[])
        .with(
            MovePlayerSystem::default(),
            "move_player",
            &["collider_list", "update_tile_transforms"],
        )
        .with(EndOfGameSystem, "end_of_game", &["collider_list"])
        .with(TextWobbleSystem, "txt_wobble", &[])
        .with(ScoreUpdaterSystem, "score_updater", &[])
        .with(
            PowerUpSystem,
            "powerups",
            &["collider_list", "update_tile_transforms", "move_player"],
        );

    if !opts.console {
        log::set_max_level(LevelFilter::Error);
    } else if opts.fps {
        game_data = game_data.with(FpsCounterSystem, "fps", &[]).with(
            FpsPrinterSystem,
            "fps_printer",
            &["fps"],
        );
    }

    // let (client, single) = Client::init().unwrap();
    // println!("{:?}", client.friends().get_friends(FriendFlags::IMMEDIATE));

    let mut game = Application::new(resources, StartGameState::default(), game_data)?;
    game.run();

    Ok(())
}

pub fn get_colours(r_a: f32, g_a: f32, b_a: f32) -> [f32; 4] {
    let (r, g, b, a) = Srgba::new(r_a / 255., g_a / 255., b_a / 255., 1.0)
        .into_linear()
        .into_components();
    [r, g, b, a]
}

///Flags for Lonely Tribes
#[derive(StructOpt, Debug)]
pub struct Flags {
    ///Enable an FPS counter in the console
    #[structopt(short, long)]
    pub fps: bool,

    ///Enable the console
    #[structopt(short, long)]
    pub console: bool,

    ///Enable debug options (disables high scores)
    ///Similar to Valve svcheats
    #[structopt(short, long)]
    pub debug: bool,

    ///Starting level, requires debug mode
    #[structopt(short, long)]
    pub level: Option<usize>,

    ///Option to enable legacy movement
    #[structopt(short, long)]
    pub timed_movement: Option<f32>,

    ///Option to use the debug level, requires debug mode
    #[cfg(debug_assertions)]
    #[structopt(long)]
    pub debug_level: bool,

    #[cfg(not(debug_assertions))]
    #[structopt(skip = false)]
    pub debug_level: bool,
}

//todos

//TODO: Music/SFX

//TODO: Export some options to conf file
//TODO: Conf Editor

//TODO: With Sprites, make sure to account for Screen Scaling

//TODO: post-processing

//TODO: Steamworks
//TODO: Story
//TODO: Steam Page
