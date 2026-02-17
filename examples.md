## Invoking Makefile

```bash
MAKEFILE_LOCATION=$1
if [ -z "$1" ]; then
    echo "Usage: $0 <makefile_location>"
    exit 1
fi

make -f "$1"
```

```neosh
argv :: arguments
if argv.len == 0 {
  echo "No arguments provided"
  exit 1
}

makefile_location :: argv[1]
make -f "$makefile_location"
```

## Shell vs Expression Parsing — Open Design Problem

The parser must distinguish command invocations from NeoSH expressions without sigils or explicit markers.

### What's unambiguous

```neosh
x :: 10                         # declaration — has ::
add(1, 2)                       # proc call — has (
x + y                           # operator — has known operator token
ls -la                          # command — -la is a flag, not a valid expression token
make -f "$x"                    # command — flag + interpolated string
{ x :: 10; x + 5 }             # block expression — has { }
```

### What's ambiguous

```neosh
foo bar                         # command `foo` with arg `bar`? two expressions?
make RELEASE=x                  # command with arg? some kind of assignment?
x -f                            # subtraction x - f? or command x with flag -f?
```

### Constraints

1. No sigils for commands — `ls`, `make`, `cd` must work bare.
2. No `$var` outside strings — NeoSH expressions use bare identifiers.
3. Whitespace should be insignificant — `x-f` and `x - f` should mean the same.
4. Variables must be passable to commands without mandatory quoting.
5. Semicolons are required between expressions on the same line.

### Observations

- NeoSH requires `;` between expressions on the same line.
  So `foo bar` cannot be two expressions without `;`.
  If `foo` is not a keyword, binding, or proc, `foo bar` must be a command.
- The expression grammar is strict: after an identifier, only `::`, `(`, `.`, `[`, or an operator is valid.
  Anything else (bare word, flag, `=` in `KEY=val`) falls outside the grammar.
- Flags (`-f`, `--verbose`) are never valid expression tokens.
- Bound names are always expressions. `x :: 10; x -f` would parse as `x - f` (subtraction).
  This means binding a name shadows any command with the same name.

### Unsolved tension

- If whitespace is insignificant, `x -f` and `x - f` parse identically.
  A bound `x` means subtraction. An unbound `x` means command with flag.
  The meaning of `-f` depends on whether `x` is bound — invisible to the reader.
- Passing variable values to commands: `make -f path` — is `path` the binding or the literal string `"path"`?
  Requiring `"$path"` is explicit but verbose for interactive shell use.
  Implicitly resolving bound names is convenient but means typos silently become literal strings.
