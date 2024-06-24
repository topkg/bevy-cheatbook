{{#include ../include/header013.md}}

# Bevy Time vs. Rust/OS time

Do *not* use [`std::time::Instant::now()`][`Instant`] to get the
current time. [Get your timing information from Bevy][cb::time], using
[`Res<Time>`].

Rust (and the OS) give you the precise time of the moment you call that
function. However, that's not what you want.

Your game systems are run by Bevy's parallel scheduler, which means that they
could be called at vastly different instants every frame! This will result in
inconsistent / jittery timings and make your game misbehave or look stuttery.

Bevy's [`Time`] gives you timing information that is consistent throughout the
frame update cycle. It is intended to be used for game logic.

This is not Bevy-specific, but applies to game development in general. Always
get your time from your game engine, not from your programming language or
operating system.

抖动时间,不要使用标准库来获取当前时间,而是要用`Res<Time>`来获取.

在游戏中需要使用游戏中的时间,因为游戏可以暂停,可以快放,可以慢放.
使用rust或OS的时间都会出现异常.

bevy负责在不同的时间调度system,如果使用非bevy时间,会导致游戏异常或卡顿.
bevy内置的时间就是用来解决Update一致性问题的.

