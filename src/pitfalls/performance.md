{{#include ../include/header-none.md}}

# Performance

## Unoptimized debug builds

You can partially enable compiler optimizations in debug/dev mode!

You can enable higher optimizations for dependencies (incl. Bevy), but not
your own code, to keep recompilations fast!

In `Cargo.toml` or `.cargo/config.toml`:

```toml
# Enable max optimizations for dependencies, but not for our code:
[profile.dev.package."*"]
opt-level = 3
```

The above is enough to make Bevy run fast. It will only slow down clean
builds, without affecting recompilation times for your project.

If your own code does CPU-intensive work, you might want to also enable some
optimization for it.

```toml
# Enable only a small amount of optimization in debug mode
[profile.dev]
opt-level = 1
```

**Warning!** If you are using a debugger (like `gdb` or `lldb`) to step through
your code, any amount of compiler optimization can mess with the experience.
Your breakpoints might be skipped, and the code flow might jump around in
unexpected ways. If you want to debug / step through your code, you might want
`opt-level = 0`.

在调试模式和开发模式,可部分启用编译器优化.常用的是对依赖进行最高优化,
对自己的代码逻辑不进行优化,方便调试.在Cargo.toml中启用下面配置,
或是在.cargo/config.toml中进行全局启用.

```toml
# Enable max optimizations for dependencies, but not for our code:
[profile.dev.package."*"]
opt-level = 3
```

如果代码是cpu密集型,还可以进一步优化.(哪个游戏逻辑不是CPU密集型呢?)

```toml
# Enable only a small amount of optimization in debug mode
[profile.dev]
opt-level = 1
```
上面的优化在使用gdb/lldb的场景下`不适用`. 在gdb/lldb要想断点不乱,
需要是哦那个`opt-level = 0`.

### Why is this necessary?

Rust without compiler optimizations is *very slow*. With Bevy in
particular, the default cargo build debug settings will lead to *awful* runtime
performance. Assets are slow to load and FPS is low.

Common symptoms:
  - Loading high-res 3D models with a lot of large textures, from GLTF
    files, can take minutes! This can trick you into thinking
    that your code is not working, because you will not see anything on
    the screen until it is ready.
  - After spawning even a few 2D sprites or 3D models, framerate may drop
    to unplayable levels.

众所周知,没有编译器优化的rust程序非常慢,在bevy中,rust的默认编译配置太慢.
 - 从gltf文件中加载大量纹理的3d模型,要几分钟.
 - 在生成几个2d/3d精灵后,帧率会急速下降.

### Why not use `--release`?

You may have heard the advice: just run with `--release`! However, this is
bad advice. Don't do it.

Release mode also disables "debug assertions": extra checks useful during
development. Many libraries also include additional stuff under that
setting. In Bevy and WGPU that includes validation for shaders and GPU API
usage. Release mode disables these checks, causing less-informative crashes,
issues with hot-reloading, or potentially buggy/invalid logic going unnoticed.

Release mode also makes incremental recompilation slow. That negates
Bevy's fast compile times, and can be very annoying while you develop.

既然调试模式下性能差,为啥不直接使用release版本? 答案是不建议这么做.
因为rust的原因,发布版是减少很多检查为代价的,前置的debug检查都会被忽略,
可能会导致panic/热加载异常/错误逻辑/无效逻辑.
这么看来,release模式并不是覆盖debug模式的,而是debug严格检查模式排雷,
release模式丢弃检查来换取高性能.

再者,release的编译是全量编译,一般的bevy项目也要花很长时间,这个代价也很大.

---

With the advice at the top of this page, you don't need to build with
`--release`, just to test your game with adequate performance. You can use
it for *actual* release builds that you send to your users.

If you want, you can also enable LTO (Link-Time-Optimization) for the actual
release builds, to squeeze out even more performance at the cost of very
slow compile times.

Here is a configuration for the most aggressive optimizations possible:

```toml
[profile.release]
lto = true
opt-level = 3
codegen-units = 1
incremental = false
debug = false
```
当然,release版本下还可以使用LTO(链接时间优化)来加快release的构建.
