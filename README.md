# About
This is a tool to evaluate Poker hands in a very efficient way using bit manipulation and lookup tables. The algorithm is based off of the work of others.

# Prerequisites
You need to have a working Rust environment and `git` installed.

# How to use it
1. Clone the repository and change into it.
2. Compile the source using `cargo run --release`
3. Change into `target/release` directory, then type `phevor --help` to see the available options.

```bash
USAGE:
    phevor [FLAGS] [OPTIONS] [hand]

FLAGS:
    -a, --all        Shows evaluations and the ranking of hands.
    -h, --help       Prints help information
    -r, --rank       Prints the ranking of hands instead of their evaluations.
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>    A file containing hands (one per line) to evaluate.

ARGS:
    <hand>    A hand (consisting of 2 hole and 5 board cards) to evaluate.
```
To evaluate a single hand simply type `phevor <hand>`. The program expects the hand to be formatted in a certain way, like so: `AsKdQcJhTc4h8c`. For example `As` means `Ace of Spades`. To evaluate multiple hands stored in a file you would have to type `phevor -f <file>`. By default it will print out the evaluations of each hand line by line. If you want to see how they rank against eachother type `phevor -f <file> -r`.

# Closing words

I hope this little application will be useful to someone out there. As always ... have fun!