//! Generic “retry with back-off” helper.
//!
//! Appears only *after* fetcher.rs compiles.

use std::future::Future;

/// Retry up to N times with exponential back-off (2^k ms).
///
/// The function is written with a GAT to avoid boxing each Future.
/// We also try to express the back-off table as a const-generic array.
///
/// ---------------- SECOND ERROR ------------------------------------
/// 1. `generic_const_exprs` is *still* unstable, so the const math
///    `1 << K` below triggers:
///
///    error[E0658]: generic const expressions are unstable
///
/// 2. **Even if** you switch to nightly and turn the feature gate on,
///    the recursive call inside the GAT makes *another* error pop up:
///
///    error[E0720]: opaque type expands to a recursive type
/// ------------------------------------------------------------------
pub async fn retry<F, T, E, const N: usize>(mut f: F) -> Result<T, E>
where
    F: for<'a> FnMut() -> F::Fut<'a>,
    F: 'static,
    E: 'static,
// ---- GAT: each call gets its own concrete Future type
    F: RetryFuture,
{
    let mut attempt = 0;
    loop {
        match f().await {
            Ok(v) => break Ok(v),
            Err(err) if attempt < N => {
                attempt += 1;
                // Generic-const back-off table
                const BACKOFF: [u64; N] = backoff_table::<N>();
                tokio::time::sleep(std::time::Duration::from_millis(BACKOFF[attempt]))
                    .await;
            }
            Err(e) => break Err(e),
        }
    }
}

/// Build `[1, 2, 4, …, 2^(N-1)]` at compile time.
const fn backoff_table<const N: usize>() -> [u64; N] {
    let mut arr = [0u64; N];
    let mut k = 0;
    while k < N {
        arr[k] = 1u64 << k;          // ← generic const expr
        k += 1;
    }
    arr
}

/// The GAT part
pub trait RetryFuture {
    type Fut<'a>: Future<Output = Result<Self::Ok, Self::Err>> + Send
    where
        Self: 'a;
    type Ok: Send;
    type Err: Send;

    fn make<'a>(&'a mut self) -> Self::Fut<'a>;
}
