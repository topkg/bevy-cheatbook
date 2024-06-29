{{#include ../include/header012.md}}

# Cameras

Cameras drive all rendering in Bevy. They are responsible for configuring what
to draw, how to draw it, and where to draw it.

Cameras驱动着Bevy内的全部渲染. 这些相机负责配置: 绘制什么, 如何绘制, 以及在哪里绘制.

You must have at least one camera entity, in order for anything to be displayed
at all! If you forget to spawn a camera, you will get an empty black screen.

你至少需要一个相机实体, 为了显示任何东西! 如果你忘记spawn一个相机, 那么你将会得到一个空空的黑色屏幕.

In the simplest case, you can create a camera with the default settings. Just
spawn an entity using [`Camera2dBundle`][bevy::Camera2dBundle] or
[`Camera3dBundle`][bevy::Camera3dBundle]. It will simply draw all renderable
entities that are [visible][cb::visibility].

最简单的例子是, 你能用默认的配置创建一个相机. 比如使用Camera2dBundle或Camera3dBundle, 
它将会绘制任何可见的实体.

This page gives a general overview of cameras in Bevy. Also see the dedicated
pages for [2D cameras][cb::camera-2d] and [3D cameras][cb::camera-3d].

Practical advice: always create [marker components][cb::component-marker] for
your camera entities, so that you can [query][cb::query] your cameras easily!

```rust,no_run,noplayground
{{#include ../code012/src/graphics/camera.rs:simple}}
```

相机驱动 Bevy 中的所有渲染.他们负责配置绘制什么、如何绘制以及在哪里绘制.

> 我理解的游戏是一个世界,相机是观看世界的一个视角,或一双眼睛.整个世界都在运行着,
但我们的视角只能看到一部分,boss的刷新不在我们视角中,但在世界中确实发生了,
一个玩家在打怪,不在我们视角中,我们的电脑上的程序不会处理这个,
但当我们走近时,服务端会将这部分数据同步过来,我也看到了一个玩家在打怪.

要看到渲染的东西,至少需要一个camera实体,如果忘了创建camera实体,啥都看不到.

在一些简单的例子中,camera可以使用默认配置,bevy会简单绘制所有可视且能渲染的实体.

有个建议:给创建的Camera贴一个标签,这样方便Query.这样的标签还有个名词:`标签组件`.

## The Camera Transform

Cameras have [transforms][cb::transform], which can be used to position or
rotate the camera. This is how you move the camera around.

For examples, see these [cookbook][chapter::cookbook] pages:
 - [3D pan-orbit camera][cookbook::pan-orbit-camera], like in 3D editor apps

If you are making a game, you should implement your own custom camera controls
that feel appropriate to your game's genre and gameplay.

camera也有变换,也能移动位置和旋转相机,也就是相机移动的方式.
制作bevy游戏,应该自定义相机控制使其更加服务和自己的游戏和玩法.

### Zooming the camera

Do not use the transform scale to "zoom" a camera! It just stretches the image,
which is not "zooming". It might also cause other issues and incompatibilities.
Use the [projection][cb::camera-projection] to zoom.

For an orthographic projection, change the scale. For a perspective projection,
change the FOV. The FOV mimics the effect of zooming with a lens.

Learn more about how to do this in [2D][cb::camera-2d::projection] or
[3D][cb::camera-3d::projection].

相机的聚焦(zoom)不是缩放,缩放是拉伸,不是聚焦,使用缩放来实现聚焦会有各种问题,
正确的方式应该是`投影`.

对于`正交投影`,应该改为缩放;对于`透视投影`,改为`FOV`,FOV模仿了镜头变焦的效果.

- 正交投影的特点是随着远近,投影物体没有大小变化
- 透视投影则有远小近大的特点

## Projection

The camera projection is responsible for mapping the coordinate system to the
viewport (commonly, the screen/window). It is what configures the coordinate
space, as well as any scaling/stretching of the image.

Bevy provides two kinds of projections:
[`OrthographicProjection`][bevy::OrthographicProjection] and
[`PerspectiveProjection`][bevy::PerspectiveProjection]. They are configurable,
to be able to serve a variety of different use cases.

Orthographic means that everything always appears the same size, regardless of
how far away it is from the camera.

Perspective means that things appear smaller the further away they are from
the camera. This is the effect that gives 3D graphics a sense of depth and
distance.

[2D cameras][cb::camera-2d] are always orthographic.

[3D cameras][cb::camera-3d] can use either kind of projection. Perspective is
the most common (and default) choice. Orthographic is useful for applications
such as CAD and engineering, where you want to accurately represent the
dimensions of an object, instead of creating a realistic sense of 3D space. Some
games (notably simulation games) use orthographic as an artistic choice.

It is possible to implement your own [custom camera
projections][cb::camera-custom-projection]. This can give you full control over
the coordinate system. However, beware that things might behave in unexpected
ways if you violate Bevy's [coordinate system conventions][cb::coords]!

相机投影负责将坐标系映射到窗口视角(通常是屏幕或窗口),这里的相机投影就是配置坐标系,
以及图像的缩放和拉伸.
bevy提供了`OrthographicProjection`正交投影和`PerspectiveProjection`透视投影两种,
两者均可配置,适用于不同场景.

正交意味着所有物体始终显示相同的尺寸,无论距相机有多远.  
透视意味着物体距离相机越远,看起来就越小.这种效果赋予 3D 图形深度感和距离感.

2d游戏使用正交,3d游戏两种都可以使用,但透视更加常用.

正交对于 CAD 和工程等应用程序非常有用,在这些应用程序中,您希望准确表示对象的尺寸,
而不是创建逼真的 3D 空间感.一些游戏（尤其是模拟游戏）使用正交文字作为艺术选择.

也可以自定义相机属性,这样就可以完全控制坐标体系,但需要注意:遵循bevy的坐标体系规则.


## HDR and Tonemapping

[See here!][cb::hdr]

高动态范围图片和色调映射,见下一章.

## Render Target

The render target of a camera determines where the GPU will draw things to. It
could be a window (for outputting directly to the screen) or an
[`Image`][bevy::Image] [asset][cb::asset] (render-to-texture).

By default, cameras output to the primary window.

```rust,no_run,noplayground
{{#include ../code012/src/graphics/camera.rs:render-target}}
```

相机的渲染目标决定了GPU会绘制那些对象,可能是窗口(直接渲染到屏幕),可能是图片资产(渲染到纹理).
默认,相机输出到主窗口.

```rust
pub enum RenderTarget {
    Window(WindowRef),
    Image(Handle<Image>),
    TextureView(ManualTextureViewHandle),
}
```
这3种对象依次是:
 - 相机渲染到窗口
 - 相机渲染到Image
 - 相机渲染到纹理视图,特别适合在bevy之外产生纹理试图的场景(eg:外部XR中创建和管理纹理试图,传给bevy做渲染)

## Viewport

The viewport is an (optional) way to restrict a camera to a sub-area of its
render target, defined as a rectangle. That rectangle is effectively treated as
the "window" to draw in.

An obvious use-case are split-screen games, where you want a camera to only draw
to one half of the screen.

```rust,no_run,noplayground
{{#include ../code012/src/graphics/camera.rs:set-viewport}}
```

If you need to find out the area a camera renders to (the viewport, if
configured, or the entire window, if not):

```rust,no_run,noplayground
{{#include ../code012/src/graphics/camera.rs:get-viewport}}
```

视窗是一种将取部分相机视角的方案之一.视窗通常是矩形,是矩形所以作为窗口绘制时就很高效.
一个常见的应用场景是分屏游戏,一个相机只绘制到半个屏幕.

上面的例子显示了相机制定视窗属性的例子,并取相机视窗的的矩形坐标大小和渲染区域大小.

## Coordinate Conversion

[`Camera`][bevy::Camera] provides methods to help with coordinate conversion
between on-screen coordinates and world-space coordinates. For an example, see
the ["cursor to world"][cookbook::cursor2world] cookbook page.

相机提供了不少方法来进行屏幕坐标和世界坐标的转换.

## Clear Color

This is the "background color" that the whole viewport will be cleared to,
before a camera renders anything.

You can also disable clearing on a camera, if you want to preserve all the
pixels as they were before.

[See this page for more info.][cb::clearcolor]

背景色其实是整个视窗刷上的颜色,在相机渲染之前就完成了.

相机可以disable颜色擦除,做局部更新时可以用到.

## Render Layers

[`RenderLayers`][bevy::RenderLayers] is a way to filter what entities should be
drawn by what cameras. Insert this [component][cb::component] onto your entities
to place them in specific "layers". The layers are integers from 0 to 31 (32
total available).

Inserting this component onto a camera entity selects what layers that camera
should render. Inserting this component onto renderable entities selects what
cameras should render those entities. An entity will be rendered if there is any
overlap between the camera's layers and the entity's layers (they have at least
one layer in common).

If an entity does not have the [`RenderLayers`][bevy::RenderLayers] component,
it is assumed to belong to layer 0 (only).

```rust,no_run,noplayground
{{#include ../code012/src/graphics/camera.rs:renderlayers}}
```

You can also modify the render layers of entities after they are spawned.

通过渲染分层这个组件,相机可以过滤掉一些实体.
分层有32个,构建相机时通过RenderLayers组件设置分层,就配置相机关心的分层列表.
同理,构建实体时也需要指定分层列表.如果相机和可渲染实体的分层列表有重叠,
则进行渲染.

在实体构建后是可以修改这个分层属性的.

> 渲染分层描述了一个实体属于哪些渲染层.相机可以可以利用分层来过滤其他层的实体,
关于渲染分层有以下几条规则:
1. 实体可以属于多个层,也可以一个层都不指定
2. 第一层为 layer 0, 带RenderLayers组件的实体,其默认分层就是0
3. 不带RenderLayers组件的实体,bevy会给个默认分层:0
4. 带RenderLayers组件的实体,如果不带任何分层,则不可见

## Camera Ordering

A camera's `order` is a simple integer value that controls the order relative
to any other cameras with the same render target.

For example, if you have multiple cameras that all render to the primary window,
they will behave as multiple "layers". Cameras with a higher order value will render
"on top of" cameras with a lower value. `0` is the default.

```rust,no_run,noplayground
{{#include ../code012/src/graphics/camera.rs:overlay}}
```

相机顺序.

可能有这样的需求: 多个相机都渲染到主窗口,这样多个相机就相当于多层,
在渲染时通过相机属性order(一个整数,默认为0)来决定顺序,
order越大,渲染在最前面.

---

多相机就是多视角,有以下场景都会包含多相机,可以说最最常见了:
 - 除了上面提到的分屏游戏,还有驾驶游戏(驾驶员视角和后视镜视角)
 - UI 层和游戏世界层分离(ui是用户界面,也就是游戏设置界面,如果要显示,一定是在游戏世界之上)
 - 后处理效果(一个相机渲染场景,另一个相机将渲染结果应用到屏幕上,并添加后处理效果,如模糊、色彩校正,目的是实现复杂的效果)
 - 深度图和阴影图生成(一个相机正常渲染,另一个相机渲染深度图和阴影图,实现光照和阴影效果)
 - 小地图或镜像

多相机可以创建更加丰富和复杂的游戏体验.

## UI Rendering

Bevy UI rendering is integrated into the cameras! Every camera will, by default,
also draw UI.

However, if you are working with multiple cameras, you probably only want your
UI to be drawn once (probably by the main camera). You can disable UI rendering
on your other cameras.

Also, UI on multiple cameras is currently broken in Bevy. Even if you want
multiple UI cameras (say, to display UI in an app with multiple windows), it
does not work correctly.

```rust,no_run,noplayground
{{#include ../code012/src/graphics/camera.rs:no-ui}}
```

ui渲染已经集成到相机中了,所以在相机中就能使用ui.
如果是多相机模式,大部分场景下,我们只需要一个ui显示在主相机中.

另外bevy现在在多相机中显示ui的功能有坏损,即使想这么干,也会运行异常.

---

bevy的ui是一个框架,ui中包含按钮/图片/文本/布局,基于ECS实现的.主要包含以下元素.
 - 节点(Node):定义 UI 元素的大小和位置.节点是所有 UI 元素的基础组件.
 - 样式(Style):定义 UI 元素的布局和对齐方式,包括大小、位置、边距、填充等.
 - 颜色(Color):定义 UI 元素的背景颜色.
 - 文本(Text):显示文字内容,可以设置字体、大小、颜色等属性.
 - 图片(Image):显示图片内容.
 - 交互(Interaction):用于处理用户交互,如点击和悬停.

ui常用于以下场景:
 - 主菜单和设置界面：创建游戏的主菜单、选项菜单和设置界面,允许玩家选择游戏模式、调整音量、设置图像质量等.
 - HUD（Heads-Up Display）：显示游戏中的实时信息,例如玩家的生命值、分数、剩余时间、弹药数量等.
 - 对话框和提示：展示游戏中的对话、任务提示、提示信息等.
 - 交互界面：创建按钮、滑块、复选框等交互元素,允许玩家与游戏进行互动.
 - 信息面板和统计数据：显示详细信息和统计数据,例如角色属性、物品描述、战斗结果等.
 - 暂停菜单：在游戏过程中允许玩家暂停游戏,并显示暂停菜单,提供继续游戏、退出游戏等选项.

也可以基于ui做游戏:
 - 卡牌游戏：例如扑克牌、纸牌游戏等,可以通过 UI 系统创建卡牌、牌堆、按钮等元素,实现拖拽、点击等交互.
 - 棋盘游戏：例如国际象棋、围棋等,可以通过 UI 系统创建棋盘、棋子,并处理玩家的点击和移动操作.
 - 文字冒险游戏：通过 UI 系统创建文本对话框、选择按钮等,玩家可以通过选择不同的选项推动故事发展.
 - 点击游戏：例如点击模拟器,通过 UI 系统创建按钮、计分板等元素,玩家通过点击按钮进行游戏.

## Disabling Cameras

You can deactivate a camera without despawning it. This is useful when you want
to preserve the camera entity and all the configuration it carries, so you can
easily re-enable it later.

Some example use cases: toggling an overlay, switching between a 2D and 3D view.

```rust,no_run,noplayground
{{#include ../code012/src/graphics/camera.rs:is_active}}
```

可以在不销毁相机实体的情况下,`停用`.保留相机实体以及配置,方便后面重用.

有些例子是通过开关控制2d/3d的切换,反复切换中,`停用`就很好用.

## Multiple Cameras

This is an overview of different scenarios where you would need more than one
camera entity.

下面是需要使用多相机的场景.

### Multiple Windows

Official example: [`multiple_windows`][example::multiple_windows].

If you want to create a Bevy app with multiple windows, you need to spawn
multiple cameras, one for each window, and set their render targets
respectively. Then, you can use your cameras to control what to display in each
window.

多窗口必须要要多相机.

### Split-Screen

Official example: [`split_screen`][example::split_screen].

You can set the camera [viewport](#viewport) to only render to a part of the
render target. This way, a camera can be made to render one half of the screen
(or any other area). Use a separate camera for each view in a split-screen game.

分屏.相机只渲染到渲染对象的一部分区域.

### Overlays

Official example: [`two_passes`][example::two_passes].

You might want to render multiple "layers" (passes) to the same render target.
An example of this might be an overlay/HUD to be displayed on top of the
main game.

The overlay camera could be completely different from the main camera. For
example, the main camera might draw a 3D scene, and the overlay camera might
draw 2D shapes. Such use cases are possible!

Use a separate camera to create the overlay. Set the [priority](#priority)
higher, to tell Bevy to render it after (on top of) the main camera. Make sure
to disable [clearing](#clear-color)!

Think about which camera you want to be responsible for [rendering the
UI](#ui-rendering). Use the overlay camera if you want it to be unaffected,
or use the main camera if you want the overlay to be on top of the UI. Disable
it on the other camera.

Use [Render Layers](#render-layers) to control what entities should be rendered
by each camera.

覆盖.游戏顶部的覆盖层,是另一个相机做的渲染,因为在顶部,还有个名词HUD(抬头显).

覆盖相机和主相机很不一样,主相机可能是3d游戏,覆盖相机大多是2d的.

多个相机覆盖时,要确保顺序更高的相机要禁止清理背景色.

思考一下,ui应该通过那个相机显示:
 - 如果想不受影响,使用覆盖相机
 - 如果想做成hud,使用主相机,并在其他相机中disabe ui

用渲染分层来控制每个相机要渲染的实体.

### Render to Image

(aka Render to Texture)

Official example: [`render_to_texture`][example::render_to_texture].

If you want to generate an image in memory, you can output to an `Image` asset.

This is useful for intermediate steps in games, such as rendering a minimap or
the gun in a shooter game. You can then use that image as part of the final
scene to render to the screen. Item previews are a similar use case.

Another use case is window-less applications that want to generate image files.
For example, you could use Bevy to render something, and then export it to a PNG
file.

渲染到Image,也称作渲染到纹理.

如果在内存中生成了一张图,可以输出到Image的资产中.
这对于游戏中的中间步骤非常有用,例如在射击游戏中渲染小地图或枪.
然后,您可以使用该图像作为最终场景的一部分渲染到屏幕上.物品预览是一个类似的用例.

另一个场景是生成图片,eg:到处一张png图片.
