{{#include ../include/header013.md}}

# System Sets

System Sets allow you to easily apply common properties to multiple systems,
such as [ordering][cb::system-order] and [run conditions][cb::rc].

Anything you add to the set will automatically be applied to all systems
belonging to the set.

A system can belong to multiple different sets, and will inherit all the
properties from all of them. You can also add additional properties to
individual systems.

All of this combined gives you a lot of flexibility and control over how your systems run.

在顺序约束和条件运行中都涉及到system集合,这个集合是组织system的一种底层次方式.

任何新增到集合中的system都会自动应用集合的约束.

一个system可以属于多个set,继承所有set的约束.也可单独为system添加属性.

## Anonymous Sets

The simplest kind of system set is when you just [add][cb::app] a tuple of
multiple [systems][cb::system] using `.add_systems`.

```rust,no_run,noplayground
{{#include ../code013/src/programming/system_sets.rs:anonymous}}
```

This syntax is useful when you just want to apply some common configuration to
multiple systems.

匿名集合,用元组.

## Named Sets

This is the more formal and powerful way to use system sets.

You can create a Rust type (`struct` or `enum`) to serve as a label/identifier,
so you can refer to the set from different places.

For a single set, create an empty `struct`. If you need to create multiple
related sets, create an `enum`. Every variant of the `enum` is a separate system
set.

You need to derive [`SystemSet`] + an assortment of required standard Rust traits:

```rust,no_run,noplayground
{{#include ../code013/src/programming/system_sets.rs:definition}}
```

Now, you can apply the set to your systems using `.in_set()`:

```rust,no_run,noplayground
{{#include ../code013/src/programming/system_sets.rs:app}}
```

You can add [run conditions][cb::rc] and [ordering
dependencies][cb::system-order] on your set using `.configure_sets`:

```rust,no_run,noplayground
{{#include ../code013/src/programming/system_sets.rs:configure}}
```

The main use case of named system sets is for logical organization, so that you
can manage your systems and refer to the whole group.

Some examples:
 - A set for all your audio-related systems, so you can disable them all if sound is muted.
 - A set for all your touchscreen input systems, so you can disable them all if there is no touchscreen.
 - A set for all your input handling systems, so you can order them to run before gameplay systems.
 - A set for all your gameplay systems, so that they only run during the in-game [state][cb::state].

具名集合.使用SystemSet特型指明,具体类型可以是struct/enum.
枚举的单个类型都是一个新的system集合.

`in_set`将system集合的system提取出来,扁平化处理后添加到某个地方.
运行条件和顺序约束的应用和普通system没啥区别.

具名集合的主要用途是组织.下面都是使用例子:
- 所有audio相关的system放一起,这样方便实现mute
- 触摸屏输入的sytem放一起,方便对触摸屏输入功能进行开关
- 所有的输入处理system放一起,这样可以让她们在游戏逻辑前面运行
- 所有游戏逻辑放在一起,这样可以确保只在某个游戏状态时运行

### With Plugins

Named sets are also very useful together with [plugins][cb::plugin]. When you are writing
a plugin, you can expose (make `pub`) some system set types, to allow users of your
plugin to control how things in your plugin run, or how their things run in relation to
your plugin. This way, you don't have to expose any of your individual systems.

Some examples:
 - You are making a physics plugin. Make a set for your whole plugin, so your users can
   easily order their systems to run before/after physics. They can also easily control
   whether your physics runs at all, by adding an extra run condition to your set.
 - You are using plugins for internal organization in your project. You have an UI plugin.
   Create a system set for the systems that need to update UI state from gameplay state,
   so that you can easily add ordering dependencies between UI and gameplay. Other plugins
   / places in your code now don't need to know about the internals of your UI plugin.

具名集合和插件配合使用.对于某个插件包的提供者来说,
对外暴露system集合好过单独暴露多个sytem.

## Common Pitfalls

WARNING! System set configuration is stored *per-[schedule][cb::schedule]!* Notice how
we had to specify `.configure_sets(Update, ...)`. It can be very easy to configure your
sets once and then just assume you can use them anywhere, but that is not true.

If you try to use them in a [schedule][cb::schedule] other than the one
where you configured them, your code will compile and run (Bevy silently
initializes the sets in each schedule), but will not work correctly, as they
will not have any of your configurations.

Some common scenarios where this can occur:
 - You configure your set in [`Update`] and try to also use it in [`FixedUpdate`], or vice versa.
 - You try to use your sets in the [`OnEnter`]/[`OnExit`] schedules of various [app states][cb::state].
 - You add a system to [`PostUpdate`] or [`PreUpdate`].

常见陷阱.

配置的system集合存储在调度中,这个集合可以用在多个调度中,
要注意:每个调度中的集合都需要配置,不然就是默认配置.

如果集合用在Update中,又用在FixedUpdate中,记住两个地方都需要配置.
如果OnEnter/OnExit调度中都用到某个集合,记住两个地方都需要配置.
一个system添加到PostUpdate/PreUpdate调度中,记住两个地方都需要配置.
