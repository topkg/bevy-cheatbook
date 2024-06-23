{{#include ../include/header012.md}}

# Performance Tunables

Bevy offers a lot of features that should improve performance in most cases, and
most of them are enabled by default. However, they might be detrimental to some
projects.

Luckily, most of them are configurable. Most users should probably not touch
these settings, but if your game does not perform well with Bevy's default
configuration, this page will show you some things you can try to change, to see
if they help your project.

Bevy's default configruation is designed with *scalability* in mind. That is, so
that you don't have to worry too much about performance, as you add more
features and complexity to your project. Bevy will automatically take care to
distribute the workload as to make good use of the available hardware (GPU, CPU
multithreading).

However, it might hurt simpler projects or have undesirable implications in some
cases.

This trade-off is good, because small and simple games will probably be fast
enough anyway, even with the additional overhead, but large and complex games
will benefit from the advanced scheduling to avoid bottlenecks. You can
develop your game without performance degrading much as you add more stuff.

性能可调参数.

bevy提供了大量功能,在大多数场景下都能提高性能,这些大多是默认开启的.
在部分项目中,这些默认开启的可能会起到反效果.

这些参数都是可调的.

bevy的设计权衡是可扩展,有些机制启用后在简单项目中不是最优,但消耗也能接受,
在复杂项目中就能得到很好的收益,eg:多线程(在自定义bevy一节中,多线程是第一个默认启用的功能).

## Multithreading Overhead

Bevy has a smart multithreaded executor, so that your [systems][cb::system] can
automatically [run in parallel][cb::ecs-intro-code] across multiple CPU cores,
when they don't need conflicting access to the same data, while [honoring ordering
constraints][cb::system-order]. This is great, because you can just keep adding
more systems to do different things and implement more features in your game,
and Bevy will make good use of modern multi-core CPUs with no effort from you!

However, the smart scheduling adds some overhead to all common operations (such
as every time a [system][cb::system] runs). In projects that have little work to
do every frame, especially if all of your systems complete very quickly, the
overhead can add up to overshadow the actual useful work you are doing!

You might want to try disabling multithreading, to see if your game might
perform better without it.

多线程,这样多个system就可以在多个cpu并行处理,配合system的`顺序约定`,
bevy就自动处理了多核调度.

bevy自动调度不是没有代价的,而且还很大,如果system都是执行简单任务,
多线程的收益不一定能覆盖住bevy自动调度的消耗.
换言之,system任务越复杂,多线程调度的收益越高.

### Disabling Multithreading for Update Schedule Only

Multithreading can be disabled per-[schedule][cb::schedule]. This means it
is easy to disable it only for your code / game logic (in the `Update` schedule),
while still leaving it enabled for all the Bevy engine internal systems.

This could speed up simple games that don't have much gameplay logic, while still
letting the engine run with multithreading.

You can edit the settings of a specific [schedule][cb::schedule] via the [app builder][cb::app]:

```rust,no_run,noplayground
{{#include ../code012/src/setup/perf.rs:singlethread-updateonly}}
```

bevy中有很多调度器,其中Update就是我们自定义的游戏逻辑,其他很多是bevy内置的system,
这里提供了一种方式只让Update调度器禁止使用多线程.这对简单项目是有性能提高的.

如上图所示,指定调度器类型为单线程即可.

### Disabling Multithreading Completely

If you want to try to completely disable multithreading for everything,
you can do so by removing the `multi-threaded` default Cargo feature.

In `Cargo.toml`

```toml
[dependencies.bevy]
version = "0.12"
default-features = false
features = [
   # re-enable everything you need, without `multi-threaded`
   # ...
]
```

[(see here for how to configure Bevy's cargo features)][cb::features]

This is generally not recommended. Bevy is designed to work with multithreading.
Only consider it if you really need it (like if you are making a special build
of your project to run on a system where it makes sense, like WASM or old
hardware).

如果要禁止所有system的多线程运行,通过features禁用`multi-threaded`即可.
这个一般出现在wasm或老旧硬件的项目中.

## Multithreading Configuration

You can configure how many CPU threads Bevy uses.

Bevy creates threads for 3 different purposes:
 - Compute: where all your systems and all per-frame work is run
 - AsyncCompute: for background processing independent from framerate
 - I/O: for loading of assets and other disk/network activity

By default, Bevy *splits/partitions* the available CPU threads as follows:
 - I/O: 25% of the available CPU threads, minimum 1, maximum 4
 - AsyncCompute: 25% of the available CPU threads, minimum 1, maximum 4
 - Compute: all remaining threads

This means *no overprovisioning*. Every hardware CPU thread is used
for one specific purpose.

This provides a good balance for mixed CPU workloads. Particularly for games
that load a lot of assets (especially if assets are loaded dynamically during
gameplay), the dedicated I/O threads will reduce stuttering and load times.
Background computation will not affect your framerate. Etc.

Examples:

|CPU Cores/Threads|# I/O|# AsyncCompute|# Compute|
|-----------------|-----|--------------|---------|
|1-3              |1    |1             |1        |
|4                |1    |1             |2        |
|6                |2    |2             |2        |
|8                |2    |2             |4        |
|10               |3    |3             |4        |
|12               |3    |3             |6        |
|16               |4    |4             |8        |
|24               |4    |4             |16       |
|32               |4    |4             |24       |

Note: Bevy does not currently have any special handling for asymmetric
(big.LITTLE or Intel P/E cores) CPUs. In an ideal world, maybe it would be nice
to use the number of big/P cores for Compute and little/E cores for I/O.

进一步还可以配置bevy能使用到的线程数,bevy创建的线程一般要干3件事:
 - 计算,所有的system和每帧前置的工作
 - 异步计算,独立于帧率的后台处理逻辑
 - io,资产加载或活跃的磁盘网络活动

默认bevy是按如下方式分配cpu线程的:
 - io: 25%可用线程数,最少1个,最多4个
 - 异步计算: 25%可用线程数,最少1个,最多4个
 - 计算:剩下线程数

这个配置很平衡,资源加载(大一点的游戏都是动态加载资源的)和io会减少游戏卡顿和加载时间,
后台的异步计算也不会受帧率影响,这是这么设计的考虑.

如上表所示: 8核的分配是2/2/4(对着表格看),24核的分配是4/4/16.

### Overprovisioning

However, if your game does very little I/O (asset loading) or background
computation, this default configuration might be sub-optimal. Those threads will
be sitting idle a lot of the time. Meanwhile, Compute, which is your frame
update loop and is important to your game's overall framerate, is limited to
fewer threads. This can be especially bad on CPUs with few cores (less than 4
total threads).

For example, in my projects, I usually load all my assets during a loading
screen, so the I/O threads are unused during normal gameplay. I rarely use
AsyncCompute.

If your game is like that, you might want to make all CPU threads available for
Compute. This could boost your framerate, especially on CPUs with few cores.
However, any AsyncCompute or I/O workloads during gameplay could impact your
game's performance / framerate consistency.

Here is how to do that:

```rust,no_run,noplayground
{{#include ../code012/src/setup/perf.rs:taskpool-overprovision}}
```

And here is an example of an entirely custom configuration:

```rust,no_run,noplayground
{{#include ../code012/src/setup/perf.rs:taskpool-custom}}
```

当然,如果我们的游戏在资产加载(io)和异步计算方面的任务非常少,默认配置就不是最优的,
在很多时间内,分配给io和异步计算的线程就是闲着的,如果cpu线程数多一点,
这点闲置对游戏还没啥大影响,如果cpu线程数很少(小于4个),这点浪费就是巨大的浪费.

eg:在屏幕加载画面中,资产已经加载完毕,这意味着没啥io,如果异步计算也少,
这种情况下,调整默认配置就是很有必要的.

## Pipelined Rendering

Bevy has a [pipelined rendering architecture][cb::render-architecture]. This
means Bevy's GPU-related [systems][cb::system] (that run on the CPU to prepare
work for the GPU every frame) will run in parallel with all the normal systems
for the next frame. Bevy will render the previous frame in parallel with the
next frame update.

This will improve GPU utilization (make it less likely the GPU will sit idle
waiting for the CPU to give it work to do), by making better use of CPU
multithreading. Typically, it can result in 10-30% higher framerate, sometimes
more.

However, it can also affect perceived input latency ("click-to-photon"
latency), often for the worse. The effects of the player's input might be
shown on screen delayed by one frame. It might be compensated by the faster
framerate, or it might not be. Here is a diagram to visualize what happens:

![Timeline comparing pipelined and non-pipelined rendering. In the pipelined
case, one additional frame is displayed before the effects of the mouse click
can be seen on-screen.](../img/pipelined-latency.png)

The actual mouse click happens in-between frames. In both cases, frame #4 is
when the input is detected by Bevy. In the pipelined case, rendering
of the previous frame is done in parallel, so an additional frame without
the input appears on-screen.

Without pipelining, the user will see their input delayed by 1 frame. With
pipelining, it will be delayed by 2 frames.

However, in the diagram above, the frame rate increase from pipelining is
big enough that overall the input is processed and displayed sooner. Your
application might not be so lucky.

bevy的渲染是管道式架构.GPU在显卡上,运行在cpu的system会将每帧要投递给GPU数据准备好,
GPU和CPU是并行运行的.这类system称为`GPU相关的system`,
其特点是当前帧的GPU system和下帧的普通system是并行执行的.

这种设计提高了GPU的利用率,配合多线程,总体能提高10-30%的帧率.

因为GPU system的实际上工作在下帧,在低延时的场景下, 对输入的感知会造成影响,
这是提高帧率的代价之一.高帧率比低帧率的影响小一些.

如上图所示,鼠标点击后,在4处bevy感知到了点击事件,
在没有pipeline机制时,在5处就通过画面反馈到玩家了,用了1帧;
在使用pipeline机制时,在6处才反馈到玩家,用了2帧.

在高帧率下,pipeline机制的代价会被收益覆盖,我们的程序具体是咋样的,
需要具体分析.

---

If you care more about latency than framerate, you might want to disable
pipelined rendering. For the best latency, you probably also want to
[disable VSync][cb::vsync].

Here is how to disable pipelined rendering:

```rust,no_run,noplayground
{{#include ../code012/src/setup/perf.rs:disable-pipelined-rendering}}
```

pipeline的开关如上图操作.

## Clustered Forward Rendering

By default, Bevy uses a Clustered Forward Rendering architecture for 3D.  The
viewport (on-screen area where the game is displayed) is split into
rectangles/voxels, so that the lighting can be handled separately for each small
portion of the scene. This allows you to use many lights in your 3D scenes,
without destroying performance.

The dimensions of these clusters can affect rendering performance. The default
settings are good for most 3D games, but fine-tuning them could improve
performance, depending on your game.

In games with a top-down-view camera (such as many strategy and simulation
games), most of the lights tend to be a similar distance away from the camera.
In such cases, you might want to reduce the number of Z slices (so that the
screen is split into smaller X/Y rectangles, but each one covering more
distance/depth):

```rust,no_run,noplayground
{{#include ../code012/src/setup/perf.rs:cluster-smallz}}
```

For games that use very few lights, or where lights affect the entire scene (
such as inside a small room / indoor area), you might want to try disabling
clustering:

```rust,no_run,noplayground
{{#include ../code012/src/setup/perf.rs:cluster-single}}
```

Changing these settings will probably result in bad performance for many games,
outside of the specific scenarios described above.

默认bevy在3d渲染中使用`集群向前渲染`,
视口（显示游戏的屏幕区域）被分割成矩形/体素，这样可以针对场景的每个小部分单独处理照明。
这样，可以在 3D 场景中使用许多灯光，而不会影响性能。

集群分组的尺寸会影响渲染性能,适合的设置会提升性能.
在采用自上而下视角的游戏中（例如许多策略和模拟游戏），大多数灯光与相机的距离都差不多。
在这种情况下，您可能需要减少 Z 切片的数量
(以便将屏幕分割成更小的 X/Y 矩形，但每个矩形覆盖更大的距离/深度).

***大多数场景下不需要这么设置,只在特殊场景下使用会提高性能.***

集群向前渲染是一种现代渲染技术.  
结合了传统的前向渲染（Forward Rendering）和集群（Clustered）分组的方法，
以更高效地处理复杂场景中的光源和阴影。
它在处理大量光源时比传统的前向渲染更高效，
并且在性能和灵活性上比延迟渲染（Deferred Rendering）有一些优势。

前向渲染（Forward Rendering）:  
传统的渲染技术，其中每个对象在渲染时直接应用所有影响它的光源。
这种方法在处理少量光源时比较高效，但随着光源数量的增加，性能会显著下降。

延迟渲染（Deferred Rendering）：  
延迟渲染首先渲染场景几何信息到多个缓冲区，然后在一个单独的光照阶段应用光源。
它能很好地处理大量光源，但需要更多的内存带宽和较复杂的后期处理。

集群分组（Clustered）：  
场景被分割成多个小的三维区域（集群）。
光源和物体的关系只在这些集群内进行计算，从而减少每个对象需要处理的光源数量。
这种方法提高了光源管理的效率。

Clustered Forward Rendering 的流程:  

空间划分：将视锥（视图体积）划分为多个集群，
这些集群通常是视锥内的固定大小的体积单元（如立方体）。

光源分配：遍历所有光源，并将每个光源分配到其影响的集群中。
每个光源的影响范围通过简单的几何计算确定，这样每个集群只会存储影响它的光源列表。

渲染阶段：在渲染每个像素时，首先确定该像素所在的集群，
然后仅应用该集群中的光源进行光照计算。
这减少了每个像素需要处理的光源数量，提高了渲染效率。

优点: 处理大量光源, 灵活, 性能优化.
