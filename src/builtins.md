{{#include ./include/header011.md}}

# List of Bevy Builtins

This page is a quick condensed listing of all the important things provided
by Bevy.

 - [SystemParams](#systemparams)
 - [Assets](#assets)
 - [File Formats](#file-formats)
 - [GLTF Asset Labels](#gltf-asset-labels)
 - [Shader Imports](#shader-imports)
 - [`wgpu` Backends](#wgpu-backends)
 - [Schedules](#schedules)
 - [Run Conditions](#run-conditions)
 - [Plugins](#plugins)
 - [Bundles](#bundles)
 - [Resources (Configuration)](#configuration-resources)
 - [Resources (Engine User)](#engine-resources)
   - [Main World](#engine-resources)
   - [Render World](#render-world-resources)
   - [Low-Level `wgpu` access](#low-level-wgpu-resources)
 - [Resources (Input)](#input-handling-resources)
 - [Events (Input)](#input-events)
 - [Events (Engine)](#engine-events)
 - [Events (System/Control)](#system-and-control-events)
 - [Components](#components)

 这里是对bevy重要概念的简要描述.

## SystemParams

These are all the special types that can be used as [system][cb::system] parameters.

system参数,这里列出的都是特定的system参数.

{{#include ./include/builtins.md:systemparams}}

## Assets

[(more info about working with assets)][cb::asset]

These are the Asset types registered by Bevy by default.

这里的asset类型(资产类型)都是bevy默认注册的.

{{#include ./include/builtins.md:assets}}

## File Formats

These are the asset file formats (asset loaders) supported by Bevy. Support
for each one can be enabled/disabled using [cargo features][cb::features]. Some
are enabled by default, many are not.

以下是bevy支持的asset资源文件格式,默认启用了一批,通过`功能`开启其他的.

{{#include ./include/builtins.md:file-formats}}

There are unofficial plugins available for adding support for even more file formats.

还有非官方的插件支持了不少其他文件格式.

## GLTF Asset Labels

[Asset path labels to refer to GLTF sub-assets.][cb::gltf-asset-path]

{{#include ./include/builtins.md:gltf-asset-labels}}

## Shader Imports

TODO

## `wgpu` Backends

{{#include ./include/builtins.md:wgpu-backends}}

## Schedules

{{#include ./include/builtins.md:schedules}}

{{#include ./include/builtins.md:render-sets}}

## Run Conditions

TODO

## Plugins

TODO

## Bundles

Bevy's built-in [bundle][cb::bundle] types, for spawning different common
kinds of entities.

Bundle是bevy内置的类型,用于生成实体,生成实体时需要指定绑定的组件,
这个Bundle里保存的就是各种组件.

{{#include ./include/builtins.md:bundles}}

## Resources

[(more info about working with resources)][cb::res]

资源主要分配置资源/引擎资源/渲染资源/wgpu资源/输入处理资源,以下逐个分析.

### Configuration Resources

These resources allow you to change the settings for how various parts of Bevy work.

These may be inserted at the start, but should also be fine to change at runtime (from a
[system][cb::system]):

{{#include ./include/builtins.md:resources-config}}

Settings that are not modifiable at runtime are not represented using resources. Instead,
they are configured via the respective [plugins](#plugins).

bevy内置的配置影响了bevy各个模块的运行方式.配置资源应该在程序启动时添加,在运行期间也要支持修改(通过system).
`运行时不修改的设置不要用资源表示,而应该使用插件表示`.

### Engine Resources

These resources provide access to different features of the game engine at runtime.

Access them from your [systems][cb::system], if you need their state, or to control the respective
parts of Bevy. These resources are in the [Main World][cb::render-architecture]. [See here for the
resources in the Render World](#render-world).

{{#include ./include/builtins.md:resources-main}}

bevy内置的引擎资源可以访问不同的功能,通过system访问这些资源,
这些资源都是在Main world中.

#### Render World Resources

These resources are present in the [Render World][cb::render-architecture]. They can be accessed
from rendering systems (that run during [render stages][cb::render::stage]).

{{#include ./include/builtins.md:resources-render}}

There are many other resources in the Render World, which are not mentioned
here, either because they are internal to Bevy's rendering algorithms, or
because they are just extracted copies of the equivalent resources in the Main
World.

render world中的资源,在render system中访问.
还有很多渲染资源没列出来,要么是因为bevy内部的渲染算法用到,要么是Main world中拷贝过来的副本.

#### Low-Level `wgpu` Resources

Using these resources, you can have direct access to the `wgpu` APIs for controlling the GPU.
These are available in both the Main World and the Render World.

使用wgpu资源,可以直接访问wgpu的API,这样就可以直接控制GPU了,
在main world和render world都能使用.

{{#include ./include/builtins.md:resources-wgpu}}

### Input Handling Resources

These resources represent the current state of different input devices. Read them from your
[systems][cb::system] to [handle user input][chapter::input].

{{#include ./include/builtins.md:resources-input}}

## Events

[(more info about working with events)][cb::event]

事件用于system传递数据,主要有输入事件/引擎事件/OS和控制事件.

### Input Events

These [events][cb::event] fire on activity with input devices. Read them to [handle user input][cb::input].

{{#include ./include/builtins.md:events-input}}

输入事件由输入设备触发,从input回调中读取.

### Engine Events

[Events][cb::event] related to various internal things happening during the
normal runtime of a Bevy app.

{{#include ./include/builtins.md:events-engine}}

bevy程序正常运行过程中会产生的各种内部事件.

### System and Control Events

Events from the OS / windowing system, or to control Bevy.

{{#include ./include/builtins.md:events-system}}

OS/窗口系统产生的事件,或控制bevy程序的事件.

## Components

The complete list of individual component types is too specific to be useful to list here.

See: [(List in API Docs)][bevy::impl::Component]

