{{#include ../include/header013.md}}

# Change Detection

Relevant official examples:
[`component_change_detection`][example::component_change_detection].

---

Bevy allows you to easily detect when data is changed. You can use this to
perform actions in response to changes.

One of the main use cases is optimization – avoiding unnecessary work by
only doing it if the relevant data has changed. Another use case is triggering
special actions to occur on changes, like configuring something or sending
the data somewhere.

bevy提供了变更检测,以下场景下会用上:
 - 优化,只做和变更数据相关的事,不做无意义的事
 - 变更触发某个动作,eg:配置变更

## Components

### Filtering

You can make a [query][cb::query] that only yields entities if specific
[components][cb::component] on them have been modified.

Use [query filters][cb::query-filter]:
 - [`Added<T>`]: detect new component instances
   - if the component was added to an existing entity
   - if a new entity with the component was spawned
 - [`Changed<T>`]: detect component instances that have been changed
   - triggers when the component is mutated
   - also triggers if the component is newly-added (as per [`Added`])

(If you want to react to removals, see [removal
detection][cb::removal-detection]. It works differently.)

```rust,no_run,noplayground
{{#include ../code013/src/programming/change_detection.rs:change-detection}}
```

组件过滤,能将某些组件变更了的实体全部query出来.
 - `Added<T>` 检测新组件实例
    - 已有实体新增组件
    - 构造一个包含组件的实体
 - `Changed<T>` 检测组件实体是否有变更
    - 组件被修改触发
    - 组件新增触发(这条等价于Added<T>)

组件删除还有`删除检测`,和变更检测的实现方式有点不同.

### Checking

If you want to access all the entities, as normal, regardless of if they have
been modified, but you just want to know if a component has been changed,
you can use special [`Ref<T>`] query parameters instead of `&` for immutable access.

For mutable access, the change detection methods are always available (because
Bevy queries actually return a special [`Mut<T>`] type whenever you have `&mut`
in the query).

```rust,no_run,noplayground
{{#include ../code013/src/programming/change_detection.rs:ref}}
```

bevy还提供了一种方法来判断某个组件是否有改变
 - 只查是否有改变,本次system执行不改变组件值. `Ref<T>.is_changed()`
 - 查是否有改变,并在本次system还支持变更组件值. `(&mut T).is_changed()`

## Resources

For [resources][cb::res], change detection is provided via methods on the
[`Res<T>`]/[`ResMut<T>`] system parameters.

```rust,no_run,noplayground
{{#include ../code013/src/programming/change_detection.rs:changed-res}}
```

资源也支持增改检测.`Res<T>.is_changed()`, `Option<Res<T>>.is_added()`.

## What gets detected?

[`Changed`] detection is triggered by [`DerefMut`]. Simply accessing
[components][cb::component] via a mutable [query][cb::query], or
[resources][cb::res] via [`ResMut`], without actually performing a `&mut`
access, will *not* trigger it. This makes change detection quite accurate.

Note: if you call a Rust function that takes a `&mut T` (mutable borrow),
that counts! It will trigger change detection even if the function does
not actually do any mutation. Be careful with helper functions!

Also, when you mutate a component, Bevy does not check if the new value
is actually different from the old value. It will always trigger the change
detection. If you want to avoid that, simply check it yourself:

```rust,no_run,noplayground
{{#include ../code013/src/programming/change_detection.rs:change-if-wrap}}
```

Change detection works on a per-[system][cb::system] granularity, and is
reliable. A system will detect changes only if it has not seen them before
(the changes happened since the last time it ran).

Unlike [events][cb::event], you do *not* have to worry about missing changes
If your system only runs sometimes (such as when using [states][cb::state]
or [run conditions][cb::rc]).

`变更检测`是由`DerefMut`触发的.

简单的获取可变组件/资源并不执行&mut 访问,是不会触发变更检测的.

如果是rust函数有个`&mut T`参数,这会触发可变检测(估计bevy对system函数有特殊处理),
所以要警惕辅助函数.

如果修改组件了,bevy不会检查前后两次的值是否相等,而是直接触发`变更检测`,
这个避免的方式是在修改组件之前,先做前后值比较,如果相等就不修改了.

变更检测以每个系统为粒度进行工作,并且是可靠的.
仅当系统以前未见过更改(自上次运行以来发生的更改)时,系统才会检测到更改.

这个变更检测不像事件,变更检测是可靠的;事件叠加状态和运行条件,可能会漏掉.

## Possible Pitfalls

Beware of frame delay / 1-frame-lag. This can occur if Bevy runs the detecting
system before the changing system. The detecting system will see the change
the next time it runs, typically on the next frame update.

If you need to ensure that changes are handled immediately / during the same
frame, you can use [explicit system ordering][cb::system-order].

变更检测的逻辑是放在system中的,所以检测和实际的变更是可能存在不确定顺序的,
如果先检测再变更,只能再下帧才会收集到变更信息.

如果想要顺序必须正确,就使用system的顺序约束来保证.

---

# Removal Detection

Relevant official examples:
[`removal_detection`][example::removal_detection].

---

Removal detection is special. This is because, unlike with [change
detection][cb::change-detection], the data does not exist in the ECS anymore
(obviously), so Bevy cannot keep tracking metadata for it.

Nevertheless, being able to respond to removals is important for some
applications, so Bevy offers a limited form of it.

删除检测,删除意味着ECS都删除了,就没法跟踪了(其实是元数据被删掉了).
bevy还是提供了一种带限制的方式.

## Components

You can check for [components][cb::component] that have been removed during the
current frame. The data is cleared at the end of every frame update. You must
make sure your detecting [system][cb::system] [is ordered after][cb::system-order]
(or is in another [schedule][cb::schedule] that runs after) the system that
does the removing.

Note: removal detection also includes despawned entities!

Use the [`RemovedComponents<T>`] special system parameter type. Internally, it
is implemented using [events][cb::event] and behaves like an [`EventReader`],
but it gives you the [`Entity`] IDs of entities whose component `T` was removed.

```rust,no_run,noplayground
{{#include ../code013/src/programming/change_detection.rs:removal-detection}}
```

(To do things with these entities, you can just use the `Entity` IDs with
[`Commands::entity()`][cb::commands] or [`Query::get()`][cb::query].)

删除组件,其元数据的删除是在每帧更新的最后执行的,那么删除检测就需要在删除system之后就行.

实体删除组件;删除实体都会触发`删除变更`.

`RemovedComponents<T>`是删除组件事件接收器,和`EventReader`很像, 会提供实体ID.

## Resources

Bevy does not provide any API for detecting when [resources][cb::res] are removed.

You can work around this using [`Option`] and a separate [`Local`][cb::local]
system parameter, effectively implementing your own detection.

```rust,no_run,noplayground
{{#include ../code013/src/programming/change_detection.rs:res-removal-detection}}
```

Note that, since this detection is local to your system, it does not have
to happen during the same frame update.

bevy没有提供资源删除检测的api.但可以通过Option来跟踪资源,通过Local来记录资源是否存在.
