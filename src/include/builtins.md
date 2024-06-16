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

Audio formats (loaded as [`AudioSource`][bevy::AudioSource] assets):

|Format    |Cargo feature|Default?|Filename extensions    |
|----------|-------------|--------|-----------------------|
|OGG Vorbis|`"vorbis"`   |Yes     |`.ogg`, `.oga`, `.spx` |
|FLAC      |`"flac"`     |No      |`.flac`                |
|WAV       |`"wav"`      |No      |`.wav`                 |
|MP3       |`"mp3"`      |No      |`.mp3`                 |

3D asset (model or scene) formats:

|Format|Cargo feature|Default?|Filename extensions|
|------|-------------|--------|-------------------|
|GLTF  |`"bevy_gltf"`|Yes     |`.gltf`, `.glb`    |

Shader formats (loaded as [`Shader`][bevy::Shader] assets):

|Format|Cargo feature          |Default?|Filename extensions      |
|------|-----------------------|--------|-------------------------|
|WGSL  |n/a                    |Yes     |`.wgsl`                  |
|GLSL  |`"shader_format_glsl"` |No      |`.vert`, `.frag`, `.comp`|
|SPIR-V|`"shader_format_spirv"`|No      |`.spv`                   |

Font formats (loaded as [`Font`][bevy::Font] assets):

|Format  |Cargo feature|Default?|Filename extensions|
|--------|-------------|--------|-------------------|
|TrueType|n/a          |Yes     |`.ttf`             |
|OpenType|n/a          |Yes     |`.otf`             |

Bevy Scenes:

|Format              |Filename extensions|
|--------------------|-------------------|
|RON-serialized scene|`.scn`,`.scn.ron`  |

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

Scenes:
 - [`SceneBundle`][bevy::SceneBundle]:
   Used for spawning scenes
 - [`DynamicSceneBundle`][bevy::DynamicSceneBundle]:
   Used for spawning dynamic scenes

Audio:
 - [`AudioBundle`][bevy::AudioBundle]:
   Play [audio][cb::audio] from an [`AudioSource`][bevy::AudioSource] asset
 - [`SpatialAudioBundle`][bevy::SpatialAudioBundle]:
   Play [positional audio][cb::audio-spatial] from an [`AudioSource`][bevy::AudioSource] asset
 - [`AudioSourceBundle`][bevy::AudioSourceBundle]:
   Play audio from a [custom data source/stream][cb::audio-custom]
 - [`SpatialAudioSourceBundle`][bevy::SpatialAudioSourceBundle]:
   Play positional audio from a [custom data source/stream][cb::audio-custom]

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
[]:#(ANCHOR_END: events-system)

[]:#(ANCHOR: events-engine)
 - [`AssetEvent<T>`][bevy::AssetEvent]:
   Sent by Bevy when [asset data][cb::asset] has been added/modified/removed; [can be used to detect changes to assets][cb::assetevent]
 - [`HierarchyEvent`][bevy::HierarchyEvent]:
   Sent by Bevy when entity [parents/children][cb::hierarchy] change
 - [`AppExit`][bevy::AppExit]:
   Tell Bevy to shut down
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
