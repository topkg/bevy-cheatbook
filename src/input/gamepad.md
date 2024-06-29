{{#include ../include/header014.md}}

# Gamepad (Controller, Joystick)

Relevant official examples:
[`gamepad_input`][example::gamepad_input],
[`gamepad_input_events`][example::gamepad_input_events].

---

Bevy has support for gamepad input hardware, using [gilrs][project::gilrs]:
console controllers, joysticks, etc. Many different kinds of hardware should
work, but if your device is not supported, you should file an issue with the
[gilrs][project::gilrs] project.

bevy利用gilrs库来实现手柄的输入.

## Gamepad IDs

Bevy assigns a unique ID ([`Gamepad`]) to each connected gamepad. For local
multiplayer, this lets you associate each device with a specific player and
distinguish which one your inputs are coming from.

You can use the [`Gamepads`] [resource][cb::res] to list the IDs of all the
currently connected gamepad devices, or to check the status of a specific one.

```rust,no_run,noplayground
{{#include ../code014/src/input/gamepad.rs:gamepads}}
```

bevy给每个连接的手柄都分配了唯一id,这个id是用于区分输入源的.

上面的例子是通过Gamepads资源显示所有已连接的手柄.

### Handling Connections / Disconnections

To detect when gamepads are connected or disconnected, you can use
[`GamepadEvent`] [events][cb::event].

Example showing how to remember the first connected gamepad ID:

```rust,no_run,noplayground
{{#include ../code014/src/input/gamepad.rs:gamepad-connect-disconnect}}
```

使用`GamepadEvent`事件来检测手柄的连接状态.

```rust
pub enum GamepadEvent {
    Connection(GamepadConnectionEvent),
    Button(GamepadButtonChangedEvent),
    Axis(GamepadAxisChangedEvent),
}
```

上面维护手柄连接状态的例子,用资源保存了一个手柄信息,并未支持多手柄.

## Handling Gamepad Inputs

The `Axis<GamepadAxis>` ([`Axis`], [`GamepadAxis`]) [resource][cb::res]
keeps track of the current value of the different axes: X/Y for each thumb
stick, and the Z axes (the analog triggers).

Buttons can be handled with the `ButtonInput<GamepadButton>`
([`ButtonInput`], [`GamepadButton`]) [resource][cb::res], similar to [mouse
buttons][input::mouse-button] or [keyboard keys][input::keyboard].

```rust,no_run,noplayground
{{#include ../code014/src/input/gamepad.rs:gamepad-input}}
```

Notice that the names of buttons in the [`GamepadButton`] `enum` are
vendor-neutral (like `South` and `East` instead of X/O or A/B).

Some game controllers have additional buttons and axes beyond what is available
on a standard controller, for example:
 - HOTAS (stick for flight sim)
 - steering wheel + pedals (for car driving games)

These are represented by the `Other(u8)` variant in [`GamepadButton`]/[`GamepadAxis`].
The `u8` value is hardware-specific, so if you want to support such devices,
your game needs to have a way for your users to configure their input bindings.

手柄输入的第一类就是遥感.
通过`Axis`和`GamepadAxis`可获取遥感的方向和模拟的Z轴.
遥感按钮是通过`ButtonInput<GamepadButton>`获取的.

```rust
#[derive(Debug, Resource)]
pub struct Axis<T> {
    /// The position data of the input devices.
    axis_data: HashMap<T, f32>,
}

// 如何从资源中获取输入,下面分析一下.起点就是system的资源参数`Res<Axis<GamepadAxis>>`.
// Axis用hashmap来维护输入的值,key为输入类型,value为输入的值;获取值就使用get().
// 从使用的流程上讲,只需要构造一个GamepadAxis来能获取到对应手柄对应的输入事件的值了.

// GamepadAxis设计的如此暴力(简单有效)
pub struct GamepadAxis {
    pub gamepad: Gamepad, // 哪个手柄的
    pub axis_type: GamepadAxisType, // 哪个类型的输入
}

// 左右手柄的xyz是在这儿区分的.
pub enum GamepadAxisType {
    LeftStickX,
    LeftStickY,
    LeftZ,
    RightStickX,
    RightStickY,
    RightZ,
    Other(u8),
}

// 遥感按钮.
// GamepadButtonType枚举, 在bevy中重新取了名字,按键的数量和w3c的标准手柄按钮是能对上的.
pub struct GamepadButton {
    pub gamepad: Gamepad,
    pub button_type: GamepadButtonType,
}
```

bevy中的手柄按钮是贴近某些手柄厂商的,不是贴近w3c标准的.
部分设备还支持非标输入,eg:开车游戏中的方向盘和踏板,飞行游戏中的飞行棒,
这些都用`Other(u8)`来处理.游戏需要提供一种方法让用户来绑定按键.

### Events

Alternatively, if you want to detect all activity as it comes in, you
can also handle gamepad inputs using [`GamepadEvent`] [events][cb::event]:

```rust,no_run,noplayground
{{#include ../code014/src/input/gamepad.rs:gamepad-input-events}}
```

如果要捕获手柄的所有事件,用`GamepadEvent`.

## Gamepad Settings

You can use the [`GamepadSettings`] [resource][cb::res] to configure dead-zones
and other parameters of the various axes and buttons. You can set the global
defaults, as well as individually per-axis/button.

Here is an example showing how to configure gamepads with custom settings
(not necessarily *good* settings, please don't copy these blindly):

```rust,no_run,noplayground
{{#include ../code014/src/input/gamepad.rs:gamepad-settings}}
```

To tie the examples together: if you have the [system][cb::system] from the
[connect/disconnect example](#handling-connections--disconnections) earlier
above on this page, to update our `MyGamepad` resource, we can configure
the system from the above example with a [run condition][cb::rc], so that
the gamepad settings are updated whenever a new gamepad is connected and
selected to be used:

```rust,no_run,noplayground
{{#include ../code014/src/input/gamepad.rs:gamepad-settings-app}}
```

每个遥感都可以设置`死区`,也可以全局配置.

## Gamepad Rumble

To cause rumble/vibration, use the [`GamepadRumbleRequest`] event. Every
event you send will add a "rumble" with a given intensity that lasts for
a given duration of time. As you send multiple events, each requested rumble
will be tracked independently, and the actual hardware vibration intensity
will be the sum of all the rumbles currently in progress.

You can also send a `Stop` event to immediately cancel any ongoing rumbling.

The intensity of each rumble is represented as two values: the "strong"
motor and the "weak" motor. These might produce different-feeling vibrations
on different hardware.

```rust,no_run,noplayground
{{#include ../code014/src/input/gamepad.rs:gamepad-rumble}}
```

手柄振动.`GamepadRumbleRequest`事件.
您发送的每个事件都会添加具有给定强度并持续给定时间的震动.
当您发送多个事件时,每个请求的震动都会被单独跟踪,
实际的硬件振动强度将是当前正在进行的所有震动的总和.

`Stop`事件会立马终止振动.
振动有时长,有强弱.

强弱能设置为渐进的.
