mod fetcher;
mod retry;

use fetcher::LocalFetcher;
use anyhow::Result;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let f = LocalFetcher;

    // ── Uses the first problematic function ─────────────────────────
    // After you fix fetcher.rs this compiles & runs,
    // but you’ll hit the retry.rs bug if you try to uncomment it.
    //
    // let data = retry::retry::<_, _, anyhow::Error, 5>(|| async {
    //     f.read(Path::new("README.md")).await
    // })
    // .await?;

    let _ = f.read(Path::new("README.md")).await?; // first hurdle
    Ok(())
}
