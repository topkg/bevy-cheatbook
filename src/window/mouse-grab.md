{{#include ../include/header013.md}}

# Grabbing the Mouse

Relevant official examples:
[`mouse_grab`][example::mouse_grab].

---

For some genres of games, you want to the mouse to be restricted to the window,
to prevent it from leaving the window during gameplay.

There are two variations on this behavior ([`CursorGrabMode`]):
 - `Confined` allows the cursor to be moved, but only within the bounds of the window.
 - `Locked` fixes the cursor in place and does not allow it to move.
   - Relative [mouse motion][input::mouse-motion] [events][cb::event] still work.

To grab the cursor:

```rust,no_run,noplayground
{{#include ../code013/src/window/mouse_grab.rs:grab}}
```

To release the cursor:

```rust,no_run,noplayground
{{#include ../code013/src/window/mouse_grab.rs:ungrab}}
```

You should grab the cursor during active gameplay and release it when
the player pauses the game / exits to menu / etc.

For relative mouse movement, you should use [mouse motion][input::mouse-motion]
instead of [cursor input][input::cursor] to implement your gameplay.

鼠标捕获,部分场景下需要光标不能离开窗口,具体场景有两种:
 - `Confined`: 光标移不出窗口
 - `Locked`: 固定在一个区域,适合射击游戏,这样光标被固定在某个位置

```rust
pub enum CursorGrabMode {
    None,
    Confined, // macOS不支持这个
    Locked, // windows不支持这个
}
```
ios/android不支持光标.

从上面例子看,捕获鼠标和ime输入一样,都是可以在system中启用/关闭的.
在游戏暂停或退出时,可以释放鼠标.

## Platform Differences

macOS does not natively support `Confined` mode. Bevy will fallback to `Locked`.
If you want to support macOS and you want to use [cursor input][input::cursor],
you might want to implement a "virtual cursor" instead.

Windows does not natively support `Locked` mode. Bevy will fallback to `Confined`.
You could emulate the locked behavior by re-centering the cursor every frame:

```rust,no_run,noplayground
{{#include ../code013/src/window/mouse_grab.rs:recenter}}
```

```rust,no_run,noplayground
{{#include ../code013/src/window/mouse_grab.rs:recenter-app}}
```

平台差异.在macOS中没有`Confined`,要实现类似效果只能实现一个虚拟光标;
windows没有`Locked`模式,只能在每帧中重新将光标放在中心来实现.
