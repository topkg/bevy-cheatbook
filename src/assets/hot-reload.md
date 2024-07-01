{{#include ../include/header09.md}}

# Hot-Reloading Assets

Relevant official examples:
[`hot_asset_reloading`][example::hot_asset_reloading].

---

At runtime, if you modify the file of an [asset][cb::asset]
that is [loaded][cb::assetserver] into the game (via the
[`AssetServer`][bevy::AssetServer]), Bevy can detect that and reload the
asset automatically. This is very useful for quick iteration. You can edit
your assets while the game is running and see the changes instantly in-game.

Not all [file formats][builtins::file-formats] and use cases are supported
equally well. Typical asset types like textures / images should work without
issues, but complex GLTF or scene files, or assets involving custom logic,
might not.

If you need to run custom logic as part of your hot-reloading
workflow, you could implement it in a [system][cb::system], using
[`AssetEvent`][bevy::AssetEvent] ([learn more][cb::assetevent]).

Hot reloading is opt-in and has to be enabled in order to work:

```rust,no_run,noplayground
{{#include ../code/src/basics.rs:asset-watch}}
```

Note that this requires the `filesystem_watcher` [Bevy cargo
feature][cb::features]. It is enabled by default, but if you have disabled
default features to customize Bevy, be sure to include it if you need it.

运行时修改了资产,bevy会检测到并自动进行reload,前提时开启资源变更监听.

对于纹理/图片来说,reload并没有什么问题,但其他格式可能会出现异常,
复杂的gltf/场景文件/自定义逻辑涉及的资源.

```rust
pub struct AssetPlugin {
    pub file_path: String,
    pub processed_file_path: String,
    pub watch_for_changes_override: Option<bool>, // 0.14该为此字段了
    pub mode: AssetMode,
}

impl Default for AssetPlugin {
    fn default() -> Self {
        Self {
            mode: AssetMode::Unprocessed,
            file_path: Self::DEFAULT_UNPROCESSED_FILE_PATH.to_string(),
            processed_file_path: Self::DEFAULT_PROCESSED_FILE_PATH.to_string(),
            watch_for_changes_override: None, // 默认没有开启监听,需要注册插件时指定.
        }
    }
}
```

---

现在是v0.14,对应的变更监听功能涉及两个:  
embedded_watcher 内存资产变更监听;file_watcher 文件资产变更监听.

---

## Shaders

Bevy also supports hot-reloading for shaders. You can edit your custom shader
code and see the changes immediately.

This works for any shader loaded from a file path, such as shaders specified
in your Materials definitions, or shaders [loaded][cb::assetserver] via the
[`AssetServer`][bevy::AssetServer].

Shader code that does not come from asset files, such as if you include it
as a static string in your source code, cannot be hot-reloaded (for obvious
reasons).

着色器也支持热加载,不管是通过材质定义的着色器还是asset server定义的着色器都一样.
