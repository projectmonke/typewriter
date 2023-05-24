<h1 align="center">Typewriter</h1>
<p align="center"><b>Typewriter</b> is a subdomain permutation tool written in Rust and heavily based on Gotator.</p>

## Features
- Permutations with the `-` character.
- Unlimited depth, limited only by your computer.

## Installation
`git clone https://github.com/projectmonke/typewriter && cd typewriter && cargo build --release && cp target/release/typewriter .`

## Usage
- `-i` specifies a single domain
- `-w` specifies the wordlist of permutations (Six2dez's `six.txt` is provided)
- `-f` specifies the wordlist containing a list of subdomains to perform permutations on.

## Example
[puredns](https://github.com/d3mondev/puredns) is recommended for resolving subdomains.
- `./typewriter -w six.txt -i example.com -d 2 > results.txt`
- `subfinder -d example.com > example_subs.txt` + `./typewriter -w six.txt -f example_subs.txt -d 2`

## Limitations
- `stdin` doesn't work yet for piping directly into it.

## To Do
- `stdin` support.
- Flag for writing to a text file directly.
- Flag for configuring the range of numbers beyond +-1 in number-inclusive permutations.
