{{#include ../include/header014.md}}

# Intro: Your Data

This page is an overview, to give you an idea of the big picture of how Bevy
works. Click on the various links to be taken to dedicated pages where you can
learn more about each concept.

这里只是简介,详细内容还需要点击链接进一步查看.

---

As mentioned in [the ECS Intro][cb::ecs-intro], Bevy stores all your data for
you and allows you easy and flexible access to whatever you need, wherever you
need it.

The ECS's data structure is called the [`World`]. That is what
stores and manages all of the data. For advanced scenarios, is possible to
have [multiple worlds][cb::multi-world], and then each one will behave as
its own separate ECS instance. However, normally, you just work with the
main World that Bevy sets up for your [App][cb::app].

You can represent your data in two different ways:
[Entities/Components](#entities--components), and [Resources](#resources).

ECS的数据全部存在一个叫world的实例中,world管理和维护了所有的数据,
对于部分高级场景,可能会存在多个world,每个world都是自己独立的ECS实例,
通常app只有一个world,关心主world即可.

数据只能依附于组件或资源出现.资源就是一个中特殊的组件`单例组件`,
一个游戏中50%以上都是`单例组件`,这种特殊的组件在bevy中有个新名词:资源.

## Entities / Components

Conceptually, you can think of it by analogy with tables, like in a database or
spreadsheet. Your different data types ([Components][cb::component]) are like
the "columns" of a table, and there can be arbitrarily many "rows"
([Entities][cb::entity]) containing values / instances of various components.
The [`Entity`] ID is like the row number. It's an integer index
that lets you find specific component values.

Component types that are empty `struct`s (contain no data) are called [marker
components][cb::component-marker]. They are useful as "tags" to identify
specific entities, or enable certain behaviors. For example, you could use them
to identify the player entity, to mark enemies that are currently chasing the
player, to select entities to be despawned at the end of the level, etc.

Here is an illustration to help you visualize the logical structure. The
checkmarks show what component types are present on each entity. Empty cells
mean that the component is not present. In this example, we have a player,
a camera, and several enemies.

|[`Entity`] (ID)|[`Transform`]|`Player`|`Enemy`|[`Camera`]|`Health`|...|
|---|---|---|---|---|---|---|
|...|||||||
|107|✓ `<translation>` `<rotation>` `<scale>`|✓|||✓ `50.0`||
|108|✓ `<translation>` `<rotation>` `<scale>`||✓||✓ `25.0`||
|109|✓ `<translation>` `<rotation>` `<scale>`|||✓ `<camera data>`|||
|110|✓ `<translation>` `<rotation>` `<scale>`||✓||✓ `10.0`||
|111|✓ `<translation>` `<rotation>` `<scale>`||✓||✓ `25.0`||
|...|||||||

Representing things this way gives you flexibility. For example, you could
create a `Health` component for your game. You could then have many entities
representing different things in your game, such as the player, NPCs, or
monsters, all of which can have a `Health` value (as well as other relevant
components).

The typical and obvious pattern is to use entities to represent "objects in the
game/scene", such as the camera, the player, enemies, lights, props, UI
elements, and other things. However, you are not limited to that. The ECS is a
general-purpose data structure. You can create entities and components to store
any data. For example, you could create an entity to store a bunch of settings
or configuration parameters, or other abstract things.

Data stored using Entities and Components is accessed using
[queries][cb::query]. For example, if you want to implement a new game
mechanic, write a [system][cb::system] (just a Rust function that takes
special parameters), specify what component types you want to access, and do
your thing. You can either iterate through all entities that match your query,
or access the data of a specific one (using the [`Entity`] ID).

```rust,no_run,noplayground
{{#include ../code014/src/programming/intro_data.rs:query}}
```

Bevy can automatically keep track of what data your [systems][cb::system] have
access to and [run them in parallel][cb::system-parallel] on multiple CPU
cores. This way, you get multithreading with no extra effort from you!

What if you want to create or remove entities and components, not just access
existing data? That requires special consideration. Bevy cannot change the
memory layout while other systems might be running. These operations can be
buffered/deferred using [Commands][cb::commands]. Bevy will apply them later
when it is safe to do so. You can also get [direct World access][cb::world]
using [exclusive systems][cb::exclusive], if you want to perform such
operations immediately.

[Bundles][cb::bundle] serve as "templates" for common sets of components, to
help you when you spawn new entities, so you don't accidentally forget anything.

```rust,no_run,noplayground
{{#include ../code014/src/programming/intro_data.rs:commands}}
```

这里没有太多高深的理论,只是形象化描述了实体和组件的关系,
如果想要了解为啥ECS性能高,为啥要这样设计数据结构的,可以查看ECS的几种实现.

抛开设计只看表现形式,实体/组件就像一个表格(或数据库的表),
不同的列是不同类型的数据(组件),可以有任意多行,每一行就是一个实体,
实体的ID就是行号(这是一个形象化的比喻,很多ECS的Entity设计就是一个唯一整形,
自增主键ID正好满足这个条件).

如果组件类型是空结构体(没有具体的值),那么这种组件称为`标记组件`.
标记特殊实体,启用特定的system,这非常有用.

上图中在一个表中表示了不同类型的实体(具体bevy实现是不是这样,需要分析源码再说).

一般是用实体来表示游戏/场景中出现的对象,eg:相机/敌人/光线/界面,但可以表示更多,
使用ECS并没有什么限制.

数据存储在实体/组件中,访问方式是query,在增量开发中,新增一个游戏机制,
在实现system时只需要通过query将符合条件的实体找出来,并应用新机制即可,
这点可以让多人并行协作,是一个很大的突破.
出了通过query-遍历实体,还可以直接通过实体ID来访问数据.

bevy通过调度机制+system的顺序机制,尽量保证每个cpu核上都跑着system,
这样的好处是开发者友好,开发者不必再自己维护多线程了.

system运行时,bevy不能修改内存布局,如果要增删实体或组件,该怎么办?
通过Commands进行延时处理,这样就可以保证安全了.
当然如果要立马生效,也可以直接通过world直接访问数据和独占system来实现.

bevy提供了很多Bundle来辅助构造实体.

### Comparison with Object-Oriented Programming

Object-Oriented programming teaches you to think of everything as "objects",
where each object is an instance of a "class". The class specifies the data
and functionality for all objects of that type, in one place. Every object
of that class has the same data (with different values) and the same
associated functionality.

This is the opposite of the ECS mentality. In ECS, any [entity][cb::entity] can
have any data (any combination of [components][cb::component]). The purpose of
entities is to identify that data. Your [systems][cb::system] are loose pieces
of functionality that can operate on any data. They can easily find what they
are looking for, and implement the desired behavior.

If you are an object-oriented programmer, you might be tempted to define a big
monolithic `struct Player` containing all the fields / properties of the player.
In Bevy, this is considered bad practice, because doing it that way can make it
more difficult to work with your data and limit performance. Instead, you should
make things granular, when different pieces of data may be accessed independently.

For example, represent the player in your game as an entity, composed of
separate component types (separate `struct`s) for things like the health, XP, or
whatever is relevant to your game. You can also attach standard Bevy components
like [`Transform`] ([transforms explained][cb::transform]) to it.

Then, each piece of functionality (each [system][cb::system]) can just
[query][cb::query] for the data it needs. Common functionality (like a
health/damage system) can be applied to any entity with the matching components,
regardless of whether that's the player or something else in the game.

If you have functionality that should only be applied to the player entity,
you can use a [marker component][cb::component-marker] (like `struct Player;`)
to narrow down your query (using a [query filter][cb::query-filter] like
`With<Player>`).

However, if some data always makes sense to be accessed together, then you
should put it in a single `struct`. For example, Bevy's [`Transform`].
With these types, the fields are not likely to be useful independently.

oo和反oo虽然是两种思想,但最终的目的都是一个:解决问题,达成目标,降低开发和维护成本.
oo是继承体系,ecs是组合思想.

oo是对象拥有数据,并拥有某些行为; ecs是数据和行为分离,每个system都是一个小的功能点,
组合不同的system同样可以实现逻辑,因为system都是碎片化的小功能点,她只关注自己的需求,
如果是oo的行为,会看到很多当前逻辑不需要的数据,整体的实现看要看整个继承体系
(这点在前几个版本不是问题), 但在后期维护的成本看,这会是个大问题.

ecs推荐小颗粒度的组件,这样方便复用.其实编成范式一直在演变,
oo如日中天时出现了ecs,并且`组合优于继承`基本深入人心了,
这是从无数个完整的项目生命周期得到的教训.

如果只想对某几个实体进行处理,便捷的方法是增加一个空的结构体组件充当标记组件,
同样走query机制.

如果某些数据一起访问时是有意义的,可以考虑放在同一个struct中.

### Additional Internal Details

The set / combination of components that a given entity has is called the
entity's Archetype. Bevy keeps track of that internally, to organize the
data in RAM. Entities of the same Archetype have their data stored together
in contiguous arrays, which allows the CPU to access and cache it efficiently.

If you add/remove component types on existing entities, you are changing
the Archetype, which may require Bevy to move previously-existing data to
a different location.

[Learn more about Bevy's component storage.][cb::component-storage]

Bevy will reuse Entity IDs. The [`Entity`] type is actually
two integers: the ID and a "generation". After you despawn some entities,
their IDs can be reused for newly-spawned entities, but Bevy will increase
the generation value.

在bevy中,实体拥有的组件列表成为原型,bevy内部会跟踪所有原型,
在内存中组织这些数据.(这是因为ECS为了迎合CPU的io特性,转换设计的,
只有要访问的数据是连续存放的,那么cpu在io时一次就可以读到未来几次需要的数据,
减少cpu的io次数,就是性能巨大提升的关键),同一个原型的实体时连续存放的,
方便cpu的缓存可以快速命中.

实体可以被销毁,但实体的id时可以复用的,一个实体内部由两个id表示:
一个是实体ID,一个是复用次数(也可以成为版本,代数,都是一个意思).
这里面还隐藏了一个关键技术:实体id复用机制.go语言有个arche也是ecs库,
实体的实现和bevy是一样的,底层有个链表来跟踪所有可复用的实体id,
不知道bevy是不是同样的实现.

## Resources

If there is only one global instance (singleton) of something, and it is
standalone (not associated with other data), create a [Resource][cb::res].

For example, you could create a resource to store your game's graphics
settings, or an interface to a non-Bevy library.

This is a simple way of storing data, when you know you don't need the
flexibility of Entities/Components.

```rust,no_run,noplayground
{{#include ../code014/src/programming/intro_data.rs:res}}
```

单例组件:资源.一个游戏,大部分都是资源,少部分灵活的才是组件.
