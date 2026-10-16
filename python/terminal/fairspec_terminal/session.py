from __future__ import annotations

import atexit
import json
import sys
from typing import TYPE_CHECKING, Callable, TypeVar

import typer
from rich.syntax import Syntax
from rich.console import Console
from rich.status import Status

from fairspec_metadata import FairspecException

from .actions.error.render import render_error

if TYPE_CHECKING:
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

    def render_text_result(self, text: str, *, status: str | None = None) -> None:
        if self.silent:
            return

        if self.json:
            sys.stdout.write(json.dumps({"result": text}, indent=2))
            sys.stdout.write("\n")
            return

        if not status:
            self._console.print(text)
            return

        self._console.print(f"{_render_status(status)} {text}")

    def render_data_result(self, data: object) -> None:
        if self.silent:
            return

        if self.json:
            sys.stdout.write(json.dumps(data, indent=2, default=_json_default))
            sys.stdout.write("\n")
            return

        text = json.dumps(data, indent=2, default=_json_default)
        self._console.print(Syntax(text, "json", theme="monokai"))

    def render_frame_result(self, frame: object) -> None:
        if self.silent:
            return

        if self.json:
            import polars as pl

            assert isinstance(frame, pl.DataFrame)
            sys.stdout.write(json.dumps(frame.to_dicts(), indent=2, default=str))
            sys.stdout.write("\n")
            return

        sys.stdout.write(str(frame))
        sys.stdout.write("\n")

    def render_report_result(self, report: Report) -> None:
        if not self.silent:
            if self.json:
                sys.stdout.write(
                    json.dumps(
                        report.model_dump(exclude_none=True), indent=2, default=str
                    )
                )
                sys.stdout.write("\n")
            elif report.valid:
                self.render_text("Validation passed", status="success")
            else:
                for error in report.errors:
                    self.render_text(render_error(error), status="error")

        if not report.valid:
            raise typer.Exit(1)

    def task(self, title: str, func: Callable[[], T]) -> T:
        if self.json or self.silent:
            try:
                return func()
            except FairspecException as exception:
                if self.debug:
                    raise
                if self.json:
                    sys.stdout.write(json.dumps({"error": str(exception)}, indent=2))
                    sys.stdout.write("\n")
                if exception.report:
                    self.render_report_result(exception.report)
                raise typer.Exit(1) from exception
            except Exception as exception:
                if self.debug:
                    raise
                if self.json:
                    sys.stdout.write(json.dumps({"error": str(exception)}, indent=2))
                    sys.stdout.write("\n")
                raise typer.Exit(1) from exception

        with Status(title, console=self._console):
            try:
                return func()
            except FairspecException as exception:
                if self.debug:
                    raise
                if exception.report:
                    sys.stdout.write("\n")
                    self.render_report_result(exception.report)
                raise typer.Exit(1) from exception
            except Exception as exception:
                if self.debug:
                    raise
                self._console.print(f"[red]\u2716[/red] {title}: {exception}")
                raise typer.Exit(1) from exception


def _render_status(status: str) -> str:
    if status == "success":
        return "[green]\u2714[/green]"
    if status == "warning":
        return "[yellow]\u26a0[/yellow]"
    if status == "error":
        return "[red]\u2716[/red]"
    return ""


def _json_default(obj: object) -> object:
    if hasattr(obj, "model_dump"):
        return obj.model_dump(exclude_none=True)  # type: ignore[union-attr]
    return str(obj)
