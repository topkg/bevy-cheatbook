{{#include ../include/header014.md}}

# Internal Parallelism

Internal parallelism is multithreading *within* a [system][cb::system].

The usual multithreading in Bevy is to run each [system][cb::system] in
parallel when possible (when there is no conflicting data access with other
systems). This is called "external parallelism".

However, sometimes, you need to write a [system][cb::system] that
has to process a huge number of [entities][cb::ecs-intro-data] or
[events][cb::event]. In that case, simple [query][cb::query] or
[event][cb::event] iteration would not scale to make good use of the CPU.

Bevy offers a solution: parallel iteration. Bevy will automatically split
all the entities/events into appropriately-sized batches, and iterate
each batch on a separate CPU thread for you, calling a function/closure
you provide.

If there are only a few entities/events, Bevy will automatically fall back
to single-threaded iteration, and it will behave the same way as if you
had just iterated normally. With a few entities/events, that is faster than
multi-threading.

Even through parallel iteration should automatically make a good decision
regardless of the number of entities/events, it is more awkward to use and
not always suitable, as you have to do everything from inside a closure
and there are other limitations.

Also, if your [system][cb::system] is unlikely to ever encounter huge
numbers of entities/events, don't bother with it and just iterate your
[queries][cb::query] and [events][cb::event] normally.

内部并行是指system内部的多线程.  
外部并行是指bevy启用多线程让system并行执行.

某些时候,在一个system内可能要遍历大量实体或事件,
此时简单的query或事件迭代就无法有效利用CPU了.

bevy提出了内部并行的方案:bevy自动将实体列表/事件列表拆分成合适大小,
每块使用一个CPU线程来运行.如果块数很少,bevy会自动合并到一个线程中跑.

即使开启了内部并行,但bevy内部的自动机制也不一定合适,因为闭包内部还有其他限制.

**如果项目不是遇到巨量实体/事件,使用正常的query循环/事件迭代就够用了.**

## Parallel Query Iteration

[Queries][cb::query] support parallel iteration to let you process many
entities across multiple CPU threads.

```rust,no_run,noplayground
{{#include ../code014/src/programming/par_iter.rs:query}}
```

One limitation of parallel iteration is that safe Rust does not allow you to
share `&mut` access across CPU threads. Therefore, it is not possible to mutate
any data outside of the current entity's own [components][cb::component].

If you need to mutate shared data, you could use something like [`Mutex`],
but beware of the added overhead. It could easily drown out any benefits
you get from parallel iteration.

## Parallel Commands

If you need to use [commands][cb::commands], there is the [`ParallelCommands`]
system parameter. It allows you to get access to [`Commands`] from within
the parallel iteration closure.

```rust,no_run,noplayground
{{#include ../code014/src/programming/par_iter.rs:commands}}
```

However, generally speaking, [commands][cb::commands] are an inefficient way to
do things in Bevy, and they do not scale well to huge numbers of entities. If
you need to spawn/despawn or insert/remove [components][cb::component]
on huge numbers of entities, you should probably do it from an [exclusive
system][cb::exclusive], instead of using [commands][cb::commands].

In the above example, we update [timers][cb::timer] stored across many
entities, and use [commands][cb::commands] to despawn any entities whose
time has elapsed. It is a good use of [commands][cb::commands], because the
timers need to be ticked for all entities, but only a few entities are likely
to need despawning at once.

## Parallel Event Iteration

[`EventReader<T>`] offers parallel iteration for [events][cb::event],
allowing you to process a huge number of events across multiple CPU threads.

```rust,no_run,noplayground
{{#include ../code014/src/programming/par_iter.rs:events}}
```

However, one downside is that you cannot use it for events that need to be
handled in order. With parallel iteration, the order becomes undefined.

Though, if you use [`.for_each_with_id`][`EventParIter`], your closure will
be given an [`EventId`], which is a sequential index to indicate which event
you are currently processing. That can help you know where you are in the
event queue, even though you are still processing events in an undefined order.

Another downside is that typically you need to be able to mutate some data in
response to events, but, in safe Rust, it is not possible to share mutable
access to anything across CPU threads. Thus, parallel event handling is
impossible for most use cases.

If you were to use something like [`Mutex`] for shared access to data, the
synchronization overhead would probably kill performance, and you'd have
been better off with regular single-threaded event iteration.

## Controlling the Batch Size

The batch size and number of parallel tasks are chosen automatically using
smart algorithms, based on how many entities/events need to be processed,
and how Bevy ECS has stored/organized the entity/component data in memory.
However, it assumes that the amount of work/computation you do for each
entity is roughly the same.

If you find that you want to manually control the batch size, you can specify
a minimum and maximum using [`BatchingStrategy`].

```rust,no_run,noplayground
{{#include ../code014/src/programming/par_iter.rs:batching-strategy}}
```
