{{#include ../include/header013.md}}

# System Piping

Relevant official examples:
[`system_piping`][example::system_piping].

---

You can compose a single Bevy [system][cb::system] from multiple Rust functions.

You can make functions that can take an input and produce an output, and be
connected together to run as a single larger system. This is called "system piping".

You can think of it as creating "modular" systems made up of multiple building
blocks. This way, you can reuse some common code/logic in multiple systems.

Note that system piping is *not* a way of communicating between systems.
If you want to pass data between systems, you should use [Events][cb::event]
instead.

Your functions will be combined and Bevy will treat them as if they were a
single big [system][cb::system] with all the combined system parameters for
data access.

将多个rust函数组合成单个system,在这个system中,函数有input/output,
最终组合成一个管道,这就是system管道.

bevy中system之间通信的机制是事件.system管道的提出不是为了解决system之间的通信.

## Example: Handling [`Result`]s

One useful application of system piping is to be able to return errors (allowing
the use of Rust's `?` operator) and then have a separate function for handling
them:

```rust,no_run,noplayground
{{#include ../code013/src/programming/system_piping.rs:system-io}}
```

Such functions cannot be [added][cb::app] individually as systems (Bevy
doesn't know what to do with the input/output). By "piping" them together,
we create a valid system that we can add:

```rust,no_run,noplayground
{{#include ../code013/src/programming/system_piping.rs:system-pipe}}
```

system管道的典型应用场景是返回错误(利用rust的?操作符),然后使用一个单独函数来处理错误.
这个在某些场景非常有用.

## Performance Warning

Beware that Bevy treats the whole chain as if it was a single big system, with
all the combined system parameters and their respective data access
requirements. This implies that parallelism could be limited, affecting
performance.

If you create multiple "piped systems" that all contain a common function which
contains any mutable access, that prevents all of them from running in parallel!

因为system组合了很多函数,导致参数多了很多,每个参数都要计算,性能就很差.

如果多个system管道都包含了一个公共函数,而这个公共函数是mut访问数据,
那么这些system不能并行执行.性能就会更差.
