//! BORUIX `yielder`：纯忙-yield 压测进程（判据实验用）。
//!
//! 与 `spinburn`（CPU 突发 + sleep 40ms，多数时间 Blocked）不同，本程序**永不**
//! 进入 Blocked：它只反复调用 `yield_now()`，是「唯一就绪进程 busy-yield」这一
//! 场景的最小复现体。
//!
//! 用途：验证调度器在「队列里只剩一个忙-yield 进程」时，是否仍能让其它进程
//! （如 shell）获得及时响应。见 docs/TODO/terminal-input.md 6.12.8。
#![no_std]
#![no_main]

use libsys::write;
use libsys::STDOUT;

#[unsafe(no_mangle)]
pub extern "C" fn user_main(_argc: isize, _argv: *const *const u8) -> i32 {
    let _ = write(STDOUT, b"[yielder] start (pure busy-yield)\n");
    loop {
        // 纯让出：不 sleep、不阻塞，永远 Ready。
        let _ = libsys::yield_now();
    }
}
