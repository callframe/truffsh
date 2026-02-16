#!/usr/bin/env python3
"""Generate build.ninja for neosh."""

from dataclasses import dataclass, fields
from json import loads
from pathlib import Path
from sys import executable as python_executable

from ninja import Writer

NINJA_WIDTH = 80
SCRIPT_NAME = Path(__file__).name

ROOT_DIR = Path(__file__).parent
BUILD_DIR = ROOT_DIR / "build"
TOOLCHAIN_FILE = ROOT_DIR / "toolchain.json"
NINJA_FILE = ROOT_DIR / "build.ninja"
NEOSH_BIN = BUILD_DIR / "neosh"


@dataclass
class Toolchain:
    """Toolchain loaded from toolchain.json.

    Field names ARE the ninja variable names — single source of truth.
    """

    mkdir: str
    mkdir_flags: str
    rm: str
    rm_flags: str

    @classmethod
    def load(cls, path: Path) -> "Toolchain":
        data = loads(path.read_text())
        expected = {f.name for f in fields(cls)}
        actual = set(data.keys())
        if expected != actual:
            missing = expected - actual
            extra = actual - expected
            parts = []
            if missing:
                parts.append(f"missing: {missing}")
            if extra:
                parts.append(f"unexpected: {extra}")
            raise ValueError(
                f"{path.name} does not match Toolchain: {', '.join(parts)}"
            )
        return cls(**data)

    def write_variables(self, writer: Writer) -> None:
        writer.variable("python", python_executable)
        for f in fields(self):
            writer.variable(f.name, getattr(self, f.name))
        writer.newline()


def configure(writer: Writer, toolchain: Toolchain) -> None:
    toolchain.write_variables(writer)

    # Rule: create build directory
    writer.rule(
        name="builddir",
        command=f"$mkdir $mkdir_flags {BUILD_DIR}",
        description=f"Creating {BUILD_DIR} directory",
    )
    writer.newline()

    # Rule: clean build directory
    writer.rule(
        name="builddir_clean",
        command=f"$rm $rm_flags {BUILD_DIR}",
        description=f"Cleaning {BUILD_DIR} directory",
    )
    writer.newline()

    # Rule: regenerate build.ninja
    writer.rule(
        name="self",
        command=f"$python {SCRIPT_NAME}",
        description="Reconfiguring build.ninja",
        generator=True,
    )
    writer.build(outputs=str(NINJA_FILE), rule="self")
    writer.newline()

    # Build targets
    writer.build(outputs="clean", rule="builddir_clean")
    writer.newline()

    writer.build(
        outputs="all",
        rule="phony",
        inputs=str(NEOSH_BIN),
    )
    writer.newline()


if __name__ == "__main__":
    toolchain = Toolchain.load(TOOLCHAIN_FILE)
    with open(NINJA_FILE, "w") as f:
        configure(Writer(f, NINJA_WIDTH), toolchain)
