{{#include ../include/header014.md}}

# Systems

Relevant official examples:
[`ecs_guide`][example::ecs_guide],
[`startup_system`][example::startup_system],
[`system_param`][example::system_param].

---

Systems are pieces of functionality to be run by Bevy. They are typically
implemented using regular Rust functions. This is how you implement all
your game logic.

These functions can only take [special parameter types][builtins::systemparam],
to specify what [data][cb::ecs-intro-data] you need access to. If you use
unsupported parameter types in your function, [you will get confusing compiler
errors!][pitfall::intosystem]

Some of the possibilities are:
 - accessing [resources][cb::res] using [`Res`]/[`ResMut`]
 - accessing [components of entities][cb::component] using [queries][cb::query] ([`Query`])
 - creating/destroying entities, components, and resources using [Commands][cb::commands] ([`Commands`])
 - sending/receiving [events][cb::event] using [`EventWriter`]/[`EventReader`]

[See here for a full list!][builtins::systemparam]

```rust,no_run,noplayground
{{#include ../code014/src/programming/systems.rs:sys-debug-res}}
```

System parameters can be grouped into tuples (which can be nested). This is
useful for organization.

```rust,no_run,noplayground
{{#include ../code014/src/programming/systems.rs:sys-param-tuple}}
```

{{#include ../include/builtins.md:systemparam-limits}}

There is also a different kind of system: [exclusive systems][cb::exclusive].
They have [full direct access to the ECS World][cb::world], so you can access
any data you want and do anything, but cannot run in parallel. For most use
cases, you should use regular parallel systems.

```rust,no_run,noplayground
{{#include ../code014/src/programming/systems.rs:exclusive}}
```

system是bevy的功能片段,通常是rust函数实现,游戏的逻辑都在这里.

system的入参是有限制的(0),打破限制会导致编译报错(4.3).
常用的入参类型有:资源/query/Commands/事件收发器.

为了方便组织,可以将同类型的入参使用元组组织.

## Runtime

In order for your systems to actually be run by Bevy, you need to configure
them via the [app builder][cb::app]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/systems.rs:systems-appbuilder}}
```

Be careful: writing a new system `fn` and forgetting to add it to your app is a
common mistake! If you run your project and your new code doesn't seem to be
running, make sure you added the system!

The above is enough for simple projects.

Systems are contained in [schedules][cb::schedule]. [`Update`] is the schedule
where you typically add any systems you want to run every frame. [`Startup`] is
where you typically add systems that should run only once on app startup. There
are also [other possibilities][builtins::schedule].

As your project grows more complex, you might want to make use of some of the
powerful tools that Bevy offers for managing when/how your systems run, such as:
[explicit ordering][cb::system-order], [run conditions][cb::rc], [system
sets][cb::systemset], [states][cb::state].

在app中可以利用调度器配置system的执行顺序,如果system函数没有添加到app中,
此是编译器不会报错,但运行没有效果,这可能要会花很多时间排查.

Startup是app启动时执行一次,Update是每帧都会执行,具体详情可以查看`调度器`章节.

当system数量非常多时,可用到下列工具:`显示顺序指定`,`运行条件`,`system集合`,`状态`.

### One-Shot Systems

Sometimes you don't want Bevy to run your system for you. In that case,
don't add it to a schedule.

If you are a writing a system that you want to call yourself whenever
you want (such as on a button press), you can do that using [one-shot
systems][cb::oneshot].

`单击`system,不是注册到app由调度机制来触发执行,而是按需执行,
eg:按钮点击执行一个system.
