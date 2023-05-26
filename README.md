<h1 align="center">Typewriter</h1>
<p align="center"><b>Typewriter</b> is a subdomain permutation tool written in Rust and heavily based on Gotator.</p>

## Features
- Permutations with the `-` character!
- Unlimited depth, limited only by your computer!
- Deduplication by default!
---

## Installation
`git clone https://github.com/projectmonke/typewriter && cd typewriter && cargo build --release && cp target/release/typewriter .`

## Usage
### Required
- `-i` specifies a single domain
- `-w` specifies the wordlist of permutations (Six2dez's `six.txt` is provided)
- `-f` specifies the wordlist containing a list of subdomains to perform permutations on.
- `-s` specifies that you are reading subdomain data from stdin.

### Optional
- `-d` specifies the depth of the permutations (dev.test1.example.com is depth 2, and so on).
- `-r` specifies the range that you want to try for subdomains with numbers (such as test1.example.com).

## Example
[puredns](https://github.com/d3mondev/puredns) is recommended for resolving subdomains.
- `./typewriter -w six.txt -i example.com -d 2 > results.txt`
- `subfinder -d example.com | ./typewriter -w six.txt -s -r 5 -d 2`

## Limitations
- Domain validation is not robust at all. Still fixing this.

## To Do
- Flag for writing to an output file.
