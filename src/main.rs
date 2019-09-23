use amethyst::{
    core::transform::{ TransformBundle, Transform},         
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    input::{InputBundle, StringBindings},
    utils::application_root_dir,
    assets::{AssetStorage, Loader, Handle},
};

mod systems;
mod components;

pub const ARENA_HEIGHT: f32 = 200.0;
pub const ARENA_WIDTH: f32 = 200.0;
pub const PLAYERS: i32 = 4;
pub const AGENTS: i32 = 40;

struct MyState;

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        
        initialise_camera(world);

        let sheet = load_sprite_sheet(world);
        let renderer = SpriteRender {
            sprite_sheet: sheet.clone(),
            sprite_number: 0,
        };
        let renderer2 = SpriteRender {
            sprite_sheet: sheet.clone(),
            sprite_number: 1,
        };
        for i in 0..PLAYERS / 2 {
            world.create_entity()
                .with(renderer.clone())
                .with(Transform::default())
                .with(components::PlayerTag(true))
                .with(components::Ship {
                    id: i,
                    x: 0f32,
                    y: (i * 20) as f32,
                    rotation: 0f32,
                })
                .with(components::Velocity {
                    dx: 0f32,
                    dy: 0f32,
                }).build();
        }

        for i in PLAYERS/2..PLAYERS {
            world.create_entity()
                .with(renderer.clone())
                .with(Transform::default())
                .with(components::PlayerTag(false))
                .with(components::Ship {
                    id: i,
                    x: 0f32,
                    y: (i * 20) as f32,
                    rotation: 0f32,
                })
                .with(components::Velocity {
                    dx: 0f32,
                    dy: 0f32,
                }).build();
        }
        for i in PLAYERS..(PLAYERS+AGENTS) {
            
            world.create_entity()
                .with(renderer2.clone())
                .with(Transform::default())
                .with(components::AITag{})
                .with(components::Velocity {
                    dx: rand::random::<f32>() * 2.0 - 1.0,
                    dy: rand::random::<f32>() * 2.0 - 1.0,
                })
                .with(components::Ship {
                    id: i + 1,
                    x: rand::random::<f32>() * ARENA_WIDTH, 
                    y: rand::random::<f32>() * ARENA_HEIGHT,
                    rotation: 0.0
                })
                .build();
        }
    }
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left. 
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "spaceship.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "spaceship.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )    
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let binding_path = config_dir.join("bindings.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(binding_path)?
        )?
        .with(systems::InputSystem, "inputs_system", &[])
        .with(systems::AISystem, "ai_system", &[])
        .with(systems::MoveSystem, "moves_system", &["inputs_system", "ai_system"]);

    let mut game = Application::new(app_root.join("assets"), MyState, game_data)?;
    game.run();

    Ok(())
}
