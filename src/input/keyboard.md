{{#include ../include/header014.md}}

# Keyboard Input

Relevant official examples:
[`keyboard_input`][example::keyboard_input],
[`keyboard_input_events`][example::keyboard_input_events].

---

This page shows how to handle keyboard keys being pressed and released.

Note: Command Key on Mac corresponds to the Super/Windows Key on PC.

Similar to [mouse buttons][input::mouse-button], keyboard input is available
as a [`ButtonInput`] [resource][cb::res], [events][cb::event], and [run
conditions][cb::rc] ([see list][bevy::input::common_conditions]). Use
whichever pattern feels most appropriate to your use case.

和鼠标按键一样,键盘按键也可作为ButtonInput资源/事件/运行条件, 选择一种合适的即可.

## Checking Key State

Most commonly for games, you might be interested in specific known keys and
detecting when they are pressed or released. You can check specific keys
using the [`ButtonInput<KeyCode>`][`ButtonInput`] [resource][cb::res].

 - Use `.pressed(…)`/`.released(…)` to check if a key is being held down
   - These return `true` every frame, for as long as the key is in the respective state.
 - Use `.just_pressed(…)`/`.just_released(…)` to detect the actual press/release
   - These return `true` only on the frame update when the press/release happened.

```rust,no_run,noplayground
{{#include ../code014/src/input/keyboard.rs:res}}
```

To iterate over any keys that are currently held, or that have been pressed/released:

```rust,no_run,noplayground
{{#include ../code014/src/input/keyboard.rs:res-iter}}
```

大多数游戏都会选择检查资源,直接查资源保存的按键状态.  
`.pressed()/.released()`,检查某个键是否已经被按下或释放.只要状态对,每帧都返回true.  
`.just_pressed()/.just_released()`,只要状态对,只在当前帧返回true.

## Run Conditions

Another workflow is to add [run conditions][cb::rc] to your systems,
so that they only run when the appropriate inputs happen.

It is highly recommended you write your own [run conditions][cb::rc],
so that you can check for whatever you want, support configurable bindings, etc…

For prototyping, Bevy offers some [built-in run conditions][input::rc]:

```rust,no_run,noplayground
{{#include ../code014/src/input/keyboard.rs:run-conditions}}
```

条件运行,仅在某些事发生时才执行system.

强烈推荐自己编写自己的运行条件,这样可以追求更大的灵活性,也支持配置按键绑定等.

上面的例子是bevy内置的运行条件.

## Keyboard Events

To get all keyboard activity, you can use [`KeyboardInput`] [events][cb::event]:

```rust,no_run,noplayground
{{#include ../code014/src/input/keyboard.rs:events}}
```

捕获所有键盘事件,使用事件.

### Physical [`KeyCode`] vs. Logical [`Key`]

When a key is pressed, the [event][cb::event] contains two important pieces of information:
 - The [`KeyCode`], which always represents a specific key on the keyboard,
   regardless of the OS layout or language settings.
 - The [`Key`], which contains the logical meaning of the key as interpreted by the OS.

When you want to implement gameplay mechanics, you want to use the [`KeyCode`].
This will give you reliable keybindings that always work, including for multilingual
users with multiple keyboard layouts configured in their OS.

When you want to implement text/character input, you want to use the [`Key`].
This can give you Unicode characters that you can append to your text string and
will allow your users to type just like they do in other applications.

If you'd like to handle special function keys or media keys on keyboards that
have them, that can also be done via the logical [`Key`].

当一个键按下时,事件会包含两部分信息:
 - KeyCode,键盘上哪个键被按下了,和OS为不同语言的布局无关
 - Key,OS对按键的解释

当实现游戏逻辑时,需要使用Keycode,这样你可以始终提供可靠的绑定.  
当实现文本/字符输入时,使用Key,这可以为您提供Unicode字符,您可以将其附加到文本字符串中,
并允许您的用户像在其他应用程序中一样进行键入。

如果您想处理具有特殊功能键或媒体键的键盘,也可以通过逻辑键来完成.

## Text Input

Here is a simple example of how to implement text input into a string (here
stored as a [local][cb::local]).

```rust,no_run,noplayground
{{#include ../code014/src/input/keyboard.rs:char}}
```

Note how we implement special handling for keys like `Backspace` and `Enter`.
You can easily add special handling for other keys that make sense in your
application, like arrow keys or the `Escape` key.

Keys that produce useful characters for our text come in as small Unicode
strings. It is possible that there might be more than one `char` per keypress
in some languages.

Note: To support text input for international users who use languages
with complex scripts (such as East Asian languages), or users who use
assistive methods like handwriting recognition, you also need to support
[IME input][input::ime], in addition to keyboard input.

文本输入,上面的例子中对回退和enter键做了特殊处理,常见的特殊处理还有Esc/方向键.

注意:为了支持使用复杂脚本的语言(CJK)的国际用户或使用手写识别等辅助方法的用户的文本输入,
除了键盘输入之外,您还需要支持 IME 输入.

## Keyboard Focus

If you are doing advanced things like caching state to detect multi-key
sequences or combinations of keys, you might end up in an inconsistent
state if the Bevy OS window loses focus in the middle of keyboard input,
such as with Alt-Tab or similar OS window switching mechanisms.

If you are doing such things and you think your algorithm might be getting
stuck, Bevy offers a [`KeyboardFocusLost`] [event][cb::event] to let you
know when you should reset your state.

```rust,no_run,noplayground
{{#include ../code014/src/input/keyboard.rs:focus-lost}}
```

如果您正在执行高级操作,例如缓存状态以检测多键序列或键组合,
如果 Bevy OS 窗口在键盘输入过程中失去焦点(例如使用 Alt-Tab 或类似的 OS 窗口切换机制),
您可能会陷入不一致的状态.

`KeyboardFocusLost`事件会上清除一些按键缓存.
