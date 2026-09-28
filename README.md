# yielder

**简体中文** | [English](#english)

BORUIX 的一个**调度器压测程序**——它不停地"让出 CPU"，用来检验调度器在这种极端情况下的表现。

```
[yielder] start (pure busy-yield)
```

---

## 它在做什么

这个程序启动后进入一个循环，反复执行同一个动作：**让出 CPU**（把运行机会交给别的进程）。

它不做计算、不睡眠、不等待任何东西——只是**不停地让出，然后立刻又变成可运行**。

这类进程的状态很特殊：

| 状态 | 含义 | 它是否处于 |
| --- | --- | --- |
| 运行中 | 正在占用 CPU | 短暂地 |
| 就绪 | 可以运行，等待被调度 | **几乎永远** |
| 阻塞 | 在等某件事（如睡眠） | **从不** |

关键在于它**从不进入阻塞态**。它总是在"可运行"的队列里，永远有活干——只是每次都主动让位。

## 为什么需要这样一个程序

这是为了验证一个具体的调度问题：**当队列里只剩下一个这样不停让出的进程时，其他进程还能不能
得到及时的响应？**

设想系统里有一个进程在不停地让出 CPU。它看起来"很有礼貌"——一直在让位。但如果调度器处理不当，
结果可能反而是**其它进程被饿死**：因为让出之后它立刻又是可运行的，调度器可能在它和其它进程之间
来回切换，而真正需要响应的进程（比如用户正在敲键盘的 shell）得不到足够的运行时间。

这个程序就是用来**复现这个场景**的最小例子。

## 它和另一个压测程序的区别

项目里还有一个类似的压测程序 `spinburn`，两者的差别恰好是实验设计的关键：

| 程序 | 行为 | 覆盖的场景 |
| --- | --- | --- |
| [`spinburn`](https://github.com/BRX-Boruix/spinburn) | 一小段计算，然后**睡眠** | 进程大部分时间处于**阻塞态** |
| **`yielder`** | 纯让出，**永不睡眠** | 进程**始终处于就绪态** |

为什么要分成两个程序？因为它们检验的是**调度器的不同分支**。

一个进程在不同状态下被调度器处理的方式不同。如果压测程序总是在睡眠，那就只测到了"睡醒之后
怎么被调度"这条路；而"进程一直在就绪队列里但不停让位"这条路上的问题，它根本碰不到。

**这两个程序合起来才构成完整的覆盖**。少任何一个，都会留下一个测不到的角落。

## 一个排除误诊的判据

这个程序还有一个更精细的用途：**帮助区分两类看起来相似、但根因完全不同的问题**。

当系统出现"某些进程响应变慢"的现象时，可能的原因至少有两种：

| 可能原因 | 说明 |
| --- | --- |
| **调度真的有问题** | 进程确实没被合理地分配运行机会 |
| **统计/记账有偏差** | 进程其实运行得不少，只是记录它的数据不准，看起来像被饿死 |

这两种情况的修复方向完全不同：前者要改调度逻辑，后者要改记账方式。**如果分不清，就可能去修一个
根本没有坏的东西**。

纯忙-让出的行为恰好能把这二者分开：这个程序的行为是**确定的、可预测的**——它就是不停地让出，
没有任何变量。所以在它运行时观察到的现象，可以更干净地归因。

## 观察方式

它本身不报告结果、也没有"通过/失败"的判定——它是一个**被观察的实验对象**，不是自检程序。

验证方式是**从外部观察**：让它跑起来，然后检查**其他进程是否仍然响应及时**（比如在 shell 里
敲键盘是否还有反应）。

**它永远不会自己退出**——除非被强制终止。这是设计如此：它的意义就在于持续制造那个特殊场景。

## 输出

启动时打印一行标识，之后不再输出。

这一点也是刻意的：如果它每次让出都打印一行，标准输出本身就会变成巨大的负担，反而干扰了要观测的
调度行为。**压测程序不应该给被测对象制造额外的、无关的压力。**

## 构建

```bash
cargo build --release
```

编译产物部署为 BORUIX 系统中的用户态程序，由测试流程拉起。

## 文件结构

```
yielder/
├── Cargo.toml    # 包定义
├── build.rs      # 注入链接脚本
├── linker.ld     # 用户态段布局
└── src/
    └── main.rs   # 纯忙-让出循环
```

## 相关项目

- [`spinburn`](https://github.com/BRX-Boruix/spinburn) —— 另一个压测程序（阻塞态路径）
- [`libsys`](https://github.com/BRX-Boruix/libsys) —— 用户态系统调用封装
- [`shell`](https://github.com/BRX-Boruix/shell) —— 响应性观察的窗口

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。

---

# English

[简体中文](#yielder) | **English**

A **scheduler stress program** for BORUIX — it yields the CPU without pause, to examine how the
scheduler behaves under that extreme.

```
[yielder] start (pure busy-yield)
```

---

## What it does

After starting, the program enters a loop repeating one action: **yield the CPU** (hand the chance to
run to another process).

It computes nothing, sleeps not at all, and waits for nothing — it just **yields endlessly and is
immediately runnable again**.

Its state is unusual:

| State | Meaning | Is it there? |
| --- | --- | --- |
| Running | Occupying the CPU | Briefly |
| Ready | Runnable, awaiting scheduling | **Almost always** |
| Blocked | Waiting on something, a sleep say | **Never** |

The crux is that it **never blocks**. It is always in the runnable queue with something to do — it
just steps aside every time.

## Why such a program is needed

It exists to verify a specific scheduling question: **when the queue holds only such a ceaselessly
yielding process, can other processes still respond in time?**

Imagine a process yielding the CPU without pause. It looks "polite" — always stepping aside. But
handled badly by the scheduler, the outcome can be the opposite: **other processes starve**. Because
the yielder is instantly runnable again, the scheduler may keep switching between it and the others,
leaving the process that genuinely needs to respond (the shell the user is typing into, say) without
enough time to run.

This program is the **minimal reproduction** of that situation.

## How it differs from the other stress program

The project has a similar stress program, `spinburn`, and the difference between them is precisely the
point of the experiment:

| Program | Behaviour | Scenario covered |
| --- | --- | --- |
| [`spinburn`](https://github.com/BRX-Boruix/spinburn) | A short burst of computation, then **sleep** | Mostly in the **blocked** state |
| **`yielder`** | Pure yielding, **never sleeps** | **Always in the ready** state |

Why two programs? Because they exercise **different branches of the scheduler**.

A process in different states is handled differently by the scheduler. If the stress program always
sleeps, only the path of "how a process is scheduled after waking" gets tested; the path of "a process
sits in the ready queue forever yet keeps yielding" is never touched.

**Together the two programs form complete coverage.** Drop either and a corner goes untested.

## A criterion that rules out a misdiagnosis

The program has a finer purpose too: **helping distinguish two problems that look alike but have
entirely different root causes**.

When the system shows "some processes respond slowly", at least two explanations are possible:

| Possible cause | Explanation |
| --- | --- |
| **Scheduling is genuinely wrong** | Processes really are not getting a fair share of runtime |
| **Accounting is off** | Processes get plenty of runtime; the data recording it is inaccurate and it *looks* like starvation |

The two call for entirely different fixes: the first needs the scheduling logic changed, the second the
accounting. **Fail to tell them apart and you may go and fix something that was never broken.**

Pure busy-yielding separates the two neatly: its behaviour is **deterministic and predictable** — it
yields and nothing else, with no variables. So what is observed while it runs can be attributed more
cleanly.

## How it is observed

It reports no results and has no pass/fail verdict — it is an **experimental subject to be observed**,
not a self-test.

Verification is **external observation**: run it, then check whether **other processes still respond
promptly** (whether the keyboard still answers in a shell, say).

**It never exits on its own** — only by force. That is by design: its whole point is to sustain that
particular situation.

## Output

It prints one identifying line at startup and nothing thereafter.

That too is deliberate: printing a line on every yield would make standard output itself an enormous
burden, interfering with the very scheduling behaviour under observation. **A stress program should
not impose extra, irrelevant pressure on the thing it is measuring.**

## Building

```bash
cargo build --release
```

The artifact is deployed as a user-space program in a BORUIX system, started by the test flow.

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

- [`spinburn`](https://github.com/BRX-Boruix/spinburn) — the other stress program (the blocked-state path)
- [`libsys`](https://github.com/BRX-Boruix/libsys) — the user-space syscall wrapper
- [`shell`](https://github.com/BRX-Boruix/shell) — the window onto responsiveness

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
