{{#include ../include/header09.md}}

# 3D Models and Scenes (GLTF)

Relevant official examples:
[`load_gltf`][example::load_gltf],
[`update_gltf_scene`][example::update_gltf_scene].

---

Bevy uses the GLTF 2.0 file format for 3D assets.

(other formats may be unofficially available via 3rd-party plugins)

bevy使用gltf2.0文件格式来表示3d资产.

## Quick-Start: Spawning 3D Models into your World

The simplest use case is to just load a "3D model" and spawn it into the game world.

"3D models" can often be complex, consisting of multiple parts. Think of a
house: the windows, roof, doors, etc., are separate pieces, that are likely
made of multiple meshes, materials, and textures. Bevy would technically
need multiple ECS Entities to represent and render the whole thing.

This is why your GLTF "model" is represented by Bevy as a
[Scene][cb::scene]. This way, you can easily spawn it, and Bevy will create
all the relevant [child entities][cb::hierarchy] and configure them correctly.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:spawn-gltf-simple}}
```

You could also use GLTF files to load an entire map/level. It works the same way.

The above example assumes that you have a simple GLTF file containing only
one "default scene". GLTF is a very flexible file format. A single file can
contain many "models" or more complex "scenes". To get a better understanding
of GLTF and possible workflows, read the rest of this page. :)

3d游戏有个常见的操作就是加载3d模型.
一个3d模型可以包含很多东西,一个桌子有桌面和桌柱,
bevy表示桌子是需要多个ECS实体才能渲染一个桌子.

通常一个3d模型就称为一个scene,bevy可以基于scene构造出所有依赖的实体.
可以通过一个gltf文件加载一系列实体.

## Introduction to GLTF

GLTF is a modern open standard for exchanging 3D assets between different
3D software applications, like game engines and 3D modeling software.

The GLTF file format has two variants: human-readable ascii/text (`*.gltf`)
and binary (`*.glb`). The binary format is more compact and preferable
for packaging the assets with your game. The text format may be useful for
development, as it can be easier to manually inspect using a text editor.

A GLTF file can contain many objects (sub-assets): meshes, materials,
textures, scenes, animation clips. When loading a GLTF file, Bevy will load
all of the assets contained inside. They will be mapped to the [appropriate
Bevy-internal asset types][builtins::asset].

GLTF是现代的一个交换3d资产的标准,让不同软件的3d资产可以进行交互,
eg:3d建模软件blender可以输出3d资产,游戏引擎可以加载这些内容.

gltf有两种格式:人类阅读友好的(*.gltf),和二进制(*.glb).

一个gltf文件可以有很多子资产:网格/材质/纹理/scene/动画.

## The GLTF sub-assets

GLTF terminology can be confusing, as it sometimes uses the same words to
refer to different things, compared to Bevy. This section will try explain
the various GLTF terms.

To understand everything, it helps to mentally consider how these concepts are
represented in different places: in your 3D modeling software (like Blender),
in the GLTF file itself, and in Bevy.

GLTF **Scenes** are what you spawn into your game world. This is typically
what you see on the screen in your 3D modeling software. Scenes combine
all of the data needed for the game engine to create all the needed
entities to represent what you want. Conceptually, think of a scene as one
"unit". Depending on your use case, this could be one "3d model",
or even a whole map or game level. In Bevy, these are represented as Bevy
Scenes with all the child ECS entities.

GLTF Scenes are composed of GLTF **Nodes**. These describe the "objects"
in the scene, typically GLTF Meshes, but can also be other things like
Cameras and Lights. Each GLTF Node has a transform for positioning it in
the scene. GLTF Nodes do not have a core Bevy equivalent; Bevy just uses
this data to create the ECS Entities inside of a Scene. Bevy has a special
[`GltfNode`][bevy::GltfNode] asset type, if you need access to this data.

GLTF **Meshes** represent one conceptual "3D object". These correspond
to the "objects" in your 3D modeling software. GLTF Meshes may be complex
and composed of multiple smaller pieces, called GLTF Primitives, each of
which may use a different Material. GLTF Meshes do not have a core Bevy
equivalent, but there is a special [`GltfMesh`][bevy::GltfMesh] asset type,
which describes the primitives.

GLTF **Primitives** are individual "units of 3D geometry", for the purposes of
rendering. They contain the actual geometry / vertex data, and reference the
Material to be used when drawing. In Bevy, each GLTF Primitive is represented
as a Bevy [`Mesh`][bevy::Mesh] asset, and must be spawned as a separate ECS
Entity to be rendered.

GLTF **Materials** describe the shading parameters for the surfaces of
your 3D models. They have full support for Physically-Based Rendering
(PBR). They also reference the textures to use. In Bevy, they are represented
as [`StandardMaterial`][bevy::StandardMaterial] assets, as used by the Bevy
PBR 3D renderer.

GLTF **Textures** (images) can be embedded inside the GLTF file, or stored
externally in separate image files alongside it. For example, you can have
your textures as separate PNG/JPEG/KTX2 files for ease of development, or
package them all inside the GLTF file for ease of distribution. In Bevy,
GLTF textures are loaded as Bevy [`Image`][bevy::Image] assets.

GLTF **Samplers** describe the settings for how the GPU should use a
given Texture. Bevy does not keep these separate; this data is stored
inside the Bevy [`Image`][bevy::Image] asset (the `sampler` field of type
[`SamplerDescriptor`][bevy::SamplerDescriptor]).

GLTF **Animations** describe animations that interpolate various values,
such as transforms or mesh skeletons, over time. In Bevy, these are loaded
as [`AnimationClip`][bevy::AnimationClip] assets.

gltf资产是有标准名称的,这个是统一的.但bevy并没有完全按gltf标准来.

GLTF场景是您在游戏世界中生成的内容.
这通常是您在3D建模软件的屏幕上看到的内容.
场景结合了游戏引擎所需的所有数据,以创建所有必要的实体来表示您想要的内容.
从概念上讲,将场景视为一个“单元”.
根据您的用例,这可能是一个“3D模型”,甚至是整个地图或游戏级别.
在Bevy中,这些表示为包含所有子ECS实体的Bevy场景.

GLTF场景由GLTF节点组成.这些节点描述了场景中的“对象”,通常是GLTF网格,
但也可以是其他东西,如相机和灯光.每个GLTF节点都有一个变换来将其定位在场景中.
GLTF节点没有核心Bevy等效项；Bevy仅使用此数据在场景内创建ECS实体.
如果您需要访问此数据,Bevy有一个特殊的GltfNode资源类型.

GLTF网格代表一个概念上的“3D对象”.它们对应于3D建模软件中的“对象”.
GLTF网格可能很复杂,由多个较小的部分组成,称为GLTF基元,
每个基元可能使用不同的材质.GLTF网格没有核心Bevy等效项,
但有一个特殊的GltfMesh资源类型,用于描述基元.

GLTF基元是单独的“3D几何单元”,用于渲染.
它们包含实际的几何/顶点数据,并引用绘制时要使用的材质.
在Bevy中,每个GLTFPrimitive都表示为BevyMesh资源,
并且必须生成为单独的ECS实体才能进行渲染.

GLTF材质描述3D模型表面的着色参数.它们完全支持基于物理的渲染(PBR).
它们还引用要使用的纹理.在Bevy中,它们表示为StandardMaterial资源,
由BevyPBR3D渲染器使用.

GLTF纹理(图像)可以嵌入GLTF文件中,也可以存储在外部单独的图像文件中.
例如,您可以将纹理作为单独的PNG/JPEG/KTX2文件以方便开发,
或者将它们全部打包到GLTF文件中以方便分发.
在Bevy中,GLTF纹理作为Bevy图像资源加载.

GLTF采样器描述了GPU应如何使用给定纹理的设置.Bevy不会将这些分开保存；
这些数据存储在Bevy图像资产(SamplerDescriptor类型的采样器字段)中.

GLTF动画描述了随时间插入各种值(例如变换或网格骨架)的动画.
在Bevy中,这些被加载为AnimationClip资产.

## GLTF Usage Patterns

A single GLTF file can contain any number of sub-assets of any of the above
types, referring to each other however they like.

Because GLTF is so flexible, it is up to you how to structure your assets.

A single GLTF file might be used:
  - To represent a single "3D model", containing a single
    GLTF Scene with the model, so you can spawn it into your game.
  - To represent a whole level, as a GLTF Scene, possibly also including
    the camera. This lets you load and spawn a whole level/map at once.
  - To represent sections of a level/map, such as a rooms, as separate GLTF Scenes.
    They can share meshes and textures if needed.
  - To contain a set of many different "3D models", each as a separate GLTF Scene.
    This lets you load and manage the whole collection at once and spawn them individually as needed.
  - … others?

单个 GLTF 文件可以包含任意数量的上述任何类型的子资源, 并以任意方式相互引用.

gltf可以用在以下场景:
 - 表示单个3D 模型
 - 将整个关卡表示为 GLTF 场景,可能还包括相机.这让您可以一次加载和生成整个关卡/地图.
 - 将关卡/地图的各个部分(例如房间)表示为单独的 GLTF 场景.如果需要,它们可以共享网格和纹理.
 - 包含一组许多不同的3D 模型,每个模型都是一个单独的 GLTF 场景.这让您可以一次加载和管理整个集合,并根据需要单独生成它们.
 - …其他？

## Tools for Creating GLTF Assets

If you are using a recent version of Blender (2.8+) for 3D modeling, GLTF
is supported out of the box. Just export and choose GLTF as the format.

For other tools, you can try these exporter plugins:
  - [Old Blender (2.79)][gltf-export-blender-old]
  - [3DSMax][gltf-export-3dsmax]
  - [Autodesk Maya][gltf-export-maya]
    - (or this [alternative][gltf-export-maya2])

Be sure to check your export settings to make sure the GLTF file contains
everything you expect.

If you need Tangents for normal maps, it is recommended that you include them
in your GLTF files. This avoids Bevy having to autogenerate them at runtime.
Many 3D editors do not enable this option by default.

创建gltf资产的工具,推荐使用开源的blender.

如果您需要法线贴图的切线,建议您将它们包含在 GLTF 文件中.
这避免了 Bevy 必须在运行时自动生成它们.许多 3D 编辑器默认不启用此选项.

### Textures

For your Textures / image data, the GLTF format specification officially
limits the supported formats to just PNG, JPEG, or Basis. However, Bevy does
not enforce such "artificial limitations". You can use any [image format
supported by Bevy][builtins::file-formats].

Your 3D editor will likely export your GLTF with PNG textures. This will
"just work" and is nice for simple use cases.

However, mipmaps and compressed textures are very important to get good GPU
performance, memory (VRAM) usage, and visual quality. You will only get these
benefits if you use a format like KTX2 or DDS, that supports these features.

We recommend that you use KTX2, which natively supports all GPU texture
functionality + additional `zstd` compression on top, to reduce file size.
If you do this, don't forget to enable the `ktx2` and `zstd` [cargo
features][cb::features] for Bevy.

You can use the [`klafsa`][project::klafsa] tool to convert all the textures
used in your GLTF files from PNG/JPEG into KTX2, with mipmaps and GPU texture
compression of your choice.

```
TODO: show an example workflow for converting textures into the "optimal" format
```

对于您的纹理/图像数据,GLTF格式规范正式将支持的格式限制为PNG、JPEG或Basis.
但是,Bevy不会强制执行这种“人为限制”.您可以使用Bevy支持的任何图像格式.

您的3D编辑器可能会使用PNG纹理导出您的GLTF.这将“正常工作”,并且非常适合简单的用例.

但是,mipmap和压缩纹理对于获得良好的GPU性能、内存(VRAM)使用率和视觉质量非常重要.
只有使用支持这些功能的KTX2或DDS等格式,您才能获得这些好处.

我们建议您使用KTX2,它原生支持所有GPU纹理功能+顶部的额外zstd压缩,以减小文件大小.
如果您这样做,请不要忘记为Bevy启用ktx2和zstd货物功能.

您可以使用klafsa工具将GLTF文件中使用的所有纹理从PNG/JPEG转换为KTX2,
并选择mipmap和GPU纹理压缩.

## Using GLTF Sub-Assets in Bevy

The various sub-assets contained in a GLTF file can be addressed in two ways:
  - by index (integer id, in the order they appear in the file)
  - by name (text string, the names you set in your 3D modeling software
    when creating the asset, which can be exported into the GLTF)

To get handles to the respective assets in Bevy, you can use the
[`Gltf`][bevy::Gltf] ["master asset"](#gltf-master-asset), or alternatively,
[AssetPath with Labels](#assetpath-with-labels).

### `Gltf` master asset

If you have a complex GLTF file, this is likely the most flexible and useful
way of navigating its contents and using the different things inside.

You have to wait for the GLTF file to load, and then use the [`Gltf`][bevy::Gltf] asset.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:gltf-complex}}
```

For a more convoluted example, say we want to directly create a 3D PBR
entity, for whatever reason. (This is not recommended; you should probably
just use scenes)

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:gltf-manual-pbr}}
```

### AssetPath with Labels

This is another way to access specific sub-assets. It is less reliable,
but may be easier to use in some cases.

Use the [`AssetServer`][bevy::AssetServer] to convert a path string into a
[`Handle`][bevy::Handle].

The advantage is that you can get handles to your sub-assets immediately,
even if your GLTF file hasn't loaded yet.

The disadvantage is that it is more error-prone. If you specify a sub-asset
that doesn't actually exist in the file, or mis-type the label, or use the
wrong label, it will just silently not work. Also, currently only using a
numerial index is supported. You cannot address sub-assets by name.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:gltf-assetpath}}
```

{{#include ../include/builtins.md:gltf-asset-labels}}

The [`GltfNode`][bevy::GltfNode] and [`GltfMesh`][bevy::GltfMesh]
asset types are only useful to help you navigate the contents of
your GLTF file. They are not core Bevy renderer types, and not used
by Bevy in any other way. The Bevy renderer expects Entities with
[`MaterialMeshBundle`][bevy::MaterialMeshBundle]; for that you need the
[`Mesh`][bevy::Mesh] and [`StandardMaterial`][bevy::StandardMaterial].

## Bevy Limitations

{{#include ../include/gltf-limitations.md}}
