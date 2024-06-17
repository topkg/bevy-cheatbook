{{#include ../include/header014.md}}

# Events

Relevant official examples:
[`event`][example::event].

---

Send data between systems! Let your [systems][cb::system] communicate with each other!

Like [resources][cb::res] or [components][cb::component], events are
simple Rust `struct`s or `enum`s. When creating a new event type, derive
the [`Event`] trait.

Then, any [system][cb::system] can send (broadcast) values of that type,
and any system can receive those events.

 - To send events, use an [`EventWriter<T>`].
 - To receive events, use an [`EventReader<T>`].

Every reader tracks the events it has read independently, so you can handle
the same events from multiple [systems][cb::system].

```rust,no_run,noplayground
{{#include ../code014/src/programming/events.rs:events}}
```

You need to register your custom event types via the [app builder][cb::app]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/events.rs:events-appbuilder}}
```

Event用于system传递数据.derive中指定Event就可以声明事件了.

任何system都可以发送事件,任何sytem都可以接收事件.
`EventWriter<T>`发送事件,`EventReader<T>`接收事件.
每个reader都是单独消费事件的.

如上面的例子,事件收发都在system入参中明确了.

## Usage Advice

Events should be your go-to data flow tool. As events can be sent from any
[system][cb::system] and received by multiple systems, they are *extremely*
versatile.

Events can be a very useful layer of abstraction. They allow you to decouple
things, so you can separate different functionality and more easily reason
about which [system][cb::system] is responsible for what.

You can imagine how, even in the simple "player level up" example shown above,
using events would allow us to easily extend our hypothetical game with more
functionality. If we wanted to display a fancy level-up effect or animation,
update UI, or anything else, we can just add more systems that read the events
and do their respective things. If the `player_level_up` system had simply
checked the player XP and managed the player level directly, without going via
events, it would be unwieldy for future development of the game.

system之间交互的,推荐使用事件,在绝大部分场景下,事件都是优先选项.
和消息系统类似,bevy的事件系统具有强大的解耦能力.

只有解耦之后,才能在一个事件发生时才能触发更多逻辑,从工程化和协作的角度来看,这点是非常重要的.

## How it all works

When you register an event type, Bevy will create an [`Events<T>`]
[resource][cb::res], which acts as the backing storage for the event queue. Bevy
also adds an "event maintenance" [system][cb::system] to clear events periodically,
preventing them from accumulating and using up memory.

Bevy ensures that events are kept around for at least two frame update cycles,
or two [fixed timestep][cb::fixedtimestep] cycles, whichever is longer. After
that, they are silently dropped. This gives your systems enough opportunity
to handle them, assuming your systems are running all the time. Beware when
adding [run conditions][cb::rc] to your systems, as you might miss some events
when your systems are not running!

If you don't like this, [you can have manual control over when events are
cleared][cb::event-manual] (at the risk of leaking / wasting memory if you
forget to clear them).

The [`EventWriter<T>`] system parameter is just syntax sugar for mutably
accessing the [`Events<T>`] [resource][cb::res] to add events to the queue. The
[`EventReader<T>`] is a little more complex: it accesses the events storage
immutably, but also stores an integer counter to keep track of how many events
you have read. This is why it also needs the `mut` keyword.

[`Events<T>`] itself is internally implemented using simple [`Vec`]s. Sending
events is equivalent to just pushing to a [`Vec`]. It is very fast,
low overhead. Events are often the most performant way to implement things
in Bevy, better than using [change detection][cb::change-detection].

当我们注册一个事件类型时,bevy会创建一个`Event<T>`资源,事件队列就存储在这个资源中.
bevy还添加了一个定期清理事件的`事件维护system`,防止内存泄漏.

bevy保证事件至少保留两帧,或两个Fixed周期(不按帧走的sytem,就是按Fixed time走的).
之后就默默被丢弃.如果system一直在运行,那么捕获事件的概率就很高了.
所以指定system的`运行条件`要非常注意,一不小心就会错过事件.
当然也可以手动清除事件,只是如果忘了清理就会造成内存泄漏.

`EventWriter<T>`参数是一个mut的语法糖,用于将事件添加到事件队列;
`EventReader<T>`参数虽然不会修改事件队列,但会更新接收事件的数量,所以同样使用了mut.

`Event<T>`内部使用`Vec`实现的,发送事件就是push,效率很高,比`变更检测`的效率还高.

## Possible Pitfalls

Beware of frame delay / 1-frame-lag. This can occur if Bevy runs the
receiving system before the sending system. The receiving system will only
get a chance to receive the events the next time it runs. If you need to
ensure that events are handled on the same frame, you can use [explicit
system ordering][cb::system-order].

If your systems have [run conditions][cb::rc], beware that they might miss
some events when they are not running! If your system does not check for events
at least once every other frame or [fixed timestep][cb::fixedtimestep], the
events will be lost.

If you want events to persist for longer than that, you can [implement a
custom cleanup/management strategy][cb::event-manual]. However, you can
only do this for your own event types. There is no solution for Bevy's
[built-in][builtins::event] types.

可能的失败.

注意部分场景下是有1帧的滞后的. eg:bevy先执行了接收system再执行发送system.
如果要确保在同帧中处理事件,那需要显示指定system的执行顺序.

使用条件运行的system,有可能会丢失事件.

如果想事件持久一些,可以自己实现事件清理策略,但这种方式只能用于自定事件,
bevy内置的事件无法修改清理策略.
