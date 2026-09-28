# yielder

BORUIX 的调度压测程序：一个永不阻塞、只反复让出 CPU 的进程。

[English](README.en.md)

## 用途

它是「队列里只剩一个忙让出进程」这一场景的最小复现体：打印一行启动信息后进入循环，反复调用
让出接口，不睡眠、不阻塞，永远处于就绪状态。

用于验证调度器在这种进程常驻时，仍能让其它进程（如 shell）及时响应：启动本程序后操作终端，
输入应无延迟、无积压。

它没有判定与退出码，行为本身就是被观测的对象。

## 对照

与 [`spinburn`](https://github.com/BRX-Boruix/spinburn) 配对使用：spinburn 以 CPU 突发加睡眠
交替运行，多数时间处于阻塞；本程序永远就绪。两者覆盖调度压力的两个极端。

## 构建

```bash
cargo build --release
```

## 文件结构

```
yielder/
├── Cargo.toml    # 包定义
├── build.rs      # 注入链接脚本
├── linker.ld     # 用户态段布局
└── src/
    └── main.rs   # 让出循环
```

## 相关项目

- [`spinburn`](https://github.com/BRX-Boruix/spinburn) —— 突发加睡眠的对照压测
- [`libsys`](https://github.com/BRX-Boruix/libsys) —— 让出接口

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。
