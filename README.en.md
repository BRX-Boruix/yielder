# yielder

A BORUIX scheduler stress program: a process that never blocks and only yields the CPU.

[简体中文](README.md)

## Purpose

It is the minimal reproduction of the "queue holds only a busy-yielding process" scenario: after
printing one startup line it enters a loop that repeatedly calls the yield interface — no sleep,
no blocking, permanently ready.

It exists to verify that the scheduler still gives other processes (such as the shell) timely
response while such a process is resident: run it, then use the terminal — input should be
immediate and unqueued.

It has no verdict and no exit code; its behaviour is what gets observed.

## Counterpart

Paired with [`spinburn`](https://github.com/BRX-Boruix/spinburn): spinburn alternates CPU bursts
with sleeps and spends most of its time blocked; this program is always ready. Together they cover
the two extremes of scheduling pressure.

## Building

```bash
cargo build --release
```

## Repository layout

```
yielder/
├── Cargo.toml    # package manifest
├── build.rs      # injects the linker script
├── linker.ld     # user-space segment layout
└── src/
    └── main.rs   # the yield loop
```

## Related projects

- [`spinburn`](https://github.com/BRX-Boruix/spinburn) — the burst-and-sleep stress counterpart
- [`libsys`](https://github.com/BRX-Boruix/libsys) — the yield interface

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
