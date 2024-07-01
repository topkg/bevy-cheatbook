{{#include ../include/header09.md}}

# Track Loading Progress

There are good community plugins that can help with this. Otherwise, this page
shows you how to do it yourself.

---

If you want to check the status of various [asset files][cb::assetserver],
you can poll it from the [`AssetServer`][bevy::AssetServer]. It will tell you
whether the asset(s) are loaded, still loading, not loaded, or encountered
an error.

To check an individual asset, you can use `asset_server.get_load_state(…)` with
a handle or path to refer to the asset.

To check a group of many assets, you can add them to a single collection
(such as a `Vec<HandleUntyped>`; [untyped handles][cb::handleuntyped] are very
useful for this) and use `asset_server.get_group_load_state(…)`.

---

Here is a more complete code example:

```rust,no_run,noplayground
{{#include ../code/examples/assets-ready.rs:example}}
```

本章介绍的自己实现资产加载进度,社区已经有了很多插件实现了这一功能.

如果想检查多个资产文件的加载进度,从asset server拉取,
状态大概有:已加载;加载中;未加载;出现错误.

如果想检查单个资产文件的加载进度,使用`get_load_state()`,入参是句柄或资产路径.

如果想要检查一组资产的状态(很多文件),将所有句柄放在`Vec<HandleUntyped>`中,
然后传给`get_group_load_state()`. 无类型句柄在此处也起了很大的作用.
