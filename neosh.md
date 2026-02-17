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

#### Comments

Comments begin with `#` and extend to the end of the line.
Comments are ignored by the parser.

#### Expressions

NeoSH has several expression forms:
- Literals: `1`, `"hello"`, `true`
- Variables: `x`, `y`
- Function calls: `add(1, 2)`
- Operators: `x + y`, `x * y`
- Block expressions: `{ x :: 10; y :: x + 5; x + y }`
- Declarations: `x :: 10`

#### Grammar (EBNF-style, Informal)

Informal sketch for orientation. Not a normative parser specification.

```ebnf
program      = { expression [ ";" ] } ;

expression   = declaration
             | procedure
             | source_expr
             | object_decl
             | type_alias
             | block
             | call
             | operator_expr
             | literal
             | identifier ;

declaration  = identifier "::" expression ;
procedure    = "proc" [ "(" [ param { "," param } ] ")" ] [ ":" type ] "=" expression ;
param        = identifier [ ":" type ] ;
source_expr  = "source" string ;
object_decl  = "object" "{" { identifier "::" type } "}" ;
type_alias   = "type" type ;
block        = "{" { expression [ ";" ] } "}" ;
call         = identifier "(" [ expression { "," expression } ] ")" ;

type         = identifier ;
literal      = int | real | string | bool ;
identifier   = (* language identifier *) ;
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

### Design Philosophy

- Everything is an expression.
- Prefer small, composable procedures.
- Bindings are immutable; data flow should be explicit.
- Type annotations are documentation, not requirements.
- Modules organize behavior by file.

### Open Questions

- **Booleans**: Undecided whether `bool` should be a language-level primitive type or a strongly typed alias (e.g. `bool :: type int`).
- **Runes**: `rune` is currently a semantic alias for `int`. It may need distinct behavior beyond being an integer depending on how string operations evolve.
