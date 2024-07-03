{{#include ../include/header013.md}}

# Run Conditions

Run Conditions (RC) are a mechanism for controlling if Bevy should run specific
[systems][cb::system], at runtime. This allows you to enable/disable systems
on-demand, so that they only run sometimes.

RCs are Rust functions that return a value of type `bool`. They can accept
any [system parameters][builtins::systemparam], like a normal system, but
they must all be read-only (immutable).

```rust,no_run,noplayground
{{#include ../code013/src/programming/run_conditions.rs:fn}}
```

RCs can be applied to individual [systems][cb::system] or to entire [system
sets][cb::systemset].

```rust,no_run,noplayground
{{#include ../code013/src/programming/run_conditions.rs:app}}
```

When applied to a single [system][cb::system], Bevy will evaluate the RC at
the last moment, right before the system would otherwise be ready to run. If
you add the same RC to multiple systems, Bevy will evaluate it separately
for each one.

When applied to a [set][cb::systemset], the run condition will only be
evaluated once, before Bevy runs any system from the set, and if it returns
false, the entire set will be skipped.

Any given system can be governed by any number of RCs. You can add multiple RCs
to one system, and it will also inherit the RCs of any [sets][cb::systemset]
it belongs to. Bevy will evaluate all the RCs, and the system will only run
if all of them return `true`.

运行条件,是返回bool的一个函数,入参可以是system参数(只读).

运行条件应用对象可以是单个system,或system集合.

单system有运行条件时,运行条件在system之前执行.如果多个system都有同样的运行条件,
bevy会单独计算运行条件.

system集合有运行条件时,运行条件只会计算一次.

system可以有多个运行条件,还可以从任何system集合中继承运行条件.

## Common Conditions

Bevy provides some built-in RCs for some common scenarios, that you can just
apply to your systems:
 - [ECS common conditions][bevy::ecs::common_conditions]:
   - For checking [states][cb::state], [resource][cb::res] values and [changes][cb::change-detection], [events][cb::event], if [entities][cb::ecs-intro-data] with specific [components][cb::component] exist, etc...
 - [Input common conditions][bevy::input::common_conditions]:
   - For [input handling][chapter::input]: running on key/button press/release.
 - [Time common conditions][bevy::time::common_conditions]:
   - For controlling systems based on [time][cb::time]: repeating on a timer, running after a delay, etc...

bevy内置了一些常用的条件:
 - ECS常用条件,状态,资源值(的变化),事件,实体是否包含指定组件等等
 - 输入常用条件,某键是否按下/释放
 - 时间常用条件,基于定时器的控制,定时器/延时到达

## Known Pitfalls

When receiving [events][cb::event] in systems that don't run every frame
update, you can miss some events that are sent while the receiving systems
are not running!

To mitigate this, you could implement a [custom cleanup
strategy][cb::event-manual], to manually manage the lifetime of the relevant
event types.

已知陷阱.事件接收+条件运行可能会导致遗漏事件,应对方法是自定义事件消息清理策略.
