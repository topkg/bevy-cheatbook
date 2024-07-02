{{#include ../include/header014.md}}

# Resources

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

Resources allow you to store a single global instance of some data type,
independently of [entities][cb::ec].

Use them for [data][cb::ecs-intro-data] that is truly global for your app, such
as configuration / settings. Resources make it easy for you to access such data
from anywhere.

To create a new resource type, simply define a Rust `struct` or `enum`, and
derive the [`Resource`] trait, similar to
[components][cb::component] and [events][cb::event].

```rust,no_run,noplayground
{{#include ../code014/src/programming/res.rs:struct}}
```

Types must be unique; there can only be at most one instance of a given type. If
you might need multiple, consider using [entities and components][cb::ec] instead.

Bevy [uses resources for many things][builtins::res]. You can use these builtin
resources to access various features of the engine. They work just like your own
custom types.

资源是可以存储某类型数据的全局单例,独立于实体.
对于app来说,资源是全局的,(配置和设置就是资源),在任何地方访问资源都非常方便.

创建资源很简单,struct/enum添加`Resource`自动实现即可(组件/事件都是如此声明的).

资源类型要唯一(这个是肯定的),只能有一个实例(单例,这个也是正常的).
如果有多个实例,使用ECS即可.

资源可以用来做很多事,bevy提供了不少内置资源,通过这些内置资源可以访问很多bevy功能.

## Accessing Resources

To access the value of a resource from [systems][cb::system], use
[`Res`]/[`ResMut`]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/res.rs:systemparam}}
```

在system中访问资源,只需要使用`Res<T>`或`ResMut<T>`.

## Managing Resources

If you need to create/remove resources at runtime, you can do so using
[commands][cb::commands] ([`Commands`]):

```rust,no_run,noplayground
{{#include ../code014/src/programming/res.rs:commands}}
```

Alternatively, using [direct World access][cb::world] from an [exclusive
system][cb::exclusive]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/res.rs:exclusive}}
```

Resources can also be set up from the [app builder][cb::app]. Do this for
resources that are meant to always exist from the start.

```rust,no_run,noplayground
{{#include ../code014/src/programming/res.rs:app}}
```

运行过程中增删资源使用Commands进行操作;
也可以在独占system中直接对world进行操作;
对于一开始就要存在的资源,在app builder中也可以直接通过app增加资源.

## Resource Initialization

If you want to be able to use `.init_resource` to create your resource,
here is how you can provide the default value.

Implement [`Default`] for simple resources:

```rust,no_run,noplayground
{{#include ../code014/src/programming/res.rs:default}}
```

For resources that need complex initialization, implement [`FromWorld`]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/res.rs:fromworld}}
```

Beware: it can be easy to get yourself into a mess of unmaintainable code
if you overuse [`FromWorld`] to do complex things.

使用init_resource()时,对于特别复杂的初始化,可以直接使用`FroWorld`.
这种灵活性的代价是`过渡使用FromWorld很有可能写出无法维护的代码`.

## Usage Advice

The choice of when to use [entities/components][cb::ec] vs. resources is
typically about how you want to access the [data][cb::ecs-intro-data]: globally
from anywhere (resources), or using ECS patterns (entities/components).

Even if there is only one of a certain thing in your game (such as the
player in a single-player game), it can be a good fit to use an entity
instead of resources, because entities are composed of multiple components,
some of which can be common with other entities. This can make your game
logic more flexible. For example, you could have a "health/damage system"
that works with both the player and enemies.

选择使用ECS还是资源取决于访问方式,从任何地方访问全局变量,使用资源,其他使用ECS.

并不是只有一个实例就应该设计为资源,如果很多实体都要用到某个数据,设计为组件.
这样的考虑会更加灵活.

### Settings

One common usage of resources is for storing settings and configuration.

However, if it is something that cannot be changed at runtime and only used when
initializing a [plugin][cb::plugin], consider putting that inside the plugin's
`struct`, instead of a resource.

资源的另一个常见使用场景是存储设置/配置.

游戏启动时初始化,运行期间不改变,最好设计为插件,而不是资源.

### Caches

Resources are also useful if you want to store some data in a way that is easier
or more efficient for you to access. For example, keeping a collection of [asset
handles][cb::handle], or using a custom datastructure for representing a game
map more efficiently than using entities and components, etc.

[Entities and Components][cb::ec], as flexible as they are, are not necessarily
the best fit for all use cases. If you want to represent your data some other
way, feel free to do so. Simply create a resource and put it there.

只要是想存储的简单点,或是访问的简单点,都可以使用资源来实现.
eg: asset资源的处理handle;游戏地图(这个用资源实现比ECS简单多了).

ECS非常灵活,但并不是在各个非方面都领先其他方案.

### Interfacing with external libraries

If you want to integrate some external non-Bevy software into a Bevy app,
it can be very convenient to create a resource to hold onto its state/data.

For example, if you wanted to use an external physics or audio engine, you
could put all its data in a resource, and write some systems to call its
functions. That can give you an easy way to interface with it from Bevy code.

If the external code is not thread-safe (`!Send` in Rust parlance), which is
common for non-Rust (e.g C++ and OS-level) libraries, you should use a
[Non-Send][cb::nonsend] Bevy resource instead. This will make sure any Bevy
system that touches it will run on the main thread.

bevy程序使用外部软件时,资源适合存储状态和数据.eg:使用外部的物理引擎或音频引擎.

如果外部库不是线程安全的,可以用!Send来指明:只在主线程中运行.
