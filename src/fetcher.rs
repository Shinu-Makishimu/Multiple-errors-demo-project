//! “Async” file fetcher with a hidden self-reference bug.

use anyhow::Result;
use std::{fs, path::Path};

pub struct LocalFetcher;

impl LocalFetcher {
    /// Reads a file and *pretends* to return a borrowed slice.
    ///
    /// ---- FIRST ERROR -----------------------------------------------
    /// The compiler complains that we’re returning a reference that
    /// escapes the temporary `String` created inside the async block:
    ///
    /// error[E0515]: cannot return reference to local data ...
    /// ----------------------------------------------------------------
    pub async fn read<'a>(&'a self, path: &'a Path) -> Result<&'a str> {
        // 👇 Creates a brand-new String **owned inside this function**
        let data = tokio::task::spawn_blocking(move || fs::read_to_string(path))
            .await??;

        // 💥 Returning `&data` ties the reference’s lifetime to `'a`,
        // but `data` will be dropped at the end of this fn. Boom.
        Ok(&data)
    }
}
