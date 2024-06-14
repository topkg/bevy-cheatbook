{{#include ../include/header014.md}}

# Commands

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

Use [`Commands`] to spawn/despawn entities, add/remove components on existing
entities, manage resources, from your [systems][cb::system].

Commands可以增删实体;从实体中增删组件;管理资源.

```rust,no_run,noplayground
{{#include ../code014/src/programming/commands.rs:example-commands}}
```

## When do these actions get applied?

[`Commands`] do not take effect immediately, because it wouldn't be safe to
modify the data layout in memory when other [systems][cb::system] could be
running in parallel. When you do anything using [`Commands`], it gets queued to
be applied later when it is safe to do so.

Within the same [schedule][cb::schedule], you can add `.before()`/`.after()`
[ordering constraints][cb::system-order] to your systems, and Bevy will
automatically make sure that Commands get applied in-between if necessary, so
that the second system can see the changes made by the first system.

Commands并不会立马生效,因为systems也在并行修改内存数据,立马生效会导致不安全,
等待安全时,bevy会将命令集应用到world上.

在调度器组织system时,bevy提供了before/after机制,在部分调度执行之前/之后插入逻辑,
bevy会择机应用Commands,保证了安全,下一个system执行时,Commands已经应用了.

before/after外加调度器内置的顺序,提供了强大的灵活性.

```rust,no_run,noplayground
{{#include ../code014/src/programming/commands.rs:order}}
```

If you do not have explicit ordering dependencies, it is undefined when Commands
will be applied. It is possible that some systems will only see the changes on
the next frame update!

Otherwise, Commands are normally applied at the end of every
[schedule][cb::schedule]. [Systems][cb::system] that live in different schedules
will see the changes. For example, Bevy's engine systems (that live in
[`PostUpdate`]) will see the entities you spawn in your systems (that live in
[`Update`]).

如果没有明确命令应用顺序,那么命令具体应用时间是不确定的,
可能在下帧才应用上.

## Custom Commands

Commands can also serve as a convenient way to do any custom manipulations
that require [full access][cb::world] to the ECS [`World`]. You can queue up
any custom code to run in a deferred fashion, the same way as the standard
commands work.

For a one-off thing, you can just pass a closure:

```rust,no_run,noplayground
{{#include ../code014/src/programming/commands.rs:command-closure}}
```

If you want something reusable, consider [one-shot systems][cb::oneshot].
They are a way to write regular Bevy systems and run them on-demand.

只要是访问ECS的world,使用Commands是非常便捷的方式.
可以将自定义逻辑以延时的方式运行,标准命令也是如此工作的.

对于一次性的事情,可以传递一个闭包.
对于想重用的事情,可以考虑`一击`system,这是常规的bevy systm,按需运行.

所谓`一击`system,就是想要时才调用,eg:按钮按下,触发一个特殊事件等等.

### Extending the Commands API

If you want something more integrated, that feels like as if it was
part of Bevy's Commands API, here is how to do it.

Create a custom type and implement the [`Command`] trait:

```rust,no_run,noplayground
{{#include ../code014/src/programming/commands.rs:command-impl}}
```

And if you want to make it extra nice to use, you can create
an extension trait to add extra methods to [`Commands`]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/commands.rs:command-ext}}
```

Note: if you want to use your custom extension method from other Rust
files, you will have to import your trait, or it will not be available:

```rust,no_run,noplayground
use crate::thing::MyCustomCommandsExt;
```

扩展Commands API

如果有更多交互需求,可扩展Commands.

bevy::ecs::system下有两个类型: 特型Command和 实现了Command的结构体Commands.

Commands是命令列表,Command是具体命令.

按照扩展的约束:只要当前crate中包含特型或类型其中之一就可以进行扩展.

所以先定义一个类型`MyCustomCommand`,让其实现Command,这样自定义命令就可以正常使用了.
如果要为自定义命令添加自定义方法,就需要添加自定义特型`MyCustomCommandsExt`,
此处需要注意:让Commands实现自定义特型.

NOTE: 如果要在其他rust文件中也使用这个扩展,就需要导入自定义特型.
