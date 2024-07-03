{{#include ../include/header014.md}}

# Exclusive Systems

Exclusive systems are [systems][cb::system] that Bevy will not run in parallel
with any other system. They can have [full unrestricted access][cb::world]
to the whole ECS [`World`], by taking a [`&mut World`] parameter.

Inside of an exclusive system, you have full control over all data stored
in the ECS. You can do whatever you want.

Some example situations where exclusive systems are useful:
 - Dump various entities and components to a file, to implement things like
   saving and loading of game save files, or scene export from an editor
 - Directly spawn/despawn [entities][cb::ec], or insert/remove [resources][cb::res],
   immediately with no delay (unlike when using [Commands][cb::commands]
   from a regular system)
 - Run arbitrary [systems][cb::system] and [schedules][cb::schedule] with your
   own custom control flow logic
 - …

See the [direct World access page][cb::world] to learn more about how to do
such things.

```rust,no_run,noplayground
{{#include ../code014/src/programming/exclusive.rs:fn}}
```

You need to add exclusive systems to the [App][cb::app], just like
regular systems. All scheduling APIs ([ordering][cb::system-order], [run
conditions][cb::rc], [sets][cb::systemset]) are supported and work the same
as with regular systems.

```rust,no_run,noplayground
{{#include ../code014/src/programming/exclusive.rs:app}}
```

独占system也是system,只不过不和其他system并行处理,正是因为这点,
独占system比普通system多了一个使用场景:无限制访问world.
而使用独占system主要是为了无限制访问world,不然使用普通system得了.

无限制访问world,意味着可以访问整个ECS,下面是独占system的几个常用使用场景:
 - dump ECS到一个文件,游戏的保存和加载,从编辑器导出场景
 - 直接增删实体,增删资源,这是立马生效的(Commands也能做到,只不过是延时的)
 - 执行任意system/调度

在app中添加独占system的方式和处理普通system的方式是一样的,
bevy内部会识别system参数,来标识独占system.
事情都是bevy做的,用户友好.

## Exclusive System Parameters

There are a few other things, besides [`&mut World`], that can be used as
parameters for exclusive systems:

{{#include ../include/builtins.md:systemparam-exclusive}}

[`SystemState`] can be used to emulate a normal system.  You can put regular
system parameters inside. This allows you to access the [`World`] as you would
from a normal system, but you can confine it to a specific scope inside your
function body, making it more flexible.

[`QueryState`] is the same thing, but for a single query.  It is a simpler
alternative to [`SystemState`] for when you just need to be able to query for
some data.

```rust,no_run,noplayground
{{#include ../code014/src/programming/exclusive.rs:systemstate}}
```

Note: if your [`SystemState`] includes [`Commands`], you must call `.apply()`
after you are done! That is when the deferred operations queued via
[commands][cb::commands] will be applied to the [`World`].

独占system的参数,出了`&mut world`,还有以下几种:
 - `Local<T>`
 - `&mut SystemState<P>`,模拟一个普通system,简化world的访问
 - `&mut QueryState<Q, F= ()>` QueryState: 从world执行查询,类似普通system的Query

从下面的SystemState泛型类型的定义来看,具体参数是system参数,
SystemState本身也实现了SystemParam特性,也是一个system参数,
所以说SystemState是对普通system参数的一层封装.
因为SystemState能访问所有ECS数据,在使用时只需要限制在特定范围就能得到最大灵活性.

如果SystemState包含了Commands,需要调用apply来让其生效.
SystemState对应普通system的参数;QueryState对应普通system的query,搭配使用.

```rust
pub struct SystemState<Param: SystemParam + 'static> {
    meta: SystemMeta,
    param_state: Param::State,
    world_id: WorldId,
    archetype_generation: ArchetypeGeneration,
}

impl<Param: SystemParam> SystemState<Param> {...}
```

## Performance Considerations

Exclusive systems, by definition, limit parallelism and multi-threading, as
nothing else can access the same ECS World while they run. The whole schedule
needs to come to a stop, to accomodate the exclusive system. This can easily
introduce a performance bottleneck.

Generally speaking, you should avoid using exclusive systems, unless you need
to do something that is only possible with them.

On the other hand, if your alternative is to use [commands][cb::commands],
and you need to process a huge number of entities, exclusive systems are faster.

[`Commands`] is effectively just a way to ask Bevy do to exclusive [`World`]
access for you, at a later time. Going through the commands queue is much
slower than just doing the exclusive access yourself.

Some examples for when exclusive systems can be faster:
 - You want to spawn/despawn a ton of entities.
   - Example: Setup/cleanup for your whole game map.
 - You want to do it every frame.
   - Example: Managing hordes of enemies.

Some examples for when normal systems with [`Commands`] can be faster:
 - You need to check some stuff every frame, but only use [commands][cb::commands] sometimes.
   - Example: Despawn enemies when they reach 0 HP.
   - Example: Spawn/despawn entities when [timers][cb::timer] finish.
   - Example: Add/remove some UI elements depending on what is happening in-game.

独占system,运行时不能并行执行,效率肯定有很大的影响,性能瓶颈一般出在这儿,
在游戏中要尽量避免使用独占system.

如果要对大量实体进行处理,使用Commands延时处理,此是性能不一定比独占system效率高,
矮个子里拔高个,两权相害取其轻.

Commands时延时执行,里面涉及队列,如果实体数量大了,性能就差了.

以下场景独占system会更快:
 - 实体的构造和销毁,游戏启动或结束清理时
 - 每帧都执行的,eg:管理大量敌人

以下场景是Commands更快:
 - 每帧都需要检查一些东西,Commands只是时不时用到

eg: 敌人没hp了,就要销毁实体;定时器到了需要构造/销毁实体;依据游戏逻辑增删UI元素.
