{{#include ../include/header09.md}}

# React to Changes with Asset Events

If you need to perform specific actions when an asset is created,
modified, or removed, you can make a [system][cb::system] that reacts to
[`AssetEvent`][bevy::AssetEvent] [events][cb::event].

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-event}}
```

**Note:** If you are handling `Modified` events and doing a mutable access to
the data, the `.get_mut` will trigger another `Modified` event for the same
asset. If you are not careful, this could result in an infinite loop! (from
events caused by your own system)

还可以使用事件来监听资产的创建/修改/删除.`AssetEvent<T>`.

如果在修改事件的处理中做了一个修改操作,会触发一个新的修改事件,
此处需要特别注意:别整出个无限循环来.
