{{#include ../include/header014.md}}

# IME Input

Bevy has support for IMEs (Input Method Editors), which is how people perform
text input in languages with more complex scripts, like East Asian languages, and
how non-keyboard text input methods (such as handwriting recognition) work. It
requires some special handling from you, however.

If you'd like all international users to be able to input text in their
language, the way they usually do in other GUI apps on their OS, you should
support IMEs. If you want good accessibility for disabled users or users
who prefer alternative text input methods like handwriting recognition, you
should support IMEs. This should be in addition to supporting [text input via
the keyboard][input::keyboard-text], which is how most users will input text.

复杂的输入需要使用IME支持,其中就包括汉字输入.
支持IME就可以支持不同语言的玩家.

## How IMEs Work

IMEs work by using a special "buffer", which shows the current in-progress
text suggestions and allows users to preview and compose the next part of
their text before confirming it. The text suggestions are provided by the OS,
but your app needs to display them for the user.

For example, imagine you have a text input box in your UI. You show the text
that the user has already inputted, with a cursor at the end.

If IME is enabled, you will get [`Ime::Preedit`][`Ime`] [events][cb::event]
for "pending" text.  You should show that "unconfirmed" text in the text
input box, but with different formatting to be visually distinct.

When the user confirms their desired input, you will get an
[`Ime::Commit`][`Ime`] [event][cb::event] with the final text. You should
then discard any previous "uncofirmed" text and append the new text to your
actual text input string.

IME的工作原理是利用一个特殊的缓冲,通过这个缓冲可以得到候选字,
也可以将后来输入的和前面的合并形成新的候选字,用户在确认之前还可以预览.
生成候选字是其他OS提供的,但app需要显示用户输入.

在整个用户输入的过程中,app需要显示用户已确认的文本,并在最后显示光标.

如果启用了IME,通过`Ime::Preedit`事件得到阻塞的文本.
通过`Ime::Commit`事件得到用户确认的文本,加上之前确认的文本就是所有输入的文本.

## How to support IMEs in your Bevy app

First, you need to inform the OS when your application is expecting text input.
You don't want the IME to accidentally activate during gameplay, etc.

Whenever you want the user to input text, you enable "IME mode" on the [`Window`].
When you are done, disable it.

If the user is not using an IME, nothing happens when you enable "IME mode". You
will still get [keyboard][input::keyboard] [events][cb::event] as usual and you
can [accept text input that way][input::keyboard-text].

If the user has an IME, you will get an [`Ime::Enabled`][`Ime`] event. At that point,
your application will no longer receive any [`KeyboardInput`] [events][cb::event].

You can then handle [`Ime::Preedit`][`Ime`] [events][cb::event] for pending/unconfirmed
text, and [`Ime::Commit`][`Ime`] for final/confirmed text.

```rust,no_run,noplayground
{{#include ../code014/src/input/ime.rs:ime}}
```

For the sake of brevity, this example just prints the events to the console.

In a real app, you will want to display the "pre-edit" text on-screen, and use
different formatting to show the cursor. On "commit", you can append the
provided text to the actual string where you normally accept text input.

要在app中启用ime,按如下顺序操作:
 - 系统安装需要的输入法
 - 在窗口启用IME功能

当app的IME开始生效时(接收Ime::Enabled事件后),就不会在接收到的KeyboardInput事件了.
此时需要将pre-edit的文本显示在屏幕上,将已确认的文本显示在正确的位置,并在最后显示光标,
在`Commit`之后意味着输入结束,接着走后续逻辑.
