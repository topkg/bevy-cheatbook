{{#include ../include/header014.md}}

# Local Resources

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

Local resources allow you to have per-[system][cb::system] data. This data
is not stored in the ECS World, but rather together with your system.
Nothing outside of your system can access it. The value will be kept across
subsequent runs of the system.

[`Local<T>`] is a system parameter similar to [`ResMut<T>`], which gives
you full mutable access to a single value of the given data type, that is
independent from entities and components.

[`Res<T>`]/[`ResMut<T>`] refer to a single global instance of the type, shared
between all systems. On the other hand, every [`Local<T>`] parameter is a
separate instance, exclusively for that system.

```rust,no_run,noplayground
{{#include ../code014/src/programming/local.rs:local-resource}}
```

The type must implement [`Default`] or [`FromWorld`]. It is automatically
initialized. It is not possible to specify a custom initial value.

A system can have multiple [`Local`]s of the same type.

local资源,可以存储每帧的system数据,资源不属于ecs世界,而是跟着system走的.
这个数据只有system才能访问.

`Local<T>`是一个system参数,类似于`ResMut<T>`,通过这个参数可以访问指定类型的数据,
听到这儿,感觉local resource和普通的资源没什么差别.
差别在于资源是所有system都可以访问的,local资源只能被特定的system访问.

local类型需要实现Default或FromWorld特性,初始化是自动的,不能指定自定义值.

一个system可以拥有多个local参数,而且这些local参数可以是同一个类型的.

## Specify an initial value

[`Local<T>`] is always automatically initialized using the default value for
the type. If that doesn't work for you, there is an alternative way to pass
data into a system.

If you need specific data, you can use a closure instead. Rust closures that
take system parameters are valid Bevy systems, just like standalone functions.
Using a closure allows you to "move data into the function".

This example shows how to initialize some data to configure a system, without
using [`Local<T>`]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/local.rs:closure}}
```

Another way to accomplish the same thing is to "return" the system from
"constructor" helper function that creates it:

```rust,no_run,noplayground
{{#include ../code014/src/programming/local.rs:constructor}}
```

local参数初始化是使用默认值,不能指定自定义值,但bevy还是提供了其他方法来实现.

不使用`Local<T>`,改为闭包.

利用闭包捕获局部变量,而闭包的是实现就是一个普通system的实现,但在函数内部,
可以使用捕获的局部变量,而局部变量的值是可以自定义的,变相达到了自定义初始化值.
