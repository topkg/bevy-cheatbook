{{#include ../include/header013.md}}

# Transforms

Relevant official examples:
[`transform`][example::transform],
[`translation`][example::translation],
[`rotation`][example::rotation],
[`3d_rotation`][example::3d_rotation],
[`scale`][example::scale],
[`move_sprite`][example::move_sprite],
[`parenting`][example::parenting],
anything that spawns 2D or 3D objects.

---

First, a quick definition, if you are new to game development:

A Transform is what allows you to place an object in the game world. It
is a combination of the object's "translation" (position/coordinates),
"rotation", and "scale" (size adjustment).

You move objects around by modifying the translation, rotate them by modifying
the rotation, and make them larger or smaller by modifying the scale.

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/transforms.rs:transform-init}}
```

变换的定义是:在游戏世界中,将一个对象如何放置.
她由对象的位置/坐标/旋转/缩放的一个组合.

在英文中,transform是变换,意义更广,涵盖了对物体位置、旋转和缩放等属性的综合操作。
translation也是变换,但更多的是表示平移(仅仅是位置移动),不涉及旋转和缩放.

## Transform Components

In Bevy, transforms are represented by **two** [components][cb::component]:
[`Transform`] and [`GlobalTransform`].

Any [Entity][cb::ecs-intro] that represents an object in the game world
needs to have both. All of Bevy's [built-in bundle types][builtins::bundle]
include them.

If you are creating a custom entity without using those [bundles][cb::bundle],
you can use one of the following to ensure you don't miss them:
 - [`SpatialBundle`] for transforms + [visibility][cb::visibility]
 - [`TransformBundle`] for just the transforms

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/transforms.rs:spatialbundle}}
```

在bevy中,变换有两个基础组件`Transform`和`GlobalTransform`,
两者组合还有空间Bunlde,变换Bundle.

world中的任何实体都需要这两个基础组件,实体构造时不能缺.

### `Transform`

[`Transform`] is what you typically work with. It is a `struct` containing the
translation, rotation, and scale. To read or manipulate these values, access it
from your [systems][cb::system] using a [query][cb::query].

If the entity has a [parent][cb::hierarchy], the [`Transform`] component is
relative to the parent. This means that the child object will move/rotate/scale
along with the parent.

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/transforms.rs:transform-mut}}
```

通常使用Transform就够了,这个结构体里包含了移动/旋转/缩放.
如果实体有继承关系,则Transform是基于父实体的.

### `GlobalTransform`

[`GlobalTransform`] represents the absolute global position in the world.

If the entity does not have a [parent][cb::hierarchy], then this will match the
[`Transform`].

The value of [`GlobalTransform`] is calculated/managed internally by Bevy
(["transform propagation"](#transform-propagation)).

Unlike [`Transform`], the translation/rotation/scale are not accessible
directly. The data is stored in an optimized way (using [`Affine3A`]) and it is
possible to have complex transformations in a hierarchy that cannot be
represented as a simple transform. For example, a combination of rotation and
scale across multiple parents, resulting in shearing.

If you want to try to convert a [`GlobalTransform`] back into a workable
translation/rotation/scale representation, you can try the methods:
 - `.translation()`
 - `.to_scale_rotation_translation()` (may be invalid)
 - `.compute_transform()` (may be invalid)

GlobalTransform 是针对world世界的绝对位置. 值的计算和管理直接由bevy内部管理,
这也是正常的,毕竟bevy是跨多个平台,自身就是要维护不同平台的不一致,
基于world的正好交给bevy管理.而Transform则是为了简化开发而独立出来的.

GlobalTransform的移动/旋转/缩放不是简单的改值,处理起来也很麻烦,
多个顶级父对象之前还存在遮挡,计算也颇为麻烦,优化也是采用`Affine3A`技术.

## Transform Propagation

The two components are synchronized by a bevy-internal system (the "transform
propagation system"), which runs in the [`PostUpdate`] [schedule][cb::schedule].

Beware: When you mutate the [`Transform`], the [`GlobalTransform`] is not
updated immediately. They will be out-of-sync until the transform propagation
system runs.

If you need to work with [`GlobalTransform`] directly, you should [add][cb::app]
your [system][cb::system] to the [`PostUpdate`] [schedule][cb::schedule] and
[order it after][cb::system-order] [`TransformSystem::TransformPropagate`][`TransformSystem`].

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/transforms.rs:globaltransform}}
```
```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/transforms.rs:globaltransform-app}}
```

变换传播,这两个基础组件在bevy内部system中调用,时机是PostUpdate调度中.
通过Transform修改值后,GlobalTransform并不会立马更新,而是等变换传播执行完才进行同步.

如果要直接操作GlobalTransform,需要在PostUpdate调度中添加system,
并将system的顺序放在变换传播(TransformSystem::TransformPropagate)后面.

## `TransformHelper`

If you need to get an up-to-date [`GlobalTransform`] in a [system][cb::system]
that has to run before transform propagation, you can use the special
[`TransformHelper`] system parameter.

It allows you to compute a specific entity's [`GlobalTransform`] immediately, on
demand.

An example of where this could be useful might be a system to make a camera
follow an entity on-screen. You need to update the camera's [`Transform`] (which
means you have to do it before Bevy's transform propagation, so it can account
for the camera's new transform), but you also need to know the current
up-to-date position of the entity you are following.

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/transforms.rs:transformhelper}}
```

Internally, [`TransformHelper`] behaves like two read-only [queries][cb::query].
It needs access to the [`Parent`] and [`Transform`] components to do its job. It
would conflict with our other `&mut Transform` query. That's why we have to use
a [param set][cb::paramset] in the example above.

Note: if you over-use [`TransformHelper`], it could become a performance issue.
It calculates the global transform for you, but it does not update the data
stored in the entity's [`GlobalTransform`]. Bevy will still do the same
computation again later, during transform propagation. It leads to repetitive
work. If your system can run after transform propagation, so it can just read
the value after Bevy updates it, you should prefer to do that instead of using
[`TransformHelper`].

如果要在变换传播之前获取最新的GlobalTransform值,需要借助TransformHelper system参数.

一个常用的场景是:相机要跟随屏幕上的实体,就需要更新相机的Transform,
而且这个操作要在变换传播之前处理完.

代码如上图所示,使用了system参数: ParamSet/TransformHelper.

TransformHelper的行为类似两个只读query,分别访问Parent和Transform组件.
因为已经有了一个camera的Transform,直接使用两个query会导致冲突,
所以使用ParamSet来避免冲突.

TransformHelper过度使用会有性能问题,因为只计算,不更新GlobalTransform的数据,
在变换传播时,bevy也是要进行同样的计算.这样就存在重复计算.
如果是在传播变换之后获取GlobalTransform,直接读就行,不需要TransformHelper,
这样性能会更高.
