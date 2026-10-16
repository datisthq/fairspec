from __future__ import annotations

import os
import threading
from collections.abc import Callable, Iterable, Iterator
from concurrent.futures import Future, ThreadPoolExecutor
from typing import TypeVar

T = TypeVar("T")
R = TypeVar("R")

_executor: ThreadPoolExecutor | None = None
_executor_lock = threading.Lock()
_state = threading.local()


def get_default_concurrency() -> int:
    """Return the default number of tasks to run at once."""
    return os.cpu_count() or 1


def get_is_concurrent_worker() -> bool:
    """Return whether the calling thread is running a concurrent task."""
    return getattr(_state, "is_worker", False)


def run_concurrent_tasks(
    func: Callable[[T], R],
    items: Iterable[T],
    *,
    concurrency: int | None = None,
) -> list[R]:
    """Apply a function to every item, returning results in input order."""
    results: list[R] = []

    for chunk in iter_concurrent_chunks(func, items, concurrency=concurrency):
        results.extend(chunk)

    return results


def iter_concurrent_chunks(
    func: Callable[[T], R],
    items: Iterable[T],
    *,
    concurrency: int | None = None,
) -> Iterator[list[R]]:
    """Apply a function to every item, yielding results in input order per chunk."""
    entries = list(items)
    size = max(1, concurrency if concurrency is not None else get_default_concurrency())

    if size == 1 or len(entries) <= 1 or get_is_concurrent_worker():
        for entry in entries:
            yield [func(entry)]
        return

    def run_task(entry: T) -> R:
        _state.is_worker = True
        try:
            return func(entry)
        finally:
            _state.is_worker = False

    executor = _get_executor()
    for start in range(0, len(entries), size):
        futures: list[Future[R]] = [
            executor.submit(run_task, entry) for entry in entries[start : start + size]
        ]
        yield [future.result() for future in futures]


def _get_executor() -> ThreadPoolExecutor:
    global _executor

    with _executor_lock:
        if _executor is None:
            _executor = ThreadPoolExecutor(
                max_workers=get_default_concurrency(),
                thread_name_prefix="fairspec",
            )
        return _executor
