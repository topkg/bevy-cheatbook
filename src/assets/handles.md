{{#include ../include/header09.md}}

# Handles

Handles are lightweight IDs that refer to a specific asset. You need them
to use your assets, for example to [spawn entities][cb::commands] like
[2D sprites][cb::sprite] or [3D models][cb::gltf], or to [access the data
of the assets][cb::asset-data].

Handles have the Rust type [`Handle<T>`][bevy::Handle], where `T` is the
[asset type][builtins::asset].

You can store handles in your [entity components][cb::component] or
[resources][cb::res].

Handles can refer to not-yet-loaded assets, meaning you can just spawn your
entities anyway, using the handles, and the assets will just "pop in" when
they become ready.

资产的句柄是一个轻量级ID,这个ID会指向真正的资产数据,
在实体构造中会使用到句柄(eg:2d精灵/3d模型).

`Handle<T>`是句柄,T为资产类型.
组件/资源都可以存储句柄.

有了句柄后,实体构造时就可以使用句柄,实际的资产可以在后面加载.

## Obtaining Handles

If you are [loading an asset from a file][cb::assetserver], the
`asset_server.load(…)` call will give you the handle. The loading of the
data happens in the background, meaning that the handle will initially refer
to an unavailable asset, and the actual data will become available later.

If you are [creating your own asset data from code][cb::asset-data::add],
the `assets.add(…)` call will give you the handle.

如何获取句柄:`asset_server.load(…)`从文件加载资产时会得到句柄,
实际的加载过程是异步的.

还有一种方式是从代码中创建资产,`assets.add()`.

## Reference Counting; Strong and Weak Handles

Bevy keeps track of how many handles to a given asset exist at any time. Bevy
will automatically unload unused assets, after the last handle is dropped.

For this reason, creating additional handles to the same asset requires you
to call `handle.clone()`. This makes the operation explicit, to ensure you are
aware of all the places in your code where you create additional handles. The
`.clone()` operation is cheap, so don't worry about performance (in most cases).

There are two kinds of handles: "strong" and "weak". Strong assets are
counted, weak handles are not. By default, handles are strong. If you want
to create a weak handle, use `.clone_weak()` (instead of `.clone()`) on an
existing handle. Bevy can unload the asset after all strong handles are gone,
even if you are still holding some weak handles.

句柄可以clone,所以就有了引用计数器,当所有的引用都释放后,bevy会自动卸载不使用的资产,
`clone`克隆的是强句柄,`clone_weak`克隆的是弱句柄,差别在与弱句柄没有引用计数器,
也不关心资产是否是已加载的.

## Untyped Handles

Bevy also has a [`HandleUntyped`][bevy::HandleUntyped] type. Use this type
of handle if you need to be able to refer to any asset, regardless of the
asset type.

This allows you to store a collection (such as [`Vec`] or [`HashMap`])
containing assets of mixed types.

You can create an untyped handle using `.clone_untyped()` on an existing
handle.

Just like regular handles, untyped handles can be strong or weak.

You need to do this to [access the asset data][cb::asset-data].

You can convert an untyped handle into a typed handle with `.typed::<T>()`,
specifying the type to use. You need to do this to [access the asset
data][cb::asset-data].

如果要引用任意类型的资产,可以使用无类型句柄`HandleUntyped`,
`clone_untyped`就能创建无类型句柄,`type::<T>`转换成有类型的.
