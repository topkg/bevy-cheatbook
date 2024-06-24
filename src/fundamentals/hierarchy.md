{{#include ../include/header09.md}}

# Hierarchical (Parent/Child) Entities

Relevant official examples:
[`hierarchy`][example::hierarchy],
[`parenting`][example::parenting].

---

Technically, the [Entities/Components][cb::ec] themselves cannot form a
hierarchy (the [ECS][cb::ecs-intro] is a flat data structure). However,
logical hierarchies are a common pattern in games.

Bevy supports creating such a logical link between entities, to form
a virtual "hierarchy", by simply adding [`Parent`][bevy::Parent] and
[`Children`][bevy::Children] components on the respective entities.

When using [Commands][cb::commands] to spawn entities,
[`Commands`][bevy::Commands] has methods for adding children to entities,
which automatically add the correct components:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:parenting}}
```

Note that this only sets up the [`Parent`][bevy::Parent] and
[`Children`][bevy::Children] components, and nothing else. Notably, it does not
add [transforms][cb::transform] or [visibility][cb::visibility] for you.  If you
need that functionality, you need to add those components yourself, using
something like [`SpatialBundle`][bevy::SpatialBundle].

You can despawn an entire hierarchy with a single [command][cb::commands]:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:despawn-recursive}}
```

技术上讲,ECS的实体/组件没有继承关系,因为这两者都是扁平化的结构.

实体,逻辑上还是有一些继承关系的,bevy就是这么设计的.

```rust
// Parent 仅仅是将Entity封装了一层.
#[derive(Component, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component, MapEntities, PartialEq))]
pub struct Parent(pub(crate) Entity);

// Children 就是一个Entity列表
#[derive(Component, Debug)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component, MapEntities))]
pub struct Children(pub(crate) SmallVec<[Entity; 8]>);
```

上图的例子就通过两种方式来人为创建了一对父子关系.
例子中并没有添加变换/可视等特性,需要我们自己单独添加.
有继承关系的实体,通过`despawn_recursive`可以将当前实体和子实体都销毁.

## Accessing the Parent or Children

To make a system that works with the hierarchy, you typically need two [queries][cb::query]:
 - one with the components you need from the child entities
 - one with the components you need from the parent entities

One of the two queries should include the appropriate component, to obtain the
entity ids to use with the other one:
 - [`Parent`][bevy::Parent] in the child query, if you want to iterate entities
   and look up their parents, or
 - [`Children`][bevy::Children] in the parent query, if you want to iterate entities
   and look up their children

For example, if we want to get the [`Transform`][bevy::Transform]
of cameras ([`Camera`][bevy::Camera]) that have a parent, and the
[`GlobalTransform`][bevy::GlobalTransform] of their parent:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:query-parent}}
```

As another example, say we are making a strategy game, and we have Units
that are children of a Squad. Say we need to make a system that works on
each Squad, and it needs some information about the children:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:query-child}}
```

如果要访问父/子实体,需要两个query(父实体对应的组件列表,子实体对应的组件列表),
随便找到哪个就能找到对应的父子了.

上面的例子展示了一父多子和多个一父多子的例子.

## Transform and Visibility Propagation

If your entities represent "objects in the game world", you probably expect
the children to be affected by the parent.

[Transform][cb::transform] propagation allows children to be positioned
relative to their parent and move with it.

[Visibility][cb::visibility] propagation allows children to be hidden if
you manually hide their parent.

Most [Bundles that come with Bevy][builtins::bundle] provide these behaviors
automatically. Check the docs for the bundles you are using.  Camera bundles,
for example, have transforms, but not visibility.

Otherwise, you can use [`SpatialBundle`][bevy::SpatialBundle] to make sure
your entities have all the necessary components.

如果要实体能显示出来,还需要受父实体来影响,就可以使用变形传播/可视传播.

bevy提供的很多Bundle都自动添加了这些行为.eg:CamearBundle,添加了变换,没有添加可视.
或者简单点,添加空间Bundle SpatialBundle,这样变换和可视都添加了.

## Known Pitfalls

### Despawning Child Entities

If you despawn an entity that has a parent, Bevy does not remove it from the
parent's [`Children`][bevy::Children].

If you then query for that parent entity's children, you will get an invaild
entity, and any attempt to manipulate it will likely lead to this error:

```
thread 'main' panicked at 'Attempting to create an EntityCommands for entity 7v0, which doesn't exist.'
```

The workaround is to manually call `remove_children` alongside the `despawn`:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:despawn-child}}
```

常见失败场景:子实体销毁.

如果子实体销毁了,但父实体没有销毁其关系,这就是问题.
报错和解决方法如上所示.
