# bfck-rust
Memory- and threadsafe Brainfuck-Virtual-Machine

## Intention
bfck was writen as an exercise in the Rust programming Language after reading through the very basics. It is intendet to
be extended with more features (e.g. implementing the multithreadding-features of the "brainfork" programming language)
in the process of the author diving deeper into the Rust language.

## Specs

### Model
bfck implements the whole Brainfuck-model. Including an infinite tape (in both, right and left direction) zero-initialized 8-bit
cells and value-wrap.
### Memory Management
bfck autoatically allocates new Memory when visiting yet unused cells and automatically frees cells, which are not needed any more.

## Usage

bfck takes one commandline Argument, the path to the brainfuck-sourcecode-file you whish to execute. Input is read from stdin and
output is writen to stdout.

```bash
./bfck foo.bf
```
will execute foo.bf

## License
All sourcecode includet in this project is published under the GPL License Version 3. You find a copy of the GPLv3 License in use [here](/LICENSE.txt) for a copy of the GPLv3 License.
