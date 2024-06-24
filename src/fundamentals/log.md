{{#include ../include/header012.md}}

# Logging, Console Messages

Relevant official examples:
[`logs`][example::logs].

---

You may have noticed how, when you run your Bevy project, you get messages
in your console window. For example:

```
2022-06-12T13:28:25.445644Z  WARN wgpu_hal::vulkan::instance: Unable to find layer: VK_LAYER_KHRONOS_validation
2022-06-12T13:28:25.565795Z  INFO bevy_render::renderer: AdapterInfo { name: "AMD Radeon RX 6600 XT", vendor: 4098, device: 29695, device_type: DiscreteGpu, backend: Vulkan }
2022-06-12T13:28:25.565795Z  INFO mygame: Entered new map area.
```

Log messages like this can come from Bevy, dependencies (like wgpu), and
also from your own code.

Bevy offers a logging framework that is much more advanced than simply using
`println`/`eprintln` from Rust. Log messages can have metadata, like the
level, timestamp, and Rust module where it came from. You can see that this
metadata is printed alongside the contents of the message.

This is set up by Bevy's [`LogPlugin`][bevy::LogPlugin]. It is part of the
[`DefaultPlugins`][bevy::DefaultPlugins] plugin group, so most Bevy users
will have it automatically in every typical Bevy project.

上图中的日志来自于bevy,确切说是wgpu.
相比rust提供的println/eprintln宏,bevy提供了一个高级的日志库.
DefaultPlugins插件列表中包含了LogPlugin插件.

## Levels

Levels determine how important a message is, and allow messages to be filtered.

The available levels are: `off`, `error`, `warn`, `info`, `debug`, `trace`.

A rough guideline for when to use each level, could be:
 - `off`: disable all log messages
 - `error`: something happened that prevents things from working correctly
 - `warn`: something unusual happened, but things can continue to work
 - `info`: general informational messages
 - `debug`: for development, messages about what your code is doing
 - `trace`: for very verbose debug data, like dumping values

日志等级,off表示不要日志,剩下的依次丰富.

## Printing your own log messages

To display a message, just use the macro named after the level of the
message. The syntax is exactly the same as with Rust's `println`. See the
[`std::fmt`][std::fmt] documentation for more details.

```rust
error!("Unknown condition!");
warn!("Something unusual happened!");
info!("Entered game level: {}", level_id);
debug!("x: {}, state: {:?}", x, state);
trace!("entity transform: {:?}", transform);
```

使用也非常简单,使用宏即可.

## Filtering messages

To control what messages you would like to see, you can configure Bevy's
[`LogPlugin`][bevy::LogPlugin]:

```rust
{{#include ../code012/src/fundamentals/log.rs:log-settings}}
```

The `filter` field is a string specifying a list of rules for what level to
enable for different Rust modules/crates. In the example above, the string
means: show up to `info` by default, limit `wgpu_core` and `wgpu_hal`
to `warn` level, for `mygame` show `debug`.

All levels higher than the one specified are also enabled. All levels lower
than the one specified are disabled, and those messages will not be displayed.

The `level` filter is a global limit on the lowest level to use. Messages
below that level will be ignored and most of the performance overhead avoided.

filter可进一步指定各个模块的日志等级.

### Environment Variable

You can override the filter string when running your app, using the `RUST_LOG`
environment variable.

```sh
RUST_LOG="warn,mygame=debug" ./mygame
```

Note that other Rust projects, such as `cargo`, also use the same
environment variable to control their logging. This can lead to unexpected
consequences. For example, doing:

```sh
RUST_LOG="debug" cargo run
```

will cause your console to also be filled with debug messages from `cargo`.

不想代码指定日志等级,也可以使用环境变量RUST_LOG来指定.不推荐这种方式.

### Different settings for debug and release builds

If you want to do different things in your Rust code for debug/release
builds, an easy way to achieve it is using conditional compilation on
"debug assertions".

```rust
{{#include ../code012/src/fundamentals/log.rs:log-settings-debugrelease}}
```

This is a good reason why [you should not use release mode during development
just for performance reasons][pitfall::perf].

On Microsoft Windows, your game EXE will also launch with a console window for
displaying log messages by default. You might not want that in release builds.
[See here.][platform::windows::noconsole]

使用条件编译来分别控制debug/release的不同日志等级.这也是提高性能的一种方式.

windows中,你肯定不想release版本弹出一个命令窗口来吧.

## Performance Implications

Printing messages to the console is a relatively slow operation.

However, if you are not printing a large volume of messages, don't worry
about it. Just avoid spamming lots of messages from performance-sensitive
parts of your code like inner loops.

You can disable log levels like `trace` and `debug` in release builds.

控制台打印日志是非常慢的操作.
如果不是大量打印日志,这点影响不算什么.
循环多的地方,就是性能敏感的地方,不要在这里打大量的日志.

release版本可是从info级别开始打起.
