## NeoSH

NeoSH is a scripting language that also serves as your shell.
The interactive shell experience is intentionally different from Bash or Zsh and is not the current focus.
The primary goal is to build a strong, expressive language for writing scripts.

### Phases

NeoSH execution is divided into three phases:

1. **Parsing**: Source code is parsed into an AST.
2. **Const-Eval**: Constant expressions are evaluated before runtime.
3. **Execution**: The AST is executed.

### Syntax, Types, and Semantics

NeoSH draws from functional languages like Haskell, OCaml, and Elixir, as well as Pascal and Perl.

#### Basics

Everything in NeoSH is an expression.
Expressions can be separated by semicolons; semicolons are only required when multiple expressions appear on the same line.
NeoSH uses a dynamic type system with runtime type checking.
Type annotations are optional.

#### Parsing Rules

The parser classifies each line using three rules, applied in order:

1. **Keyword-led** — line starts with a known keyword (`if`, `else if`, `else`, `for`, `while`, `match`, `proc`, `object`, `source`, `type`) → parsed as an expression.
2. **Declaration** — line contains `::` → parsed as a binding (`name :: expression`).
3. **Command** — otherwise → parsed as a command invocation.

These rules apply uniformly everywhere: top level, inside blocks, inside procedure bodies.

#### Types

NeoSH has numeric types, strings, runes, and user-defined types via objects.

Numeric types:
- `int` — signed integer at system word width.
- `uint` — unsigned integer.
- `real` — double-precision floating point.

Other types:
- `rune` — semantic alias for `int`. Represents a character value.
- `string` — type alias for `[rune]` (`string :: type [rune]`).
- `[T]` — array of type `T`.

Type aliasing is supported with the `type` keyword:

```neosh
my_type :: type int
```

#### Built-in Procedures

NeoSH provides a small set of built-in procedures for operations that would be too slow to implement in userspace:

- `len(x)` — returns the length of a string or array.

More built-ins may be added where performance justifies it.

#### Comments

Comments begin with `#` and extend to the end of the line.
Comments are ignored by the parser.

#### Expressions

NeoSH has several expression forms:
- Literals: `1`, `"hello"`, `true`
- Variables: `x`, `y`
- Procedure calls: `add(1, 2)`
- Operators: `x + y`, `x * y`
- Block expressions: `{ x :: 10; y :: x + 5; x + y }`
- Declarations: `x :: 10`
- Control flow: `if`, `for`, `while`, `match`

#### Strings

Strings support concatenation with `+` and interpolation with `$`.

- `"$name"` — interpolates a variable.
- `"${expr}"` — evaluates an expression and interpolates the result.
- `"hello" + " world"` — concatenation with `+`.

`${}` also works outside strings to capture the result of an expression or command:

```neosh
greeting :: "hello" + " world"
path :: "$home/.config"
listing :: ${ls -la}
info :: "files: ${ls -la}"
```

#### Control Flow

Control flow keywords are expressions. They do not require a `::` binding.

```neosh
if x == 0 {
    echo "zero"
} else if x == 1 {
    echo "one"
} else {
    echo "other"
}

for item in items {
    echo "$item"
}

while running {
    poll()
}
```

#### Grammar (EBNF-style, Informal)

Informal sketch for orientation. Not a normative parser specification.

```ebnf
program        = { expression [ ";" ] } ;

expression     = declaration
               | control_flow
               | procedure
               | source_expr
               | object_decl
               | type_alias
               | command
               | block
               | call
               | capture
               | operator_expr
               | literal
               | identifier ;

declaration    = identifier "::" expression ;
procedure      = "proc" [ "(" [ param { "," param } ] ")" ] [ ":" type ] "=" expression ;
param          = identifier [ ":" type ] ;
source_expr    = "source" string ;
object_decl    = "object" "{" { identifier "::" type } "}" ;
type_alias     = "type" type ;
block          = "{" { expression [ ";" ] } "}" ;
call           = identifier "(" [ expression { "," expression } ] ")" ;
capture        = "${" { expression } "}" ;

control_flow   = if_expr | for_expr | while_expr | match_expr ;
if_expr        = "if" expression block [ "else" "if" expression block ] [ "else" block ] ;
for_expr       = "for" identifier "in" expression block ;
while_expr     = "while" expression block ;
match_expr     = "match" expression "{" { pattern "->" expression } "}" ;

command        = (* any line not matching the above rules *) ;

type           = identifier | "[" type "]" ;
literal        = int | real | string | bool ;
identifier     = (* language identifier *) ;
```

#### Variables

Names are bound to values, not assigned.
All bindings are immutable.
Declaring the same name again creates a shadow; the earlier binding is no longer visible in that scope.
Shadowing does not deallocate the earlier value.
Use shadowing deliberately, especially in loops.

```neosh
x :: 10
y :: x + 5
```

#### Procedures

Procedures are the core abstraction in NeoSH.
A procedure is defined with the `proc` keyword and bound to a name like any other value.
Procedures optionally accept parameters and always return a value.
The last expression in a procedure body is its return value.

```neosh
empty :: proc = ()
do_add :: proc(x, y) = x + y
# With type annotations:
# do_sub :: proc(x: int, y: int): int = x - y
```

#### Modules

One file is one module.
A module can `source` another module, which triggers parsing and const-eval for that file.
Sourcing binds the module's exported values to a name.
There is no formal package concept; multiple files may act as one, but NeoSH does not enforce or define this.

```neosh
io :: source "io.nsh"

print_hello :: proc = io.println("Hello World!")
```

#### Objects

Objects define structured data with named, typed fields.
An object is declared with the `object` keyword and bound to a name like any other value.
Object values are constructed using literal syntax: `TypeName{field: value, ...}`.
Objects can be passed to and returned from procedures.

```neosh
person :: object {
    name :: string
    age :: int
}

create_person :: proc(name: string, age: int): person = person{name: name, age: age}
```

### Shell

NeoSH is a shell. External commands and binaries are first-class citizens, not strings passed to a subprocess API.
Executing programs should feel as natural as calling a procedure.

#### Commands

Any line that does not start with a keyword and does not contain `::` is parsed as a command invocation.
Commands are bare by default — no sigils, no wrappers.

```neosh
ls -la
make -f "Makefile"
git clone https://github.com/user/repo
cd "/tmp"
```

Variables are passed to commands using string interpolation:

```neosh
path :: "/tmp"
make -f "$path/Makefile"
```

#### Output Capture

Command output (stdout) can be captured into a value using `${}`:

```neosh
files :: ${ls -la}
uid :: ${id -u}
url :: ${curl -sL "https://api.example.com/data" | jq -r ".url"}
```

`${}` evaluates its contents and returns the result as a string.
Inside a string, `${}` interpolates the result:

```neosh
echo "Your UID is ${id -u}"
```

#### Silencing Output

A common pattern is binding to `_` to discard output. `_` is a regular name — shadowing allows rebinding it freely:

```neosh
_ :: git pull
_ :: curl -sL "https://example.com"
```

#### Exit Codes

The `status` keyword holds the exit code of the last executed command.

```neosh
curl -sL "https://example.com"
code :: status

if status != 0 {
    echo "Request failed"
    exit 1
}
```

#### Pipes

Commands and expressions are both expressions, so NeoSH uses a single pipe operator.
Pipes pass the output of one expression as input to the next.

```neosh
# The exact pipe operator (| vs |>) is undecided.
# These examples use | as placeholder.
ls | grep ".nsh"
cat "data.txt" | sort | uniq
data | filter(is_valid) | transform
```

#### Environment

Commands and binaries inherit and interact with the shell environment (PATH, working directory, environment variables).
The exact mechanism for reading and setting environment state is under development.

### Design Philosophy

- Everything is an expression.
- Commands are first-class — not wrapped in special syntax.
- Prefer small, composable procedures.
- Bindings are immutable; data flow should be explicit.
- Type annotations are documentation, not requirements.
- Modules organize behavior by file.
- The language provides primitives; users build their own tools.

### Open Questions

- **Booleans**: Undecided whether `bool` should be a language-level primitive type or a strongly typed alias (e.g. `bool :: type int`).
- **Runes**: `rune` is currently a semantic alias for `int`. It may need distinct behavior beyond being an integer depending on how string operations evolve.
- **Pipe operator**: One pipe for both commands and expressions. Undecided between `|` and `|>`. Trade-off: `|` is minimal but visually close to `||` (logical OR); `|>` is more readable but an extra character.
- **Redirections**: Silencing is handled by `_ ::`, but redirecting output to files and capturing stderr still need syntax. Bash-style (`>`, `>>`, `2>&1`) is not readable enough.
- **Environment access**: How environment variables are read, set, and passed to commands.
- **Status scope**: Undecided whether `status` is set by external commands only, or also by procedure calls.
