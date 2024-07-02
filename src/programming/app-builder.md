{{#include ../include/header014.md}}

# The App

Relevant official examples: All of them ;)

In particular, check out the complete game examples:
[`alien_cake_addict`][example::alien_cake_addict],
[`breakout`][example::breakout].

---

To enter the Bevy runtime, you need to configure an [`App`]. The app is how you
define the structure of all the things that make up your project:
[plugins][cb::plugin], [systems][cb::system] (and their configuration/metadata:
[run conditions][cb::rc], [ordering][cb::system-order], [sets][cb::systemset]),
[event][cb::event] types, [states][cb::state], [schedules][cb::schedule]…

You typically create your [`App`] in your project's `main` function.  However,
you don't have to add everything from there. If you want to add things to your
app from multiple places (like other Rust files or crates), use
[plugins][cb::plugin]. As your project grows, you will need to do that to keep
everything organized.

```rust,no_run,noplayground
{{#include ../code014/src/programming/app_builder.rs:main}}
```

Note: use tuples with `add_systems`/`add_plugins`/`configure_sets` to add
multiple things at once.

[Component][cb::component] types do not need to be registered.

Schedules cannot (yet) be modified at runtime; all [systems][cb::system] you
want to run must be added/configured in the [`App`] ahead of time. You can
control individual systems using [run conditions][cb::rc]. You can also
dynamically enable/disable entire schedules using the [`MainScheduleOrder`]
[resource][cb::res].

app是组织项目的起点,通过app可以配置`插件`/`system(包括其运行条件/顺序/集合)`/
`事件`/`状态`/`调度`等等.

利用app可以按需添加功能,当项目增长时需要考虑如何组织代码,`插件`是最好的选择.

app的几个`add_函数`都是支持元组类型的,也就是说一次可以添加多个对象.
app不需要注册组件类型.

调度在运行期间不能修改,所以需要通过app明确配置完.
单个system是可以配置运行条件的;
利用`MainScheduleOrder`资源可以动态开启/关闭某个调度(这个是个高级用法,很少用到).

## Builtin Bevy Functionality

The Bevy game engine's own functionality is represented as a [plugin group][cb::plugingroup].
Every typical Bevy app must first add it, using either:
 - [`DefaultPlugins`] if you are making a full game/app.
 - [`MinimalPlugins`] for something like a headless server.

bevy游戏引擎自己的逻辑使用`插件组`来表示,bevy提供了两个默认的功能组:
 - DefaultPlugins,默认插件列表
 - MinimalPlugins,最小插件列表

## Setting up data

Normally, you can set up [your data][cb::ecs-intro-data] from
[systems][cb::system]. Use [Commands][cb::commands] from regular systems, or
use [exclusive systems][cb::exclusive] to get [full World access][cb::world].

Add your setup systems to the [`Startup`] [schedule][cb::schedule] for
things you want to initialize at launch, or use [state][cb::state] enter/exit
systems to do things when transitioning between menus, game modes, levels, etc.

However, you can also initialize data directly from the app builder. This
is common for [resources][cb::res], if they need to be present at all
times. You can also get [direct World access][cb::world].

```rust,no_run,noplayground
{{#include ../code014/src/programming/app_builder.rs:world}}
```
构造实体有以下几种方式:
 - 在普通system中利用Commands实现
 - 使用独占system来访问world的所有数据
 - 在Startup调度中使用构造system来实现
 - 在翻译菜单/游戏模型/等级时利用state在system开头或退出时实现
 - 在app中直接使用world来构造

通常利用app来直接初始化数据(资源,在剩下的游戏时间中都能被访问).

## Quitting the App

To cleanly shut down bevy, send an [`AppExit`] [event][cb::event] from any
[system][cb::system]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/app_builder.rs:appexit}}
```

You can specify the exit code to return to the OS. If Bevy receives
multiple [`AppExit`] events, success will only be returned if all
of them report success. If some report an error, the last event will
determine the actual exit code of the process.

退出bevy使用,在任意system中发出`AppExit`事件即可,
同时可以带上返回码.
