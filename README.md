<div align="center">
    <img src="logo/waffle.png">
    <p>Is a little stack-based VM built for educational purposes.</p>
    <blockquote> <b>WAFFLE</b> (<i>verb</i>): use many words but say nothing important; avoid making decision or stating clear opinion.</blockquote>
</div>


## Usage

```shell script
λ  ./target/debug/waffle --help
WAFFLE 0.1
iy <github.com/Nyanguy>
It's a stack-based VM!

USAGE:
    waffle [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    syrup      Creates a bytecode file.
    titbits    Reverses specified bytecode file into the waffle source code.
    with       Runs specified bytecode file.
```


## How does it work?

You can find example code [here](test). 
Each instruction takes 8 bits with integers and float numbers as 64 bits (a subject to change in the future as an argument to the VM).
Allocated memory size can be managed with a cli argument. By default it's 1Kb.
