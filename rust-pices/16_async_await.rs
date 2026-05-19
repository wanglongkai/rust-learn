use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

fn main() {
    // async 函数返回 Future。Future 本身是惰性的，需要执行器轮询才会运行。
    let future = async_main();
    let result = block_on(future);
    println!("异步结果 = {result}");
}

async fn async_main() -> u32 {
    let user_id = fetch_user_id().await;
    let score = fetch_score(user_id).await;
    score + 1
}

async fn fetch_user_id() -> u32 {
    // 真实项目中，这里可能是网络请求、文件 IO 或数据库查询。
    42
}

async fn fetch_score(user_id: u32) -> u32 {
    user_id * 2
}

// 这是一个最小演示执行器，只适合轮询立即就绪的 Future。
// 生产项目通常使用 tokio、async-std 或 smol 等运行时。
fn block_on<F: Future>(future: F) -> F::Output {
    let waker = dummy_waker();
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);

    loop {
        match Future::poll(Pin::as_mut(&mut future), &mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn dummy_waker() -> Waker {
    unsafe fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    let raw_waker = RawWaker::new(std::ptr::null(), &VTABLE);

    // SAFETY: 这个 RawWaker 不持有真实资源，只用于演示立即完成的 Future。
    unsafe { Waker::from_raw(raw_waker) }
}
