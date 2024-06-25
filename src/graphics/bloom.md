{{#include ../include/header012.md}}

# Bloom

The "Bloom" effect creates a glow around bright lights. It is not a
physically-accurate effect, though it is inspired by how light looks through
a dirty or imperfect lens.

Bloom does a good job of helping the perception of very bright light,
especially when outputting HDR to the display hardware is not supported.
Your monitor can only display a certain maximum brightness, so Bloom is a
common artistic choice to try to convey light intensity brighter than can
be displayed.

Bloom looks best with a [Tonemapping][cb::tonemap] algorithm that desaturates
very bright colors. Bevy's default is a good choice.

Bloom requires [HDR mode][cb::hdr] to be enabled on your camera. Add the
[`BloomSettings`][bevy::BloomSettings] component to the camera to enable
bloom and configure the effect.

```rust,no_run,noplayground
{{#include ../code012/src/graphics/bloom.rs:bloom-config}}
```

bloom(绽放)效果会在明亮的灯光周围产生辉光。
尽管它的灵感来自光线透过肮脏或不完美的镜头的样子，但它并不是物理上精确的效果。

bloom 可以很好地帮助人们感知非常明亮的光线，
尤其是在不支持将 HDR 输出到显示硬件的情况下。
您的显示器只能显示特定的最大亮度，因此 Bloom 是一种常见的艺术选择，
可以尝试传达比可显示的更亮的光强度。

bloom一般配合色调映射一起使用,能对非常明亮的光纤`去饱和`,
bevy默认的色调映射算法就是很好的选择.

bloom是通过BloomSettings组件启用的,bloom是依赖hdr的.
hdr默认是disabled的,要使用bloom就需要打开hdr.

## Bloom Settings

Bevy offers many parameters to tweak the look of the bloom effect.

The default mode is "energy-conserving", which is closer to how real light
physics might behave. It tries to mimic the effect of light scattering,
without brightening the image artificially. The effect is more subtle and "natural".

There is also an "additive" mode, which will brighten everything and make it feel
like bright lights are "glowing" unnaturally. This sort of effect is quite common
in many games, especially older games from the 2000s.

Bevy offers three bloom "presets":
 - `NATURAL`: energy-conerving, subtle, natural look.
 - `OLD_SCHOOL`: "glowy" effect, similar to how older games looked.
 - `SCREEN_BLUR`: very intense bloom that makes everything look blurred.

You can also create an entirely custom configuration by tweaking all the
parameters in [`BloomSettings`][bevy::BloomSettings] to your taste. Use the
presets for inspiration.

Here are the settings for the Bevy presets:

```rust,no_run,noplayground
// NATURAL
BloomSettings {
    intensity: 0.15,
    low_frequency_boost: 0.7,
    low_frequency_boost_curvature: 0.95,
    high_pass_frequency: 1.0,
    prefilter_settings: BloomPrefilterSettings {
        threshold: 0.0,
        threshold_softness: 0.0,
    },
    composite_mode: BloomCompositeMode::EnergyConserving,
};

// OLD_SCHOOL
BloomSettings {
    intensity: 0.05,
    low_frequency_boost: 0.7,
    low_frequency_boost_curvature: 0.95,
    high_pass_frequency: 1.0,
    prefilter_settings: BloomPrefilterSettings {
        threshold: 0.6,
        threshold_softness: 0.2,
    },
    composite_mode: BloomCompositeMode::Additive,
};

// SCREEN_BLUR
BloomSettings {
    intensity: 1.0,
    low_frequency_boost: 0.0,
    low_frequency_boost_curvature: 0.0,
    high_pass_frequency: 1.0 / 3.0,
    prefilter_settings: BloomPrefilterSettings {
        threshold: 0.0,
        threshold_softness: 0.0,
    },
    composite_mode: BloomCompositeMode::EnergyConserving,
};
```

bevy 提供了许多参数来调整光晕效果的外观。
默认效果是`energy-conserving`,这更接近真实光物理的表现。
它试图模仿光散射的效果，而不人为地使图像变亮。效果更加微妙和“自然”。

另一种效果是`additive`,这会照亮一切，让人感觉明亮的灯光不自然地发光。
这种效果在许多游戏中都很常见，尤其是 2000 年代的老游戏。

bevy提供3种默认配置:
 - NATURAL, 看起来是自然的, `energy-conserving`
 - OLD_SCHOOL, 发光效果, 和以前的老游戏类似
 - SCREEN_BLUR, 非常强烈的绽放，让一切看起来都很模糊。

不喜欢预设的可以自己调节.

---

```rust
pub enum BloomCompositeMode {
    EnergyConserving,
    Additive,
}
```

## Visualization

Here is an example of Bloom in 3D:

![The Bloom effect on street lamps.](../img/bloom_3d.png)

And here is a 2D example:

![The Bloom effect on a simple hexagon.](../img/bloom_2d.png)
