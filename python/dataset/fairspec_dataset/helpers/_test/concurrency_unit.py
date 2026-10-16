from __future__ import annotations

import os
import threading
import time

import pytest

from ..concurrency import iter_concurrent_chunks, run_concurrent_tasks


class TestRunConcurrentTasks:
    def test_should_preserve_input_order(self):
        def work(number: int) -> int:
            time.sleep(0.01 * (number % 3))
            return number

        assert run_concurrent_tasks(work, range(12), concurrency=4) == list(range(12))

    def test_should_return_empty_list_for_no_items(self):
        assert run_concurrent_tasks(lambda item: item, [], concurrency=4) == []

    def test_should_run_in_the_calling_thread_when_concurrency_is_one(self):
        idents = run_concurrent_tasks(
            lambda _: threading.get_ident(), range(8), concurrency=1
        )

        assert set(idents) == {threading.get_ident()}

    def test_should_run_in_the_calling_thread_for_a_single_item(self):
        idents = run_concurrent_tasks(lambda _: threading.get_ident(), [1], concurrency=8)

        assert idents == [threading.get_ident()]

    def test_should_use_several_threads_when_concurrency_is_above_one(self):
        width = min(4, os.cpu_count() or 1)
        if width < 2:
            pytest.skip("requires more than one core")

        barrier = threading.Barrier(width, timeout=5)

        def work(_: int) -> int:
            barrier.wait()
            return threading.get_ident()

        idents = run_concurrent_tasks(work, range(width), concurrency=width)

        assert len(set(idents)) == width

    def test_should_run_nested_calls_in_the_same_thread(self):
        def outer(_: int) -> tuple[int, list[int]]:
            inner = run_concurrent_tasks(
                lambda _: threading.get_ident(), range(4), concurrency=4
            )
            return threading.get_ident(), inner

        results = run_concurrent_tasks(outer, range(4), concurrency=4)

        for ident, inner_idents in results:
            assert set(inner_idents) == {ident}

    def test_should_raise_the_first_failing_task_error(self):
        def work(number: int) -> int:
            if number in (0, 2):
                raise ValueError(f"failed {number}")
            return number

        with pytest.raises(ValueError, match="failed 0"):
            run_concurrent_tasks(work, range(4), concurrency=4)


class TestIterConcurrentChunks:
    def test_should_not_start_the_next_chunk_when_the_caller_breaks(self):
        calls: list[int] = []
        lock = threading.Lock()

        def work(number: int) -> int:
            with lock:
                calls.append(number)
            return number

        for _ in iter_concurrent_chunks(work, range(10), concurrency=2):
            break

        assert sorted(calls) == [0, 1]

    def test_should_yield_single_item_chunks_when_serial(self):
        chunks = list(iter_concurrent_chunks(lambda item: item, range(3), concurrency=1))

        assert chunks == [[0], [1], [2]]
