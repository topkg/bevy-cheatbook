{{#include ../include/header09.md}}

# Time and Timers

Relevant official examples:
[`timers`][example::timers],
[`move_sprite`][example::move_sprite].

---

## Time

The [`Time`][bevy::Time] [resource][cb::res] is your main global source
of timing information, that you can access from any [system][cb::system]
that does anything that needs time. [You should derive all timings from
it][pitfall::time].

Bevy updates these values at the beginning of every frame.

bevy中的时间是资源,可通过任意system访问,bevy会在每帧之前更新这个值.

### Delta Time

The most common use case is "delta time" – how much time passed between
the previous frame update and the current one. This tells you how fast the
game is running, so you can scale things like movement and animations. This
way everything can happen smoothly and run at the same speed, regardless of
the game's frame rate.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:time-delta}}
```

时间最常用的场景是:增量时间,上一帧到现在过去了多长时间.
有了这个时间就可以控制物体的移动和动画的播放,这样无论游戏帧率是多少,
所有的显示都是顺滑,并按同样的速度进行的.

上图所示,就是1秒移动的10个单位,这种处理方式大大减少了开发难度.

### Ongoing Time

[`Time`][bevy::Time] can also give you the total running time since startup.
Use this if you need a cumulative, increasing, measurement of time.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:time-monotonic}}
```

持续时间,如果要想计算:累计/增量/测量时间,使用这个.

## Timers and Stopwatches

There are also facilities to help you track specific intervals or timings:
[`Timer`][bevy::Timer] and [`Stopwatch`][bevy::Stopwatch]. You can create
many instances of these, to track whatever you want. You can use them in
your own [component][cb::component] or [resource][cb::res] types.

Timers and Stopwatches need to be ticked. You need to have some system
calling `.tick(delta)`, for it to make progress, or it will be inactive.
The delta should come from the [`Time`][bevy::Time] resource.

定时器和秒表. 这是两个跟踪固定间隔的工具,可以创建很多来跟踪各种事物,
可以在组件或资源中使用.

不管是定时器还是秒表,都需要在system中调用`.tick(delta)`进行启动或取消,
其中参数delta就来自于Time资源.

### Timer

[`Timer`][bevy::Timer] allows you to detect when a certain interval of time
has elapsed. Timers have a set duration. They can be "repeating" or
"non-repeating".

Both kinds can be manually "reset" (start counting the time interval from the
beginning) and "paused" (they will not progress even if you keep ticking them).

Repeating timers will automatically reset themselves after they reach their
set duration.

Use `.finished()` to detect when a timer has reached its set duration. Use
`.just_finished()`, if you need to detect only on the exact tick when the
duration was reached.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:timer}}
```

Note that Bevy's timers do *not* work like typical real-life timers (which
count downwards toward zero). Bevy's timers start from zero and count *up*
towards their set duration. They are basically like stopwatches with extra
features: a maximum duration and optional auto-reset.

定时器,允许检查某个间隔的时间是否已经到了.定时器除了可以设置时长,
还可以添加是否重复的标签,不管是不是重复的定时器,都可以进行reset/paused操作.

重复性定时器在达到时长后,会自动调用reset来重置状态.

使用`.finished()`来检测定时有没有到达.
使用`.just_finished()`仅在定时到达时返回true.

### Stopwatch

[`Stopwatch`][bevy::Stopwatch] allow you to track how much time has passed
since a certain point.

It will just keep accumulating time, which you can check with
`.elapsed()`/`.elapsed_secs()`. You can manually reset it at any time.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:stopwatch}}
```

秒表可以用来跟踪某个时间点到现在过了多长时间.
秒表会一直累积时间.
