// ============================================================
// 第 16 课：异步与 await (async/await & Futures)
// ------------------------------------------------------------
// 学习目标：
//   1. 理解 async fn 和 .await 的本质：返回 Future + 暂停/恢复点。
//   2. 理解 Future 是惰性的，必须由"运行时 (runtime)"驱动执行。
//   3. 用一个最小可运行的执行器把概念跑通。
//   4. 知道生产项目应该用 tokio / async-std 之类成熟运行时。
// 前置知识：第 11、13 课。
// 本课覆盖：async fn / .await / Future / 最小执行器 / 运行时选择。
// 运行方式：
//   rustc 16_async_await.rs && ./16_async_await
//   （本课刻意不引入任何第三方依赖，让你看到"async 是什么"的本质）
// ============================================================
//
// 一句话：
//   async fn  —— 编译器帮你把"普通函数体"改写成一个"状态机 / Future"。
//   .await    —— 等待另一个 Future 完成的"暂停点"，期间所在任务可以让出 CPU。
//   runtime   —— 负责拿到 Future 并不停 poll，直到 Ready；真实运行时还做 IO 调度、定时器、任务调度。

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

fn main() {
    // ------------------------------------------------------------
    // 1. async fn 看起来像普通函数，但调用它"不"立刻执行；
    //    它返回一个 Future，Future 必须被 poll 才会推进。
    // ------------------------------------------------------------
    let future = async_main(); // <-- 没有执行任何代码，只是构造了状态机
    let result = block_on(future);
    println!("异步结果 = {result}");

    // ------------------------------------------------------------
    // 2. async 块：临时构造 Future，不必显式定义函数
    // ------------------------------------------------------------
    let result2 = block_on(async {
        let a = fetch_user_id().await;
        let b = fetch_score(a).await;
        a + b
    });
    println!("async block 结果 = {result2}");
}

// ------------------------------------------------------------
// 3. async fn 示例：用 .await 把多个异步步骤"串"起来
//    这看起来和同步代码一模一样，但每个 .await 都是一个潜在的暂停点：
//    在真实运行时上，这里能切去执行别的任务，IO 完成时再唤醒。
// ------------------------------------------------------------
async fn async_main() -> u32 {
    let user_id = fetch_user_id().await;
    let score = fetch_score(user_id).await;
    score + 1
}

async fn fetch_user_id() -> u32 {
    // 真实项目里这里可能是 HTTP / 数据库 / 文件 IO
    42
}

async fn fetch_score(user_id: u32) -> u32 {
    user_id * 2
}

// ============================================================
// 4. 最小执行器 (executor)
// ------------------------------------------------------------
// 真实运行时（tokio、async-std、smol）做的事情，可以拆成两部分：
//   - 调度器：决定"现在 poll 哪个 Future"。
//   - 反应器：监听 IO 就绪、定时器到期，调用 Waker 唤醒任务。
//
// 下面的 block_on 是一个"傻瓜版"执行器：只能处理"一次 poll 就 Ready"的 Future。
// 对真正会 Pending 的 Future（比如等待网络）几乎无法正确运行。
// 这里仅用于解释 Future 的运转机制。
// ============================================================
fn block_on<F: Future>(future: F) -> F::Output {
    let waker = dummy_waker();
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);

    loop {
        match Future::poll(Pin::as_mut(&mut future), &mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => std::thread::yield_now(), // 真实运行时会等待 Waker 通知
        }
    }
}

// 一个"什么也不做"的 Waker —— 真实场景里 Waker 负责唤醒挂起的任务
fn dummy_waker() -> Waker {
    unsafe fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    let raw_waker = RawWaker::new(std::ptr::null(), &VTABLE);
    // SAFETY: VTABLE 中所有函数都是空实现，且 RawWaker 不持有真实资源。
    unsafe { Waker::from_raw(raw_waker) }
}

// ============================================================
// 在真实项目里你会怎么写 —— 仅作示意，不在本文件运行
// ------------------------------------------------------------
// Cargo.toml:
//   [dependencies]
//   tokio = { version = "1", features = ["full"] }
//   reqwest = { version = "0.12", features = ["json"] }
//
// src/main.rs:
//   use anyhow::Result;
//
//   #[tokio::main]                          // 帮你启动 tokio 运行时
//   async fn main() -> Result<()> {
//       // 1) 串行：用 .await 一步步等
//       let body = reqwest::get("https://httpbin.org/get").await?.text().await?;
//       println!("body len = {}", body.len());
//
//       // 2) 并发：用 join! 让两个 Future 同时跑
//       let (a, b) = tokio::join!(fetch("/a"), fetch("/b"));
//       println!("a={a:?}, b={b:?}");
//
//       // 3) 派生任务：放到调度器后台跑
//       let h = tokio::spawn(async { heavy().await });
//       let v = h.await?;
//       println!("v = {v}");
//
//       // 4) 超时：tokio::time::timeout
//       let r = tokio::time::timeout(
//           std::time::Duration::from_secs(2),
//           slow_call(),
//       ).await;
//       println!("timeout result = {r:?}");
//       Ok(())
//   }
//   async fn fetch(p: &str) -> Result<String> { /* ... */ unimplemented!() }
//   async fn heavy() -> i32 { 42 }
//   async fn slow_call() -> i32 { 0 }
// ============================================================

// ============================================================
// 本课小结
//   - async fn / async {} 不会立刻执行，它们生成 Future。
//   - .await 是一个暂停点：当前任务挂起，等待该 Future 完成。
//   - 必须有运行时来 poll Future——单文件玩具运行时只能跑"立即就绪"的 Future。
//   - 真实项目几乎一定用 tokio（生态最大，axum / reqwest / sqlx 都基于它）。
//
// 常见陷阱
//   - 调用 async fn 不 .await → Future 没人推动，等于什么都没做（编译器会给 warning）。
//   - 在 async 里用 std::thread::sleep / std::fs 等阻塞 API
//       → 会阻塞整个执行线程，把同事的任务全卡死。
//       要用 tokio::time::sleep / tokio::fs。
//   - 持有 std::sync::Mutex 的 guard 跨 .await
//       → 编译报错或潜在死锁，改用 tokio::sync::Mutex 或缩小持有范围。
//   - CPU 密集任务直接放在 async 里 → 拖垮 reactor，应该 tokio::task::spawn_blocking。
//   - 想着"加更多 await 就更并发" → 串行的 a.await; b.await 仍然是串行的；
//       真正并发要 tokio::join! / futures::join_all / select!。
//
// 最佳实践
//   - 选 tokio，#[tokio::main] 起步。
//   - 异步代码里只用异步 API；阻塞 / CPU 密集 → spawn_blocking。
//   - 取消任务用 select! + 取消令牌 / drop handle；不要自己写 flag。
//   - 处理大量并发请求用 buffer_unordered / Semaphore 控制并发度。
//   - 调用层接口用 async fn ...->Result<T, anyhow::Error>，错误用 ? 传播。
//
// 思考题
//   1. async fn foo() -> i32 { 42 } 的真实返回类型是什么？为什么不能直接拿到 i32？
//   2. .await 编译后大致变成了什么？(提示：状态机 + 局部变量保存)
//   3. 单线程运行时和多线程运行时各适合什么场景？
//   4. 为什么"协程 / async"在 Rust 这种没 GC 的语言里特别难做（提示：自引用、Pin）？
// ============================================================
