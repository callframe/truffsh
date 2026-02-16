#!/usr/bin/env python3
from ninja import Writer
from pathlib import Path
from json import loads
from dataclasses import dataclass
from sys import executable as python_executable

NINJA_WIDTH = 80
DOLLAR_SYMBOL = "$"
PYTHON = python_executable
THISFILE = Path(__file__).name


class Paths:
    WORKING_DIR = Path(__file__).parent
    MIMALLOC_DIR = WORKING_DIR / "mimalloc"
    BUILD_DIR = WORKING_DIR / "build"


class Files:
    TOOLCHAIN_FILE = Paths.WORKING_DIR / "toolchain.json"
    NINJA_FILE = Paths.WORKING_DIR / "build.ninja"
    NEOSH_FILE = Paths.BUILD_DIR / "neosh"


class Rules:
    BUILD_DIR_RULE = "builddir"
    BUILD_DIR_CLEAN_RULE = "builddir_clean"
    CLEAN_RULE = "clean"
    SELF_RULE = "self"
    ALL_RULE = "all"


class Variables:
    PYTHON = "python"
    MKDIR = "mkdir"
    MKDIR_FLAGS = "mkdir_flags"
    BUILD_DIR = "builddir"
    RM = "rm"
    RM_FLAGS = "rm_flags"


@dataclass
class Toolchain:
    mkdir: str
    mkdir_flags: str
    rm: str
    rm_flags: str

    def write(self, writer: Writer) -> None:
        writer.variable(Variables.PYTHON, PYTHON)
        writer.variable(Variables.MKDIR, self.mkdir)
        writer.variable(Variables.MKDIR_FLAGS, self.mkdir_flags)
        writer.variable(Variables.RM, self.rm)
        writer.variable(Variables.RM_FLAGS, self.rm_flags)
        writer.newline()


@dataclass
class Rule:
    name: str
    command: str
    description: str

    def write(self, writer: Writer) -> None:
        writer.rule(self.name, self.command, self.description)
        writer.newline()


@dataclass
class BiRule:
    rule: Rule
    rule_clean: Rule

    def write(self, writer: Writer) -> None:
        self.rule.write(writer)
        self.rule_clean.write(writer)


def command(texts: list[str]) -> str:
    return " ".join(texts)


def ref(var: str) -> str:
    return f"{DOLLAR_SYMBOL}{var}"


def dirrule(path: Path) -> BiRule:
    builddir_rule = Rule(
        name=Rules.BUILD_DIR_RULE,
        command=command([ref(Variables.MKDIR), ref(Variables.MKDIR_FLAGS), str(path)]),
        description=f"Creating {path} directory",
    )

    builddir_clean_rule = Rule(
        name=Rules.BUILD_DIR_CLEAN_RULE,
        command=command([ref(Variables.RM), ref(Variables.RM_FLAGS), str(path)]),
        description=f"Cleaning {path} directory",
    )

    return BiRule(builddir_rule, builddir_clean_rule)


def build_dir_rule(writer: Writer) -> None:
    dirrule(Paths.BUILD_DIR).write(writer)


def self_rule(writer: Writer) -> None:
    writer.rule(
        name=Rules.SELF_RULE,
        command=command([ref(Variables.PYTHON), THISFILE]),
        description="Reconfiguring build.ninja",
        generator=True,
    )

    writer.build(
        outputs=str(Files.NINJA_FILE),
        rule=Rules.SELF_RULE,
    )
    writer.newline()


def clean_rule(writer: Writer) -> None:
    writer.build(
        outputs=Rules.CLEAN_RULE,
        rule=Rules.BUILD_DIR_CLEAN_RULE,
    )
    writer.newline()


def all_rule(writer: Writer) -> None:
    writer.build(
        outputs=Rules.ALL_RULE,
        rule=Rules.BUILD_DIR_RULE,
        implicit=str(Files.NEOSH_FILE),
    )
    writer.newline()


def read_file(path: Path) -> str:
    file = open(path, "r")
    content = file.read()
    file.close()
    return content


if __name__ == "__main__":
    tc_data = loads(read_file(Files.TOOLCHAIN_FILE))
    toolchain = Toolchain(**tc_data)

    with open(Files.NINJA_FILE, "w") as file:
        writer = Writer(file, NINJA_WIDTH)
        toolchain.write(writer)
        build_dir_rule(writer)
        self_rule(writer)
        clean_rule(writer)
        all_rule(writer)
