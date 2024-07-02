{{#include ../include/header014.md}}

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

# Entities

[See here for more explanation on how storing data in the ECS works.][cb::ecs-intro-data]

Conceptually, an entity represents a set of values for different components.
Each component is a Rust type (`struct` or `enum`) and an entity can be used to
store a value of that type.

Technically, an entity is just a simple integer ID (imagine the "row number" in
a table/spreadsheet) that can be used to find related data values (in different
"columns" of that table).

In Bevy, [`Entity`] is this value. It consists of two integers:
the ID and the "generation" (allowing IDs to be reused, after you despawn old
entities).

You can create ("spawn") new entities and destroy ("despawn") entities using
[`Commands`][cb::commands] or [exclusive `World` access][cb::world].

```rust,no_run,noplayground
{{#include ../code014/src/programming/ec.rs:spawn-despawn}}
```

Many of your entities might need to have the same common components. You can use
[Bundles][cb::bundle] to make it easier to spawn your entities.

实体,就是组件集合,组件的rust类型是struct/enum.

在ECS设计中,实体通常是一个简单的整数ID,通过这个id可以找到实际的实体数据.
bevy也是这么设计的.

```rust
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
#[cfg_attr(feature = "bevy_reflect", reflect_value(Hash, PartialEq))]
#[cfg_attr(
    all(feature = "bevy_reflect", feature = "serialize"),
    reflect_value(Serialize, Deserialize)
)]
#[repr(C, align(8))]
pub struct Entity {
    #[cfg(target_endian = "little")]
    index: u32, // 对外暴露的ID.
    generation: NonZeroU32, // 版本,代数,重用一次自增一次.
    #[cfg(target_endian = "big")]
    index: u32,
}
```

很多实体可能需要相同的组件,bevy提供了多个模板Bundle,用于简化实体的构造.

# Components

Components are the data associated with entities.

To create a new component type, simply define a Rust `struct` or `enum`, and
derive the [`Component`] trait.

```rust,no_run,noplayground
{{#include ../code014/src/programming/ec.rs:component}}
```

Types must be unique – an entity can only have one component per Rust type.

实体不能包含多个同一类型的组件,因为实体是按类型区别组件的,重复会导致panic.

## Newtype Components

Use wrapper (newtype) structs to make unique components out of simpler types:

```rust,no_run,noplayground
{{#include ../code014/src/programming/ec.rs:component-newtype}}
```

新类型组件,就是了防止两个组件的类型一致而添加了一层封装.

## Marker Components

You can use empty structs to help you identify specific entities. These are
known as "marker components". Useful with [query filters][cb::query-filter].

```rust,no_run,noplayground
{{#include ../code014/src/programming/ec.rs:component-marker}}
```

标记组件,空结构体,这个用处太多了.eg:友军和敌军大部分组件都是类似的,
利用标记组件来做区分.

## Accessing Components

Components can be accessed from [systems][cb::system], using [queries][cb::query].

You can think of the query as the "specification" for the data you want
to access. It gives you access to specific component values from entities
that match the query's signature.

```rust,no_run,noplayground
{{#include ../code014/src/programming/ec.rs:query}}
```

访问组件一般都是通过query来匹配实体,遍历实体列表,再访问组件.

## Adding/removing Components

You can add/remove components on existing entities, using [`Commands`][cb::commands] or
[exclusive `World` access][cb::world].

```rust,no_run,noplayground
{{#include ../code014/src/programming/ec.rs:insert-remove}}
```

增删实体的组件有两种方式:
 - Commands
 - 独占system访问world
