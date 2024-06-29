{{#include ../include/header014.md}}

# Gestures

Multi-finger gestures on a Touchpad or Touchscreen are a very common
way to implement various operations, like panning, zooming, and rotating.

触摸屏上的多指手势识别可进行平移/缩放/旋转等常见操作,
而手势识别的实现还是有套路的.

## Platform Gesture Events

Bevy offers [events][cb::event] that allow you to handle gestures as they
are detected / implemented by the OS.

Currently, only macOS and iOS are supported. Other platforms may be supported
in the future.

The supported gestures are:

 - [`RotationGesture`]: rotating with two fingers
 - [`PinchGesture`]: pinch-to-zoom with two fingers
 - [`PanGesture`]: panning gesture
 - [`DoubleTapGesture`]: double-tap gesture

```rust,no_run,noplayground
{{#include ../code014/src/input/gesture.rs:platform-gesture-events}}
```

依赖平台的手势识别事件,mac系列是支持的.

## Custom Touchpad Gestures

It is not currently possible to implement your own gestures on a touchpad,
because there is no API to detect the individual fingers that are touching
the touchpad.

pad上的自定义手势识别还不支持,因为没有api来跟踪单个手指.

## Custom Touchscreen Gestures

You can (and probably should) implement your own touchscreen gestures. Bevy
offers multi-touch detection, tracking each finger that is currently on the
screen. Implementing your own gestures is be a good way to make touchscreen
input behave appropriately to your application.

[See here for more info on touchscreen input in Bevy.][input::touch]

触摸屏上的手势识别可以尝试一下.
