use std::thread;

/// Python's `run_concurrent_tasks`, which fairspec-dataset keeps private: chunks of up to
/// `concurrency` items run on scoped threads, results in input order.
pub fn run_concurrent_tasks<T, R, F>(func: F, items: Vec<T>, concurrency: Option<usize>) -> Vec<R>
where
    T: Send,
    R: Send,
    F: Fn(T) -> R + Sync,
{
    let size = concurrency
        .unwrap_or_else(|| thread::available_parallelism().map_or(1, usize::from))
        .max(1);
    let mut results = Vec::with_capacity(items.len());
    let mut entries = items.into_iter().peekable();
    while entries.peek().is_some() {
        let chunk: Vec<T> = entries.by_ref().take(size).collect();
        if chunk.len() == 1 {
            results.extend(chunk.into_iter().map(&func));
            continue;
        }
        let func = &func;
        thread::scope(|scope| {
            let handles: Vec<_> = chunk
                .into_iter()
                .map(|entry| scope.spawn(move || func(entry)))
                .collect();
            for handle in handles {
                results.push(
                    handle
                        .join()
                        .unwrap_or_else(|panic| std::panic::resume_unwind(panic)),
                );
            }
        });
    }
    results
}
