{{#include ../include/header013.md}}

# Visibility

Relevant official examples:
[`parenting`][example::parenting].

---

Visibility is used to control if something is to be rendered or not. If you
want an entity to exist in the world, just not be displayed, you can hide it.

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/visibility.rs:visibility}}
```

可视化控制一个对象是否进行渲染,不进行可视化,等同于隐藏.

## Visibility Components

In Bevy, visibility is represented by **multiple** [components][cb::component]:
 - [`Visibility`]: the user-facing toggle (here is where you set what you want)
 - [`InheritedVisibility`]: used by Bevy to keep track of the state from any [parent entities][cb::hierarchy]
 - [`ViewVisibility`]: used by Bevy to track if the entity should actually be displayed

Any [Entity][cb::ecs-intro] that represents a renderable object in
the game world needs to have them all. All of Bevy's [built-in bundle
types][builtins::bundle] include them.

If you are creating a custom entity without using those [bundles][cb::bundle],
you can use one of the following to ensure you don't miss them:
 - [`SpatialBundle`] for [transforms][cb::transform] + visibility
 - [`VisibilityBundle`] for just visibility

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/visibility.rs:spatialbundle}}
```

If you don't do this correctly (say, you manually add just the `Visibility`
component and forget the others, because you don't use a bundle), your entities
will not render!

可视化由多个可视化组件表示:
 - `Visibility` 面对用户的开关
 - `InheritedVisibility` 由bevy跟踪的实体状态,从继承体系上得到
 - `ViewVisibility` 由bevy跟踪,看实体实际上是否应该显示

### `Visibility`

[`Visibility`] is the "user-facing toggle". This is where you specify what you
want for the current entity:
 - `Inherited` (default): show/hide depending on [parent][cb::hierarchy]
 - `Visible`: always show the entity, regardless of parent
 - `Hidden`: always hide the entity, regardless of parent

If the current entity has any [children][cb::hierarchy] that have `Inherited`,
their visibility will be affected if you set the current entity to `Visible`
or `Hidden`.

If an entity has a parent, but the parent entity is missing the
visibility-related components, things will behave as if there was no parent.

对于实体,可设置3个值:
 - `Inherited` 默认, 是否显示跟着父对象走
 - `Visible` 不受父对象影响,显示
 - `Hidden` 不受父对象影响,隐藏

当前实体有子实体,那么变更设置会影响子实体的显示效果.
如果当前实体有父实体,但父实体没有`可视组件`,那么当前实体的行为就和没有父实体类似.

### `InheritedVisibility`

[`InheritedVisibility`] represents the state the current entity would have based
on its [parent][cb::hierarchy]'s visibility.

The value of [`InheritedVisibility`] should be considered read-only. It is
managed internally by Bevy, in a manner similar to [transform
propagation][cb::transform-propagate]. A "visibility propagation"
[system][cb::system] runs in the [`PostUpdate`] [schedule][cb::schedule].

If you want to read the up-to-date value for the current frame, you should
[add][cb::app] your [system][cb::system] to the [`PostUpdate`]
[schedule][cb::schedule] and [order it after][cb::system-order]
[`VisibilitySystems::VisibilityPropagate`][`VisibilitySystems`].

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/visibility.rs:inheritedvisibility}}
```
```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/visibility.rs:inheritedvisibility-app}}
```

继承可视化,可视状态跟着父实体走.InheritedVisibility的值是只读,由bevy维护.
和变换传播类似,可视传播的system也是在PostUpdate调度中.

如果要获取当前帧的可视状态,需要添加system在PostUpdate调度中,
且放在可视传播(VisibilitySystems::VisibilityPropagate)的后面.

### `ViewVisibility`

[`ViewVisibility`] represents the actual final decision made by Bevy about
whether this entity needs to be rendered.

The value of [`ViewVisibility`] is read-only. It is managed internally by Bevy.

It is used for "culling": if the entity is not in the range of
any Camera or Light, it does not need to be rendered, so Bevy will hide it
to improve performance.

Every frame, after "visibility propagation", Bevy will check what entities
can be seen by what view (camera or light), and store the outcome in these
components.

If you want to read the up-to-date value for the current frame, you should
[add][cb::app] your [system][cb::system] to the [`PostUpdate`]
[schedule][cb::schedule] and [order it after][cb::system-order]
[`VisibilitySystems::CheckVisibility`][`VisibilitySystems`].

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/visibility.rs:viewvisibility}}
```
```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/visibility.rs:viewvisibility-app}}
```

最终可视化,最终能不能显示就看这个值,只读,由bevy内部维护.

有个剔除规则: 如果当前对象不在Camera的范围内,或不在灯光范围内,
那这个对象就不用渲染.这样可以提高性能,也符合逻辑.

每帧,在可视传播执行完后,bevy会检查Camera/灯光和实体的关系,
然后将结果存储在ViewVisibility组件的值中.

如果想要获取最终可视的值,在PostUpdate中添加system,
在VisibilitySystems::CheckVisibility之后获取值即可.
