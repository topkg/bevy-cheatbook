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

如果你工作在linux或MacOS上, 你也可以为Windows用户构建Windows EXEs.

### Web

Bevy works quite well on the [web (using WebAssembly)][platform::wasm],
but with some limitations.

Bevy在web平台(使用WebAssembly, 平台::wasm)运行得很好, 但是呢, 有一些限制.

Multithreading is not supported, so you will have limited performance and
possible audio glitches. Rendering is limited to the features of the WebGL2
API, meaning worse performance and limitations like only supporting a maximum
of 256 lights in 3D scenes. These limitations can be lifted by enabling the
new WebGPU support, but then you will have limited browser compatibility.

不支持多线程, 所以你只有限定的性能, 和可能的audio glitches(音频故障audio glitch通常指
音频播放或录制过程中出现的短暂、不正常的声音干扰).

渲染受限于WebGL2 API的特性, 这意味着更糟的性能和限制, 比如在3D场景中, 最大只支持256 lights.
启用新的WebGPU支持后, 这些限制能被改善, 但这也意味着你将拥有有限的浏览器兼容性.

For inspiration, check out the entries in the Bevy Game Jams
([third][bevy::jam-03], [second][bevy::jam-02], [first][bevy::jam-01]). Many
of them have web builds you can play in your browser.

### Mobile

Apple iOS is well-supported and most features work well. There are developers
in the Bevy community that have successfully shipped Bevy-based apps to the
App Store.

很好地支持Apple iOS, 很多特性运行得不错. Bevy社区有些开发者已经成功地将基于Bevy的apps发布到
App Store上.

Android support is not as good as iOS, but very usable (as of Bevy 0.12). If
you find bugs, broken features, or other issues, please report them.

Android支持没有IOS那么好, 但也是可用的(如Bevy0.12)

Bevy has been known to have issues with emulator devices. It is recommended
you test your app on real hardware.

据称, Bevy在模拟设备上有些问题. 建议你在真实硬件上测试你的app.

### Game Consoles(游戏控制台)

Unfortunately, due to NDA requirements, developing for consoles is inaccessible
to most community developers who work in the open, and Bevy support is still
mostly nonexistent.

不幸的是, 由于NDA需求(保密协议，Non-Disclosure Agreement), 对于很多开放社区的开发者而言, 
基于控制台的研发是不可获得的.

At some point, there was someone in the community working on PlayStation
support. I do not know if they are still around, or anything about the
status of that work. If you are interested, join [Discord][bevy::discord]
and ask around. Maybe you can find each other and work together.

一度, 社区内有人曾研究PlayStation的支持性. 我不清楚现在是否仍然在做以及任务的进展情况, 
如果你感兴趣的话, 可以加入上面的网站并咨询.

The Rust Programming Language aims to make Nintendo Switch a supported target,
but that work is in its early days and has not progressed enough to be useful
for Bevy yet. It should be possible to work on Nintendo Switch support in
the open, without NDAs, using emulators.

Rust编程语言致力于支持Nintendo Switch, 但是工作进展仍然停留在早期阶段, 并没有太多的进展,
使其可用于Bevy.

The Steam Deck, and other such "handheld PCs", are well supported. Such
devices run special versions of standard Desktop OSs (Linux, Windows) and are
designed to support PC games out of the box. To develop for these devices,
just make regular Linux/Windows builds of your game and ideally try them on
an actual device, so you can see how the handheld experience is like and make
sure your game feels good on such a device.

Steam Deck和其他的一些手柄PCs, 被很好地支持. 这些设备运行在特定版本的桌面OSs内.
