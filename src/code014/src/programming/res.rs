use bevy::prelude::*;

#[derive(Resource, Default)]
struct MyOtherResource;

impl MyOtherResource {
    fn do_mut_stuff(&mut self) {}
}

// ANCHOR: struct
#[derive(Resource)]
struct GoalsReached {
    main_goal: bool,
    bonus: u32,
}
// ANCHOR_END: struct

// ANCHOR: systemparam
fn my_system(
    // these will panic if the resources don't exist
    mut goals: ResMut<GoalsReached>,
    other: Res<MyOtherResource>,
    // use Option if a resource might not exist
    mut fancy: Option<ResMut<MyFancyResource>>,
) {
    if let Some(fancy) = &mut fancy {
        // TODO: do things with `fancy`
    }
    // TODO: do things with `goals` and `other`
}
// ANCHOR_END: systemparam

// ANCHOR: fromworld
#[derive(Resource)]
struct MyFancyResource {/* stuff */}

impl FromWorld for MyFancyResource {
    fn from_world(world: &mut World) -> Self {
        // You have full access to anything in the ECS World from here.

        // For example, you can access (and mutate!) other things:
        {
            let mut x = world.resource_mut::<MyOtherResource>();
            x.do_mut_stuff();
        }

        // You can load assets:
        let font: Handle<Font> = world.resource::<AssetServer>().load("myfont.ttf");

        MyFancyResource { /* stuff */ }
    }
}
// ANCHOR_END: fromworld

// ANCHOR: default
// simple derive, to set all fields to their defaults
#[derive(Resource, Default)]
struct GameProgress {
    game_completed: bool,
    secrets_unlocked: u32,
}

#[derive(Resource)]
struct StartingLevel(usize);

// custom implementation for unusual values
impl Default for StartingLevel {
    fn default() -> Self {
        StartingLevel(1)
    }
}

// on enums, you can specify the default variant
#[derive(Resource, Default)]
enum GameMode {
    Tutorial,
    #[default] // 这个default标识如果GameMode::default()时,默认会返回SinglePlayer::default()
    Singleplayer,
    Multiplayer,
}
// ANCHOR_END: default

// ANCHOR: commands
fn my_setup(mut commands: Commands /* ... */) {
    // add (or overwrite if existing) a resource, with the given value
    // 对于同一类型调用insert_resource(),会覆盖资源的值.
    commands.insert_resource(GoalsReached {
        main_goal: false,
        bonus: 100,
    });
    // ensure resource exists (create it with its default value if necessary)
    // 如果资源已存在,直接返回;如果资源不存在,使用`FromWorld`特型初始化值.
    // 任何资源,只要实现了Default特型,就自动实现了FromWorld特型.
    // 所以使用init_resource()的类型,一般要实现Default特型.
    commands.init_resource::<MyFancyResource>();
    // remove a resource (if it exists)
    // 如果资源不存在,直接返回.
    commands.remove_resource::<MyOtherResource>();
}
// ANCHOR_END: commands

// ANCHOR: exclusive
fn my_setup2(world: &mut World) {
    // The same methods as with Commands are also available here,
    // but we can also do fancier things:

    // Check if resource exists
    if !world.contains_resource::<MyFancyResource>() {
        // Get access to a resource, inserting a custom value if unavailable
        let _bonus = world
            .get_resource_or_insert_with(|| GoalsReached {
                main_goal: false,
                bonus: 100,
            })
            .bonus;
    }
}
// ANCHOR_END: exclusive

fn main() {
    // ANCHOR: app
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(StartingLevel(3))
        .init_resource::<MyFancyResource>()
        // ...
        // ANCHOR_END: app
        .add_systems(Startup, (my_setup, my_setup2))
        .add_systems(Update, my_system)
        .run();
}
