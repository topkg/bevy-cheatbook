{{#include ../include/header014.md}}

# Non-Send Resources

"Non-send" refers to data types that must only be accessed from the "main
thread" of the application. Such data is marked by Rust as `!Send` (lacking
the [`Send`] trait).

Some (often system) libraries have interfaces that cannot be safely used from
other threads. A common example of this are various low-level OS interfaces
for things like windowing, graphics, or audio. If you are doing advanced
things like creating a Bevy plugin for interfacing with such things, you
may encounter the need for this.

Normally, Bevy works by running all your [systems][cb::system] on a
thread-pool, making use of many CPU cores. However, you might need to ensure
that some code always runs on the "main thread", or access data that is not
safe to access in a multithreaded way.

Non-Send资源,意味只能从主线程访问指定类型的数据.rust会将这些数据标记为`!Send`.

很多底层OS接口都不能在多线程中安全切换,eg:窗口,图像,音频.

通常bevy运行system是利用线程池来运行,以提高多核CPU的利用率.

部分场景下,部分代码是需要一直跑在主线程中的,还有多线程访问数据是不安全的.

## Non-Send Systems and Data Access

To do this, you can use a [`NonSend<T>`] / [`NonSendMut<T>`] system parameter.
This behaves just like [`Res<T>`] / [`ResMut<T>`], letting you access an ECS
[resource][cb::res] (single global instance of some data), except that the
presence of such a parameter forces the Bevy scheduler to always run the
[system][cb::system] on the main thread. This ensures that data never has to be
sent between threads or accessed from different threads.

One example of such a resource is [`WinitWindows`] in Bevy.  This is the
low-level layer behind the [window entities][cb::window] that you typically use
for window management. It gives you more direct access to OS window management
functionality.

```rust,no_run,noplayground
{{#include ../code014/src/programming/non_send.rs:nonsend}}
```
```rust,no_run,noplayground
{{#include ../code014/src/programming/non_send.rs:nonsend-app}}
```

Non-Send system和数据访问.`NonSend<T>, NonSendMut<T>`这是访问资源的升级版,
添加的约束是system只在主函数中运行.

另一个常见的就是WinitWindows资源,是窗口实体背后窗口管理器的后端,
只能在主线程访问.

只需要指定NonSend,bevy在检查system参数时就会明白这个system是主线程跑的.

## Custom Non-Send Resources

Normally, to insert [resources][cb::res], their types must be [`Send`].

Bevy tracks non-Send resources separately, to ensure that they can only be
accessed using [`NonSend<T>`] / [`NonSendMut<T>`].

It is not possible to insert non-send resources using
[`Commands`][cb::commands], only using [direct World access][cb::world]. This
means that you have to initialize them in an [exclusive system][cb::exclusive],
[`FromWorld`] impl, or from the [app builder][cb::app].

```rust,no_run,noplayground
{{#include ../code014/src/programming/non_send.rs:insert-nonsend}}
```
```rust,no_run,noplayground
{{#include ../code014/src/programming/non_send.rs:insert-nonsend-app}}
```

Or, for simple things, if you don't need a full-blown system:

```rust,no_run,noplayground
{{#include ../code014/src/programming/non_send.rs:insert-nonsend-app-world}}
```

If you just need to write a [system][cb::system] that must run on
the main thread, but you don't actually have any data to store,
you can use [`NonSendMarker`] as a dummy.

```rust,no_run,noplayground
{{#include ../code014/src/programming/non_send.rs:nonsend-marker}}
```

自定义Non-Send资源.

一般添加的资源都是Send的(可在多线程访问的).

自定义的Non-Send资源只能通过world直接访问,Commands是无法添加这类资源的.

world直接访问意味着:独占system/FromWorld/app三种途径可以实现.
`world.insert_non_send_resource()`.

还有一种情况,system没有特别的入参,可使用`NonSend<NonSendMarker>`来告诉bevy,
system运行在主线程上.
