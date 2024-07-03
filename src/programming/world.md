{{#include ../include/header013.md}}

# Direct World Access

(This page is WIP)

---

The [`World`] is where Bevy ECS stores all data and associated metadata. It
keeps track of [resources][cb::res], [entities and components][cb::ec].

Typically, the [`App`]'s runner will run all [schedules][cb::schedule] (which,
in turn, run their [systems][cb::system]) on the main world. Regular
[systems][cb::system] are limited in what data they can access from the world,
by their [system parameter types][builtins::systemparam].  Operations that
manipulate the world itself are only done indirectly using
[`Commands`][cb::commands]. This is how most typical Bevy user code behaves.

However, there are also ways you can get full direct access to the world, which
gives you full control and freedom to do anything with any data stored in the
Bevy ECS:
 - [Exclusive systems][cb::exclusive]
 - [`FromWorld`] impls
 - Via the [`App`] [builder][cb::app]
 - Manually created [`World`]s for purposes like [tests][cb::system-tests] or scenes
 - [Custom Commands][cb::commands-custom]

Direct world access lets you do things like:
 - Freely spawn/despawn entities, insert/remove resources, etc., taking effect immediately
   (no delay like when using [`Commands`][cb::commands] from a regular [system][cb::system])
 - Access any component, entities, and resources you want
 - Manually run arbitrary systems or schedules

This is especially useful if you want to do things that do not fit within
Bevy's typical execution model/flow of just running systems once every frame.

With direct world access, you can implement custom control flow, like
looping some systems multiple times, selecting different systems to run in
different circumstances, exporting/importing data from files like scenes or
game saves, …

bevy将ECS的数据存储在world中,包括:资源/实体/组件.

通常app的所有调度都跑在主world中,普通system通过system参数类型跑在指定world中,
大部分代码都是通过Commands间接维护world.

直接维护world的几种途径如下:
 - 独占system
 - FromWorld的实现
 - app
 - 为特殊目的手动创建的world(eg:测试/场景)
 - 自定义Commands

直接访问world可以做以下几种事:
 - 自由的构造/销毁实体;自由的增删资源等,都是立马生效
 - 访问任意组件/实体/资源
 - 手动执行任意system/调度

如果在每帧的执行调度上,bevy内置的调度机制不满足需求,那么直接访问world是一种选择.

直接访问world意味着可以实现自定义逻辑流,eg:循环system多次,
不同的环境选择不同的system,游戏或场景的导入/导出.

## Working with the `World`

Here are some ways that you can make use of the direct world access APIs.

### `SystemState`

The easiest way to do things is using a [`SystemState`].

This is a type that "imitates a system", behaving the same way as a
[system][cb::system] with various parameters would. All the same behaviors like
[queries][cb::query], [change detection][cb::change-detection], and even
[`Commands`][cb::commands] are available. You can use any [system
params][builtins::systemparam].

It also tracks any persistent state, used for things like [change
detection][cb::change-detection] or caching to improve performance. Therefore,
if you plan on reusing the same [`SystemState`] multiple times, you should store
it somewhere, rather than creating a new one every time. Every time you call
`.get(world)`, it behaves like another "run" of a system.

If you are using [`Commands`], you can choose when you want to apply them to the
world. You need to manually call `.apply(world)` on the [`SystemState`], to
apply them.

```rust,no_run,noplayground
// TODO: write code example
```

访问world最常见的手法是`SystemState`,用法和普通的system类似,能实现的功能也类似.

可以跟踪持久化的状态,这点可用于`变更检测`或缓存(提高性能).
如果要多次重用同一SystemState,就应该考虑持久化,避免每次执行都生成一个新的.

### Running a System

```rust,no_run,noplayground
// TODO: write code example
```

### Running a Schedule

If you want to run many systems (a common use-case is
[testing][cb::system-tests]), the easiest way is to construct an impromptu
[schedule][cb::schedule]. This way you reuse all the scheduling logic that Bevy
normally does when running systems. They will run with multithreading, etc.

This is also useful if you want custom control flow. For example, Bevy's
[states][cb::state] and [fixed timestep][cb::fixedtimestep] abstractions
are implemented just like this! There is an exclusive system that can contain
loops, if/else branching, etc. to implement fancy algorithms and run entire
schedules of systems as appropriate!

```rust,no_run,noplayground
// TODO: write code example
```

如果要运行多个system(eg:测试用例),最简单的方式是构造一个调度,其他地方能重用.

若要实现自定义控制流,调度也是非常有用的,bevy的固定时间戳就是这样实现的.

### Navigating by Metadata

The world contains a lot of metadata that allows navigating all the data
efficiently, such as information about all the stored components, entities,
archeypes.

```rust,no_run,noplayground
// TODO: write code example
```

world包含了很多元数据,这个元数据之间有关联,可以方便做到导航,
eg:存储所有实体/组件/原型的信息.
