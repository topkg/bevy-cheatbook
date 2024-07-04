{{#include ../include/header014.md}}

# One-Shot Systems

One-Shot Systems are [systems][cb::system] that you intend to call yourself,
whenever you want. For example: on a button press, upon triggering a special
item or ability in your game, etc…

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:example}}
```

单击system,ui操作,游戏中的补蓝等.

## Registration

You should not add these systems to a [schedule][cb::schedule].

Instead, you can register them into the [`World`], to get a [`SystemId`].
You can then store that [`SystemId`] somewhere and use it to run the
system later.

The most convenient way is probably to use [`FromWorld`] and put your
[`SystemId`]s in a [resource][cb::res]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:register-fromworld}}
```

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:register-fromworld-app}}
```

Alternative: register from an [exclusive system][cb::exclusive]:

<details>
  <summary>Code:</summary>

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:register-exclusive}}
```

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:register-exclusive-app}}
```

</details>

Or from the [app builder][cb::app]:

<details>
  <summary>Code:</summary>

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:register-app}}
```

</details>

单击system不属于调度,而是属于world.使用`register_system`来注册system.
(add_system是ecs添加system).

这里有个概念,world中的单击system是有systemID的,类似于实体id一样,是个引用.
register_system注册后会返回systemID,后续调用需要这个ID.

这个ID可以保存起来,是的,推荐使用资源将ID保存起来,为了方便区分,还可以使用hash存储起来.

如上图所示,定一个资源来存储hash.实现FromWorld特型(自动填充资源).
或者直接操纵world来添加资源,或者通过app来添加资源.
(推荐使用FromWorld特型).

## Running

The easiest way is using [Commands][cb::commands] ([`Commands`]):

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:run-commands}}
```

This queues up the system to be run later, whenever Bevy decides to
apply the [Commands][cb::commands].

If you want to run a one-shot system immediately, like a normal function
call, you need [direct `World` access][cb::world]. Do it from an [exclusive
system][cb::exclusive]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:run-exclusive}}
```

Either way, the one-shot system's [Commands][cb::commands]
are automatically applied immediately when it runs.

运行,通过`Commands.run_system(systemID)`实现.这样是延时执行的.
如果要立马生效,用独占system来直接访问world.

### Without Registration

It is possible to also run one-shot systems without [registering](#registration)
them beforehand:

```rust,no_run,noplayground
{{#include ../code014/src/programming/one_shot_systems.rs:run-once}}
```

If you do this, Bevy is unable to store any data related to the system:
 - [Locals][cb::local] will not retain their value from a previous run.
 - [Queries][cb::query] will not be able to cache their lookups, leading to slower performance.
 - etc…

It is therefore recommended to register your one-shot systems, unless
you really only intend to run them once.

bevy还允许不注册直接运行单击system.`world.run_system_once(my_oneshot_system_fn);`
只是这样bevy就不能缓存system相关数据了:
 - Local
 - Query

## Performance Considerations

To run a one-shot system, exclusive [`World`] access is required. The
system can have arbitrary parameters, and Bevy cannot validate its data
access against other systems, like it does when the system is part of a
[schedule][cb::schedule]. So, no multi-threading allowed.

In practice, this isn't usually a problem, because the use cases for
one-shot systems are things that happen rarely.

But maybe don't overuse them! If something happens regularly, consider
doing it from a normal system that is part of a [schedule][cb::schedule],
and controlling it with [run conditions][cb::rc] instead.

如果使用独占system来运行单击system,此时会独占world,性能就是一个大问题.

这个机制存在的理由是:实际场景下必须使用独占system来运行单击system的场景非常少.

只要不过渡使用即可.如果单击system发生的非常频繁,需考虑转换成普通system.
