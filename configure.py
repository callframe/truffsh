#!/usr/bin/env python3
from ninja import Writer
from pathlib import Path
from json import loads
from dataclasses import dataclass

NINJA_WIDTH = 80
DOLLAR_SYMBOL = "$"


class Paths:
    WORKING_DIR = Path(__file__).parent
    MIMALLOC_DIR = WORKING_DIR / "mimalloc"
    BUILD_DIR = WORKING_DIR / "build"


class Files:
    TOOLCHAIN_FILE = Paths.WORKING_DIR / "toolchain.json"
    NINJA_FILE = Paths.WORKING_DIR / "build.ninja"


class Rules:
    BUILD_DIR_RULE = "builddir"
    BUILD_DIR_CLEAN_RULE = "builddir_clean"
    CLEAN_RULE = "clean"


class Variables:
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
    writer.newline()


def clean_rule(writer: Writer) -> None:
    writer.build(
        outputs=Rules.CLEAN_RULE,
        rule=Rules.BUILD_DIR_CLEAN_RULE,
        implicit=Rules.BUILD_DIR_RULE,
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
        clean_rule(writer)
