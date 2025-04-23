use std::future::Future;
#[cfg(not(test))]
use std::sync::LazyLock;
#[cfg(not(test))]
use tokio::runtime::{Builder, Runtime as TokioRuntime};

/// Global multi threaded tokio runtime
#[cfg(not(test))]
static RT: LazyLock<TokioRuntime> =
    LazyLock::new(|| Builder::new_multi_thread().enable_all().build().unwrap());

/// Wrapper around calling a Future that must be run within a tokio multi thread runtime.
/// This is required when the underlying holochain function uses `tokio::block_in_place`,
/// which must be run within a multi thread runtime.
pub(crate) struct MultiThreadRun;

impl MultiThreadRun {
    pub async fn exec<F, T>(future: F) -> T
    where
        F: Future<Output = T>,
    {
        // When a Future is called via uniffi bindings, there is no tokio runtime.
        // Instead they piggyback on the async runtime from the foreign calling language.
        //
        // See https://mozilla.github.io/uniffi-rs/latest/internals/async.html
        #[cfg(not(test))]
        return RT.block_on(future);

        // When a Future is called within a test, we already have a tokio multi thread runtime.
        #[cfg(test)]
        return future.await;
    }
}
