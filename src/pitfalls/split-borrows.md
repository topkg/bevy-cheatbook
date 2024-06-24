{{#include ../include/header013.md}}

# Borrow multiple fields from struct

When you have a [component][cb::component] or [resource][cb::res], that is
larger struct with multiple fields, sometimes you want to borrow several of
the fields at the same time, possibly mutably.

```rust,no_run,noplayground
struct MyThing {
    a: Foo,
    b: Bar,
}

fn my_system(mut q: Query<&mut MyThing>) {
    for thing in q.iter_mut() {
        // 只有在这里重新借用,才不会引用规则冲突(同时出现共享引用和独占可变引用).
        helper_func(&thing.a, &mut thing.b); // ERROR!
    }
}

// 如果这两个参数引用同一个结构体的多各字段,那么会出现冲突
fn helper_func(foo: &Foo, bar: &mut Bar) {
    // do something
}
```

This can result in a compiler error about conflicting borrows:

```
error[E0502]: cannot borrow `thing` as mutable because it is also borrowed as immutable
    |
    |         helper_func(&thing.a, &mut thing.b); // ERROR!
    |         -----------  -----         ^^^^^ mutable borrow occurs here
    |         |            |
    |         |            immutable borrow occurs here
    |         immutable borrow later used by call
```

The solution is to use the "reborrow" idiom, a common but non-obvious trick in Rust programming:

```rust,no_run,noplayground
// add this at the start of the for loop, before using `thing`:
let thing = &mut *thing;

// or, alternatively, Bevy provides a method, which does the same:
let thing = thing.into_inner();
```

Note that this line triggers [change detection][cb::change-detection]. Even if
you don't modify the data afterwards, the component gets marked as changed.

通过重新引用来绕开`变更检测`,
重新引用本身就是为了在不同的作用域内使用同一个对象，而无需放弃当前的借用。

## Explanation

Bevy typically gives you access to your data via special wrapper types (like
[`Res<T>`], [`ResMut<T>`], and [`Mut<T>`] (when [querying][cb::query] for
components mutably)). This lets Bevy track access to the data.

These are "smart pointer" types that use the Rust [`Deref`] trait to dereference
to your data. They usually work seamlessly and you don't even notice them.

However, in a sense, they are opaque to the compiler. The Rust language allows
fields of a struct to be borrowed individually, when you have direct access to
the struct, but this does not work when it is wrapped in another type.

The "reborrow" trick shown above, effectively converts the wrapper into a
regular Rust reference. `*thing` dereferences the wrapper via [`DerefMut`], and
then `&mut` borrows it mutably. You now have `&mut MyStuff` instead of
`Mut<MyStuff>`.

bevy是通过不同的封装类型来访问不同的数据的.
这些封装类型有点向智能指针,会自动解引用.

上面的重新引用就是将封装类型转换成rust引用类型.
rust的是允许结构体的字段被单独引用的.bevy的封装类型使用时就需要注意了.
