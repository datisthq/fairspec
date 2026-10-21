use std::cell::Cell;
use std::thread;

thread_local! {
    static IS_WORKER: Cell<bool> = const { Cell::new(false) };
}

pub fn get_default_concurrency() -> usize {
    thread::available_parallelism().map_or(1, |count| count.get())
}

/// Whether the calling thread is running a concurrent task; nested calls then run inline.
pub fn get_is_concurrent_worker() -> bool {
    IS_WORKER.with(Cell::get)
}

/// Applies `func` to every item, returning results in input order.
pub fn run_concurrent_tasks<T, R, F>(func: F, items: Vec<T>, concurrency: Option<usize>) -> Vec<R>
where
    T: Send,
    R: Send,
    F: Fn(T) -> R + Sync,
{
    iter_concurrent_chunks(func, items, concurrency)
        .flatten()
        .collect()
}

/// Applies `func` to every item lazily, yielding results in input order per chunk.
pub fn iter_concurrent_chunks<T, R, F>(
    func: F,
    items: Vec<T>,
    concurrency: Option<usize>,
) -> impl Iterator<Item = Vec<R>>
where
    T: Send,
    R: Send,
    F: Fn(T) -> R + Sync,
{
    let size = concurrency.unwrap_or_else(get_default_concurrency).max(1);
    let is_sequential = size == 1 || items.len() <= 1 || get_is_concurrent_worker();
    let chunk_size = if is_sequential { 1 } else { size };

    let mut entries = items.into_iter().peekable();
    std::iter::from_fn(move || {
        entries.peek()?;
        let chunk: Vec<T> = entries.by_ref().take(chunk_size).collect();
        if is_sequential {
            return Some(chunk.into_iter().map(&func).collect());
        }
        let func = &func;
        Some(thread::scope(|scope| {
            let handles: Vec<_> = chunk
                .into_iter()
                .map(|entry| {
                    scope.spawn(move || {
                        IS_WORKER.with(|is_worker| is_worker.set(true));
                        func(entry)
                    })
                })
                .collect();
            handles
                .into_iter()
                .map(|handle| {
                    handle
                        .join()
                        .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
                })
                .collect()
        }))
    })
}
