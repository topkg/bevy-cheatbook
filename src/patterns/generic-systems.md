{{#include ../include/header09.md}}

# Generic Systems

Bevy [systems][cb::system] are just plain rust functions, which means they
can be generic. You can add the same system multiple times, parametrized to
work on different Rust types or values.

bevy的system是函数,可以进行泛化,这样可以多次添加system,支持具体类型有差异而已.

## Generic over Component types

You can use the generic type parameter to specify what
[component][cb::component] types (and hence what [entities][cb::ecs-intro])
your [system][cb::system] should operate on.

This can be useful when combined with Bevy [states][cb::state].
You can do the same thing to different sets of entities depending on state.

组件类型泛化,前提是system能支持泛化的组件.

结合state,可对不同state的实体做同样的事.

### Example: Cleanup

One straightforward use-case is for cleanup. We can make a generic cleanup
system that just despawns all entities that have a certain component
type. Then, trivially run it on exiting different states.

```rust,no_run,noplayground
{{#include ../code/examples/generic-systems.rs:cleanup}}
```

Menu entities can be tagged with `cleanup::MenuExit`, entities from the game
map can be tagged with `cleanup::LevelUnload`.

We can add the generic cleanup system to our state transitions, to take care
of the respective entities:

```rust,no_run,noplayground
{{#include ../code/examples/generic-systems.rs:main}}
```

## Using Traits

You can use this in combination with Traits, for when you need some sort of
varying implementation/functionality for each type.

泛型system可以结合特型.

### Example: Bevy's Camera Projections

(this is a use-case within Bevy itself)

Bevy has a [`CameraProjection`][bevy::CameraProjection] trait. Different
projection types like [`PerspectiveProjection`][bevy::PerspectiveProjection]
and [`OrthographicProjection`][bevy::OrthographicProjection] implement that
trait, providing the correct logic for how to respond to resizing the window,
calculating the projection matrix, etc.

There is a generic system `fn camera_system::<T: CameraProjection +
Component>`, which handles all the cameras with a given projection type. It
will call the trait methods when appropriate (like on window resize events).

The [Bevy Cookbook Custom Camera Projection
Example][cb::camera-custom-projection] shows this API in action.

相机投影是特型,正交/透视是两个实现,system的泛化参数可以指定为特型,
调度添加system时再决定具体类型,极大提高了灵活性.

## Using Const Generics

Now that Rust has support for Const Generics, functions can also be
parametrized by values, not just types.

```rust,no_run,noplayground
{{#include ../code/examples/generic-systems.rs:const}}
```

Note that these values are static / constant at compile-time. This can be
a severe limitation. In some cases, when you might suspect that you could
use const generics, you might realize that you actually want a runtime value.

If you need to "configure" your system by passing in some data, you could,
instead, use a [Resource][cb::res] or [Local][cb::local].

Note: As of Rust 1.65, support for using `enum` values as const generics is
not yet stable. To use `enum`s, you need Rust Nightly, and to enable the
experimental/unstable feature (put this at the top of your `main.rs` or
`lib.rs`):

```rust,no_run,noplayground
#![feature(adt_const_params)]
```

常量泛化,使用const修饰,编译期会将常量和system绑定在一起.
和配置不同,常量泛化是代码中的硬编码,因为是常量所以运行期不能修改.

如果是配置,一样可以将数据传递给system,不同的场景可以使用不同的实现方案.
配置可以存放在资源或local中.

从rust v1.65开始就支持了enum,这也是个非常灵活的狠角色,
用enum作为常量泛化还不稳定,但现在bevy都是v0.14了,大多例子都是enum.
