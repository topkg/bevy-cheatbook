{{#include ../include/header014.md}}

# Drag-and-Drop (Files)

Relevant official examples:
[`drag_and_drop`][example::drag_and_drop].

---

Bevy supports the Drag-and-Drop gesture common on most desktop operating
systems, but only for files, not arbitrary data / objects.

If you drag a file (say, from the file manager app) into a Bevy app, Bevy
will produce a [`FileDragAndDrop`] [event][cb::event], containing the path
of the file that was dropped in.

```rust,no_run,noplayground
{{#include ../code014/src/input/dnd.rs:dnd-file}}
```

拖拽,在桌面平台,且只支持文件.通过`FileDragAndDrop`事件捕获.

## Detecting the Position of the Drop

You may want to do different things depending on where the cursor was when the
drop gesture ended. For example, add the file to some collection, if it was
dropped over a specific UI element/panel.

Unfortunately, this is currently somewhat tricky to implement, due to [`winit`
bug #1550][winit::1550]. Bevy does not get [`CursorMoved`] [events][cb::event]
while the drag gesture is ongoing, and therefore does not respond to the
mouse cursor. Bevy completely loses track of the cursor position.

Checking the cursor position from the [`Window`] will also not work.

Systems that use cursor events to respond to cursor movements will not work
during a drag gesture. This includes Bevy UI's [`Interaction`] detection,
which is the usual way of detecting when a UI element is hovered over.

您可能想要根据放下手势结束时光标所在的位置执行不同的操作.
例如,如果将文件拖放到特定的 UI 元素/面板上,则将文件添加到某个集合.
因为winit功能缺失,这个功能还未实现.

### Workaround

The only way to workaround this issue is to store the file path somewhere
temporarily after receiving the drop event. Then, wait until the next
[`CursorMoved`] event, and then process the file.

Note that this might not even be on the next frame update. The next cursor
update will happen whenever the user moves the cursor. If the user does not
immediately move the mouse after dropping the file and leaves the cursor in the
same place for a while, there will be no events and your app will have no way of
knowing the cursor position.

解决此问题的唯一方法是在收到放置事件后将文件路径临时存储在某处.
然后,等待下一个 CursorMoved 事件,然后处理该文件.
请注意,这甚至可能不是在下一帧更新时.
只要用户移动光标,就会发生下一次光标更新.
如果用户在放下文件后没有立即移动鼠标,而是将光标停留在同一位置一段时间,
则不会发生任何事件,您的应用也无法知道光标的位置.
