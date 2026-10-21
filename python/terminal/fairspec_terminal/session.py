from __future__ import annotations

import atexit
import json
import sys
from typing import TYPE_CHECKING, Callable, TypeVar

import click
from rich.console import Console
from rich.status import Status
from rich.syntax import Syntax
from rich.text import Text

from fairspec_metadata import FairspecException

from .actions.error.render import render_error
from .models.output import ReportResult, TextResult

if TYPE_CHECKING:
    import polars as pl
    from fairspec_metadata import Report

T = TypeVar("T")


class Session:
    silent: bool
    debug: bool
    json: bool

    def __init__(
        self, *, silent: bool = False, debug: bool = False, json: bool = False
    ) -> None:
        self.silent = silent
        self.debug = debug
        self.json = json
        self._console = Console()

        if not self.silent and not self.json:
            sys.stdout.write("\n")
            atexit.register(lambda: sys.stdout.write("\n"))

    def render_text(self, text: str, *, status: str | None = None) -> None:
        if self.silent or self.json:
            return

        if not status:
            self._console.print(text)
            return

        self._console.print(f"{_render_status(status)} {text}")

    def render_text_result(self, text: str, *, status: str | None = None) -> TextResult:
        self.render_text(text, status=status)
        return TextResult(result=Text.from_markup(text).plain)

    def render_raw_result(self, text: str) -> None:
        if self.silent or self.json:
            return

        # bypasses the console, whose markup would swallow "[...]" in file content
        sys.stdout.write(text if text.endswith("\n") else f"{text}\n")

    def render_data_result(self, data: T) -> T:
        if not self.silent and not self.json:
            text = json.dumps(data, indent=2, default=_json_default)
            self._console.print(Syntax(text, "json", theme="monokai"))

        return data

    def render_frame_result(self, frame: pl.DataFrame) -> list[object]:
        if self.json:
            return list(frame.to_dicts())

        if not self.silent:
            sys.stdout.write(str(frame))
            sys.stdout.write("\n")

        return []

    def render_report_result(self, report: Report) -> ReportResult:
        if not report.valid:
            _exit_on_close(1)

        if self.json:
            errors = [error.model_dump(exclude_none=True) for error in report.errors]
            return ReportResult(valid=report.valid, errors=errors)

        if not self.silent:
            if report.valid:
                self.render_text("Validation passed", status="success")
            else:
                for error in report.errors:
                    self.render_text(render_error(error), status="error")

        return ReportResult(valid=report.valid, errors=list(report.errors))

    def task(self, title: str, func: Callable[[], T]) -> T:
        if self.json or self.silent:
            try:
                return func()
            except Exception as exception:
                if self.debug:
                    raise
                if self.json:
                    sys.stdout.write(json.dumps({"error": str(exception)}, indent=2))
                    sys.stdout.write("\n")
                raise click.exceptions.Exit(1) from exception

        with Status(title, console=self._console):
            try:
                return func()
            except FairspecException as exception:
                if self.debug:
                    raise
                if exception.report:
                    sys.stdout.write("\n")
                    self.render_report_result(exception.report)
                raise click.exceptions.Exit(1) from exception
            except Exception as exception:
                if self.debug:
                    raise
                self._console.print(f"[red]✖[/red] {title}: {exception}")
                raise click.exceptions.Exit(1) from exception


# fireargs prints the result after the handler returns, so the exit waits for the
# command's context to close
def _exit_on_close(code: int) -> None:
    context = click.get_current_context(silent=True)
    if context is None:
        return

    def exit() -> None:
        raise click.exceptions.Exit(code)

    context.call_on_close(exit)


def _render_status(status: str) -> str:
    if status == "success":
        return "[green]✔[/green]"
    if status == "warning":
        return "[yellow]⚠[/yellow]"
    if status == "error":
        return "[red]✖[/red]"
    return ""


def _json_default(obj: object) -> object:
    if hasattr(obj, "model_dump"):
        return obj.model_dump(exclude_none=True)  # type: ignore[union-attr]
    return str(obj)
