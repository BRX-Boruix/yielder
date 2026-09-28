# yielder

A BORUIX **scheduler stress program**: it yields the CPU without pause and **never blocks**.

[简体中文](README.md)

## What it does

After starting it enters a loop repeating one action: **yield the CPU**. It computes nothing, sleeps not at all, and waits for nothing.

Its state is unusual:

| State | Is it there? |
| --- | --- |
| Running | Briefly |
| Ready | **Almost always** |
| Blocked | **Never** |

## What it verifies

**When the queue holds only such a ceaselessly yielding process, can other processes still respond in time?**

A process that yields endlessly looks "polite", but handled badly by the scheduler the outcome can be other processes starving — it is instantly runnable again, so the scheduler may keep switching between it and the others, leaving the process that genuinely needs to respond (the `shell` a user is typing into, say) without enough runtime. This program is the minimal reproduction of that situation.

## How it differs from spinburn

| Program | Behaviour | State covered |
| --- | --- | --- |
| [`spinburn`](https://github.com/BRX-Boruix/spinburn) | Computes, then **sleeps** | **Blocked** most of the time |
| **`yielder`** | Pure yielding, **never sleeps** | **Always ready** |

The two exercise different branches of the scheduler and **together form complete coverage**: testing only the sleep path never touches the problems on the "always in the ready queue yet keeps yielding" path.

## A criterion that rules out a misdiagnosis

When a system shows "responses are getting slow", at least two explanations are possible, calling for entirely different fixes:

| Possible cause | Fix |
| --- | --- |
| Scheduling is genuinely wrong | Change the scheduling logic |
| Accounting is off | Change how it is recorded |

This program's behaviour is deterministic and predictable (it yields and nothing else, with no variables), so what is observed while it runs can be attributed more cleanly, separating the two.

## Usage

Once started, **observe from outside** — check whether other processes still respond promptly. It has no verdict and **never exits on its own**; it must be forcibly terminated.

```
[yielder] start (pure busy-yield)
```

Nothing further after startup: printing on every yield would make standard output itself a burden, interfering with the scheduling behaviour under observation.

## Building

```bash
cargo build --release
```

## Layout

```
yielder/
├── Cargo.toml    # package definition
├── build.rs      # injects the linker script
├── linker.ld     # user-space section layout
└── src/
    └── main.rs   # the pure busy-yield loop
```

## Related projects

- [`spinburn`](https://github.com/BRX-Boruix/spinburn) — the blocked-state stress contrast
- [`libsys`](https://github.com/BRX-Boruix/libsys) — the user-space syscall wrapper

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
