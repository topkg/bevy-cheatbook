{{#include ./include/header09.md}}

# Bevy Asset Management

Assets are the data that the game engine is working with: all of your images,
3D models, sounds, scenes, game-specific things like item descriptions,
and more!

Bevy has a flexible system for loading and managing your game assets
asynchronously (in the background, without causing lag spikes in your game).

In your code, you refer to individual assets using [handles][cb::handle].

Asset data can be [loaded from files][cb::assetserver] and also [accessed from
code][cb::asset-data]. [Hot-reloading][cb::asset-hotreload] is supported to
help you during development, by reloading asset files if they change while the
game is running.

If you want to write some code to do something when assets finish loading, get
modified, or are unloaded, you can use [asset events][cb::assetevent].

游戏引擎会用到的东西,如所有的图片,3d模型,声音,场景,游戏特定数据(物品描述)等等,
这些都是资产.

bevy加载管理资产非常灵活,支持异步.
bevy使用`handles`来引用不同的资源,术语是`句柄`,一个轻量级的ID.

资产是从文件中加载,在代码中访问,开发阶段还支持`热加载`,
游戏运行过程中,如果资产发生了变化,还可以进行`reload`.

通过资产事件,在资产加载完毕后可进行修改和卸载.
