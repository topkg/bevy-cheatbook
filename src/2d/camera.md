{{#include ../include/header012.md}}

# 2D Camera Setup

[Cameras][cb::camera] in Bevy are mandatory to see anything: they configure the
rendering.

This page will teach you about the specifics of 2D cameras. If you want to learn about
general non-2D specific functionality, see the [general page on cameras][cb::camera].

bevy中的相机就是用来看东西的.

## Creating a 2D Camera

Bevy provides a [bundle][cb::bundle] ([`Camera2dBundle`][bevy::Camera2dBundle])
that you can use to [spawn][cb::commands] a camera [entity][cb::entity]. It
has reasonable defaults to set up everything correctly.

You might want to set the [transform][cb::transform], to position the camera.

```rust,no_run,noplayground
{{#include ../code012/src/d2/camera.rs:basic-setup}}
```

`Camera2dBundle`就是用于生成2d相机实体的,构造时就能指定相机的位置和变换.

## Projection

The [projection][cb::camera-projection] is what determines how coordinates map to the
[viewport][cb::camera-viewport] (commonly, the screen/window).

2D cameras always use an Orthographic projection.

When you spawn a 2D camera using [`Camera2dBundle`][bevy::Camera2dBundle],
it adds the [`OrthographicProjection`][bevy::OrthographicProjection]
[component][cb::component] to your [entity][cb::entity]. When
you are working with 2D cameras and you want to access
the projection, you should [query][cb::query] for
[`OrthographicProjection`][bevy::OrthographicProjection].

```rust,no_run,noplayground
{{#include ../code012/src/d2/camera.rs:query-projection}}
```

Note that this is different from [3D][cb::camera-3d::projection]. If you are
making a library or some other code that should be able to handle both 2D and
3D, you cannot make a single [query][cb::query] to access both 2D and 3D
cameras. You should create separate [systems][cb::system], or at least two
separate queries, to handle each kind of camera. This makes sense, as you will
likely need different logic for 2D vs. 3D anyway.

投影决定了世界的坐标系统如何映射到视窗(通常是屏幕或窗口).
2d游戏总是使用正交投影.Camera2dBundle中自动包含了OrthographicProjection,
如果要在过程中访问正交投影的值,直接在query中访问`OrthographicProjection`即可.

3d中的处理有些差异.具体说如果我们写一个lib,需要处理好2d/3d的不同,
主要是没法通过单个query查到2d/3d相机中的正交投影,
要么是分成不同的system,要么是使用两个单独的query.
这是有道理的,因为无论如何您可能需要不同的 2D 与 3D 逻辑.

### Caveat: near/far values

The projection contains the `near` and `far` values, which indicate the minimum
and maximum Z coordinate (depth) that can be rendered, relative to the position
([transform][cb::transform]) of the camera.

[`Camera2dBundle`][bevy::Camera2dBundle] sets them appropriately for 2D:
`-1000.0` to `1000.0`, allowing entities to be displayed on both positive and
negative Z coordinates. However, if you create the
[`OrthographicProjection`][bevy::OrthographicProjection] yourself, to change any
other settings, you need to set these values yourself. The default value of the
[`OrthographicProjection`][bevy::OrthographicProjection] struct is designed for
3D and has a `near` value of `0.0`, which means you might not be able to see
your 2D entities.

```rust,no_run,noplayground
{{#include ../code012/src/d2/camera.rs:projection-near-far}}
```

A more foolproof way to go about this is to use a temporary variable, to let the
bundle do its thing, and then mutate whatever you want. This way, you don't have
to worry about the exact values or getting anything wrong:

```rust,no_run,noplayground
{{#include ../code012/src/d2/camera.rs:projection-mut}}
```

远近.正价投影不会应为远近有大小之分.投影的near/far值仅仅表示z轴的值,
表示离相机的距离.Camera2dBundle将这两个值设置为-1000.0和1000.0,
在这个范围内的实体都可以参与渲染.
(之前说的渲染过滤超过1000个单位就不显示了,这个阈值来至这里.)

如果是自己创建正交投影组件,就需要注意这些值的设置,
正交投影是为3d相机而设计的,这个结构体的near字段默认值是0,
也就是说如果在2d相机中,如果不设置near,就不会看到任何实体.

如果怕丢, 就将正交投影组件的值放在一个临时变量中,所有的2d相机都使用这个,
这样就不用担心在相机内部设置时少了设置.

### Scaling Mode

You can set the [`ScalingMode`][bevy::ScalingMode] according to how you want to
handle window size / resolution.

The default for Bevy 2D cameras is to have 1 screen pixel correspond to 1 world
unit, thus allowing you to think of everything in "pixels". When the window is
resized, that causes more or less content to be seen.

If you want to keep this window resizing behavior, but change the mapping of screen
pixels to world units, use `ScalingMode::WindowSize(x)` with a value other than `1.0`.
The value represents the number of screen pixels for one world unit.

If, instead, you want to always fit the same amount of content
on-screen, regardless of resolution, you should use something like
`ScalingMode::FixedVertical` or `ScalingMode::AutoMax`. Then, you can directly
specify how many units you want to display on-screen, and your content will
be upscaled/downscaled as appropriate to fit the window size.

```rust,no_run,noplayground
{{#include ../code012/src/d2/camera.rs:scalingmode}}
```

根据对窗口大小/分辨率的处理,可以设置`ScalingMode`缩放模式.
2d相机默认1屏幕像素对应1个世界单位,这样窗口大小的改变会直接影响看到世界的大小.

如果想在改变窗口大小的同时还是能看到同样的世界,
`ScalingMode::WindowSize(x)`中的x就不能设置为1.0.
`ScalingMode::FixedVertical`或`ScalingMode::AutoMax`能保持原窗口看到的世界.
也可以直接指定要看到的世界的尺寸,或者将内容进行缩放到时和窗口大小.

---

正交投影下的缩放模式.

```rust
pub enum ScalingMode {
    // Fixed 手动直接投影大小,忽略窗口resize,图像拉伸,参数为世界单位.
    Fixed {
        width: f32,
        height: f32,
    },
    // 指定视窗大小,参数是一个世界单位等于多少像素.
    WindowSize(f32),
    // 保持宽高比,要满足最小限制,单位是世界单位.
    AutoMin {
        min_width: f32,
        min_height: f32,
    },
    // 和AutoMin类似,要满足最大限制.
    AutoMax {
        max_width: f32,
        max_height: f32,
    },
    // 保持宽高比,宽度调整,高度不变,参数是投影所需的高度,世界单位.
    FixedVertical(f32),
    // 高度调整.
    FixedHorizontal(f32),
}
```

### Zooming

To "zoom" in 2D, you can change the orthographic projection's `scale`. This
allows you to just scale everything by some factor, regardless of the
[`ScalingMode`][bevy::ScalingMode] behavior.

```rust,no_run,noplayground
{{#include ../code012/src/d2/camera.rs:zoom-scale}}
```

Alternatively, you can reconfigure the [`ScalingMode`][bevy::ScalingMode]. This
way you can be confident about how exactly coordinates/units map to the
screen. This also helps avoid scaling artifacts with 2D assets, especially
pixel art.

```rust,no_run,noplayground
{{#include ../code012/src/d2/camera.rs:zoom-scalingmode}}
```

Consider having a list of predefined "zoom levels" / scale values, so that you
can make sure your game always looks good.

If you are making a pixel-art game, you want to make sure the default texture
filtering mode is set to Nearest (and not Linear), if you want your pixels
to appear crisp instead of blurry:

```rust,no_run,noplayground
{{#include ../code012/src/d2/camera.rs:default-nearest}}
```

However, when *downscaling*, Linear (the default) filtering is preferred
for higher quality. So, for games with high-res assets, you want to leave
it unchanged.

聚焦,通过正交投影的scale来改变,在2d相机中,可以将所有事物按同样的缩放因子进行缩放.
zoom和ScalingMode互不影响.两者都对缩放有影响,选择其中一种即可.

使用zoom可以有效防止2d资产在缩放过程产生残影.

在游戏中设置一个预定义的缩放表,游戏看起来会更好.

如果您正在制作像素艺术游戏,并且希望像素显得清晰而不是模糊,
则需要确保将默认纹理过滤模式设置为`最近`(而不是`线性`).
但是,在缩小尺寸时,首选线性(默认)过滤以获得更高的质量.
因此,对于具有高分辨率资源的游戏,您希望保持其不变.
