{{#include ../include/header013.md}}

# Queries

Relevant official examples:
[`ecs_guide`][example::ecs_guide].

---

Queries let you access [components of entities][cb::ecs-intro].

Use the [`Query`] [system parameter][cb::system], where you can specify the data
you want to access, and optionally additional [filters][cb::query-filter].

Think of the types you put in your [`Query`] as a "specification" for selecting
what entities you want to access. Queries will match only those entities in the
ECS World that fit your specification. You are then able to access the relevant
data from any such entities.

The first type parameter for a query is the data you want to access. Use `&` for
shared/readonly access and `&mut` for exclusive/mutable access. Use [`Option`] if
the component is not required (you want to find entities with or without that
component. If you want multiple components, put them in a tuple.

通过query可以访问实体的组件.

```rsut
pub struct Query<'world, 'state, D, F = ()>
where
    D: QueryData, // 特型,用于从world中查找数据.
    F: QueryFilter, // 特型,用于过滤查询的结果.
{ /* private fields */ }
```

Query是一个泛型结构体,有两个参数.前一个参数只能是引用类型(不然所有权没了),
且如果不确定某个组件有没有,使用Option,如果实体包含此组件,就返回;否则返回空.
如果要访问多个组件,使用元组.

### Iterating

The most common operation is to simply iterate to access the component values of
every entity that matches the query:

```rust,no_run,noplayground
{{#include ../code013/src/programming/queries.rs:sys-simple-query}}
```

If you want to know the entity IDs of the entities you are accessing, you can
put the special [`Entity`] type in your query. This is useful if you need
to later perform specific operations on those entities.

```rust,no_run,noplayground
{{#include ../code013/src/programming/queries.rs:query-entity}}
```

上面显示了query组件最常见的遍历方式.
也可以query实体,得到的是实体ID,如上面的例子.

### Accessing Specific Entities

To access the [components][cb::component] from one specific [entity][cb::entity]
only, you need to know the [`Entity`] ID:

```rust,no_run,noplayground
{{#include ../code013/src/programming/queries.rs:query-get}}
```

If you want to access the data from several entities all at once, you can use
`many`/`many_mut` (panic on error) or `get_many`/`get_many_mut` (return
[`Result`]).  These methods ensure that all the requested entities exist and
match the query, and will produce an error otherwise.

```rust,no_run,noplayground
{{#include ../code013/src/programming/queries.rs:query-many}}
```

访问特定的实体,提前保存好实体id即可.
如果要同时访问多个实体,可以使用many/many_mut或get_many/get_many_mut.

### Unique Entities

If you know that only one matching entity is supposed to exist (the query is
expected to only ever match a single entity), you can use `single`/`single_mut`
(panic on error) or `get_single`/`get_single_mut` (return [`Result`]). These
methods ensure that there exists exactly one candidate entity that can match
your query, and will produce an error otherwise.

You do not need to know the [`Entity`] ID.

```rust,no_run,noplayground
{{#include ../code013/src/programming/queries.rs:query-single}}
```

唯一实体,eg:我们操纵的角色只有一个,此时可用single/single_mut或get_single/get_many_mut.
此时不需要实体ID.

### Combinations

If you want to iterate over all possible combinations of N entities, Bevy
provides a method for that too. Be careful: with a lot of entities, this
can easily become very slow!

```rust,no_run,noplayground
{{#include ../code013/src/programming/queries.rs:query-combinations}}
```

将query结果进行组合,使其变为数组.当实体数量大时,性能会很差.
这个的意义在与补充标准`query-for`的遍历方式.

## Bundles

Queries work with individual components. If you created an entity using a
[bundle][cb::bundle], you need to query for the specific components from
that bundle that you care about.

A common beginner mistake is to query for the bundle type!

bundle只在构造时有用,不能用在query处.

## Query Filters

Add query filters to narrow down the entities you get from the query.

This is done using the second (optional) generic type parameter of the
[`Query`] type.

Note the syntax of the query: first you specify the data you want to access
(using a tuple to access multiple things), and then you add any additional
filters (can also be a tuple, to add multiple).

Use [`With`]/[`Without`] to only get entities that have specific components.

```rust,no_run,noplayground
{{#include ../code013/src/programming/queries.rs:sys-query-filter}}
```

This is useful if you don't actually care about the data stored inside these
components, but you want to make sure that your query only looks for entities
that have (or not have) them. If you want the data, then put the component in
the first part of the query (as shown previously), instead of using a filter.

Multiple filters can be combined:
 - in a tuple to apply all of them (AND logic)
 - using the `Or<(…)>` wrapper to detect any of them (OR logic).
   - (note the tuple inside)

query的第二个泛型参数是过滤,可省略,也可以是元组组成.
多个过滤可以进行组合:
 - 用元组,里面是`逻辑与`关系
 - 用`Or<..>`, `逻辑或`关系

## Query Transmutation

If you want one function with a [`Query`] parameter to call another function
with a different (but compatible) [`Query`] parameter, you can create the
needed [`Query`] from the one you have using something called [`QueryLens`].

```rust,no_run,noplayground
{{#include ../code013/src/programming/queries.rs:query-transmute}}
```

Note: when we call `debug_positions` from each function, it will access
different entities! Even though the `Query<&Transform>` parameter type does not
have any additional [filters][cb::query-filter], it was created by transmuting
via [`QueryLens`], and therefore it can only access the entities and components
of the original [`Query`] that it was derived from. If we were to add
`debug_positions` to Bevy as a regular system, it would access the transforms of
all entities.

Also note: this has some performance overhead; the transmute operation is not
free. Bevy normally caches some query metadata across multiple runs of a
system. When you create the new query, it has to make a copy of it.

部分场景下,system会调用另一个函数,另一个函数的参数也是一个query,
只不过这个query参数是调用方system 入参query的兼容部分(只是包含部分组件),
如上面的例子所示,可以使用一个叫`QueryLen`的技术来实现转换.

```rust
pub fn transmute_lens<NewD>(&mut self) -> QueryLens<'_, NewD>
where
    NewD: QueryData,
```

这是对QueryData的处理,将要查询的组件缩小了范围.组件范围缩小了,
意味着将system函数拆分为小函数的过程中,部分小函数只关心自己需要的那部分数据,
好处是小函数也容易复用.

这么做也是有代价的,性能是一个,内存也是一个.
因为bevy通常会缓存查询的元数据,方便多次调用system时能提高性能,
但新建一个query时,需要拷贝.

