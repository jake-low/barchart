`barchart` produces simple terminal barcharts.

It reads lines from STDIN and reproduces them on STDOUT. For any line beginning with a number (ignoring any leading whitespace), it appends a bar representing the value of that number.

## Examples

`barchart` works great for commands that output a sequence of lines each beginning with a number.

Use it to visualize the number of lines in some files:

```
$ git ls-files | xargs -n 1 wc -l | sort -nr | column -t | barchart
383  Cargo.lock  ■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
128  src/main.rs ■■■■■■■■■■■■■■■■■■■■■■
80   README.md   ■■■■■■■■■■■■■■
15   Cargo.toml  ■■■
13   LICENSE     ■■■
1    .gitignore  ■
```

Or the top committers to a git repository (shown here for [git](https://git-scm.com/) itself).

```
$ git shortlog -s -n --all | head -10 | expand | barchart
 27145  Junio C Hamano          ■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  4065  Jeff King               ■■■■■■■■
  2162  Johannes Schindelin     ■■■■
  1985  Ævar Arnfjörð Bjarmason ■■■■
  1824  Nguyễn Thái Ngọc Duy    ■■■■
  1404  Shawn O. Pearce         ■■■
  1160  René Scharfe            ■■■
  1118  Linus Torvalds          ■■
  1106  Elijah Newren           ■■
   954  Michael Haggerty        ■■
```

Or anything you're counting using a `sort | uniq -c` pipeline, like the top commands in your shell history.

```
$ cat ~/.zsh_history | cut -d ' ' -f1 | sort | uniq -c | sort -nr | head | barchart
   4362 cat    ■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
   3567 git    ■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
   1370 python ■■■■■■■■■■■■■■■■■■■■■
    970 cd     ■■■■■■■■■■■■■■■
    898 ls     ■■■■■■■■■■■■■■
    565 mv     ■■■■■■■■■
    557 vim    ■■■■■■■■■
    536 brew   ■■■■■■■■
    517 echo   ■■■■■■■■
    492 rm     ■■■■■■■■
```

## Installation

You can install `barchart` via Homebrew.

```
brew install jake-low/tools/barchart
```

Alternately, if you have a Rust toolchain installed you can download the source code for a [release](https://github.com/jake-low/barchart/releases) and build it with `cargo build`.

## Features

`barchart` aligns the start of each bar by measuring the displayed width of the prefix string, so it should work pretty well even if the bar labels contain non-ASCII characters. It handles both integers and decimal numbers (even when followed by a % sign). If it cannot parse a number from the start of the line, it will pass that line through unmodified (so output containing header rows will work just fine).

## Credit

This program is inspired by Jude Melton-Houghton's [shell script of the same name](https://github.com/TurkeyMcMac/barchart). I wrote my own implementation mainly in order to handle decimal numbers.

## License

The source code of this program is available under the ISC license. See the LICENSE file for details.
