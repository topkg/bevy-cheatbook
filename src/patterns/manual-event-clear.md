{{#include ../include/header09.md}}

# Manual Event Clearing

The [event][cb::event] queue needs to be cleared periodically,
so that it does not grow indefinitely and waste unbounded memory.

Bevy's default cleanup strategy is to clear events every frame, but with double
buffering, so that events from the previous frame update stay available. This
means that you can handle the events only until the end of the next frame
after the one when they are sent.

This default works well for systems that run every frame and check for events
every time, which is the typical usage pattern.

However, if you have systems that do not read events every frame, they might
miss some events. Some common scenarios where this occurs are:
  - systems with an early-return, that don't read events every time they run
  - when using [fixed timestep][cb::fixedtimestep]
  - systems that only run in specific [states][cb::state],
    such as if your game has a pause state
  - when using custom [run criteria][cb::runcriteria] to control
    your systems

To be able to reliably manage events in such circumstances, you might want
to have manual control over how long the events are held in memory.

You can replace Bevy's default cleanup strategy with your own.

To do this, simply add your event type (wrapped as [`Events<T>`][bevy::Events])
to the [app builder][cb::app] using `.init_resource`, instead of `.add_event`.

(`.add_event` is actually just a convenience method that initializes the
[resource][cb::res] and adds Bevy's built-in system ([generic][cb::system-generic]
over your event type) for the default cleanup strategy)

You must then clear the events at your discretion. If you don't do this often
enough, your events might pile up and waste memory.

手动清理事件,事件队列会定时清理,所以不用担心内存泄漏.

bevy是默认每帧都清理事件,单因为执行顺序,可能部分事件会跨两帧.
要处理事件就必须在下帧结束之前处理.

同理,处理事件最好每帧都执行,但很多场景下并不是每帧都执行检测.
 - 固定时间戳
 - system在检测逻辑之前提前返回了
 - system只在特定state运行
 - system有运行条件

在上面的场景中,需要手动控制事件的清理.bevy提供了便捷的方法:
在app中用初始化资源代替添加事件,参数为资源(具体类型是事件),
bevy识别到后,知道遇到这个事件就丢到对应的资源中.

之后需要手动清理事件,不然就会出现内存泄漏.

## Example

We can create [generic systems][cb::system-generic] for this. Implement
the custom cleanup strategy, and then add that [system][cb::system] to your
[`App`][bevy::App] as many times as you need, for each [event][cb::event] type
where you want to use your custom behavior.

```rust,no_run,noplayground
{{#include ../code/examples/manual-event-clear.rs:main}}
```
