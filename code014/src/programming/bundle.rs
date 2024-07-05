use bevy::prelude::*;

#[derive(Component)]
struct Health {
    hp: f32,
    extra: f32,
}
#[derive(Component)]
struct PlayerXp(u32);
#[derive(Component)]
struct PlayerName(String);
#[derive(Component)]
struct Enemy;
#[derive(Component)]
struct Player;
#[derive(Component)]
struct Friendly;
#[derive(Component)]
struct LowHpMarker;
#[derive(Component)]
struct CurrentModifier;
#[derive(Component)]
struct PlayerPendingAction;
#[derive(Component)]
struct StatusEffect;

// ANCHOR: bundle
// 给自定义类型添加Bundle的derive之后,自定类型就实现了Bundle特型.
// Bundle特性的功能在于实体可以增删组件,实现方式是用Bundle表示一组实体的组件.
// Bundle中的组件不能重复,否则会panic.
//
// Bundle主要功能还是向实体中新增Bundle,
// 如果某个实体已经有了新增的组件,那么实体中原始组件的值会被覆盖.
// (这意味着一个实体可以包含多个Bundle,多个Bundle可以包含相同的组件,
// 但单个Bundle中不能包含重复的组件).
//
// Bundle仅仅是组件的组合,不关心组件的行为,组件的行为由system考虑.
// 所以在设计上`Query`的对象是组件,而不是Bundle.
//
// 少部分场景下实体可以删除Bundle,此时会删除所有Bundle中的组件,
// 如果删除后实体并无其他组件时,实体会被删除.
#[derive(Bundle)]
struct PlayerBundle {
    xp: PlayerXp,
    name: PlayerName,
    health: Health,
    marker: Player,

    // We can nest/include another bundle.
    // Add the components for a standard Bevy Sprite:
    sprite: SpriteBundle,
}
// ANCHOR_END: bundle

// ANCHOR: cleanup-bundle
/// Contains all components to remove when
/// resetting the player between rooms/levels.
#[derive(Bundle)]
struct PlayerResetCleanupBundle {
    status_effect: StatusEffect,
    pending_action: PlayerPendingAction,
    modifier: CurrentModifier,
    low_hp_marker: LowHpMarker,
}
// ANCHOR_END: cleanup-bundle

// ANCHOR: bundle-default
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            xp: PlayerXp(0),
            name: PlayerName("Player".into()),
            health: Health {
                hp: 100.0,
                extra: 0.0,
            },
            marker: Player,
            sprite: Default::default(),
        }
    }
}
// ANCHOR_END: bundle-default

fn setup(mut commands: Commands) {
    let e_player = Entity::PLACEHOLDER;
    // ANCHOR: bundle-spawn
    commands.spawn(PlayerBundle {
        xp: PlayerXp(0),
        name: PlayerName("Player 1".into()),
        health: Health {
            hp: 100.0,
            extra: 0.0,
        },
        marker: Player,
        sprite: SpriteBundle {
            // TODO
            ..Default::default()
        },
    });
    // ANCHOR_END: bundle-spawn
    // ANCHOR: bundle-spawn-default
    commands.spawn(PlayerBundle {
        name: PlayerName("Player 1".into()),
        ..Default::default()
    });
    // ANCHOR_END: bundle-spawn-default
    // ANCHOR: bundle-spawn-loose
    commands.spawn((
        SpriteBundle {
            // ...
            ..default()
        },
        Health {
            hp: 50.0,
            extra: 0.0,
        },
        Enemy,
        // ...
    ));
    // ANCHOR_END: bundle-spawn-loose
    // ANCHOR: cleanup-bundle-remove
    commands
        .entity(e_player)
        .remove::<PlayerResetCleanupBundle>();
    // ANCHOR_END: cleanup-bundle-remove
}

fn main() {
    let app = App::new().add_systems(Startup, setup);
}
