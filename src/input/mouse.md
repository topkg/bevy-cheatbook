{{#include ../include/header014.md}}

# Mouse

Relevant official examples:
[`mouse_input`][example::mouse_input],
[`mouse_input_events`][example::mouse_input_events].

---

## Mouse Buttons

Similar to [keyboard input][input::keyboard], mouse buttons are available as a
[`ButtonInput`] [resource][cb::res], [events][cb::event], and [run
conditions][cb::rc] ([see list][bevy::input::common_conditions]). Use whichever
pattern feels most appropriate to your use case.

### Checking Button State

You can check the state of specific mouse buttons using the
[`ButtonInput<MouseButton>`][`MouseButton`] [resource][cb::res]:

 - Use `.pressed(…)`/`.released(…)` to check if a button is being held down
   - These return `true` every frame, for as long as the button is in the respective state.
 - Use `.just_pressed(…)`/`.just_released(…)` to detect the actual press/release
   - These return `true` only on the frame update when the press/release happened.

```rust,no_run,noplayground
{{#include ../code014/src/input/mouse.rs:mouse-button-input}}
```

You can also iterate over any buttons that have been pressed or released:

```rust,no_run,noplayground
{{#include ../code014/src/input/mouse.rs:mouse-button-input-iter}}
```

通过ButtonInput<MourseButton>资源获取鼠标按键状态.

### Run Conditions

Another workflow is to add [run conditions][cb::rc] to your systems,
so that they only run when the appropriate inputs happen.

It is highly recommended you write your own [run conditions][cb::rc],
so that you can check for whatever you want, support configurable bindings, etc…

For prototyping, Bevy offers some [built-in run conditions][input::rc]:

```rust,no_run,noplayground
{{#include ../code014/src/input/mouse.rs:run-conditions}}
```

运行条件,上面的例子中演示了鼠标中键按下和左键拖拽.

### Mouse Button Events

Alternatively, you can use [`MouseButtonInput`] [events][cb::event] to get
all activity:

```rust,no_run,noplayground
{{#include ../code014/src/input/mouse.rs:mouse-button-events}}
```

鼠标事件.

## Mouse Scrolling / Wheel

To detect scrolling input, use [`MouseWheel`] [events][cb::event]:

```rust,no_run,noplayground
{{#include ../code014/src/input/mouse.rs:scroll-events}}
```

The [`MouseScrollUnit`] enum is important: it tells you the type of scroll
input. `Line` is for hardware with fixed steps, like the wheel on desktop
mice. `Pixel` is for hardware with smooth (fine-grained) scrolling, like
laptop touchpads.

You should probably handle each of these differently (with different
sensitivity settings), to provide a good experience on both types of hardware.

**Note:** the `Line` unit is not guaranteed to have whole number values/steps!
At least [macOS][platform::macos] does non-linear scaling / acceleration of
scrolling at the OS level, meaning your app will get weird values for the number
of lines, even when using a regular PC mouse with a fixed-stepping scroll wheel.

鼠标滚轮事件.

```rust
pub enum MouseScrollUnit {
    Line, // 按行滚动, 适合鼠标硬件
    Pixel, // 按像素滚动, 适合笔记本触摸板
}
```

mac系统对于Line滚动时,滚动的行数不是线性变化的,需要注意这个平台可能出现问题.


## Mouse Motion

Use this if you don't care about the exact position of the mouse cursor,
but rather you just want to see how much the mouse moved from frame to
frame. This is useful for things like controlling a 3D camera.

Use [`MouseMotion`] [events][cb::event]. Whenever the mouse is moved, you
will get an event with the delta.

```rust,no_run,noplayground
{{#include ../code014/src/input/mouse.rs:mouse-motion}}
```

You might want to [grab/lock the mouse inside the game
window][cookbook::mouse-grab].

不关心鼠标具体移动了多少,而关心帧间移动的差值,这种需求在3d相机中还是有很多的.
使用`MouseMotion`事件来实现.

## Mouse Cursor Position

Use this if you want to accurately track the position of the pointer /
cursor. This is useful for things like clicking and hovering over objects
in your game or UI.

You can get the current coordinates of the mouse pointer, from the respective
[`Window`] (if the mouse is currently inside that window):

```rust,no_run,noplayground
{{#include ../code014/src/input/mouse.rs:cursor-position}}
```

To detect when the pointer is moved, use [`CursorMoved`] [events][cb::event]
to get the updated coordinates:

```rust,no_run,noplayground
{{#include ../code014/src/input/mouse.rs:cursor-events}}
```

Note that you can only get the position of the mouse inside a window;
you cannot get the global position of the mouse in the whole OS Desktop /
on the screen as a whole.

The coordinates you get are in "window space". They represent window
pixels, and the origin is the **top left** corner of the window.

They do not relate to your camera or in-game coordinates in any way. [See
this cookbook example][cookbook::cursor2world] for converting these window
cursor coordinates into world-space coordinates.

To track when the mouse cursor enters and leaves your window(s), use
[`CursorEntered`] and [`CursorLeft`] [events][cb::event].

如果您想准确跟踪指针/光标的位置,请使用此选项.
这对于在游戏或 UI 中单击或悬停在对象上等操作非常有用.

`cursor_position()`可以从当前窗口获取光标当前坐标.
`EventReader<CursorMoved>`获取光标移动事件.

光标在窗口中就能获取到坐标,出了窗口就获取不到了.
能获取到的坐标都是基于窗口的,窗口的原点在坐上角.
这个坐标和相机/游戏中的坐标不是同一套,两者的坐标转换可以翻一下前面的"坐标"一节.
