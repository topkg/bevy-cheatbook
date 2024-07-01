{{#include ../include/header09.md}}

# Access the Asset Data

To access the actual asset data from systems, use the
[`Assets<T>`][bevy::Assets] [resource][cb::res].

You can identify your desired asset using the [handle][cb::handle].

[untyped handles][cb::handleuntyped] need to be "upgraded" into typed handles.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-access}}
```

在system中访问实际的资产数据,使用`Assets<T>`资源,配合句柄就能锚定具体的资产.
使用无类型句柄时,需要先升级到具体某个类型的句柄.

使用Option来处理资产没有加载完毕的问题.

## Creating Assets from Code

You can also add assets to [`Assets<T>`][bevy::Assets] manually.

Sometimes you need to create assets from code, rather than [loading them
from files][cb::assetserver]. Some common examples of such use-cases are:
  - creating texture atlases
  - creating 3D or 2D materials
  - procedurally-generating assets like images or 3D meshes

To do this, first create the data for the asset (an instance of the
[asset type][builtins::asset]), and then add it `.add(…)` it to the
[`Assets<T>`][bevy::Assets] resource, for it to be stored and tracked by
Bevy. You will get a [handle][cb::handle] to use to refer to it, just like
any other asset.

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-add}}
```

代码中创建资产.常用场景有:
 - 创建纹理
 - 创建2d/3d材质
 - 生成图片或3d网格

生成一个资产实例,并调用 `Assets<T>.add()`,之后bevy就能跟踪次资产了.
