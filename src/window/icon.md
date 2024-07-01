{{#include ../include/header013.md}}

# Setting the Window Icon

You might want to set a custom Window Icon. On Windows and Linux, this is
the icon image shown in the window title bar (if any) and task bar (if any).

Unfortunately, Bevy does not yet provide an easy and ergonomic built-in way
to do this. However, it can be done via the `winit` APIs.

The way shown here is quite hacky. To save on code complexity, instead of
using Bevy's asset system to load the image in the background, we bypass
the assets system and directly load the file using the `image` library.

There is some WIP on adding a proper API for this to Bevy; see PRs
[#1163][bevy::1163], [#2268][bevy::2268], [#5488][bevy::5488],
[#8130][bevy::8130], and [Issue #1031][bevy::1031].

This example shows how to set the icon for the primary/main window, from
a Bevy startup system.

```rust,no_run,noplayground
{{#include ../code013/examples/window-icon.rs:main}}
```

Note: that [`WinitWindows`] is a [non-send resource][cb::nonsend].

Note: you need to add `winit` and `image` to your project's dependencies,
and they must be the same versions as used by Bevy. As of Bevy 0.13, that
should be `winit = "0.29"` and `image = "0.24"`. If you don't know which
version to use, you can use `cargo tree` or check `Cargo.lock` to see which
is the correct version.

窗口设置的图标会显示在标题栏和任务栏.
在bevy中只能通过winit API来实现.

为了简化复杂度,不是通过资产来加载图片,而是直接使用image库来加载.

在使用过程中有几点需要注意:
 - `WinitWindows`资源要指定为NonSend(只在主线程上运行)
 - 需要依赖`winit`和`image`,且版本还要和bevy最新版保持一致
