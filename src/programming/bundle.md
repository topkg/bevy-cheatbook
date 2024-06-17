{{#include ../include/header014.md}}

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

# Bundles

You can think of Bundles like "templates" for creating entities.
They make it easy to create [entities][cb::entity] with a common set of
[components][cb::component] types.

By creating a bundle type, instead of adding your components one by one, you
can make sure that you will never accidentally forget some important component
on your entities. The Rust compiler will give an error if you do not set all
the fields of a struct, thus helping you make sure your code is correct.

Bevy provides many [built-in bundle types][builtins::bundle] that you can use
to spawn common kinds of entities.

bundle可翻译为一捆一包,将多个组件划分为一包,再参与实体的构造,
简单点说就是构造实体打造出的特殊类型,为什么要bundle,自然是为了api的统一.

bundle还有个好处,将部分组件划分为一包,这样在构造实体时就不会意外少了某个组件,
ecs架构中少一个组件,运行结果将大大不同.rust编译器会检查bundle的每个字段,
这也是变相保证了代码的正确性(这有点硬扯了).

bevy提供了多种内置的bundle,方便生成不同类型的实体.

## Creating Bundles

To create your own bundle, derive [`Bundle`] on a `struct`:

```rust,no_run,noplayground
{{#include ../code014/src/programming/bundle.rs:bundle}}
```

When you have nested bundles, everything gets flattened.
You end up with an entity that has all the included component
types. If a type appears more than once, that's an error.

不管bundle嵌套多少,最终都是扁平化处理,实体会拥有所有的组件类型,如果重复会出错.

### Using Bundles

You can then use your bundle when you spawn your entities:

```rust,no_run,noplayground
{{#include ../code014/src/programming/bundle.rs:bundle-spawn}}
```

If you want to have default values (similar to Bevy's bundles):

```rust,no_run,noplayground
{{#include ../code014/src/programming/bundle.rs:bundle-default}}
```

Now you can do this:

```rust,no_run,noplayground
{{#include ../code014/src/programming/bundle.rs:bundle-spawn-default}}
```

具体实现Bundle特型的类型,在Commands.spawn()中可以传入具体的实例,
具体类型还可以实现Default来简化写法.

### Bundles for Removal

Bundles can also be useful to represent a set of components that you
want to be able to easily remove from an entity.

```rust,no_run,noplayground
{{#include ../code014/src/programming/bundle.rs:cleanup-bundle}}
```

```rust,no_run,noplayground
{{#include ../code014/src/programming/bundle.rs:cleanup-bundle-remove}}
```

The component types included in the bundle will be removed from the
entity, if any of them exist on the entity.

实体删除Bundle的套路是:
`commands.entity(实体id).remove::<具体的Bundle类型>();`

## Loose components as bundles

Technically, Bevy also considers arbitrary tuples of components as bundles:

```
(ComponentA, ComponentB, ComponentC)
```

This allows you to easily spawn an entity using a loose bunch of components (or
bundles), or add more arbitrary components when you spawn entities. However,
this way you don't have the compile-time correctness advantages that a
well-defined `struct` gives you.

```rust,no_run,noplayground
{{#include ../code014/src/programming/bundle.rs:bundle-spawn-loose}}
```

You should strongly consider creating proper `struct`s, especially if you are
likely to spawn many similar entities. It will make your code easier to maintain.

bevy设计的Bundle是一个非常灵活的类型,无限嵌套让Bundle的表现力非常强大,
也复用rust编译器的正确性检查功能.仅凭这一点,
最好还是使用结构体将松散的组件列表维护起来, 特别是要构造很多相似的实体时,代码更容易维护.

所以说尽量使用Bundle来组织组件.

## Querying

Note that you cannot [query][cb::query] for a whole bundle. Bundles are just a
convenience when creating the entities. Query for the individual component types
that your [system][cb::system] needs to access.

This is *wrong*:

```rust,no_run,noplayground
fn my_system(query: Query<&SpriteBundle>) {
  // ...
}
```

Instead, do this:

```rust,no_run,noplayground
fn my_system(query: Query<(&Transform, &Handle<Image>)>) {
  // ...
}
```

(or whatever specific components you need in that system)

ECS查询时使用组件,因为Bundle设计的使用场景是构造实体,不是这儿的查询.

