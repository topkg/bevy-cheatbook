{{#include ../include/header011.md}}

# 3D objects not displaying

This page will list some common issues that you may encounter, if you are
trying to spawn a 3D object, but cannot see it on the screen.

3d对象没有在屏幕上渲染出来,可能是以下错误.

## Missing visibility components on parent

If your entity is in a hierarchy, all its parents need to have
[visibility][cb::visibility] components. It is required even if those parent
entities are not supposed to render anything.

Fix it by inserting a [`VisibilityBundle`][bevy::VisibilityBundle]:

```rust
{{#include ../code011/src/pitfalls/d3_not_rendering.rs:insert-visibilitybundle}}
```

Or better, make sure to spawn the parent entities correctly in the first place.
You can use a [`VisibilityBundle`][bevy::VisibilityBundle] or
[`SpatialBundle`][bevy::SpatialBundle] (with [transforms][cb::transform]) if you
are not using a bundle that already includes these components.

如果实体是继承的,需要检查父辈的`可见性`,即使父辈实体不需要任何渲染,也需要`可见性`组件.
解决方法是1:给父辈实体添加`可见性`组件;2:尽量在父辈实体构造时设置可见性.

可见性的组件有两种内置的:SpatialBundle(空间Bundle)和VisibilityBundle(可见性Bundle).

## Too far from camera

If something is further away than a certain distance from the camera, it will be
culled (not rendered). The default value is `1000.0` units.

You can control this using the `far` field of
[`PerspectiveProjection`][bevy::PerspectiveProjection]:

```rust
{{#include ../code011/src/pitfalls/d3_not_rendering.rs:perspective-far}}
```

如果物体离相机有1000个单位,在渲染时会被过滤掉的,所以要渲染就不要里相机太远.
当然也可以将1000单位改为更大的数值.

## Missing Vertex Attributes

Make sure your [`Mesh`][bevy::Mesh] includes all vertex attributes required
by your shader/material.

Bevy's default PBR [`StandardMaterial`][bevy::StandardMaterial]
requires *all* meshes to have:
 - Positions
 - Normals

Some others that may be required:
 - UVs (if using textures in the material)
 - Tangents (only if using normal maps, otherwise not required)

If you are generating your own mesh data, make sure to provide everything
you need.

If you are loading meshes from asset files, make sure they include everything
that is needed (check your export settings).

If you need Tangents for normal maps, it is recommended that you include them
in your GLTF files. This avoids Bevy having to autogenerate them at runtime.
Many 3D editors (like Blender) do not enable this option by default.

确保着色器/材质包含的Mesh拥有所需要的顶点信息.

bevy的默认pbr(基于物理的渲染)是标准材质StandardMaterial,包含位置和法线.
其他材质可能需要UV(材质使用纹理时才需要)和切线(法线贴图时才需要).

如果自己生成mesh时,确保提供所需信息.如果从资产文件中导入mesh时,
确保其提供了所需信息.

尽量避免让bevy在运行时自动生成这些信息,不然性能就拉低了.
很多3d模型编辑器(eg:blender)是默认不启用这些选项的.

## Incorrect usage of Bevy GLTF assets

Refer to the [GLTF page][cb::gltf] to learn how to correctly
use GLTF with Bevy.

GLTF files are complex. They contain many sub-assets, represented by
different Bevy types. Make sure you are using the correct thing.

Make sure you are spawning a GLTF Scene, or using the correct
[`Mesh`][bevy::Mesh] and [`StandardMaterial`][bevy::StandardMaterial]
associated with the correct GLTF Primitive.

If you are using an asset path, be sure to include a label for the sub-asset you want:

```rust,no_run,noplayground
{{#include ../code011/src/pitfalls/d3_not_rendering.rs:gltf-ass-label}}
```

If you are spawning the top-level [`Gltf`][bevy::Gltf] [master asset][cb::gltf-master], it won't work.

If you are spawning a GLTF Mesh, it won't work.

gltf 3d模型文件非常复杂,包含各种bevy需要的资产,gltf场景需要使用正确的原语来生成.
资产生成好了,在bevy中引用时,路径需要注意.

不能生成顶级gltf主资产,不能生成gltf mesh,这些都不能正常工作.

## Unsupported GLTF

{{#include ../include/gltf-limitations.md}}

## Vertex Order and Culling

By default, the Bevy renderer assumes Counter-Clockwise vertex order and has
back-face culling enabled.

If you are generating your [`Mesh`][bevy::Mesh] from code, make sure your
vertices are in the correct order.

默认情况下,bevy渲染假设逆时针顶点顺序,背面剔除.
如果是在代码中生成mesh,需要确保顶点的顺序是正确的.

## Unoptimized / Debug builds

Maybe your asset just takes a while to load? Bevy is very slow without
compiler optimizations. It's actually possible that complex GLTF files with
big textures can take over a minute to load and show up on the screen. It
would be almost instant in optimized builds. [See here][pitfall::perf].

未优化的代码,可能需要几分钟来加载资源.
