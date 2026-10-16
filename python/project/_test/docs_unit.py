from __future__ import annotations

import json
import os
import re

import pytest

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
REPO = os.path.dirname(ROOT)
DOCS = ["AGENTS.md", "README.md", "CONTRIBUTING.md"]

PACKAGES = [
    "dataset",
    "fairspec",
    "library",
    "metadata",
    "project",
    "table",
    "terminal",
]

BUILTIN_SCRIPTS = ["install", "exec", "add", "remove", "dlx", "why", "publish"]


class TestDocs:
    @pytest.mark.parametrize("doc", DOCS)
    def test_names_only_scripts_that_exist(self, doc: str) -> None:
        scripts = read_scripts()
        for name in re.findall(r"pnpm (?:run )?([\w:-]+)", read_code(read_doc(doc))):
            if name in BUILTIN_SCRIPTS:
                continue
            assert name in scripts, f'{doc} names "pnpm {name}"'

    @pytest.mark.parametrize("doc", DOCS)
    def test_names_only_paths_that_exist(self, doc: str) -> None:
        for value in re.findall(r"`([^`\s]+)`", read_doc(doc)):
            if not get_is_repo_path(value):
                continue
            base = REPO if value.startswith(".claude/") else ROOT
            path = os.path.join(base, value.rstrip("/"))
            assert os.path.exists(path), f'{doc} names "{value}"'

    def test_exposes_skills_to_claude_code_as_a_symlink(self) -> None:
        path = os.path.join(REPO, ".claude", "skills")
        target = os.readlink(path)
        resolved = os.path.abspath(os.path.join(os.path.dirname(path), target))
        assert resolved == os.path.join(REPO, ".agents", "skills")


def read_doc(name: str) -> str:
    with open(os.path.join(ROOT, name), encoding="utf-8") as file:
        return file.read()


def read_scripts() -> list[str]:
    scripts: list[str] = []
    for base in [REPO, ROOT]:
        with open(os.path.join(base, "package.json"), encoding="utf-8") as file:
            scripts.extend(json.load(file)["scripts"])
    return scripts


def read_code(content: str) -> str:
    fences = re.findall(r"```[\w]*\n([\s\S]*?)```", content)
    spans = re.findall(r"`([^`\n]+)`", content)
    return "\n".join([*fences, *spans])


def get_is_repo_path(value: str) -> bool:
    value = value.rstrip("/")
    if "/" not in value:
        return False
    if value.startswith(".claude/"):
        return True
    return any(value.startswith(f"{name}/") for name in PACKAGES)
