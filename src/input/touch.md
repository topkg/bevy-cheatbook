{{#include ../include/header09.md}}

# Touchscreen

Relevant official examples:
[`touch_input`][example::touch_input],
[`touch_input_events`][example::touch_input_events].

---

Multi-touch touchscreens are supported. You can track multiple fingers on
the screen, with position and pressure/force information. Bevy does not
offer gesture recognition.

The [`Touches`][bevy::Touches] [resource][cb::res] allows you to track any
fingers currently on the screen:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:touches}}
```

Alternatively, you can use [`TouchInput`][bevy::TouchInput] [events][cb::event]:

```rust,no_run,noplayground
{{#include ../code/examples/input.rs:touch-events}}
```

触摸支持`多指`,在屏幕上可跟踪多指的按下和压力信息,bevy原生没有提供手势识别,
如果有需要,游戏中需要自己实现.

`Touches`资源可以跟踪手指信息;`TouchInput`事件也能达到同样的效果.
