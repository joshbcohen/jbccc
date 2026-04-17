# jbccc
C Compiler written in Rust. `jbccc` -> Joshua B. Cohen's C Compiler. This compiler follows the implementation guide explained in [Writing A C Compiler](https://norasandler.com/book/) by Nora Sandler. I started this during my batch at the [Recurse Center](https://www.recurse.com/).

## Supported Functionality
* This currently only runs the compiler driver- it will preprocess any C file passed in, and (theoretically) should assemble and link any assembly files output by the compiler. We'll see if this all works in a few commits.

## Usage
```
Usage: jbccc [OPTIONS] <C_FILENAME>

Arguments:
  <C_FILENAME>

Options:
  -l, --lex            Run the lexer only.
  -p, --parse          Run the lexer and parser.
  -c, --codegen        Run the lexer, parser, and assembly AST generator.
  -S, --emit-assembly  Run all compiler steps besides assembler, generating a file at <c_filename>.s.
  -h, --help           Print help
  -V, --version        Print version
```
