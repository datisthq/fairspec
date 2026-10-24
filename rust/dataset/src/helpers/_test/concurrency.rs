use std::collections::HashSet;
use std::sync::{Barrier, Mutex};
use std::thread::{self, ThreadId};
use std::time::Duration;

use crate::helpers::concurrency::{iter_concurrent_chunks, run_concurrent_tasks};

#[test]
fn test_should_preserve_input_order() {
    let work = |number: u64| {
        thread::sleep(Duration::from_millis(10 * (number % 3)));
        number
    };
    let items: Vec<u64> = (0..12).collect();
    assert_eq!(run_concurrent_tasks(work, items.clone(), Some(4)), items);
}

#[test]
fn test_should_return_empty_list_for_no_items() {
    let result: Vec<i32> = run_concurrent_tasks(|item: i32| item, vec![], Some(4));
    assert!(result.is_empty());
}

#[test]
fn test_should_run_in_the_calling_thread_when_concurrency_is_one() {
    let ids = run_concurrent_tasks(|_: i32| thread::current().id(), (0..8).collect(), Some(1));
    let ids: HashSet<ThreadId> = ids.into_iter().collect();
    assert_eq!(ids, HashSet::from([thread::current().id()]));
}

#[test]
fn test_should_run_in_the_calling_thread_for_a_single_item() {
    let ids = run_concurrent_tasks(|_: i32| thread::current().id(), vec![1], Some(8));
    assert_eq!(ids, [thread::current().id()]);
}

// Python skips on a single core; Rust spawns real threads per task, so it always runs
#[test]
fn test_should_use_several_threads_when_concurrency_is_above_one() {
    let width = 4;
    let barrier = Barrier::new(width);
    let work = |_: usize| {
        barrier.wait();
        thread::current().id()
    };
    let ids = run_concurrent_tasks(work, (0..width).collect(), Some(width));
    assert_eq!(ids.into_iter().collect::<HashSet<_>>().len(), width);
}

#[test]
fn test_should_run_nested_calls_in_the_same_thread() {
    let outer = |_: i32| {
        let inner =
            run_concurrent_tasks(|_: i32| thread::current().id(), (0..4).collect(), Some(4));
        (thread::current().id(), inner)
    };
    let results = run_concurrent_tasks(outer, (0..4).collect(), Some(4));
    for (id, inner_ids) in results {
        assert!(inner_ids.iter().all(|inner_id| *inner_id == id));
    }
}

#[test]
fn test_should_raise_the_first_failing_task_error() {
    let work = |number: i32| {
        if number == 0 || number == 2 {
            return Err(format!("failed {number}"));
        }
        Ok(number)
    };
    let result: Result<Vec<i32>, String> = run_concurrent_tasks(work, (0..4).collect(), Some(4))
        .into_iter()
        .collect();
    assert_eq!(result.unwrap_err(), "failed 0");
}

#[test]
fn test_should_not_start_the_next_chunk_when_the_caller_breaks() {
    let calls = Mutex::new(Vec::new());
    let work = |number: i32| {
        calls.lock().unwrap().push(number);
        number
    };
    let mut chunks = iter_concurrent_chunks(work, (0..10).collect(), Some(2));
    chunks.next();
    drop(chunks);
    let mut calls = calls.into_inner().unwrap();
    calls.sort();
    assert_eq!(calls, [0, 1]);
}

#[test]
fn test_should_yield_single_item_chunks_when_serial() {
    let chunks: Vec<Vec<i32>> =
        iter_concurrent_chunks(|item: i32| item, (0..3).collect(), Some(1)).collect();
    assert_eq!(chunks, [vec![0], vec![1], vec![2]]);
}
