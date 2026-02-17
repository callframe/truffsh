## NeoSH
NeoSH is a shell that tries to provide a better experience for writing scripts. 
The primary goal of NeoSH is to write scripts instead of replacing your shell.

### Phases
NeoSH is divided into three phases:

1. **Parsing**: The input code is parsed and as much as possible is turned into an AST.
2. **Const-Eval**: Constant expressions are evaluated before execution.
3. **Execution**: The AST is executed.

### Syntax, Types and Semantics
NeoSH is inspired by Functional Languages like Haskell, OCaml and Elixir, but also draws inspiration from Pascal and Perl.

#### Basics
Everything is an expression. Expressions can be separated by semicolons but this is only required when multiple expressions are on the same line.
A dynamic type system provides the ability to check types and ensure type safety. There is no requirement for type annotations.

#### Types


#### Comments
NeoSH supports comments using the `#` symbol. Comments are ignored by the parser and do not affect the execution of the code.

#### Expressions
There are different kinds of expressions in NeoSH.
Some examples are:
- Literals: `1`, `"hello"`, `true`
- Variables: `x`, `y`
- Function calls: `add(1, 2)`
- Operators: `x + y`, `x * y`
- Block expressions: `{ x :: 10; y :: x + 5; x + y }`
- Declarations: `x :: 10`

#### Variables
In NeoSH, variables are bound to a value, not assigned. Variables are always immutable and cannot be reassigned.
Declaring a variable with the same name as an existing variable will result in shadowing. The earlier variable looses it's binding but will NOT be garbage collected.
Be careful with shadowing, especially in loops to prevent stack overflow.

```neosh
x :: 10
y :: x + 5
```

#### Procedures
The core of NeoSH is the procedure system. Procedures represent a set of instructions that execute when a procedure is invoked.
Procedures are defined using the `proc` keyword. Procedures are nothing more than a variable that holds a set of instructions.
Procedures can optionally accept parameters but must always return a value. The last expression in a procedure is the return value.

```neosh
empty :: proc = ()
do_add :: proc(x, y) = x + y
# Types can be specified using: (<type> represents a placeholder)
# do_sub :: proc(x: <type>, y: <type>): <type> = x - y
```

#### Modules
NeoSH builds on top of modules. A module source's other modules.
Sourcing a module will invoke parsing, as well as local constant evaluation.
Through sourcing, it is possible to import procedures, constants, and variables.
One file represents a module. Multiple files could be considered a package but there is no concept of a package in NeoSH.
When sourcing a module, you bind the module's contents to a variable like procedures.

```neosh
io :: source "io.nsh"

print_hello :: proc = io.println("Hello World!")
```

#### Objects
```neosh
person :: object {
    name :: string
    age :: int
}

create_preson :: proc(name: string, age: int): person = person{name: name, age: age}
```
