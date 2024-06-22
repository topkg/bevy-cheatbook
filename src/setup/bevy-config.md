{{#include ../include/header012.md}}

# Configuring Bevy

Bevy is very modular and configurable. It is implemented as many separate
cargo crates, allowing you to remove the parts you don't need. Higher-level
functionality is built on top of lower-level foundational crates, and can
be disabled or replaced with alternatives.

The lower-level core crates (like the Bevy ECS) can also be used completely
standalone, or integrated into otherwise non-Bevy projects.

bevy是模块化和配置化的,由很多单独的crate组成,可按需移除.
高层次功能依赖低层次功能,均可以替换或disable.

ECS就是bevy底层的功能,可单独拿出来使用,甚至在非bevy项目中使用.

## Bevy Cargo Features

In Bevy projects, you can enable/disable various parts of Bevy using cargo features.

Many common features are enabled by default. If you want to disable some of
them, you need to disable all of them and re-enable the ones you need.
Unfortunately, Cargo does not let you just disable individual default features.

Here is how you might configure your Bevy:

```toml
[dependencies.bevy]
version = "0.12"
# Disable the default features if there are any that you do not want
default-features = false
features = [
  # These are the default features:
  # (re-enable whichever you like)

  # Bevy functionality:
  "multi-threaded",     # Run with multithreading
  "bevy_asset",         # Assets management
  "bevy_audio",         # Builtin audio
  "bevy_gilrs",         # Gamepad input support
  "bevy_scene",         # Scenes management
  "bevy_winit",         # Window management (cross-platform Winit backend)
  "bevy_render",        # Rendering framework core
  "bevy_core_pipeline", # Common rendering abstractions
  "bevy_gizmos",        # Support drawing debug lines and shapes
  "bevy_sprite",        # 2D (sprites) rendering
  "bevy_pbr",           # 3D (physically-based) rendering
  "bevy_gltf",          # GLTF 3D assets format support
  "bevy_text",          # Text/font rendering
  "bevy_ui",            # UI toolkit
  "animation",          # Animation support
  "tonemapping_luts",   # Support different camera Tonemapping modes (enables KTX2+zstd)
  "default_font",       # Embed a minimal default font for text/UI

  # File formats:
  "png",    # PNG image format for simple 2D images
  "hdr",    # HDR images
  "ktx2",   # Preferred format for GPU textures
  "zstd",   # ZSTD compression support in KTX2 files
  "vorbis", # Audio: OGG Vorbis

  # Platform-specific:
  "x11",                   # Linux: Support X11 windowing system
  "android_shared_stdcxx", # Android: use shared C++ library
  "webgl2",                # Web: use WebGL2 instead of WebGPU

  # These are other (non-default) features that may be of interest:
  # (add any of these that you need)

  # Bevy functionality:
  "asset_processor",      # Asset processing
  "filesystem_watcher",   # Asset hot-reloading
  "subpixel_glyph_atlas", # Subpixel antialiasing for text/fonts
  "serialize",            # Support for `serde` Serialize/Deserialize
  "async-io",             # Make bevy use `async-io` instead of `futures-lite`
  "pbr_transmission_textures", # Enable Transmission textures in PBR materials
                               # (may cause issues on old/lowend GPUs)

  # File formats:
  "dds",  # Alternative DirectX format for GPU textures, instead of KTX2
  "jpeg", # JPEG lossy format for 2D photos
  "webp", # WebP image format
  "bmp",  # Uncompressed BMP image format
  "tga",  # Truevision Targa image format
  "exr",  # OpenEXR advanced image format
  "pnm",  # PNM (pam, pbm, pgm, ppm) image format
  "basis-universal", # Basis Universal GPU texture compression format
  "zlib", # zlib compression support in KTX2 files
  "flac", # Audio: FLAC lossless format
  "mp3",  # Audio: MP3 format (not recommended)
  "wav",  # Audio: Uncompressed WAV
  "symphonia-all", # All Audio formats supported by the Symphonia library
  "shader_format_glsl", # GLSL shader support
  "shader_format_spirv", # SPIR-V shader support

  # Platform-specific:
  "wayland",              # (Linux) Support Wayland windowing system
  "accesskit_unix",       # (Unix-like) AccessKit integration for UI Accessibility
  "bevy_dynamic_plugin",  # (Desktop) support for loading of `DynamicPlugin`s

  # Development/Debug features:
  "dynamic_linking",   # Dynamic linking for faster compile-times
  "trace",             # Enable tracing for performance measurement
  "detailed_trace",    # Make traces more verbose
  "trace_tracy",       # Tracing using `tracy`
  "trace_tracy_memory", # + memory profiling
  "trace_chrome",      # Tracing using the Chrome format
  "wgpu_trace",        # WGPU/rendering tracing
  "debug_glam_assert", # Assertions to validate math (glam) usage
  "embedded_watcher",  # Hot-reloading for Bevy's internal/builtin assets
]
```

(See [here][bevy::features] for a full list of Bevy's cargo features.)

bevy的思想是crate组合,这就让bevy提供的功能能按需启用,默认的plugin包含了大量常用功能,
也可以自己按需包含.

下面是默认启用的功能,可以在Cargo.toml中通过`default-features = false`进行disable.
功能主要分类为: bevy功能,文件格式,具体平台绑定值.

默认支持的bevy功能:
 - `multi-threaded`: 多线程
 - `bevy_asset`: 资产管理
 - `bevy_audio`: 内置音频
 - `bevy_gilrs`: 手柄输入支持
 - `bevy_scene`: scene管理
 - `bevy_winit`: 窗口管理(跨平台)
 - `bevy_render`: 渲染框架核心
 - `bevy_core_pipeline`: 通用渲染抽象层
 - `bevy_gizmos`: 调试用,绘制线条和形状的小玩意
 - `bevy_sprite`: 2d 精灵渲染
 - `bevy_pbr`: 3d(基于物理的)的渲染
 - `bevy_gltf`: gltf 3d资产格式支持. (gltf是免费3d传输存储格式,跨平台,对GL图形api友好)
 - `bevy_text`: 文本/字体渲染
 - `bevy_ui`: ui工具包
 - `animation`: 动画
 - `tonemapping_luts`: 支持不同的相机色调映射模式(支持启动ktx2+zstd)
 - `default_font`: 内嵌一个最小字体,用于支持text/ui

默认支持的文件格式:
 - `png`: 简单2d图
 - `hdr`: 高分辨率图
 - `ktx2`: GPU纹理推荐格式
 - `zstd`: 支持zstd压缩的ktx2文件
 - `vorbis`: 音频ogg格式(vorbis ogg)

默认支持的具体平台绑定值:
 - `x11`: linux默认支持的窗口后端
 - `android_shared_stdcxx`: android默认后端
 - `webgl2`: web端默认使用WebGL2代替WebGPU

非默认支持的bevy功能:
 - `asset_processor`: 资产处理
 - `filesystem_watcher` 资产热加载
 - `subpixel_glyph_atlas` 文本字体的`子像素抗锯齿`
 - `serialize`: serder序列化(serder是rust中的一个序列化/反序列化库)
 - `async-io`: 使用async-io代替futures-lite
 - `pbr_transmission_textures` : 在bpr物料中启用传输纹理(bpr是基于物理的渲染,让物料在各种光照下都能呈现逼真效果)

非默认支持的文件格式:
 - `dds`: GPU纹理,代替directx格式,不是ktx2
 - `jpeg`: 有损2d图片
 - `webp`: webp图片格式
 - `bmp`: 未压缩bmp图片格式
 - `tga`: 一家美国公司开发的图片格式
 - `exr`: OpenEXR标准中的高动态范围图片
 - `pnm`: 便携且简单存储的图片格式
 - `basis-universal`: google开发的贴图格式
 - `zlib`: ktx2,外加zlib压缩
 - `flac`: 有损音频格式
 - `mp3`: 适用于音乐下载和存储,游戏场景不推荐
 - `wav`: 未压缩音频格式
 - `symphonia-all`: rust音频库,支持多种音频格式的编解码库
 - `shader_format_glsl`: glsl着色器
 - `shader_format_spirv` spri-着色器(默认支持的着色器是wgsl)

非默认支持的具体平台绑定值:
 - `wayland`: linux支持的窗口后端(sway/hyprland均是基于wayland的)
 - `accesskit_unix`: unix-like ui无障碍交互工具包
 - `bevy_dynamic_plugin`: 桌面,支持动态插件加载

非默认支持的开发/调试功能:
 - `dynamic_linking`: 动态链接: 编译会更快
 - `trace`: 开启性能观测跟踪
 - `detailed_trace`: 更详细的跟踪数据
 - `trace_tracy`: 跟踪使用tracy(tracy是一个可视化帧分析和性能调试工具)
 - `trace_chrome`: 跟踪数据使用chrome格式(rust跟踪数据可导出到chrome中查看)
 - `wgpu_trace`: 开启 wgpu/渲染 跟踪
 - `embedded_watcher`: bevy内部资产/内置资产的热加载

### Graphics / Rendering

For a graphical application or game (most Bevy projects), you can include
`bevy_winit` and your selection of Rendering features. For
[Linux][platform::linux] support, you need at least one of `x11` or `wayland`.

`bevy_render` and `bevy_core_pipeline` are required for any application using
Bevy rendering.

If you only need 2D and no 3D, add `bevy_sprite`.

If you only need 3D and no 2D, add `bevy_pbr`. If you are [loading 3D models
from GLTF files][cb::gltf], add `bevy_gltf`.

If you are using Bevy UI, you need `bevy_text` and `bevy_ui`. `default_font`
embeds a simple font file, which can be useful for prototyping, so you don't
need to have a font asset in your project. In a real project, you probably
want to use your own fonts, so your text can look good with your game's art
style. In that case, you can disable the `default_font` feature.

If you want to draw debug lines and shapes on-screen, add `bevy_gizmos`.

If you don't need any graphics (like for a dedicated game server, scientific
simulation, etc.), you may remove all of these features.

对于带界面的程序,需要带`bevy_winit`.如果是linux,`x11`和`wayland`至少选一个.

只要是渲染的,都需要`bevy_render`和`bevy_core_pipeline`.

如果只是2d不包含3d的,`bevy_sprite`要包含;
如果只是3d而不是2d的,`bevy_pbr`要包含,如果3d资产是通过gltf文件加载的,
`bevy_gltf`要包含.

如果使用bevy ui,`bevy_text`和`bevy_ui`是需要的,
`default_font`内嵌了一个简单的字体,原型开发很用,
真实项目中,还需要将其替换成自己的字体,那时就不需要`default_font`了.

如果要画一些调试的线或形状,`bevy_gizmos`有帮助.

如果不需要图像(eg:只是一个游戏服务器),上面这些都可以移除.

### File Formats

You can use the relevant cargo features to enable/disable support for loading
assets with various different file formats.

See [here][builtins::file-formats] for more information.

### Input Devices

If you do not care about [gamepad (controller/joystick)][input::gamepad]
support, you can disable `bevy_gilrs`.

`bevy_gilrs`是手柄支持,默认是支持的,可依需求去掉.

### Platform-specific

#### Linux Windowing Backend

On [Linux][platform::linux], you can choose to support X11, Wayland,
or both. Only `x11` is enabled by default, as it is the legacy system
that should be compatible with most/all distributions, to make your builds
smaller and compile faster. You might want to additionally enable `wayland`,
to fully and natively support modern Linux environments. This will add a few
extra transitive dependencies to your project.

Some Linux distros or platforms might struggle with X11 and work better with
Wayland. You should enable both for best compatibility.

linux下默认启用的是x11,虽然是老一代的产物,但稳定性和兼容性依然最强.
大多数发行版都安装了x11,能让程序小很多.wayland是新标准,需要额外的依赖.

并不是说wayland一定好于x11,看发行版对窗口后端的兼容性.

#### WebGPU vs WebGL2

On [Web/WASM][platform::web], you have a choice between these two rendering backends.

WebGPU is the modern experimental solution, offering good performance and
full feature support, but browser support for it is limited (only known to
work in very recent versions of Chrome and Firefox nightly).

WebGL2 gives the best compatibility with all browsers, but has worse performance
and some limitations on what kinds of graphics features you can use in Bevy.

The `webgl2` cargo feature selects WebGL2 if enabled. If disabled, WebGPU is used.

web/wasm上能打的就WebGPU和WebGL2.
WebGPU是高性能,且功能支持全面,但只有最近几个版本的chrome/firefox才支持.
WebGL2对所有浏览器都兼容的非常好,但性能差了不少.

虽然webgl2是默认开启的,但可以按需替换.

### Development Features

While you are developing your project, these features might be useful:

开发中以下几个功能可能非常有用.

#### Asset hot-reloading and processing

The `filesystem_watcher` feature enables support for [hot-reloading of
assets][cb::asset-hotreload], supported on desktop platforms.

The `asset_processor` feature enables support for [asset
processing][cb::asset-processor], allowing you to automatically convert and
optimize assets during development.

资产的热加载和处理.
桌面平台,`filesystem_watcher`支持资产的热加载.
`asset_processor`允许自动转换和优化资产.

#### Dynamic Linking

`dynamic_linking` causes Bevy to be built and linked as a shared/dynamic
library. This will make recompilation *much* faster during development.

This is only supported on desktop platforms. Known to work very well on Linux.
Windows and macOS are also supported, but are less tested and have had issues in
the past.

It is not recommended to enable this for release builds you intend to publish
to other people, unless you have a very good special reason to and you know
what you are doing. It introduces unneeded complexity (you need to bundle
extra files) and potential for things to not work correctly. You should only
use it during development.

For this reason, it may be convenient to specify the feature as a commandline
option to `cargo`, instead of putting it in your `Cargo.toml`. Simply run your
project like this:

```sh
cargo run --features bevy/dynamic_linking
```

You could also add this to your [IDE/editor configuration][cb::ide].

`动态链接`只在桌面系统支持,运行最好的是linux,至于windows和macOS刚刚60分.
release版本不推荐开启`动态链接`,要开启这个功能,只能从命令行操作,Cargo.toml是不行的.

```shell
cargo run --features bevy_dynamic_plugin
```

#### Tracing

The features `trace` and `wgpu_trace` may be useful for profiling and
diagnosing performance issues.

`trace_chrome` and `trace_tracy` choose the backend you want to use to
visualize the traces.

See [Bevy's official docs on profiling][bevy::profiling] to learn more.

跟踪,主要是在性能剖析和诊断上非常有效.
`trace`和`wgpu_trace`可开启跟踪,`trace_chrome`和`trace_tracy`是可选的跟踪可视化后端,用一个就行.
