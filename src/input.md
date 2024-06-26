{{#include ./include/header014.md}}

# Input Handling

Bevy supports the following inputs:
  - [Keyboard][input::keyboard] (detect when keys are pressed or released, or for text input)
  - [Mouse][input::mouse]:
    - [Motion][input::mouse-motion] (moving the mouse, not tied to OS cursor)
    - [Cursor][input::cursor] (absolute pointer position)
    - [Buttons][input::mouse-button]
    - [Scrolling][input::mouse-wheel] (mouse wheel or touchpad gesture)
    - [Touchpad Gestures][input::touchpad-gesture] (only macOS/iOS supported)
  - [Touchscreen][input::touch] (with multi-touch)
  - [Gamepad (Controller, Joystick)][input::gamepad] (via the [gilrs][project::gilrs] library)
  - [Drag-and-Drop][input::dnd] (only for files)
  - [IME][input::ime] (for advanced text input, to support multilingual users)

The following notable input devices are ***not*** supported:
 - Accelerometers and gyroscopes for device tilt
 - Other sensors, like temperature sensors
 - Tracking individual fingers on a multi-touch trackpad, like on a touchscreen
 - Microphones and other audio input devices
 - MIDI (musical instruments), but there is an unofficial plugin: [`bevy_midi`][project::bevy_midi].

bevy支持以下输入:
 - 键盘按键的按下和释放,文本输入
 - 鼠标移动,光标位置,鼠标按键,鼠标滚轮
 - 多指触摸,触摸板手势(仅限与mac系列),触摸滚动
 - 手柄
 - 文件拖拽
 - IME 高级文本输入

bevy还不支持以下设备:
 - 加速计和陀螺仪
 - 传感器(eg:温度计)
 - 多点触控板跟踪手指
 - 麦克风和其他音频输入设备
 - 乐器MIDI, (有个非官方的bevy_midi)

---

For most input types (where it makes sense), Bevy provides two ways of
dealing with them:
  - by checking the current state via [resources][cb::res] ([input resources][builtins::res-input]),
  - or via [events][cb::event] ([input events][builtins::event-input]).

Some inputs are only provided as events.

Checking state is done using [resources][cb::res] such as [`ButtonInput`] (for
binary inputs like keys or buttons), [`Axis`] (for analog inputs), [`Touches`]
(for fingers on a touchscreen), etc. This way of handling input is very
convenient for implementing game logic. In these scenarios, you typically
only care about the specific inputs mapped to actions in your game. You can
check specific buttons/keys to see when they get pressed/released, or what
their current state is.

[Events][cb::event] ([input events][builtins::event-input]) are a lower-level,
more all-encompassing approach. Use them if you want to get all activity
from that class of input device, rather than only checking for specific inputs.

对于大多数输入,bevy提供了两种处理方式:资源和事件.
部分输入只有事件.

检查资源的状态(ButtonInput的按键;Axis的遥感方向;Touches的触摸)等.
这种处理输入的方式对实现游戏逻辑来说非常方便.

事件输入是一个低级别的处理方式,处理也更加全面.
如果想从该类输入设备获取所有活动,而不是仅检查特定输入,请使用它们.

## Input Mapping

Bevy does not yet offer a built-in way to do input mapping (configure key
bindings, etc). You need to come up with your own way of translating the
inputs into logical actions in your game/app.

There are some community-made plugins that may help with that: [see the
input-section on bevy-assets][bevyassets::input]. My personal recommendation:
[Input Manager plugin by Leafwing Studios][project::lwim]. It is opinionated
and unlikely to suit all games, but if it works for you, it is very high quality.

It may be a good idea to build your own abstractions specific to your
game. For example, if you need to handle player movement, you might want to
have a system for reading inputs and converting them to your own internal
"movement intent/action events", and then another system acting on those
custom events, to actually move the player. Make sure to use [explicit
system ordering][cb::system-order] to avoid lag / frame delays.

bevy没有内置输入映射(eg:绑定组合键),这个应该由游戏逻辑来自定义.
社区有些插件可以实现这些功能,本书作者推荐使用[这个插件](https://github.com/leafwing-studios/leafwing-input-manager),
这个插件在某些场景下非常有用(潜在意思是,没有银弹,没有万灵丹).

在游戏中维护一个抽象层非常有必要.
如果需要处理角色移动,需要在system中读取输入,并转换成自己内部的事件,
其他system就能基于内部事件继续搭积木完善游戏逻辑.

## Run Conditions

Bevy also provides [run conditions][cb::rc] ([see all of them
here][bevy::input::common_conditions]) that you can attach to your systems, if
you want a specific system to only run when a specific key or button is pressed.

This way, you can do input handling as part of the
[scheduling/configuration][cb::ecs-intro-code] of your [systems][cb::system], and
avoid running unnecessary code on the CPU.

Using these in real games is not recommended, because you have to hard-code the
keys, which makes it impossible to make user-configurable keybindings.

To support configurable keybindings, you can implement your own run conditions
that check your keybindings from your user preferences.

If you are using the [LWIM plugin][project::lwim], it also provides support for
[a similar run-condition-based workflow][lwim::common_conditions].

bevy为system提供了运行条件,可以利用这个机制让system只在某些键被按下时运行.

可以通过"调度/配置"(后面章节中提到)来避免非必要的代码占用cpu资源.

这种方式是将硬编码key写在代码中了,用户无法重新绑定,所以不推荐在真实游戏中使用.

插件(LWIM,就是上面推荐的input管理插件)同样提供了一个小型的基于运行条件的工作流.
