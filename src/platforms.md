{{#include ./include/header012.md}}

# Bevy on Different Platforms

This chapter is a collection of platform-specific information, about using
Bevy with different operating systems or environments.

Feel free to suggest things to add.

## Platform Support

Bevy aims to also make it easy to target different platforms, such as the
various desktop operating systems, web browsers (via WebAssembly), mobile
(Android and iOS), and game consoles. Your Bevy code can be the same for all
platforms, with differences only in the build process and environment setup.

However, that vision is not fully met yet. Currently, support for non-desktop
platforms is limited, and requires more complex configuration.

Bevy的目的是针对不同的平台, 让其操作简单, 比如不同的台式操作系统、 web浏览器(通过WebAssembly)、
移动端(Android和iOS), 以及游戏控制台

所有的平台上, 你的Bevy代码可以是一样的, 除了构建过程及环境启动时的差异.

然而, 目前版本还没有完全满足上面情况. 现在, 对非台式机平台的支持时有限的, 并且还要求更复杂的配置.

### Desktop

Bevy trivially works out-of-the-box on the three major desktop operating
systems: Linux, macOS, Windows. No special configuration is required.

Bevy工作在三种主流的台式机操作系统: Linux, macOS, Windows. 不需要特别的配置.

See the following pages for specific tips/advice when developing for the
desktop platforms:

针对不同操作系统开发时, 请查看如下说明

 - [Linux][platform::linux]
 - [macOS][platform::macos]
 - [Windows][platform::windows]

All Bevy features are fully supported on each of the above.

以上系统, Bevy的全部特性均被支持.

You can also build Windows EXEs for your Windows users, if you are working
in [Linux][cross::linux-windows] or [macOS][cross::macos-windows].

如果你工作在linux活MacOS上, 你也可以为Windows用户构建Windows EXEs.

### Web

Bevy works quite well on the [web (using WebAssembly)][platform::wasm],
but with some limitations.

Multithreading is not supported, so you will have limited performance and
possible audio glitches. Rendering is limited to the features of the WebGL2
API, meaning worse performance and limitations like only supporting a maximum
of 256 lights in 3D scenes. These limitations can be lifted by enabling the
new WebGPU support, but then you will have limited browser compatibility.

For inspiration, check out the entries in the Bevy Game Jams
([third][bevy::jam-03], [second][bevy::jam-02], [first][bevy::jam-01]). Many
of them have web builds you can play in your browser.

### Mobile

Apple iOS is well-supported and most features work well. There are developers
in the Bevy community that have successfully shipped Bevy-based apps to the
App Store.

Android support is not as good as iOS, but very usable (as of Bevy 0.12). If
you find bugs, broken features, or other issues, please report them.

Bevy has been known to have issues with emulator devices. It is recommended
you test your app on real hardware.

### Game Consoles

Unfortunately, due to NDA requirements, developing for consoles is inaccessible
to most community developers who work in the open, and Bevy support is still
mostly nonexistent.

At some point, there was someone in the community working on PlayStation
support. I do not know if they are still around, or anything about the
status of that work. If you are interested, join [Discord][bevy::discord]
and ask around. Maybe you can find each other and work together.

The Rust Programming Language aims to make Nintendo Switch a supported target,
but that work is in its early days and has not progressed enough to be useful
for Bevy yet. It should be possible to work on Nintendo Switch support in
the open, without NDAs, using emulators.

The Steam Deck, and other such "handheld PCs", are well supported. Such
devices run special versions of standard Desktop OSs (Linux, Windows) and are
designed to support PC games out of the box. To develop for these devices,
just make regular Linux/Windows builds of your game and ideally try them on
an actual device, so you can see how the handheld experience is like and make
sure your game feels good on such a device.
