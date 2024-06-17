[]:#(ANCHOR: assets)
 - [`Image`][bevy::Image]:
   Pixel data, used as a texture for 2D and 3D rendering;
   also contains the [`SamplerDescriptor`][bevy::SamplerDescriptor] for texture filtering settings
 - [`TextureAtlas`][bevy::TextureAtlas]:
   2D "Sprite Sheet" defining sub-images within a single larger image
 - [`Mesh`][bevy::Mesh]:
   3D Mesh (geometry data), contains vertex attributes (like position, UVs, normals)
 - [`Shader`][bevy::Shader]:
   GPU shader code, in one of the supported languages (WGSL/SPIR-V/GLSL)
 - [`ColorMaterial`][bevy::ColorMaterial]:
   Basic "2D material": contains color, optionally an image
 - [`StandardMaterial`][bevy::StandardMaterial]:
   "3D material" with support for Physically-Based Rendering
 - [`AnimationClip`][bevy::AnimationClip]:
   Data for a single animation sequence, can be used with [`AnimationPlayer`][bevy::AnimationPlayer]
 - [`Font`][bevy::Font]:
   Font data used for text rendering
 - [`Scene`][bevy::Scene]:
   Scene composed of literal ECS entities to instantiate
 - [`DynamicScene`][bevy::DynamicScene]:
   Scene composed with dynamic typing and reflection
 - [`Gltf`][bevy::Gltf]:
   [GLTF Master Asset][cb::gltf-master]: index of the entire contents of a GLTF file
 - [`GltfNode`][bevy::GltfNode]:
   Logical GLTF object in a scene
 - [`GltfMesh`][bevy::GltfMesh]:
   Logical GLTF 3D model, consisting of multiple `GltfPrimitive`s
 - [`GltfPrimitive`][bevy::GltfPrimitive]:
   Single unit to be rendered, contains the Mesh and Material to use
 - [`AudioSource`][bevy::AudioSource]:
   Audio data for `bevy_audio`
 - [`FontAtlasSet`][bevy::FontAtlasSet]:
   (internal use for text rendering)
 - [`SkinnedMeshInverseBindposes`][bevy::SkinnedMeshInverseBindposes]:
   (internal use for skeletal animation)

目前资源asset类型包含以下:

 - `Image`: 像素数据,在2d/3d渲染中用作纹理,也包含纹理过滤的GPU采样描述
 - `TextureAtlas`: 纹理图集,多个对象可以从一个大图中取Image,这样可以减少纹理切换的开销
 - `Mesh`: 3D Mesh对象是一种几何数据,包含顶点属性(eg:位置/UV/法线)
 - `Shader`: 着色器,支持WGSL/SPIRV/GLSL语法
 - `ColorMaterial`: 基础的2d材质,包含Image中的颜色/透明度
 - `StandardMaterial`: 支持基于物理渲染的3d材质
 - `AnimationClip`: 单个动画序列帧, 由`AnimationPlayer`使用
 - `Font`: 文本渲染使用到的字体数据
 - `Scene`: 一组实体/组件/资源的几何,world可复用的一部分
 - `DynamicScene`: 可序列资源和动态实体的集合
 - `Gltf`: gltf主要资源,gltf文件全部内容的索引.gltf是opengl传递格式,用于传输和加载3d场景和模型
 - `GltfNode`: scene中的逻辑gltf对象
 - `GltfMesh`: gltf 3d模型
 - `GltfPrimitive`: gltf原语,单个可渲染单元,包括Mesh和Material
 - `AudioSource`: bevy_audio使用的音频数据
 - `FontAtlasSet`: 内部使用的,用于文本渲染
 - `SkinnedMeshInverseBindposes`: 内部使用的,骨骼动画

[]:#(ANCHOR_END: assets)

[]:#(ANCHOR: file-formats)
Image formats (loaded as [`Image`][bevy::Image] assets):

|Format    |Cargo feature      |Default?|Filename extensions           |
|----------|-------------------|--------|------------------------------|
|PNG       |`"png"`            |Yes     |`.png`                        |
|HDR       |`"hdr"`            |Yes     |`.hdr`                        |
|KTX2      |`"ktx2"`           |Yes     |`.ktx2`                       |
|KTX2+zstd |`"ktx2", "zstd"`   |Yes     |`.ktx2`                       |
|JPEG      |`"jpeg"`           |No      |`.jpg`, `.jpeg`               |
|WebP      |`"webp"`           |No      |`.webp`                       |
|OpenEXR   |`"exr"`            |No      |`.exr`                        |
|TGA       |`"tga"`            |No      |`.tga`                        |
|PNM       |`"pnm"`            |No      |`.pam`, `.pbm`, `.pgm`, `.ppm`|
|BMP       |`"bmp"`            |No      |`.bmp`                        |
|DDS       |`"dds"`            |No      |`.dds`                        |
|KTX2+zlib |`"ktx2", "zlib"`   |No      |`.ktx2`                       |
|Basis     |`"basis-universal"`|No      | `.basis`                     |

常见的图片格式包括:png/jpeg/webp/bmp.

- hdr是高动态范围图像（High-Dynamic Range Image）的文件格式,能存储更多亮度和色彩信息
- ktx2是(Khronos Texture Container)格式的第二代版本，专为高校存储和传输纹理数据而设计,特别是3d和有效开发领域
- exr是一种高动态范围(HDR)图像格式,是OpenEXR标准的一部分
- tga的全称为Tagged Image File Format或Tagged Graphics，一家美国公司开发的,在图形设计/视频编辑/游戏开发有广泛应用
- pnm是Netpbm（Network Portable Graphics）图像文件格式家族的一部分,使用简单便携的方式来存储和交换图像数据
- dds全称为DirectDraw Surface,微软开发的一种图像文件格式,主要用于directX程序,可优化3d图形渲染的性能
- basis是google开发的Basis Universal贴图格式,转为webgl和其他3d实时应用设计,保持高质量的同时还减少了文件大小,适合游戏/vr/网络场景,可作为jpeg/png的代替品,存储效率和跨平台兼容性都非常优秀

***KTX2格式由Khronos Group开发，这个组织也负责维护OpenGL、Vulkan和Gltf等图形API标准。***

Audio formats (loaded as [`AudioSource`][bevy::AudioSource] assets):

|Format    |Cargo feature|Default?|Filename extensions    |
|----------|-------------|--------|-----------------------|
|OGG Vorbis|`"vorbis"`   |Yes     |`.ogg`, `.oga`, `.spx` |
|FLAC      |`"flac"`     |No      |`.flac`                |
|WAV       |`"wav"`      |No      |`.wav`                 |
|MP3       |`"mp3"`      |No      |`.mp3`                 |

音频格式,默认启用的ogg.
 - ogg: 开源,音质比mp3好;兼容性不好. 在网络流媒体场景下特别能打
 - flac: 无损;文件体积大. 在音乐库,高质量流媒体场景下特别能打
 - wav: 无损; 未压缩体积非常大. 专业录音,音频编辑场景下特别能打
 - mp3: 体积小; 有损音质差. 音乐下载,在线音乐服务场景下特别能打

3D asset (model or scene) formats:

|Format|Cargo feature|Default?|Filename extensions|
|------|-------------|--------|-------------------|
|GLTF  |`"bevy_gltf"`|Yes     |`.gltf`, `.glb`    |

3d资源格式只支持gltf.

Shader formats (loaded as [`Shader`][bevy::Shader] assets):

|Format|Cargo feature          |Default?|Filename extensions      |
|------|-----------------------|--------|-------------------------|
|WGSL  |n/a                    |Yes     |`.wgsl`                  |
|GLSL  |`"shader_format_glsl"` |No      |`.vert`, `.frag`, `.comp`|
|SPIR-V|`"shader_format_spirv"`|No      |`.spv`                   |

着色器默认支持wgsl.
 - wgsl: WebGPU Shading Language, 是WebGPU编写着色器的语法
 - glsl: 是OpenGL 着色器语法
 - spir-v: 是khronos group 为vulkan设计的,是一种着色器的低级中间件表示(ir格式),跨平台,高性能

其中vert是顶点着色器,frag是段着色器,geom是几何着色器.

Font formats (loaded as [`Font`][bevy::Font] assets):

|Format  |Cargo feature|Default?|Filename extensions|
|--------|-------------|--------|-------------------|
|TrueType|n/a          |Yes     |`.ttf`             |
|OpenType|n/a          |Yes     |`.otf`             |

字体ttf/otf两种都是默认支持的.

Bevy Scenes:

|Format              |Filename extensions|
|--------------------|-------------------|
|RON-serialized scene|`.scn`,`.scn.ron`  |

场景格式支持scn,scn通常是游戏场景文件,包含了场景必要的信息,
如地形/单位位置/胜利条件,最初被实时策略游戏使用,现在使用非常广泛.
`.scn.ron`是使用rust对象标记格式进行编码的,ron是一种类似json的格式,
rust原生支持ron.

[]:#(ANCHOR_END: file-formats)

[]:#(ANCHOR: wgpu-backends)
[`wgpu`][project::wgpu] (and hence Bevy) supports the following backends:

|Platform|Backends (in order of priority)|
|--------|-------------------------------|
|Linux   |Vulkan, GLES3                  |
|Windows |DirectX 12, Vulkan, GLES3      |
|macOS   |Metal                          |
|iOS     |Metal                          |
|Android |Vulkan, GLES3                  |
|Web     |WebGPU, WebGL2                 |

On GLES3 and WebGL2, some renderer features are unsupported and performance is worse.

WebGPU is experimental and few browsers support it.

`wgpu`是rust编写的图形库,实现了WebGPU规范,为web和原生应用提供了跨平台一致的api,
性能很高,相比WebGL,WebGPU更加底层,对于GPU是直接访问,性能和灵活性都高很多.
跨平台是wgpu的特点,目前安卓到linux/web/macOS/windows都是支持的.
是下一代图形处理技术.复杂图形渲染/三维实时可视化场景下是强项,
适用于游戏开发/数据可视化/科学计算.`可以说bevy能起飞,wgpu就是核心基石之一`.
wgpu跨平台,Vulkan(通用跨平台图像库,下一代的技术)/Metal(Apple)/D3D12(windows)/
OpenGL/WebGL2和WebGPU(wasm),所以可以跨平台.

从上面的表格上看各个平台都有了支持.未来不知道是否可以支持主机平台.

[]:#(ANCHOR_END: wgpu-backends)

[]:#(ANCHOR: bundles)

[(List in API Docs)][bevy::impl::Bundle]

Any tuples of up to 15 [`Component`][bevy::Component] types are valid bundles.

General:
 - [`SpatialBundle`][bevy::SpatialBundle]:
   Contains the required [transform][cb::transform] and [visibility][cb::visibility]
   components that must be included on *all* entities that need rendering or [hierarchy][cb::hierarchy]
 - [`TransformBundle`][bevy::TransformBundle]:
   Contains only the transform types, subset of `SpatialBundle`
 - [`VisibilityBundle`][bevy::VisibilityBundle]:
   Contains only the visibility types, subset of `SpatialBundle`

bevy提供了实现Bundle的具体类型,都是元组类型,最多能指定15个组件,
bevy依据不同的场景内置了一些Bundle实现,下面看看具体有哪些实现.

通用Bundle包含以下类型:

`SpatialBundle`: 空间Bundle. (如果要继承,需要包含下面4个组件),下列组件应包含在内:
 - `Visibility` 实体是否要显示
 - `ComputedVisibility` 由算法决定实体是否要显示或提取出来做渲染
 - `Transform` 放置或移动实体,表示实体基于父对象的的位置.(如果没有父对象,就是基于帧的位置)
 - `GlobalTransform` 实体的全局变换,表示实体基于帧的位置

***在Main调度中Update之后是PostUpdate,在PostUpdate中会执行TransformPropagate(这是一个system集合),
TransformPropagate集合中有个system是Transform,在这个Transform system中会执行GlobalTransform的处理.***

`Main调度` -- `PostUpdate调度` -- `TransformPropagate系统集` -- `Transform系统` -- `处理GlobalTransform`.

```rust
pub struct SpatialBundle {
    pub visibility: Visibility,
    pub transform: Transform,
    pub computed: ComputedVisibility,
    pub global_transform: GlobalTransform,
}
```

`TransformBundle`: 变换Bundle,是`SpatialBundle`的变换子集.

```rust
pub struct TransformBundle {
    pub local: Transform,
    pub global: GlobalTransform,
}
```

`VisibilityBundle`: 可视Bundle,是`SpatialBundle`的可视子集.

```rust
pub struct VisibilityBundle {
    pub visibility: Visibility,
    pub computed: ComputedVisibility,
}
```

Scenes:
 - [`SceneBundle`][bevy::SceneBundle]:
   Used for spawning scenes
 - [`DynamicSceneBundle`][bevy::DynamicSceneBundle]:
   Used for spawning dynamic scenes

场景Bundle.

```rust
pub struct SceneBundle {
    pub scene: Handle<Scene>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

pub struct DynamicSceneBundle {
    pub scene: Handle<DynamicScene>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}
```

scene是描述场景的数据,主要是实体/组件的数据,可以进行序列号和反序列,
有了场景数据,就能进行存盘或从磁盘加载.关卡设计都可以用到scene.
静态场景常表示静态的,在设计时就定义好的场景;动态场景则是动态生成,运行时更新的场景.
动态场景提供了更加灵活的更新和增量修改能力.

Audio:
 - [`AudioBundle`][bevy::AudioBundle]:
   Play [audio][cb::audio] from an [`AudioSource`][bevy::AudioSource] asset
 - [`SpatialAudioBundle`][bevy::SpatialAudioBundle]:
   Play [positional audio][cb::audio-spatial] from an [`AudioSource`][bevy::AudioSource] asset
 - [`AudioSourceBundle`][bevy::AudioSourceBundle]:
   Play audio from a [custom data source/stream][cb::audio-custom]
 - [`SpatialAudioSourceBundle`][bevy::SpatialAudioSourceBundle]:
   Play positional audio from a [custom data source/stream][cb::audio-custom]

音频Bundle.
 - `AudioBundle` 从AudioSource资源播放音频
 - `SpatialAudioBundle` 从AudioSource资源播放空间音频
 - `AudioSourceBundle` 从`自定义源/自定义流`播放音频
 - `SpatialAudioSourceBundle` 从`自定义源/自定义流`播放空间音频

```rust
pub struct AudioSource {
    // 音频的原始数据,格式为wav/ogg/flac/mp3中的一种,
    // bevy默认开启支持的ogg,其他格式需要在功能出开启支持.
    pub bytes: Arc<[u8]>,
}
```

Bevy 3D:
 - [`Camera3dBundle`][bevy::Camera3dBundle]:
   3D camera, can use perspective (default) or orthographic projection
 - [`TemporalAntiAliasBundle`][bevy::TemporalAntiAliasBundle]:
   Add this to a 3D camera to enable TAA
 - [`ScreenSpaceAmbientOcclusionBundle`][bevy::ScreenSpaceAmbientOcclusionBundle]:
   Add this to a 3D camera to enable SSAO
 - [`MaterialMeshBundle`][bevy::MaterialMeshBundle]:
   3D Object/Primitive: a Mesh and a custom Material to draw it with
 - [`PbrBundle`][bevy::PbrBundle]:
   `MaterialMeshBundle` with the default Physically-Based Material ([`StandardMaterial`][bevy::StandardMaterial])
 - [`DirectionalLightBundle`][bevy::DirectionalLightBundle]: 
   3D directional light (like the sun)
 - [`PointLightBundle`][bevy::PointLightBundle]: 
   3D point light (like a lamp or candle)
 - [`SpotLightBundle`][bevy::SpotLightBundle]: 
   3D spot light (like a projector or flashlight)

bevy 3d Bundle.
 - `Camera3dBundle` 3d摄像头,用于透视投影(默认)或正交投影
 - `TemporalAntiAliasBundle` taa(抗锯齿),可用于3d摄像头
 - `ScreenSpaceAmbientOcclusionBundle` ssao(屏幕空间环境光遮蔽),可用于3d摄像头
 - `MaterialMeshBundle` 网格和材质,3d原语
 - `PbrBundle` 是`MaterialMeshBundle`的一种,材质是`StandardMaterial`(一种基于pbr的材质,可从Color/Image中直接生成)
 - `DirectionalLightBundle` 3d定向光(eg:阳光).(定向光在现实中并不存在,但近似为很远的光,月光就属于定向光)
 - `PointLightBundle` 3d点光源(eg:蜡烛/台灯).(点光源是从中心点向各个方向发射光的灯)
 - `SpotLightBundle` 3d聚光灯(eg:投影仪/手电筒).

透视投影（Perspective Projection）和3D正交投影（Orthographic Projection）是计算机图形学中两种基本的投影方式，
用于将三维场景投影到二维平面上。它们在视觉效果和应用场景上有显著的区别。  
`透视投影`模拟了人类眼睛和相机的视角特性，即远处的物体看起来会更小，近处的物体会更大。
这种投影方式能够提供逼真的深度感。  
`正交投影`不会根据物体距离相机的远近而缩放物体大小。
它适合于需要精确测量和对比的场景，如CAD软件、2D游戏和某些类型的策略游戏。

Temporal Anti-Aliasing(`TAA`)是一种抗锯齿技术，用于计算机图形学中，以减少动态场景中出现的锯齿状边缘。
TAA 利用多帧信息来平滑对象边缘，从而在运动中保持高质量的视觉效果。

屏幕空间环境光遮蔽 (Screen Space Ambient Occlusion, `SSAO`) 是一种在实时渲染中常用的技术，
用于模拟环境光遮蔽（Ambient Occlusion, AO）的效果。
AO 是一种全局光照效果，用于模拟光线在物体之间反弹时被遮挡的情况，
通常用于增强物体之间的阴影和细节，使场景看起来更加真实和立体。

物理基础渲染（Physically Based Rendering, `PBR`）
是一种通过模拟光与物体表面交互的物理现象来实现更逼真和一致视觉效果的渲染方法。
PBR广泛应用于游戏、电影和虚拟现实等领域，它能在各种光照条件下生成一致且逼真的图像。

Bevy 2D:
 - [`Camera2dBundle`][bevy::Camera2dBundle]:
   2D camera, uses orthographic projection + other special configuration for 2D
 - [`SpriteBundle`][bevy::SpriteBundle]: 
   2D sprite ([`Image`][bevy::Image] asset type)
 - [`SpriteSheetBundle`][bevy::SpriteSheetBundle]:
   2D sprite ([`TextureAtlas`][bevy::TextureAtlas] asset type)
 - [`MaterialMesh2dBundle`][bevy::MaterialMesh2dBundle]:
   2D shape, with custom Mesh and Material (similar to 3D objects)
 - [`Text2dBundle`][bevy::Text2dBundle]:
   Text to be drawn in the 2D world (not the UI)

bevy 2d Bundle.
 - `Camera2dBundle` 2d摄像头,用于正交投影和其他2D的特殊配置
 - `SpriteBundle` 精灵.Image asset资源表示
 - `SpriteSheetBundle` 精灵. TextureAtlas asset资源表示. TextureAtlas(包含多个纹理图集,eg: spritesheet 精灵表,tilemap 瓦片图)
 - `MaterialMesh2dBundle` 2d形状,带网格和材质.(和3d的MaterialMeshBundle类似)
 - `Text2dBundle` 2d世界要渲染的文本(通过2d Camera2dBundle在2d scene要渲染的文本)

`sprite` 精灵,一个独立的图像元素,通常用于表示游戏中的角色、道具、背景元素等。
精灵可以在屏幕上移动、旋转、缩放，并且可以进行动画以展现各种效果。

```rust
pub struct SpriteBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}
```

Bevy UI:
 - [`NodeBundle`][bevy::NodeBundle]:
   Empty node element (like HTML `<div>`)
 - [`ButtonBundle`][bevy::ButtonBundle]:
   Button element
 - [`ImageBundle`][bevy::ImageBundle]:
   Image element ([`Image`][bevy::Image] asset type)
 - [`AtlasImageBundle`][bevy::AtlasImageBundle]:
   Image element ([`TextureAtlas`][bevy::TextureAtlas] asset type)
 - [`TextBundle`][bevy::TextBundle]:
   Text element

ui Bundle.
 - `NodeBundle` 空Node元素,可包含各种子node的容器,是最基础的ui元素
 - `ButtonBundle` 按钮
 - `ImageBundle` Image图像
 - `AtlasImageBundle` TextureAtlas图像.(包含多个纹理图集)
 - `TextBundle` 文本
[]:#(ANCHOR_END: bundles)

[]:#(ANCHOR: resources-config)
 - [`ClearColor`][bevy::ClearColor]:
   Global renderer background color to clear the window at the start of each frame
 - [`GlobalVolume`][bevy::GlobalVolume]:
   The overall volume for playing audio
 - [`AmbientLight`][bevy::AmbientLight]:
   Global renderer "fake lighting", so that shadows don't look too dark / black
 - [`Msaa`][bevy::Msaa]:
   Global renderer setting for Multi-Sample Anti-Aliasing (some platforms might only support the values 1 and 4)
 - [`UiScale`][bevy::UiScale]:
   Global scale value to make all UIs bigger/smaller
 - [`GizmoConfig`][bevy::GizmoConfig]:
   Controls how [gizmos][cb::gizmos] are rendered
 - [`WireframeConfig`][bevy::WireframeConfig]:
   Global toggle to make everything be rendered as wireframe
 - [`GamepadSettings`][bevy::GamepadSettings]:
   Gamepad input device settings, like joystick deadzones and button sensitivities
 - [`WinitSettings`][bevy::WinitSettings]:
   Settings for the OS Windowing backend, including update loop / power-management settings
 - [`TimeUpdateStrategy`][bevy::TimeUpdateStrategy]:
   Used to control how the [`Time`][bevy::Time] is updated
 - [`Schedules`][bevy::Schedules]:
   Stores all [schedules][cb::schedule], letting you register additional functionality at runtime
 - [`MainScheduleOrder`][bevy::MainScheduleOrder]:
   The sequence of [schedules][cb::schedule] that will run every frame update

以下是常用的配置资源.
 - `ClearColor` 背景色
 - `GlobalVolume` 音量大小
 - `AmbientLight` 环境光(有了环境光,阴影就不会太暗或直接是黑色)
 - `Msaa` 多重采样抗锯齿,部分平台只支持1-4个样本.bevy默认是4个样本
 - `UiScale` UI缩放值,可影响所有UI
 - `GizmoConfig` 小玩意的配置
 - `WireframeConfig` 网格线框是否显示
 - `GamepadSettings` 手柄设置(eg:摇杆死区和按钮灵敏度)
 - `WinitSettings` 窗口后端设置,包括更新循环/电源管理设置(winit封装了各个平台窗口的功能)
 - `TimeUpdateStrategy` 时间更新策略.默认的`自动策略`就很好了;在测试/网络处理,可以手动设置下一个时间.(每个游戏系统都有一个时间系统的)
 - `Schedules` 存储了所有的调度器,在运行时可以注册附加逻辑
 - `MainScheduleOrder` 指定每帧调度器执行顺序(当然是在Main调度器中的,因为每帧逻辑更新就是在Main调度器中)

多重采样抗锯齿（Multisample Anti-Aliasing，`MSAA`）是一种图形处理技术，
用于减少在渲染3D图形时出现的锯齿边缘（aliasing）。
锯齿边缘是由于有限的分辨率和像素数量在描绘斜线或曲线时造成的。
MSAA通过在每个像素内采样多个位置并平均这些样本的颜色来平滑这些锯齿边缘。
[]:#(ANCHOR_END: resources-config)

[]:#(ANCHOR: resources-main)
 - [`Time`][bevy::Time]:
   Global time-related information (current frame delta time, time since startup, etc.)
 - [`FixedTime`][bevy::FixedTime]:
   Tracks remaining time until the next [fixed update][cb::fixedtimestep]
 - [`AssetServer`][bevy::AssetServer]:
   Control the asset system: Load assets, check load status, etc.
 - [`Assets<T>`][bevy::Assets]:
   Contains the actual data of the loaded assets of a given type
 - [`State<T>`][bevy::State]:
   The current value of a [states type][cb::state]
 - [`NextState<T>`][bevy::NextState]:
   Used to queue a transition to another [state][cb::state]
 - [`Gamepads`][bevy::Gamepads]:
   Tracks the IDs for all currently-detected (connected) gamepad devices
 - [`SceneSpawner`][bevy::SceneSpawner]:
   Direct control over spawning Scenes into the main app World
 - [`FrameCount`][bevy::FrameCount]:
   The total number of frames
 - [`ScreenshotManager`][bevy::ScreenshotManager]:
   Used to request a screenshot of a window to be taken/saved
 - [`AppTypeRegistry`][bevy::AppTypeRegistry]:
   Access to the Reflection Type Registry
 - [`AsyncComputeTaskPool`][bevy::AsyncComputeTaskPool]:
   Task pool for running background CPU tasks
 - [`ComputeTaskPool`][bevy::ComputeTaskPool]:
   Task pool where the main app schedule (all the systems) runs
 - [`IoTaskPool`][bevy::IoTaskPool]:
   Task pool where background i/o tasks run (like asset loading)
 - [`WinitWindows`][bevy::WinitWindows] ([non-send][cb::nonsend]):
   Raw state of the `winit` backend for each window
 - [`NonSendMarker`][bevy::NonSendMarker]:
   Dummy resource to ensure a system always runs on the main thread

以下是常用的引擎资源.
 - `Time` 全局和时间相关的信息(当前帧的增量时间,启动到现在的时间等)
 - `FixedTime` 跟踪到下一次FixedUpdate的剩余时间
 - `AssetServer` Asset资产服务,控制着asset资产的加载/检查加载状态等
 - `Assets<T>` 指定资产的实际数据
 - `State<T>` 状态类型的当前值
 - `NextState<T>` 要转换到另一个状态,就将转换进行排队
 - `Gamepads` 跟踪当前已连接的手柄设备ID列表(ebiten中还需要自己查,这个就直接自己处理了,棒)
 - `SceneSpawner` 场景生成,直接生成场景到main world
 - `FrameCount` 总帧数
 - `ScreenshotManager` 窗口截屏(目前大部分游戏都支持截屏), 此资源是开关控制
 - `AppTypeRegistry` 访问注册类型
 - `AsyncComputeTaskPool` 异步GPU任务池
 - `ComputeTaskPool` Main调度中system运行的任务池
 - `IoTaskPool` 后台IO任务池(eg:资产加载)
 - `WinitWindows` (non-send),窗口的原始状态
 - `NonSendMarker` 确保system运行在main线程的虚拟资源
[]:#(ANCHOR_END: resources-main)

[]:#(ANCHOR: resources-wgpu)
 - [`RenderDevice`][bevy::RenderDevice]:
   The GPU device, used for creating hardware resources for rendering/compute
 - [`RenderQueue`][bevy::RenderQueue]:
   The GPU queue for submitting work to the hardware
 - [`RenderAdapter`][bevy::RenderAdapter]:
   Handle to the physical GPU hardware
 - [`RenderAdapterInfo`][bevy::RenderAdapterInfo]:
   Information about the GPU hardware that Bevy is running on

以下是常用的wgpu资源.
 - `RenderDevice` GPU设备,用于创建渲染/计算的硬件资源
 - `RenderQueue` 提交任务给硬件的GPU队列
 - `RenderAdapter` 物理GPU硬件句柄
 - `RenderAdapterInfo` bevy正在使用的GPU硬件信息
[]:#(ANCHOR_END: resources-wgpu)

[]:#(ANCHOR: resources-render)
 - [`MainWorld`][bevy::MainWorld]:
   (extract schedule only!) access data from the Main World
 - [`RenderGraph`][bevy::RenderGraph]:
   [The Bevy Render Graph][cb::render::graph]
 - [`PipelineCache`][bevy::PipelineCache]:
   Bevy's manager of render pipelines. Used to store render pipelines used by the app, to avoid
   recreating them more than once.
 - [`TextureCache`][bevy::TextureCache]:
   Bevy's manager of temporary textures. Useful when you need textures to use internally
   during rendering.
 - [`DrawFunctions<P>`][bevy::DrawFunctions]:
   Stores draw functions for a given phase item type
 - [`RenderAssets<T>`][bevy::RenderAssets]:
   Contains handles to the GPU representations of currently loaded asset data
 - [`DefaultImageSampler`][bevy::DefaultImageSampler]:
   The default sampler for [`Image`][bevy::Image] asset textures
 - [`FallbackImage`][bevy::FallbackImage]:
   Dummy 1x1 pixel white texture. Useful for shaders that normally need a texture, when
   you don't have one available.

以下是常用的渲染资源.
 - `MainWrold` **仅提取调度器**, 访问Main world的数据
 - `RenderGraph` 渲染图
 - `PipelineCache` 渲染管道,要避免重复创建
 - `TextureCache` 临时纹理管理,渲染时非常有用
 - `DrawFunctions` 绘制函数, 存储指定阶段元素的类型对应的绘制函数
 - `RenderAssets<T>` 渲染资产
 - `DefaultImageSampler` Image资产问题的默认采样器
 - `FallbackImage` 1x1的虚拟白色纹理,对于着色器来说,需要纹理但没有纹理时,使用这个
[]:#(ANCHOR_END: resources-render)

[]:#(ANCHOR: resources-input)
 - [`Input<KeyCode>`][bevy::KeyCode]:
   Keyboard key state, as a binary [Input][bevy::Input] value
 - [`Input<MouseButton>`][bevy::MouseButton]:
   Mouse button state, as a binary [Input][bevy::Input] value
 - [`Input<GamepadButton>`][bevy::GamepadButton]:
   Gamepad buttons, as a binary [Input][bevy::Input] value
 - [`Axis<GamepadAxis>`][bevy::GamepadAxis]:
   Analog [Axis][bevy::Axis] gamepad inputs (joysticks and triggers)
 - [`Axis<GamepadButton>`][bevy::GamepadButton]:
   Gamepad buttons, represented as an analog [Axis][bevy::Axis] value
 - [`Touches`][bevy::Touches]:
   The state of all fingers currently touching the touchscreen
 - [`Gamepads`][bevy::Gamepads]:
   Registry of all the connected [`Gamepad`][bevy::Gamepad] IDs

以下是常用的输入资源.
 - `Input<KeyCode>` 键盘按键状态(KeyCode表示某个键,状态为已按下/刚刚按下/刚刚释放)
 - `Input<MouseButton>` 鼠标按键
 - `Input<GamepadButton>` 手柄按键
 - `Axis<GamepadAxis>` 手柄摇杆方向
 - `Axis<GamepadButton>` 手柄遥感按键
 - `Touches` 触摸状态
 - `Gamepads` 已连接手柄的ID列表
[]:#(ANCHOR_END: resources-input)

[]:#(ANCHOR: events-input)
 - [`MouseButtonInput`][bevy::MouseButtonInput]:
   Changes in the state of mouse buttons
 - [`MouseWheel`][bevy::MouseWheel]:
   Scrolling by a number of pixels or lines ([`MouseScrollUnit`][bevy::MouseScrollUnit])
 - [`MouseMotion`][bevy::MouseMotion]:
   Relative movement of the mouse (pixels from previous frame), regardless of the OS pointer/cursor
 - [`CursorMoved`][bevy::CursorMoved]:
   New position of the OS mouse pointer/cursor
 - [`KeyboardInput`][bevy::KeyboardInput]:
   Changes in the state of keyboard keys (keypresses, not text)
 - [`ReceivedCharacter`][bevy::ReceivedCharacter]:
   Unicode text input from the OS (correct handling of the user's language and layout)
 - [`Ime`][bevy::Ime]:
   Unicode text input from IME (support for advanced text input in different scripts)
 - [`TouchInput`][bevy::TouchInput]:
   Change in the state of a finger touching the touchscreen
 - [`GamepadEvent`][bevy::GamepadEvent]:
   Changes in the state of a gamepad or any of its buttons or axes
 - [`GamepadRumbleRequest`][bevy::GamepadRumbleRequest]:
   Send these events to control gamepad rumble
 - [`TouchpadMagnify`][bevy::TouchpadMagnify]:
   Pinch-to-zoom gesture on laptop touchpad (macOS)
 - [`TouchpadRotate`][bevy::TouchpadRotate]:
   Two-finger rotate gesture on laptop touchpad (macOS)

以下是常用的输入事件.
 - `MouseButtonInput` 鼠标事件
 - `MouseWheel` 鼠标滚轮事件
 - `MouseMotion` 鼠标的相对(上帧)移动事件
 - `CursorMoved` 光标移动事件
 - `KeyboardInput` 键盘事件(按键不是输入)
 - `ReceivedCharacter` 输入Unicode文本事件
 - `Ime` 输入法输入Unicode文本事件
 - `TouchInput` 触摸板事件
 - `GamepadEvent` 手柄事件
 - `GamepadRumbleRequest` 控制手柄震动的事件
 - `TouchpadMagnify` 触摸板捏合缩放事件(macOS支持)
 - `TouchpadRotate` 触摸板二指旋转事件(macOS支持)
[]:#(ANCHOR_END: events-input)

[]:#(ANCHOR: events-system)
 - [`RequestRedraw`][bevy::RequestRedraw]:
   In an app that does not refresh continuously, request one more update before going to sleep
 - [`FileDragAndDrop`][bevy::FileDragAndDrop]:
   The user drag-and-dropped a file into our app
 - [`CursorEntered`][bevy::CursorEntered]:
   OS mouse pointer/cursor entered one of our windows
 - [`CursorLeft`][bevy::CursorLeft]:
   OS mouse pointer/cursor exited one of our windows
 - [`WindowCloseRequested`][bevy::WindowCloseRequested]:
   OS wants to close one of our windows
 - [`WindowCreated`][bevy::WindowCreated]:
   New application window opened
 - [`WindowClosed`][bevy::WindowClosed]:
   Bevy window closed
 - [`WindowDestroyed`][bevy::WindowDestroyed]:
   OS window freed/dropped after window close
 - [`WindowFocused`][bevy::WindowFocused]:
   One of our windows is now focused
 - [`WindowMoved`][bevy::WindowMoved]:
   OS/user moved one of our windows
 - [`WindowResized`][bevy::WindowResized]:
   OS/user resized one of our windows
 - [`WindowScaleFactorChanged`][bevy::WindowScaleFactorChanged]:
   One of our windows has changed its DPI scaling factor
 - [`WindowBackendScaleFactorChanged`][bevy::WindowBackendScaleFactorChanged]:
   OS reports change in DPI scaling factor for a window

以下是常用的OS/窗口/控制事件.
 - `RequestRedraw` 在不连续刷新的app中,在sleep之前请求1次或多次更新的事件
 - `FileDragAndDrop` 文件拖拽事件
 - `CursorEntered` 光标移入窗口的事件
 - `CursorLeft` 光标移出窗口的事件
 - `WindowCloseRequested` OS想要关闭一个窗口的事件
 - `WindowCreated` 新窗口已打开事件
 - `WindowClosed` 窗口已关闭事件
 - `WindowDestroyed` 窗口关闭后,OS已释放窗口的事件
 - `WindowFocused` 窗口聚焦事件
 - `WindowMoved` 窗口移动事件
 - `WindowResized` 窗口resize事件
 - `WindowScaleFactorChanged` DPI缩放因子修改导致窗口变更事件
 - `WindowBackendScaleFactorChanged` 系统报告的窗口DPI缩放因子改变事件

[]:#(ANCHOR_END: events-system)

[]:#(ANCHOR: events-engine)
 - [`AssetEvent<T>`][bevy::AssetEvent]:
   Sent by Bevy when [asset data][cb::asset] has been added/modified/removed; [can be used to detect changes to assets][cb::assetevent]
 - [`HierarchyEvent`][bevy::HierarchyEvent]:
   Sent by Bevy when entity [parents/children][cb::hierarchy] change
 - [`AppExit`][bevy::AppExit]:
   Tell Bevy to shut down

以下是常用的引擎事件.
 - `AssetEvent<T>` 资产增删改事件,可用于检测资产变化
 - `HierarchyEvent` 实体父子关系变更事件
 - `AppExit` 让bevy关闭的事件
[]:#(ANCHOR_END: events-engine)

[]:#(ANCHOR: systemparams)

[(List in API Docs)][bevy::impl::SystemParam]

In regular [systems][cb::system]:

[]:#(ANCHOR: systemparam-regular)

 - [`Commands`][bevy::Commands]:
   Manipulate the ECS using [commands][cb::commands]
 - [`Query<T, F = ()>`][bevy::Query] (can contain tuples of up to 15 types):
   Access to [entities and components][cb::ec]
 - [`Res<T>`][bevy::Res]:
   Shared access to a [resource][cb::res]
 - [`ResMut<T>`][bevy::ResMut]:
   Exclusive (mutable) access to a [resource][cb::res]
 - `Option<Res<T>>`:
   Shared access to a resource that may not exist
 - `Option<ResMut<T>>`:
   Exclusive (mutable) access to a resource that may not exist
 - [`Local<T>`][bevy::Local]:
   Data [local][cb::local] to the system
 - [`EventReader<T>`][bevy::EventReader]:
   Receive [events][cb::event]
 - [`EventWriter<T>`][bevy::EventWriter]:
   Send [events][cb::event]
 - [`&World`][bevy::World]:
   Read-only [direct access to the ECS World][cb::world]
 - [`ParamSet<...>`][bevy::ParamSet] (with up to 8 params):
   Resolve [conflicts between incompatible system parameters][cb::paramset]
 - [`Deferred<T>`][bevy::Deferred]:
   Custom ["deferred mutation"][cb::deferred], similar to `Commands`, but for your own things
 - [`RemovedComponents<T>`][bevy::RemovedComponents]:
   [Removal detection][cb::removal-detection]
 - [`Gizmos`][bevy::Gizmos]:
   A way to [draw lines and shapes][cb::gizmos] on the screen for debugging and dev purposes
 - [`Diagnostics`][bevy::Diagnostics]:
   A way to [report measurements/debug data][cb::diagnostics] to Bevy for tracking and visualization
 - [`SystemName`][bevy::SystemName]:
   The name (string) of the system, may be useful for debugging
 - [`ParallelCommands`][bevy::ParallelCommands]:
   Abstraction to help use `Commands` when you will do your own parallelism
 - [`WorldId`][bevy::WorldId]:
   The World ID of the [world][cb::world] the system is running on
 - [`ComponentIdFor<T>`][bevy::ComponentIdFor]:
   Get the [`ComponentId`][bevy::ComponentId] of a given [component][cb::component] type
 - [`Entities`][bevy::Entities]:
   Low-level ECS metadata: All entities
 - [`Components`][bevy::Components]:
   Low-level ECS metadata: All components
 - [`Bundles`][bevy::Bundles]:
   Low-level ECS metadata: All bundles
 - [`Archetypes`][bevy::Archetypes]:
   Low-level ECS metadata: All archetypes
 - [`SystemChangeTick`][bevy::SystemChangeTick]:
   Low-level ECS metadata: Tick used for change detection
 - [`NonSend<T>`][bevy::NonSend]:
   Shared access to [Non-`Send`][cb::nonsend] (main thread only) data
 - [`NonSendMut<T>`][bevy::NonSendMut]:
   Exclusive access to [Non-`Send`][cb::nonsend] (main thread only) data
 - `Option<NonSend<T>>`:
   Shared access to [Non-`Send`][cb::nonsend] (main thread only) data that may not exist
 - `Option<NonSendMut<T>>`:
   Exclusive access to [Non-`Send`][cb::nonsend] (main thread only) data that may not exist
 - [`StaticSystemParam`][bevy::StaticSystemParam]:
   Helper for generic system abstractions, to avoid lifetime annotations
 - tuples containing any of these types, with up to 16 members

常规system,不独占ecs world.
 - `Commands`: 命令,维护ecs最常用的方式
 - `Query<T, F= ()>`: 访问实体和组件的方式,最多可以访问15个组件,system最常使用的参数
 - `Res<T>`: 共享访问资源的方式
 - `ResMut<T>`: 独占访问资源的方式
 - `Option<Res<T>>`: 共享访问资源的方式,资源可能不存在
 - `Option<ResMut<T>>`: 独占访问资源的方式,资源可能不存在
 - `Local<T>`: system自己的数据,local不保存在world中,而是在system中,在system后续运行中也存在
 - `EventReader<T>`: 接收事件
 - `EventWriter<T>`: 发送事件
 - `&world`: 直接访问world的方式,共享的
 - `ParamSet<...>`: 解决system参数不兼容的方式,rust在编译期不知道bevy的ecs,运行时ecs要遵循rust的规则,通过此参数可以告诉bevy要处理不兼容
 - `Deferred<T>`: 延时变更,Commands底层也是使用这个,有需要可以直接使用
 - `RemovedComponents<T>`: 删除组件
 - `Gizmos`: 在调试或开发时绘制的一些小玩意儿,线条或形状
 - `Diagnostics`: 跟踪或显示bevy的诊断数据
 - `SystemName`: 在调试可能用作识别system
 - `ParallelCommands`: 并行命令
 - `WorldId`: world id 标识
 - `ComponentIdFor<T>`: 从组件类型获取组件ID
 - `Entities`: ecs底层元数据: 所有实体
 - `Components`: ecs底层元数据: 所有组件
 - `Bundles`: ecs底层元数据: 所有bundle信息
 - `Archetypes`: ecs底层元数据: 所有原型.一个原型表示共享同一组组件的实体集
 - `SystemChangeTick`: ecs底层元数据: `变更检测`使用到的tick
 - `NonSend<T>`: 共享访问Non-Send数据. Non-Send数据只能在主线程中访问,eg:窗口/图形/音频/和OS底层接口交互的.
 - `NonSendMut<T>`: 独占访问Non-Send数据.
 - `Option<NonSend<T>>`: 共享访问Non-Send数据, 数据可能不存在.
 - `Option<NonSendMut<T>>`: 独占访问Non-Send数据, 数据可能不存在.
 - `StaticSystemParam`: 简化生命周期写法的辅助器
 - 元组,最多16个类型

[]:#(ANCHOR_END: systemparam-regular)

In [exclusive systems][cb::exclusive]:

[]:#(ANCHOR: systemparam-exclusive)

 - [`&mut World`]:
   Full [direct access to the ECS World][cb::world]
 - [`Local<T>`]:
   Data [local][cb::local] to the system
 - [`&mut SystemState<P>`][`SystemState`]:
   Emulates a regular system, allowing you to easily access data from the World.
   `P` are the system parameters.
 - [`&mut QueryState<Q, F = ()>`][`QueryState`]:
   Allows you to perform queries on the World, similar to a [`Query`] in regular systems.

[]:#(ANCHOR_END: systemparam-exclusive)
 
[]:#(ANCHOR: systemparam-limits)

独占system的参数(此类system在执行时会独占world):
 - `&mut world`: 直接访问world的方式,独占的
 - `Local<T>`: system自己的数据,local不保存在world中,而是在system中,在system后续运行中也存在
 - `&mut SystemState<P>` `SystemState`: 模拟普通system,可以简单从world访问数据,P就是system参数
 - `&mut QueryState<Q, F= ()>` `QueryState`: 从world执行查询,类似普通system的Query

Your function can have a maximum of 16 total parameters. If you need more,
group them into tuples to work around the limit. Tuples can contain up to
16 members, but can be nested indefinitely.

函数最多有16个参数,如果需要更多,就使用元组来突破限制.
元组同样最多只能有16个参数,但没有嵌套限制.

[]:#(ANCHOR_END: systemparam-limits)

Systems running during the [Extract schedule][cb::render::extract] can also use
[`Extract<T>`][bevy::Extract], to access data from the Main World instead of the
Render World. `T` can be any read-only system parameter type.

在外部调度器中的system也可以使用`Extract<T>`从Main(不是Render)中访问数据,
T可以是任意system 参数类型.

[]:#(ANCHOR_END: systemparams)

[]:#(ANCHOR: gltf-asset-labels)
The following asset labels are supported (`{}` is the numerical index):
  - `Scene{}`: GLTF Scene as Bevy [`Scene`][bevy::Scene]
  - `Node{}`: GLTF Node as [`GltfNode`][bevy::GltfNode]
  - `Mesh{}`: GLTF Mesh as [`GltfMesh`][bevy::GltfMesh]
  - `Mesh{}/Primitive{}`: GLTF Primitive as Bevy [`Mesh`][bevy::Mesh]
  - `Mesh{}/Primitive{}/MorphTargets`: Morph target animation data for a GLTF Primitive
  - `Texture{}`: GLTF Texture as Bevy [`Image`][bevy::Image]
  - `Material{}`: GLTF Material as Bevy [`StandardMaterial`][bevy::StandardMaterial]
  - `DefaultMaterial`: as above, if the GLTF file contains a default material with no index
  - `Animation{}`: GLTF Animation as Bevy [`AnimationClip`][bevy::AnimationClip]
  - `Skin{}`: GLTF mesh skin as Bevy [`SkinnedMeshInverseBindposes`][bevy::SkinnedMeshInverseBindposes]

GLTF (Graphics Library Transmission Format)资源中使用的标签主要涉及以下几方面:
描述3D模型、场景结构、材质、纹理等元素的元数据。
这些标签帮助组织和定义了GLTF文件中的不同组件，
使得它们能够在不同的平台和应用程序间有效传输和渲染。
以下是一些关键的GLTF资源标签及其用途概述： 

- asset: 包含了GLTF文件的基本元数据，如版本号、版权信息、生成工具等。
- scene: 定义场景的基本构成，一个GLTF文件可以包含多个场景，每个场景可以引用不同的节点集合。
- scenes: 场景列表，每个场景定义了场景的根节点，以及初始默认场景。
- nodes: 节点列表，定义了3D空间中的对象（如模型、灯光、相机等）及其变换（位置、旋转、缩放）。
- meshes: 网格列表，描述了3D对象的几何形状，包括顶点、索引和顶点属性（如法线、UV坐标）。
- materials: 材质列表，定义了如何渲染网格表面，包括颜色、纹理、光照模型等属性。
- textures: 纹理列表，存储图像数据，用于材质贴图、环境映射等。
- images: 图像数据列表，可以是图片文件的引用或直接嵌入的图像数据。
- samplers: 定义了如何采样纹理，如过滤和环绕方式。
- accessors: 数据访问器，描述如何从缓冲区视图中读取和解释数组数据（如顶点坐标、颜色数据等）。
- bufferViews: 缓冲区视图，指定缓冲区中数据的偏移量和长度，以及数据的字节排列方式。
- buffers: 缓冲区，存储大量的原始二进制数据，如顶点坐标、索引等。
- cameras: 相机定义，描述了如何从3D场景渲染到2D图像，包括透视相机和正交相机。
- animations: 动画列表，定义了场景中节点或材质随时间变化的动画序列。
- skins: 皮肤信息，用于绑定网格顶点到骨骼，实现蒙皮动画。
- extensions: 扩展，允许在GLTF规范基础上添加额外的功能或特定平台的支持。
- extras: 附加数据，用于存储不被GLTF规范直接定义的信息，通常供特定应用程序或工具使用。

bevy中使用到了: scene/node/mesh/mesh原语/变形动画数据/纹理/材质/默认材质/动画/皮肤.

[]:#(ANCHOR_END: gltf-asset-labels)

[]:#(ANCHOR: schedules)
Internally, Bevy has these built-in [schedules][cb::schedule]:
 - [`Main`][bevy::Main]:
   runs every frame update cycle, to perform general app logic
 - [`ExtractSchedule`][bevy::ExtractSchedule]:
   runs after `Main`, to copy data from the Main World into the Render World
 - [`Render`][bevy::Render]:
   runs after `ExtractSchedule`, to perform all rendering/graphics, in parallel with the next `Main` run

在bevy内置了如下调度器:
 - `Main`主调度器,每帧都会运行,用于执行app的一般逻辑.
 - `ExtractSchedule`外部调度器,在Main之后运行,用于将Main世界的数据拷贝到Render世界.
 - `Render`渲染调度器,在ExtractSchedule之后运行,执行巡染操作,与下一个Main并行执行.

The `Main` schedule simply runs a sequence of other schedules:

On the first run (first frame update of the app):
 - [`PreStartup`][bevy::PreStartup]
 - [`Startup`][bevy::Startup]
 - [`PostStartup`][bevy::PostStartup]

On every run (controlled via the [`MainScheduleOrder`][bevy::MainScheduleOrder] [resource][cb::res]):
 - [`First`][bevy::First]: any initialization that must be done at the start of every frame
 - [`PreUpdate`][bevy::PreUpdate]: for engine-internal systems intended to run before user logic
 - [`StateTransition`][bevy::StateTransition]: perform any pending [state][cb::state] transitions
 - [`RunFixedUpdateLoop`][bevy::RunFixedUpdateLoop]: runs the [`FixedUpdate`][bevy::FixedUpdate] schedule as many times as needed
 - [`Update`][bevy::Update]: for all user logic (your systems) that should run every frame
 - [`PostUpdate`][bevy::PostUpdate]: for engine-internal systems intended to run after user logic
 - [`Last`][bevy::Last]: any final cleanup that must be done at the end of every frame

`FixedUpdate` is for all user logic (your systems) that should run at a [fixed timestep][cb::fixedtimestep].

`StateTransition` runs the
[`OnEnter(...)`][bevy::OnEnter]/[`OnTransition(...)`][bevy::OnTransition]/[`OnExit(...)`][bevy::OnExit]
schedules for your [states][cb::state], when you want to change state.

Main调度器的工作仅仅是按顺序调用以下调度器:

首帧运行:
 - PreStartup
 - Startup
 - PostStartup

每帧运行:
 - First: 帧初始化
 - PreUpdate: 引擎内部的前置处理,会先于用户逻辑执行
 - StateTransition: 执行挂起状态的转换
 - RunFixedUpdateLoop: 按需执行多次FixedUpdate调度器
 - Update: 所有用户逻辑(我们编写的system)
 - PostUpdate: 引擎内部的后置处理
 - Last: 帧清理

`FixedUpdate`调度器适合这种system:不依赖显示器刷新率,而是有自己固定的频率,
app如果执行快,就可能跳过FixedUpdate,如果执行慢,就可能执行多次FixedUpdate.

当你想要改变状态时,StateTransition会运行OnEnter/OnTransition/OnExit调度器.

[]:#(ANCHOR_END: schedules)

[]:#(ANCHOR: render-sets)
The [`Render`][bevy::Render] schedule is organized using [sets][cb::systemset] ([`RenderSet`][bevy::RenderSet]):
 - `ExtractCommands`: apply [deferred][cb::deferred] buffers from systems that ran in `ExtractSchedule`
 - `Prepare`/`PrepareFlush`: set up data on the GPU (buffers, textures, etc.)
 - `Queue`/`QueueFlush`: generate the render jobs to be run (usually [phase items][cb::render::phaseitem])
 - `PhaseSort`/`PhaseSortFlush`: sort and batch [phase items][cb::render::phaseitem] for efficient rendering
 - `Render`/`RenderFlush`: execute the [render graph][cb::render::graph] to actually trigger the GPU to do work
 - `Cleanup`/`CleanupFlush`: clear any data from the render World that should not persist to the next frame

The `*Flush` variants are just to apply any [deferred][cb::deferred] buffers after every step, if needed.

Render调度器是使用RenderSet组织的:
 - ExtractCommands: 外部命令, 从ExtractSchedule外部调度器中获取buffer
 - Prepare/PrepareFlush: 在GPU上设置数据(buffer/纹理等)
 - Queue/QueueFlush: 生成渲染任务
 - PhaseSort/PhaseSortFlush: 分阶段处理, 为了更有效的渲染做必要的排序和分批次处理
 - Render/RenderFlush: 触发GPU开始工作
 - Cleanup/CleanupFlush: 在渲染世界中,清除下一帧不需要的持久化的数据

`Flush`系列操作是在每步中,延时对buffer进行处理.

[]:#(ANCHOR_END: render-sets)
