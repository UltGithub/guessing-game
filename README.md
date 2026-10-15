# Guessing Game
[![CI](https://github.com/UltGithub/guessing-game/actions/workflows/ci.yml/badge.svg)](https://github.com/UltGithub/guessing-game/actions/workflows/ci.yml)


A command-line number guessing game written in Rust. The computer picks a random number between 1 and 100, and you try to guess it.
After each guess you are told whether it was too small or too big.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)

## Run

```bash
git clone https://github.com/UltGithub/guessing-game.git
cd guessing-game
cargo run --release
```

## Options

```bash
cargo run --release -- --max 1000 --attempts 10
```

| Option | Description | Default |
| --- | --- | --- |
| `-m, --max <N>` | Upper bound of the secret number | 100 |
| `-a, --attempts <N>` | Maximum number of attempts | unlimited |


## Example

```text
Guess the number!
Please input your guess.
1
You guessed: 1
Too small!
Please input your guess.
100
You guessed: 100
Too bing!
Please input your guess.
50
You guessed: 50
Too bing!
Please input your guess.
25
You guessed: 25
Too bing!
Please input your guess.
12
You guessed: 12
Too small!
Please input your guess.
20
You guessed: 20
Too small!
Please input your guess.
23
You guessed: 23
Too bing!
Please input your guess.
22
You guessed: 22
Too bing!
Please input your guess.
21
You guessed: 21
YOU WIN!
```

## Tests

```bash
cargo test
```

## Project layout
- `src/lib.rs` - game logic (input parsing, guess comparison) with unit tests
- `src/main.rs` - terminal input/output

## License
MIT
