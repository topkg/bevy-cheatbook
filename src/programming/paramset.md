{{#include ../include/header014.md}}

# Param Sets

For safety reasons, a [system][cb::system] cannot have multiple parameters
whose data access might have a chance of mutability conflicts over the
same data.

Some examples:
 - Multiple incompatible [queries][cb::query].
 - Using [`&World`] while also having other system parameters to access specific data.
 - …

Consider this example [system][cb::system]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/paramset.rs:conflict}}
```

The two [queries][cb::query] are both trying to mutably access `Health`. They
have different [filters][cb::query-filter], but what if there are entities that
have both `Player` and `Enemy` components? If we know that shouldn't happen, we
can add [`Without`] filters, but what if it is actually valid for our game?

Such code will compile (Rust cannot know about Bevy ECS semantics), but will
result in a runtime panic. When Bevy tries to run the system, it will panic with
a message about conflicting system parameters:

```
thread 'main' panicked at bevy_ecs/src/system/system_param.rs:225:5:
error[B0001]: Query<&mut game::Health, bevy_ecs::query::filter::With<game::Enemy>> in
system game::reset_health accesses component(s) game::Health in a way that conflicts
with a previous system parameter. Consider using `Without<T>` to create disjoint Queries
or merging conflicting Queries into a `ParamSet`.
```

Bevy provides a solution: wrap any incompatible parameters in a [`ParamSet`]:

```rust,no_run,noplayground
{{#include ../code014/src/programming/paramset.rs:paramset}}
```

This ensures only one of the conflicting parameters can be used at the same time.
Bevy will now happily run our system.

The maximum number of parameters in a param set is 8.

system的多个入参不能对同一数据有竞争访问.  
多个不兼容的query;同时拥有&world(独占世界)和其他数据的访问.

rust是不知道的bevy的ECS语义的,所以system入参竞争是无法通过编译器报告出来的,
运行时会导致panic.

bevy提供了一个方法: 使用ParamSet将不兼容的入参封装一下.
这样bevy会保证在同一时间只会有个一个竞争参数在使用.

ParamSet最大参数个数为8.
