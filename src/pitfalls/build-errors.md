{{#include ../include/header013.md}}

# Strange Build Errors

Sometimes, you can get strange and confusing build errors when trying to
compile your project.

编译出现奇怪的错误,可按以下方式尝试解决.

## Update your Rust

First, make sure your Rust is up-to-date. Bevy only supports the latest
stable version of Rust, or nightly.

If you are using [`rustup`][rustup] to manage your Rust installation, you
can run:

```shell
rustup update
```

更新rust版本,因为bevy只支持最新rust版本.

## Clear the cargo state

Many kinds of build errors can often be fixed by forcing `cargo` to regenerate
its internal state (recompute dependencies, etc.). You can do this by deleting
the `Cargo.lock` file and the `target` directory.

```shell
rm -rf target Cargo.lock
```

Try building your project again after doing this. It is likely that the
mysterious errors will go away.

If not, another reason might be that you have multiple versions of Bevy if
your dependency tree. If you are using 3rd-party plugins, make sure you have
specified the correct versions of all the plugins you use and that they are
compatible with the Bevy version you are using.

If none of this helps you, your issue might require further
investigation. Reach out to the Bevy community via GitHub or
[Discord][bevy::discord], and ask for help.

If you are using bleeding-edge Bevy ("main"), and the above does not solve
the problem, your errors might be caused by 3rd-party plugins. See [this
page](../setup/bevy-git.md#how-to-use-bleeding-edge-bevy) for solutions.

{{#include ../include/resolver2.md}}

很多编译错误都是因为cargo内部状态导致的,清理之后重新生成可解决大部分问题.
`rm -rf target Cargo.lock` 之后重新编译即可.
如果这步还不行,就需要排除多bevy版本的问题,特别是使用第三方插件时,
需要注意多bevy版本的问题.

如果到这步还不能解决问题,需要到社区寻找方法.

---

新的cargo解析器

Cargo 最近添加了一种新的依赖解析器算法，该算法与旧算法不兼容。 Bevy 需要新的解析器。
如果要新建项目,使用cargo new, 这样就启用了新的解析算法.

如果项目是单包项目(项目中只有一个Cargo.toml),下面两个设置都会自动使用新的解析算法:

```toml
[package]
edition = "2021"

# 或是,二选一即可.
[package]
resolver = "2"
```

如果项目是多包项目工作空间,在workspace的顶级Cargo.toml中要添加:

```toml
[workspace]
resolver = "2"
```
