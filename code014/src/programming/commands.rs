use bevy::prelude::*;

#[derive(Component)]
struct ComponentA;
#[derive(Component, Default)]
struct ComponentB;
#[derive(Component, Default)]
struct ComponentC;
#[derive(Bundle, Default)]
struct MyBundle {
    b: ComponentB,
}
#[derive(Resource)]
struct MyResource;

impl MyResource {
    fn new() -> Self {
        Self
    }
}

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Enemy;
#[derive(Component)]
struct Friendly;
#[derive(Component)]
struct PlayerName(String);
#[derive(Component)]
struct PlayerXp(u32);
#[derive(Component)]
struct Health {
    hp: f32,
    extra: f32,
}
#[derive(Bundle)]
struct PlayerBundle {
    name: PlayerName,
    xp: PlayerXp,
    health: Health,
    _p: Player,
    sprite: SpriteBundle,
}

// ANCHOR: example-commands
// 这个Commands的例子将主要使用的场景都罗列出来了.
fn spawn_things(mut commands: Commands) {
    // manage resources
    commands.insert_resource(MyResource::new());
    commands.remove_resource::<MyResource>();
    // 增删资源,同类型的资源都只有一份.(如果要增加多份就使用组件,资源都是单例组件).

    // create a new entity using `spawn`,
    // providing the data for the components it should have
    // (typically using a Bundle)
    commands.spawn(PlayerBundle {
        name: PlayerName("Henry".into()),
        xp: PlayerXp(1000),
        health: Health {
            hp: 100.0,
            extra: 20.0,
        },
        _p: Player,
        sprite: Default::default(),
    });
    // 构造一个实体,单组件.

    // you can use a tuple if you need additional components or bundles
    // (tuples of component and bundle types are considered bundles)
    // (note the extra parentheses)
    let my_entity_id = commands
        .spawn((
            // add some components
            ComponentA,
            ComponentB::default(),
            // add some bundles
            MyBundle::default(),
            TransformBundle::default(),
        ))
        .id(); // get the Entity (id) by calling `.id()` at the end

    // 构造一个实体,多组件.
    // Bundle是特征,从单元元组到14个元素的元组,都实现了Bundle.
    //
    // Commands::spawn()是构造实体,返回的是system::EntityCommands,
    // EntityCommands是一个命令列表,这些命令会修改实体. id()会返回实体(一个轻量级的标识:数值类型).

    // add/remove components of an existing entity
    commands
        .entity(my_entity_id)
        .insert(ComponentC::default())
        .remove::<ComponentA>()
        .remove::<(ComponentB, MyBundle)>();
    // 对实体进行组件增删操作.
    // Commands.entity()会根据Entity查到对应的EntityCommands,
    // EntityCommands的insert/remove就是对具体实体进行组件变更操作.

    // remove everything except the given components / bundles
    commands
        .entity(my_entity_id)
        .retain::<(TransformBundle, ComponentC)>();
    // 只保留指定的组件,其他全部删除.
}

// 改变敌对关系,增加敌人组件,删除友善组件.
fn make_all_players_hostile(
    mut commands: Commands,
    // we need the Entity id, to perform commands on specific entities
    query: Query<Entity, With<Player>>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            // add an `Enemy` component to the entity
            .insert(Enemy)
            // remove the `Friendly` component
            .remove::<Friendly>();
    }
}

// 删除所有实体的敌人组件.
fn despawn_all_enemies(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
// ANCHOR_END: example-commands

// ANCHOR: command-closure
fn my_system(mut commands: Commands) {
    let x = 420;

    commands.add(move |world: &mut World| {
        // do whatever you want with `world` here

        // note: it's a closure, you can use variables from
        // the parent scope/function
        eprintln!("{}", x);
    });
}
// ANCHOR_END: command-closure

// ANCHOR: command-impl
use bevy::ecs::world::Command;

struct MyCustomCommand {
    // you can have some parameters
    data: u32,
}

impl Command for MyCustomCommand {
    fn apply(self, world: &mut World) {
        // do whatever you want with `world` and `self.data` here
    }
}

// use it like this
fn my_other_system(mut commands: Commands) {
    commands.add(MyCustomCommand {
        data: 920, // set your value
    });
}
// ANCHOR_END: command-impl

// ANCHOR: command-ext
pub trait MyCustomCommandsExt {
    // define a method that we will be able to call on `commands`
    fn do_custom_thing(&mut self, data: u32);
}

// implement our trait for Bevy's `Commands`
impl<'w, 's> MyCustomCommandsExt for Commands<'w, 's> {
    fn do_custom_thing(&mut self, data: u32) {
        self.add(MyCustomCommand { data });
    }
}

fn my_fancy_system(mut commands: Commands) {
    // now we can call our custom method just like Bevy's `spawn`, etc.
    commands.do_custom_thing(42);
}
// ANCHOR_END: command-ext

fn spawn_new_enemies_if_needed() {}
fn enemy_ai() {}

fn _main() {
    let mut app = App::new();
    // ANCHOR: order
    app.add_systems(Update, spawn_new_enemies_if_needed);

    // This system will see any newly-spawned enemies when it runs,
    // because Bevy will make sure to apply the first system's Commands
    // (thanks to the explicit `.after()` dependency)
    app.add_systems(Update, enemy_ai.after(spawn_new_enemies_if_needed));
    // ANCHOR_END: order
}
