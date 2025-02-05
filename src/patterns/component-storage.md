{{#include ../include/header09.md}}

# Component Storage (Table/Sparse-Set)

Bevy ECS provides two different ways of storing data: tables and sparse sets.
The two storage kinds offer different performance characteristics.

The kind of storage to be used can be chosen per [component][cb::component]
type.  When you derive the [`Component`][bevy::Component] trait, you can
specify it. The default, if unspecified, is table storage. You can have
components with a mixture of different storage kinds on the same entity.

The rest of this page is dedicated to explaining the performance trade-offs
and why you might want to choose one storage kind vs. the other.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:component-storage}}
```

bevy的ecs支持两种不同的存储:表格和稀疏集.
每种组件类型都可以选择自己的存储类型.默认是表格.
同一个实体是可以同时使用不同存储的.

## Table Storage

Table storage is optimized for fast [query][cb::query] iteration. If the
way you usually use a specific component type is to iterate over its data
across many entities, this will offer the best performance.

However, adding/removing table components to existing entities is a relatively
slow operation. It requires copying the data of all table components for
the entity to a different location in memory.

It's OK if you have to do this sometimes, but if you are likely to add/remove
a component very frequently, you might want to switch that component type
to sparse-set storage.

You can see why table storage was chosen as Bevy's default. Most component
types are rarely added/removed in practice. You typically spawn entities with
all the components they should have, and then access the data via queries,
usually every frame. Sometimes you might add or remove a component to change
an entity's behavior, but probably not nearly as often, or every frame.

表格存储的特点是迭代查询非常快.这种模式下增删很慢,因为涉及到所有表格数据的移动.
频繁增删使用表格存储就是灾难.

默认就是表格存储,因为大多数场景下,动态增删是一个低频操作.

## Sparse-Set Storage

Sparse-Set storage is optimized for fast adding/removing of a component to
existing entities, at the cost of slower querying. It can be more efficient
for components that you would like to add/remove very frequently.

An example of this might be a [marker component][cb::component-marker]
indicating whether an enemy is currently aware of the player. You might
want to have such a component type, so that you can easily use a [query
filter][cb::query-filter] to find all the enemies that are currently
tracking the player. However, this is something that can change every frame,
as enemies or the player move around the game level. If you add/remove this
component every time the visibility status changed, that's a lot of additions
and removals.

You can see that situations like these are more niche and do not apply
to most typical component types. Treat sparse-set storage as a potential
optimization you could try in specific circumstances.

Even in situations like the example above, it might not be a performance win.
Everything depends on your application's unique usage patterns. You have to
measure and try.

稀疏集存储就是增删非常快,query慢.

如果组件就是临时性的,频繁增删,就使用这个.

使用存储来提高性能,还不如优化游戏逻辑来得快.

## Table Fragmentation

Furthermore, the actual memory layout of the "tables" depends on the set of
all table components that each of your entities has.

ECS queries perform best when many of the entities they match have the same
overall set of components.

Having a large number of entities, that all have the same component types, is
very efficient in terms of data access performance. Having diverse entities
with a varied mixture of different component types, means that their data
will be fragmented in memory and be less efficient to access.

Sparse-Set components do not affect the memory layout of tables. Hence,
components that are only used on a few entities or as a "temporary effect",
might also be good candidates for sparse-set storage. That way they don't
fragment the memory of the other (table) components. Systems that do not
care about these components will be completely unaffected by them existing.

如果大量实体都有某个组件,按这个组件查询,效率会非常高.

大量不连续的实体,性能就只能退化为普通的了.

## Overall Advice

While this page describes the general performance characteristics and gives
some guidelines, you often cannot know if something improves performance
without benchmarking.

When your game grows complex enough and you have something to benchmark,
you could try to apply sparse-set storage to situations where it might make
sense, as described above, and see how it affects your results.
