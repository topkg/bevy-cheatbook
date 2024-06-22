{{#include ../include/header013.md}}

# Dev Tools and Editors for Bevy

Bevy does not yet have an official editor or other such tools. An official
editor is planned as a long-term future goal. In the meantime, here are
some community-made tools to help you.

bevy官方没有编辑器或类似工具,但社区提供了.

---

## Editor

[`bevy_inspector_egui`][project::bevy_inspector_egui] gives you a simple
editor-like property inspector window in-game. It lets you modify the values of
your components and resources in real-time as the game is running.

`bevy_inspector_egui`在游戏中提供了一个类似编辑器的属性检查器窗口,
可以实时修改组件/资源的值.目前有3k+项目在使用这个工具.

[`bevy_editor_pls`][project::bevy_editor_pls] is an editor-like interface that
you can embed into your game. It has even more features, like switching app
states, fly camera, performance diagnostics, and inspector panels.

`bevy_editor_pls`提供了类似编辑器的接口,可内嵌到游戏中,
调试功能特别多:切换app状态,相机fly,性能诊断,平面检查. 
目前有500+项目在使用.

[`space_editor`][project::space_editor] is another such editor that can be
embedded into your game. It seems to be designed for a Unity-inspired prefab
workflow.

`space_editor`是又一个可以内嵌到游戏中的编辑器,受unity的启发而产生的项目.

You can also use [Blender][project::blender] as a level/scene editor,
by exporting your scenes to [GLTF][cb::gltf]. The [Blender Bevy Components
Workflow][project::blender_bevy_components_workflow] project improves on this
experience, by allowing you to setup your Bevy ECS [Components][cb::component]
in Blender, include them in the exported GLTF, and use them in Bevy.

`blender`也可用作等级/场景编辑器,适用于将scene导出到gltf的场景.

blender是一个开源的三维计算机图形软件,广泛用于动画制作,视觉特效,建模,渲染和游戏开发等领域.
blender停止了自己的游戏引擎开发,但支持导出数据给其他游戏引擎使用.

## Diagnostics

[`bevy_mod_debugdump`][project::bevy_mod_debugdump] is a tool to help visualize
your [App Schedules](../programming/app-builder.md) (all of the registered
[systems](../programming/systems.md) with their [ordering
dependencies](../programming/system-order.md)), and the Bevy Render Graph.

`bevy_mod_debugdump`是调度可视化的诊断工具,还包含了bevy的渲染图诊断.
