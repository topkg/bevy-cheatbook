{{#include ../include/header014.md}}

# Plugins

Relevant official examples:
[`plugin`][example::plugin],
[`plugin_group`][example::plugin_group].

---

As your project grows, it can be useful to make it more modular. You can
split it into "plugins".

Plugins are simply collections of things to be added to the [App
Builder][cb::app]. Think of this as a way to add things to the app from
multiple places, like different Rust files/modules or crates.

The simplest way to create a plugin is by just writing a Rust function
that takes [`&mut App`][`App`]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/plugins.rs:plugin-fn}}
```

An alternative way is by creating a `struct` and implementing the [`Plugin`] trait:

```rust,no_run,noplayground
{{#include ../code014/src/programming/plugins.rs:plugin-struct}}
```

The benefit of using a `struct` is that you could extend it with configuration
parameters or generics if you want to make your plugin configurable.

Either way, you get `&mut` access to the [`App`], so you can add whatever
you want to it, just like you can do from your `fn main()`.

You can now add your plugins to your [`App`] from elsewhere (most commonly
`fn main()`). Bevy will just call your plugin implementation above. In effect,
everything the plugin adds will be flattened into your [`App`] alongside
everything that is already there.

```rust,no_run,noplayground
{{#include ../code014/src/programming/plugins.rs:plugin-app}}
```

For internal organization in your own project, the main value of plugins
comes from not having to declare all your Rust types and functions as
`pub`, just so they can be accessible from `fn main` to be added to the
app builder. Plugins let you add things to your [app][cb::app] from multiple
different places, like separate Rust files / modules.

You can decide how plugins fit into the architecture of your game.

Some suggestions:
 - Create plugins for different [states][cb::state].
 - Create plugins for various sub-systems, like physics or input handling.

随着项目的增长,需要考虑模块化,此时需要用到`插件`,将部分逻辑拆分到插件中.
最简单的插件实现就是一个函数.其次是用结构体实现`Plugin`特性.

为了追求灵活性,struct实现的插件,可以接受一些配置参数.
之前很多篇幅中都有使用到这点,eg:设置背景色,默认插件列表中有大部分都是可配置插件.

虽然在任何地方都可以在app中添加插件,不过在main中处理这些是主流方案.
在组织项目时,插件的主要值的类型需要用pub声明.

下面是使用插件的建议:
 - 不同状态使用不同的插件
 - 不同的子system使用不同插件,eg:输入/物理部分

## Plugin Groups

Plugin groups register multiple plugins at once.  Bevy's [`DefaultPlugins`]
and [`MinimalPlugins`] are examples of this.

To create your own plugin group, implement the [`PluginGroup`] trait:

```rust,no_run,noplayground
{{#include ../code014/src/programming/plugins.rs:plugin-groups}}
```

When adding a plugin group to the [app][cb::app], you can disable some
plugins while keeping the rest.

For example, if you want to manually set up logging (with your own `tracing`
subscriber), you can disable Bevy's [`LogPlugin`]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/plugins.rs:plugin-groups-disable}}
```

Note that this simply disables the functionality, but it cannot actually
remove the code to avoid binary bloat. The disabled plugins still have to
be compiled into your program.

If you want to slim down your build, you should look at disabling Bevy's
default [cargo features][cb::features], or depending on the various Bevy
sub-crates individually.

插件组,一次性注册多个插件.方便不同场景下使用,免得每新建一个项目就需要注册很多插件,
这是用户友好的表现,bevy内置了两个常用的插件组:DefaultPlugins/MinimalPlugins.

如果要实现自己的插件组,只需要实现PluginGroup特型即可.这应该是一个比较常用的功能.
在app处添加插件组时还能disable部分插件(这种方式只是屏蔽了特定的插件,
不能减少二进制大小).

如果实在要减少二进制大小,需要在cargo功能处做disable,或者将子包独立,从依赖出着手.

## Plugin Configuration

Plugins are also a convenient place to store settings/configuration that are
used during initialization/startup. For settings that can be changed at runtime,
it is recommended that you put them in [resources][cb::res] instead.

```rust,no_run,noplayground
{{#include ../code014/src/programming/plugins.rs:plugin-config}}
```

Plugins that are added using [Plugin Groups][cb::plugingroup] can also be
configured. Many of Bevy's [`DefaultPlugins`] work this way.

```rust,no_run,noplayground
{{#include ../code014/src/programming/plugins.rs:defaultplugins-config}}
```

插件在初始化或启动时可以配置(struct插件才行,这也是大部分插件的选择).
运行时也可以修改,不过推荐将配置放在资源中(这种场景应该非常少见).

ps:bevy还是有点理想主义的,各个包和机制都设计得非常灵活,但实际场景下,
绝大部分只使用到了其中几条,这点在bevy的很多地方都都体现了:eg:这儿+ecs.

## Publishing Crates

Plugins give you a nice way to publish Bevy-based libraries for other people
to easily include into their projects.

Bevy offers some official guidance for good practices when you develop plugins
you want to publish for other people to use. [You can read it here.][bevy::plugin-guidelines]

Don't forget to submit an entry to [Bevy Assets][bevyassets] on the official
website, so that people can find your plugin more easily. You can do this
by making a PR in [the Github repo][project::bevyassets].

If you are interested in supporting bleeding-edge Bevy (main), [see here
for advice][cb::git-plugins].

插件作为模块化分割了逻辑,就能分享给他人.
