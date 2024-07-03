{{#include ../include/header013.md}}

# System Order of Execution

Bevy's scheduling algorithm is designed to deliver maximum performance by
running as many [systems][cb::system] as possible in parallel across the
available CPU threads.

This is possible when the systems do not conflict over the data they need
to access. However, when a system needs to have mutable (exclusive) access
to a piece of data, other systems that need to access the same data cannot
be run at the same time. Bevy determines all of this information from the
system's function signature (the types of the parameters it takes).

In such situations, the order is *nondeterministic* by default. Bevy takes
no regard for when each system will run, and the order could even change
every frame!

system的执行顺序

bevy的调度算法设计的目标是最高性能,做法是让每个CPU线程都运行一个system.
如果system访问的数据不存在冲突,那么调度算法就达到了目的.
如果一个system对某个数据是可变访问时(独占),那么其他依赖此数据的system就不能并行执行,
至于system要访问哪些数据,bevy可以从函数签名中了解信息.

默认情况下(未指定顺序),顺序是不确定的,bevy也不会主动干预,每帧system的执行顺序都可能不同.

即要提高性能,又要避免数据竞争导致的性能损失,最好的方式就是明确system的执行顺序.

## Explicit System Ordering

If a specific system must always run before or after some other systems,
you can add ordering constraints:

```rust,no_run,noplayground
{{#include ../code013/src/programming/system_order.rs:app}}
```

When you have a lot of systems that you need to configure, it can start to
get unwieldy. Consider using [system sets][cb::systemset] to organize and
manage your systems.

显示指定system之间的顺序.
有时有很多system要配置顺序,就可以使用system集合来组织和管理.

## Does it even matter?

In many cases, you don't need to worry about this.

However, sometimes you need to rely on specific systems to run in a particular
order. For example:

  - Maybe the logic you wrote in one of your systems needs any modifications
    done to that data by another system to always happen first?
  - One system needs to receive [events][cb::event] sent by another system.
  - You are using [change detection][cb::change-detection].

In such situations, systems running in the wrong order typically causes
their behavior to be delayed until the next frame. In rare cases, depending
on your game logic, it may even result in more serious logic bugs!

It is up to you to decide if this is important.

With many things in typical games, such as juicy visual effects, it probably
doesn't matter if they get delayed by a frame. It might not be worthwhile
to bother with it. If you don't care, leaving the order ambiguous may also
result in better performance.

On the other hand, for things like handling the player input controls,
this would result in annoying lag or worse, so you should probably fix it.

大部分场景下都不需要关心顺序,少部分场景下才需要顺序约束:
 - 一个system修改数据,另一个system使用,且有先后顺序
 - 一个system接收的事件是另一个system发出的
 - 变更检测

上面这些都是实际的游戏逻辑决定的,出现异常就意味着bug.

大部分视觉特效都不关心一帧中的延时,并发执行甚至还能提升性能.

## Circular Dependencies

If you have multiple systems mutually depending on each other, then it is
clearly impossible to resolve the situation completely like that.

You should try to redesign your game to avoid such situations, or just accept
the consequences. You can at least make it behave predictably, using explicit
ordering to specify the order you prefer.

循环依赖.需要重新涉及游戏逻辑,或直接不管.
不过游戏开发的底线是运行结果可预测.
