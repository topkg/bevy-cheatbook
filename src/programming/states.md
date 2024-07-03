{{#include ../include/header013.md}}

# States

Relevant official examples:
[`state`][example::state].

---

States allow you to structure the runtime "flow" of your app.

This is how you can implement things like:
 - A menu screen or a loading screen
 - Pausing / unpausing the game
 - Different game modes
 - …

In every state, you can have different [systems][cb::system] running. You
can also add setup and cleanup systems to run when entering or exiting a state.

---

To use states, first define an `enum` type. You need to derive
[`States`] + an assortment of required standard Rust traits:

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:definition}}
```

Note: you can have multiple orthogonal states! Create multiple types
if you want to track multiple things independently!

You then need to register the state type(s) in the [app builder][cb::app]:

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:app-register}}
```

在Bevy游戏引擎中,States是一个关键概念,用于管理游戏的不同阶段或模式.
例如,一个游戏可能有主菜单/游戏进行中/暂停等不同状态.
Bevy中的状态管理系统允许开发者在这些不同状态之间进行切换,
并定义在每个状态下应执行哪些系统和逻辑.

每个状态都可以划分不同的system进行运行(这个很容易办到,
只需要在system入口判断是不是匹配的状态即可,不匹配就结束).

状态需要由enum定义,指定实现了States特型.

可以使用多个正交状态,可完成更加丰富的需求.
状态需要在app中注册.

## Running Different Systems for Different States

If you want some [systems][cb::system] to only run in specific states,
Bevy offers an [`in_state`] [run condition][cb::rc]. Add it
to your systems. You probably want to create [system sets][cb::systemset]
to help you group many systems and control them at once.

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:app-example}}
```

Bevy also creates special [`OnEnter`], [`OnExit`],
and [`OnTransition`] [schedules][cb::schedule] for each
possible value of your state type. Use them to perform setup and cleanup for
specific states. Any systems you add to them will run once every time the state
is changed to/from the respective values.

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:app-example-transitions}}
```

哪些system在哪些状态下运行,出了上面提到的简单方法,还可以使用运行条件实现.
bevy提供了一个`in_state`的运行条件.

```rust
pub fn in_state<S: States>(state: S) -> impl FnMut(Option<Res<State<S>>>) -> bool + Clone {
    move |current_state: Option<Res<State<S>>>| match current_state {
        Some(current_state) => *current_state == state,
        None => false,
    }
}
```

看上面的源码,实现原理和上面提到的简单方法是一样的,这里是运行条件实现的.

对比看上面的例子, system集合和状态结合,威力非常巨大.

OnEnter/OnExit/OnTransition, 是状态进入/退出/改变时的调度.
利用这几个调度可以跟踪状态的切换.

### With Plugins

This can also be useful with [Plugins][cb::plugin]. You can set up all the state
types for your project in one place, and then your different plugins can just add
their systems to the relevant states.

You can also make plugins that are configurable, so that it is possible to specify
what state they should add their systems to:

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:plugin-config}}
```

Now you can configure the plugin when adding it to the app:

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:plugin-config-app}}
```

When you are just using [plugins][cb::plugin] to help with internal
organization of your project, and you know what systems should go into each
state, you probably don't need to bother with making the plugin configurable
as shown above. Just hardcode the states / add things to the correct states
directly.

插件也可以根据状态来分类,eg:插件中添加的system只绑定到某几个状态.
如果只用插件来组织项目,在插件中对状态进行硬编码会更加简单.
这里的硬编码是相对于上面的例子来说的(上面的例子是通过参数传递状态).

总的来说,组织项目的有:
 - 插件
 - 调度
 - system 集合
 - 状态

## Controlling States

Inside of systems, you can check the current state using the
[`State<T>`] [resource][cb::res]:

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:check-state}}
```

To change to another state, you can use the [`NextState<T>`]:

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:change-state}}
```

This will queue up state transitions to be performed during the next frame
update cycle.

状态在运行逻辑中会变化,也就是说system中会改变状态.

```rust
pub trait States: 'static + Send + Sync + Clone + PartialEq + Eq + Hash + Debug {
    const DEPENDENCY_DEPTH: usize = 1usize;
}

#[derive(Resource, Debug)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Resource)
)]
pub struct State<S: States>(pub(crate) S);
```

对状态的控制:
 - `Res<State<MyGameModeState>>.get()`获取状态
 - `ResMut<NextState<MyPausedState>>.set()`设置状态

对状态的变更,会放在队列中,在下帧的更新循环(StateTransition调度)中进行更新.

## State Transitions

Every frame update, a [schedule][cb::schedule] called
[`StateTransition`] runs. There, Bevy will check if
any new state is queued up in [`NextState<T>`] and perform
the transition for you.

The transition involves several steps:
 - A [`StateTransitionEvent`] [event][cb::event] is sent.
 - The [`OnExit(old_state)`][`OnExit`] [schedule][cb::schedule] is run.
 - The [`OnTransition { from: old_state, to: new_state }`][`OnTransition`] [schedule][cb::schedule] is run.
 - The [`OnEnter(new_state)`][`OnEnter`] [schedule][cb::schedule] is run.

[`StateTransitionEvent`] is useful in any [systems][cb::system] that run
regardless of state, but want to know if a transition has occurred. You can use
it to detect state transitions.

The [`StateTransition`] [schedule][cb::schedule] runs after [`PreUpdate`] (which
contains Bevy engine internals), but before [`FixedMain`] ([fixed
timestep][cb::fixedtimestep]) and [`Update`], where your game's
[systems][cb::system] usually live.

Therefore, state transitions happen before your game logic for the current frame.

If doing state transitions once per frame is not enough for you, you can add
additional transition points, by adding Bevy's [`apply_state_transition`]
[system][cb::system] wherever you like.

```rust,no_run,noplayground
{{#include ../code013/src/programming/states.rs:app-custom-transition}}
```

状态变更: 在当前帧的StateTransition中执行上帧`NextState<T>`的状态变更,具体为:
 - 发送StateTransitionEvent事件
 - OnExit(旧状态)调度运行
 - OnTransition(from:旧状态,to:新状态)调度执行
 - OnEnter(新状态)调度运行

StateTransition调度在PreUpdate之后执行(PreUpdate是bevy内部引擎包含的),
在FixedMain和Update之前.因此状态变更是在当前帧逻辑之前发生的.

如果每帧发生一次状态变更还不够,可以使用`apply_state_transition` system
添加变更点.

## Known Pitfalls

### System set configuration is per-schedule!

This is the same general caveat that applies any time you configure [system sets][cb::systemset].

Note that `app.configure_sets()` is *per-[schedule][cb::schedule]!* If you configure some sets
in one [schedule][cb::schedule], that configuration does not carry over to other schedules.

Because states are so schedule-heavy, you have to be especially careful. Don't assume
that just because you configured a set, you can use it anywhere.

For example, your sets from [`Update`] and [`FixedUpdate`] will not work in
[`OnEnter`]/[`OnExit`] for your various state transitions.

system集合要对每个调度都做配置,少了配置就会出问题.

### Events

This is the same general caveat that applies to any [systems][cb::system] with
[run conditions][cb::rc] that want to receive [events][cb::event].

When receiving [events][cb::event] in systems that don't run all the time, such
as during a pause state, you will miss any events that are sent while when
the receiving systems are not running!

To mitigate this, you could implement a [custom cleanup
strategy][cb::event-manual], to manually manage the lifetime of the relevant
event types.

接收事件+运行条件,会导致漏事件,解决方法是自定义事件清理策略,
手动维护相关事件的生命周期.
