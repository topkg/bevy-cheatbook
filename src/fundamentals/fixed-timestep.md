{{#include ../include/header013.md}}

# Fixed Timestep

Relevant official examples:
[`fixed_timestep`][example::fixed_timestep].

---

If you need to run some [systems][cb::system] at a fixed rate, independent
of the display frame rate, Bevy provides a solution.

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/fixed_timestep.rs:basic}}
```

Every frame update, Bevy will run the [`FixedUpdate`] schedule as many times as
needed to catch up. If the game is running slow, it might run multiple times. If
the game is running fast, it might be skipped.

This happens before the regular [`Update`] schedule runs for that frame, but
after [state transitions][cb::state].

The default fixed timestep interval is 64 Hz. If you want something else,
you can configure it as follows:

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/fixed_timestep.rs:configure}}
```

如果部分system不需要以来帧率,bevy也提供了方案:`固定时间戳`.
这类system放在FixedUpdate调度中,bevy会评估当前帧的可用时间,
如果时间够用就会多次执行system,如果时间不够用就跳过.
但不管单帧运行多次次,bevy会努力保证调用的平均固定速率.
默认调度间隔是64Hz.也可以调整为96Hz或以时间为间隔,如上图.

## Checking the Time

Just use [`Res<Time>`] as normal. When your system is running in
[`FixedUpdate`], Bevy will automatically detect that, and all the timing
information (such as delta) will represent the fixed timestep instead of the
display frame rate.

```rust,no_run,noplayground
{{#include ../code013/src/fundamentals/fixed_timestep.rs:time}}
```

If you need to access the regular frame-time from a system running under
fixed timestep, you can use `Res<Time<Virtual>>` instead. `Res<Time<Real>>`
gives you the real (wall-clock) time, without pausing or scaling.

If you need to access the fixed-timestep-time from a system running outside
of fixed timestep, you can use `Res<Time<Fixed>>` instead.

在system中也可以使用Time,bevy会自动维护这个资源,但这个资源里的数据含义会有变化,
从帧率改为了时间步长.

Time提供了3种内置时间:
 - Virtual, bevy内置的普通时间
 - Fixed, bevy基于Virtual继续封装的固定时间戳
 - Real, 真实世界的时间,这个可能用的非常少,在日志打印时会用到

## Should I put my systems in `Update` or `FixedUpdate`?

The purpose of fixed timestep is to make gameplay code behave predictably
and reliably. Things such as physics and simulation work best if they are
computed with fixed time intervals, as that avoids floating point errors
from accumulating and glitchy behavior from variable framerate.

The following things should probably be done in [`FixedUpdate`]:
 - Physics and collision detection
 - Networking / netcode
 - AI for enemies and NPCs (pathfinding, decisions, etc.)
 - Spawning/despawning gameplay-related entities
 - Other simulation and decision-making

However, anything that directly affects what is displayed on-screen should
run per-frame, in order to look smooth. If you do movement or animation under
fixed timestep, it will look choppy, especially on high-refresh-rate screens.

The following things should probably be done in [`Update`]:
 - Camera movement and controls
 - Animations
 - UI
 - Visual effects
 - Anything that is part of your game's graphics/visuals or interactivity
 - [App state][cb::state] transitions

固定时间戳引入的目的是为了让游戏代码更具有可预测性和可靠性.
诸如物理和模拟之类的事情如果以固定的时间间隔进行计算，效果最好，
因为这可以避免浮点错误的累积和可变帧速率的故障行为。

下列场景应该使用固定时间戳:
 - 物理和碰撞检测
 - 网络代码
 - 敌人和NPC的AI,包括(寻路/决策等)
 - 实体的生成和销毁(FixedUpdate调度在Update之前,在挂起状态调整之后)
 - 其他模拟和决策

任何在屏幕上显示的都应该按帧运行,这样看起来就会很流畅.
这句话的意思是(能渲染的放在Update中,决策渲染的逻辑放在FixedUpdate中).

下列场景因该使用Update:
 - Camera的移动和控制
 - 动画
 - UI
 - 显示效果
 - 图形/视觉效果/交互
 - app状态转换

### Bridging the Gap

Sometimes there is a logical conflict:

For something like player movement, you want it to be computed reliably as part
of your gameplay/physics simulation, but you also want it to look smooth on-screen.

For input handling, you want it to be responsive and handled every frame, but
you also have game mechanics that need to respond to it.

The most elegant solution to both of these problems is to handle synchonization
yourself using custom types.

并不是所有的system都有个明显的划分,经常会出现逻辑冲突,常见的有:

角色移动,可以作为游戏物理模拟的的可靠计算逻辑,也希望在屏幕上看起来丝滑.
输入处理,既需要每帧都处理,也希望游戏机制能响应.

类似的问题,最优雅的方式是使用自定义的类型来完成同步.

#### Movement

For player (and other) movement, you could create your own custom component type
to use instead of [`Transform`]. Implement your player movement using your own
types. Then have a system in [`Update`] to sync/update [`Transform`] from that,
with some interpolation to make it look smooth.

```rust,no_run,noplayground
// TODO show how to do this
```

Transform组件负责移动,优雅的方式是创建一个类似的组件来负责移动,
FixedUpdate负责决策,中间的卡顿通过在Update中进行`插值`来更新Transform,
通过这种方式来达到界面顺滑不卡顿的目的.

#### Input Handling

If you use [`Res<ButtonInput<...>>`][`ButtonInput`] and
`.just_pressed`/`.just_released` to check for key/button presses, beware that
the state is updated once per frame. This API is not reliable inside
[`FixedUpdate`]. Use [events][cb::event] for input handling instead, or roll
your own abstractions.

One way to do this is to put your input handling systems in [`PreUpdate`], order
them after Bevy's [`InputSystem`] [set][cb::systemset], and do your input
handling there. Convert it into your own custom [event][cb::event] types or some
other useful representation, which you can then handle from your gameplay code
in [`FixedUpdate`].

```rust,no_run,noplayground
// TODO show how to do this
```

按钮的按键检测是每帧检测,放在调用次数不确定的FixedUpdate中,也不能确保逻辑已经能执行,
所以放在FixedUpdate并不可靠.该使用事件代替,或自己手搓抽象层来完成.

有种实现方式是这样的:在PreUpdate(每帧调度中,仅在First之后,在挂起状态处理之后)处理输入,
通过bevy的InputSystem set中进行排序,将输入变为自定义事件,然后在FixedUpdate中处理.

## Timing Caveats

Fixed timestep does not run in real-world time! You cannot rely on it for timing!

For example, if you try to play audio from it, or send network packets, you will
notice that they don't actually occur at the fixed timestep interval. They will
not be evenly spaced!

Your [systems][cb::system] are still called as part of the regular frame-update
cycle. Every frame update, Bevy will run the [`FixedMain`]
[schedule][cb::schedule] as many times as needed to catch up.

This means if you specify, for example, a 60 Hz fixed timestep interval, your
systems will not actually run in 1/60 second intervals in real time.

What will happen is the following:
 - If the display frame rate is faster than the timestep, some frame update cycles
   will skip the [`FixedMain`] schedule entirely.
 - If the display frame rate is slower than the timestep, some frame update cycles
   will run the [`FixedMain`] multiple times.

In any case, [`FixedMain`] will run right before
[`Update`], where your per-frame systems live.

固定时间戳不是真实时间,计时不能依赖这个.

音频播放/网络发包,这些都不能以来固定时间戳,这个运行间隔并不是固定步长,
只是平均起来是固定步长.

实际上的运行可能是以下几类:
 - 帧率比固定时间戳快,FixedUpdate可能会忽略
 - 帧率比固定时间戳慢,FixedUpdate可能会执行多次

## Additional Schedules

[`FixedUpdate`] is actually part of a larger [`FixedMain`]
[schedule][cb::schedule], which also contains other [schedules][cb::schedule]:

 - [`FixedFirst`]
 - [`FixedPreUpdate`]
 - [`FixedUpdate`]
 - [`FixedPostUpdate`]
 - [`FixedLast`]

They are analogous to the [schedules][cb::schedule] in [`Main`], that run every
frame update. They can be used for analogous purposes (to contain "engine
systems" from Bevy and plugins).

上面是主调度(Main)在每帧的调度顺序(不是首帧的).
