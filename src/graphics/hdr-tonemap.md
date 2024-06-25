{{#include ../include/header012.md}}

# HDR

HDR (High Dynamic Range) refers to the ability of the game engine to handle
very bright lights or colors. Bevy's rendering is HDR internally. This means
you can have objects with colors that go above `1.0`, very bright lights,
or bright emissive materials. All of this is supported for both 3D and 2D.

This is not to be confused with HDR display output, which is the ability to
produce a HDR image to be displayed by a modern monitor or TV with HDR
capabilities. Bevy has no support for this yet.

The internal HDR image has to be converted down to SDR (Standard Dynamic
Range) before it can be displayed on the screen. This process is called
[Tonemapping][cb::tonemap]. Bevy supports different algorithms that can
result in a different look. It is an artistic choice what tonemapping
algorithm to use for your game.

HDR(高动态范围)是指游戏引擎处理非常明亮的灯光或颜色的能力.
bevy内部渲染支持hdr,也就是说对象的颜色可调节到1.0之上,非常亮的那种,
灯光/发光材质均如此.2d/3d均支持.

这里的hdr不是hdr显示输出(生成 HDR 图像并由具有 HDR 功能的现代显示器或电视显示的能力),
bevy还不支持hdr显示输出.

在屏幕显示之前,内部的hdr会转换称标准动态范围(sdr)这个处理过程成为`色调映射`.
bevy支持多种`色调映射`算法,看起来都不一样,选哪一种需要尝试一二.

## Camera HDR configuration

There is a per-camera toggle that lets you decide whether you want Bevy to
preserve the HDR data internally, to make it possible for subsequent passes
(such as postprocessing effects) to use it.

```rust,no_run,noplayground
{{#include ../code012/src/graphics/hdr_tonemap.rs:hdr-config}}
```

If it is enabled, Bevy's intermediate textures will be in HDR format. The
shaders output HDR values and Bevy will store them, so they can be used in later
rendering passes. This allows you to enable effects like [Bloom][cb::bloom],
that make use of the HDR data. [Tonemapping][cb::tonemap] will happen as a
post-processing step, after the HDR data is no longer needed for anything.

If it is disabled, the shaders are expected to output standard RGB colors in
the 0.0 to 1.0 range. [Tonemapping][cb::tonemap] happens in the shader. The
HDR information is not preserved. Effects that require HDR data, like Bloom,
will not work.

It is disabled by default, because this results in better performance and
reduced VRAM usage for applications with simple graphics that do not need it.

If you have both HDR and MSAA enabled, it is possible you might encounter
issues. There might be visual artifacts in some cases. It is also unsupported on
Web/WASM, crashing at runtime. Disable MSAA if you experience any such issues.

相机的hdr配置.如果启用,中间的纹理将启用hdr格式,着色器输出dhr值,由bevy存储,
并进行后续的渲染操作.bloom特效就使用到了这些hdr数据.
当后续操作不再使用hdr数据时,色调映射就会将hdr转换为sdr,作为后处理的一部分.

如果不启用hdr,着色器输出的颜色就是[0.0,1.0],色调映射在着色器就完成了.
bevy也不会存储hdr数据,依赖hdr数据的特效(eg:bloom盛开的效果)就不起作用.

hdr默认是disabled的.因为简单一点的程序并不需要,且可以减少内存使用,性能也高一点.
追求高质量效果的,可以添加上dhr.

hdr和msaa(多重抗锯齿)同时使用会导致问题(视觉残影);
hdr+msaa还不支持wasm/web(这两个平台会导致panic),
如果出现异常,建议disable msaa.

## Tonemapping

Tonemapping is the step of the rendering process where the colors of pixels are
converted from their in-engine intermediate repesentation into the final values
as they should be displayed on-screen.

This is very important with HDR applications, as in that case the image can
contain very bright pixels (above 1.0) which need to be remapped into a range
that can be displayed.

Tonemapping is enabled by default. Bevy allows you to configure it via the
([`Tonemapping`][bevy::Tonemapping]) component, per-camera. Disabling it is not
recommended, unless you know you only have very simple graphics that don't need
it. It can make your graphics look incorrect.

```rust,no_run,noplayground
{{#include ../code012/src/graphics/hdr_tonemap.rs:tonemap-config}}
```

Bevy supports many different tonemapping algorithms. Each of them results in a
different look, affecting colors and brightness. It can be an artistic choice. You
can decide what algorithm looks best for your game. Bevy's default is TonyMcMapface,
which, despite the silly name, provides very good results for a wide variety of
graphics styles. See the ([`Tonemapping`][bevy::Tonemapping]) documentation for
an explanation of each of the available choices.

Some tonemapping algorithms (incl. the default TonyMcMapface) require the
`tonemapping_luts` [cargo feature][cb::features]. It is enabled by default. Be
sure to re-enable it if you disable default features and you need it. Enabling
it also enables the `ktx2` and `zstd` features, because it works by embedding
special data in KTX2 format into your game, which is used during tonemapping.

The following tonemapping algorithms *DO NOT* require the special data from
`tonemapping_luts`:
 - Reinhard
 - ReinhardLuminance
 - AcesFitted
 - SomewhatBoringDisplayTransform

The following tonemapping algorithms *require* the special data from `tonemapping_luts`:
 - AgX
 - TonyMcMapface
 - BlenderFilmic

If you want to make a smaller game binary (might be important for Web games),
you could reduce bloat by changing the default tonemapping to something
simpler and disabling the [cargo features][cb::features].

色调映射是渲染过程中其中一个步骤,
负责将bevy引擎的像素颜色(中间表示形式)转换为屏幕最终渲染的状态.

对于hdr来说,色调映射尤为重要,因为hdr的颜色都太亮了,必须重新映射为sdr才能显示.
色调映射默认已经开启了,bevy通过Tonemapping组件来配置色调映射,
每个相机都可以配置,不推荐disable,除非我们的图形非常简单.

bevy支持多重色调映射算法,不同的算法对颜色和亮度的影响不同.按需选择.
默认是`TonyMcMapface`. 部分算法依赖`tonemapping_luts`功能,这个功能是默认开启的.
如果禁用了DefaultPlugins,记得re-enable这个功能.这个功能还启用了ktx2/zstd的支持,
因为它的工作原理是将KTX2格式的特殊数据嵌入到游戏中,在色调映射过程中使用.

依赖`tonemapping_luts`的算法包括:默认的`TonyMcMapface`,以及`Agx`和`BlenderFilmic`.
如果对程序大小特别敏感的,可以换成其他算法,然后禁用`tonemapping_luts`功能.

---

色调映射是一个着色器,尝试将线性输入刺激映射到给定相机实体的感知均匀的图像中.

```rust
pub enum Tonemapping {
    None,
    Reinhard,
    ReinhardLuminance,
    AcesFitted,
    AgX,
    SomewhatBoringDisplayTransform,
    TonyMcMapface,
    BlenderFilmic,
}
```

## Color Grading

Color Grading is a manipulation of the overall look of the image.

Together with tonemapping, this affects the "tone"/"mood" of the final image.

This is also how you can implement a "retina" effect, where the camera
dynamically adapts to very dark (such as inside a cave) and very bright
(such as in daylight) scenes, by adjusting exposure/gamma.

You can also adjust color saturation. Heavily desaturating the image can
result in a greyscale or muted appearance, which can be a great artistic
choice for apocalyptic or horror games.

You can configure these parameters via the [`ColorGrading`][bevy::ColorGrading]
component:

```rust,no_run,noplayground
{{#include ../code012/src/graphics/hdr_tonemap.rs:color-grading}}
```

颜色分级是对图像整体外观的处理,常配合色调映射一起使用.

这也是实现“视网膜”效果的方法,
其中相机通过调整曝光/伽玛动态适应非常黑暗(例如在洞穴内)和非常明亮(例如在日光下)的场景。

还可以调整颜色饱和度.严重降低图像的饱和度可能会导致灰度或柔和的外观,
这对于世界末日或恐怖游戏来说是一个很好的艺术选择。

通过ColorGrading组件来配置颜色分级的参数.

---

Retina 效果指的是苹果公司在其设备上使用的一种高分辨率显示技术，
目的是使屏幕上的像素密度达到或超过人眼在正常视距下无法分辨的程度，
从而提供更清晰、更细腻的显示效果。

Retina 显示技术本身并不直接涉及自动调节曝光(exposure)或伽马(gamma)值.
True Tone 显示:这种技术使用多个传感器来监测周围环境光线的颜色和亮度,
然后自动调整显示屏的白平衡和色温,使屏幕颜色看起来更自然.

## Deband Dithering

Deband dithering helps color gradients or other areas with subtle changes in
color to appear higher-quality, without a "color banding" effect.

It is enabled by default, and can be disabled per-camera.

```rust,no_run,noplayground
{{#include ../code012/src/graphics/hdr_tonemap.rs:deband-dither}}
```

Here is an example image without dithering (top) and with dithering (bottom).
Pay attention to the quality/smoothness of the green color gradient on the
ground plane. In games with photorealistic graphics, similar situations can
arise in the sky, in dark rooms, or lights glowing with a bloom effect.

![Visual comparison of a scene simple cube on a flat green plane, with dithering disabled/enabled.](../img/dithering.png)

去色带抖动有助于颜色渐变或其他颜色发生细微变化的区域显得更高质量，而不会出现“色带”效果。

去色带抖动是通过组件DebandDither来配置的.

上图是这是没有抖动（顶部）和有抖动（底部）的示例图像。
请注意地面上的绿色渐变的质量/平滑度。
在具有逼真图形的游戏中，天空、黑暗的房间或具有绽放效果的灯光中也会出现类似的情况。

---

Deband 和 Dithering 是图像处理中的两个技术，用于不同的目的。

Debanding 是一种减少或消除图像中颜色条纹（banding）现象的技术。
颜色条纹是指在颜色渐变区域中，由于颜色深度不足而产生的明显色带。
这种现象通常出现在具有平滑渐变的区域，如天空、阴影或渐变背景中。

Debanding 技术通过在图像中引入微小的颜色变化，打破这些明显的色带，
从而使颜色过渡看起来更加平滑和自然。

Dithering 是一种通过引入伪随机噪声来模拟更多颜色和灰度级别的技术，
特别是在颜色深度较低的情况下。
Dithering 的目的是通过在相邻像素间引入细微的颜色变化，
使得整体效果看起来更加平滑，减少颜色量化引起的视觉伪影。

在一些场景中，debanding 和 dithering 可以结合使用。
首先，应用 debanding 技术消除明显的颜色条纹，
然后通过 dithering 技术进一步平滑颜色过渡，模拟更多的颜色和灰度级别。

总结

Debanding：用于减少或消除图像中的颜色条纹现象，使颜色过渡更加平滑。  
Dithering：通过引入伪随机噪声模拟更多颜色和灰度级别，减少颜色量化引起的伪影。
