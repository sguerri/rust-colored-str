# Welcome to colored-str for Rust

[![](https://badgen.net/github/license/sguerri/rust-colored-str)](https://www.gnu.org/licenses/)
[![](https://badgen.net/badge/Open%20Source%20%3F/Yes%21/blue?icon=github)](#)

> Coloring terminal by parsing string content

This crate is an extension to the [colored](https://crates.io/crates/colored) crate that enables terminal coloring. It provides a trait and a macro to parse a given string that incorporates style flags.

**Main features**
 - Translate [colored](https://crates.io/crates/colored) features into string templating
 - Multiple styles and substyles in one same string
 - Macros to replace `format!` and `println!` with colored strings

---

- [Welcome to colored-str for Rust](#welcome-to-colored-str-for-rust)
  - [Usage](#usage)
  - [How to use](#how-to-use)
  - [Limitations](#limitations)
  - [Examples](#examples)
  - [List of styles](#list-of-styles)
  - [Dependencies](#dependencies)
  - [Issues](#issues)
  - [License](#license)
  - [Contributors](#contributors)

## Usage

This crate is [on crates.io](https://crates.io/crates/colored-str) and can be used by adding `colored-str` to your dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
colored-str = "0.0.2"
```

## How to use

Styles must be written within `<...>` opening flag and `</>` closing flag.

Style variations must be written within `<+...>` opening flag and `<->` closing flag.

See below examples.

## Limitations

**Blocks cannot be overlapped**  
Such code `<red> ... <blue> ... </> ... </>` will not work properly.  
This is true as well for variations : `<red><+blue> ... <+bold> ... <-><-></>` will not work properly.  

**A style cannot be removed**  
With `<red+bold> ... here I want to keep red only => impossible </>`.  
The workaround is as follows: `<red> <+bold> ... <-> here I have red only </>`

## Examples

```rust
use colored_str::coloredln;

coloredln!("<red>this is red</>");
coloredln!("<#FF0000>this is also red</>");
coloredln!("<blue+red>this is red again</>");
coloredln!("<red+on_blue>this is red on blue</>");
coloredln!("<red+on_#0000FF>this is also red on blue</>");
coloredln!("<bold>this is bold</>");
coloredln!("<red>there is a first line\nthen a second</>");
```

You can add variables as per `println!`

```rust
use colored_str::coloredln;

let message = "this is red";
coloredln!("<red>{message}</>");
coloredln!("<red>{}</>", message);
```

You can add styles adjustments in a block

```rust
use colored_str::coloredln;

coloredln!("<red>this is red <+bold>this is red and bold<-> then red again </>");
coloredln!("<red>this is red <+bold+on_blue>this is red on blue and bold<-> then red again </>");
```

You can also use it as a trait

```rust
use colored_str::Colored;
let s: String = "<red>this is red</>".colored().to_string();
println!("{}", s);
```

## List of styles

### Colors

- `black`
- `red`
- `green`
- `yellow`
- `blue`
- `magenta`
- `purple`
- `cyan`
- `white`

All can be used as backgound using `on_` prefix.

### Light/Bright Colors

- `lblack`
- `lred`
- `lgreen`
- `lyellow`
- `lblue`
- `lmagenta`
- `lpurple`
- `lcyan`
- `lwhite`

All can be used as backgound using `on_` prefix.

### Decorations

- `bold`
- `underline`
- `italic`
- `dimmed`
- `reverse`
- `reversed`
- `blink`
- `hidden`
- `strikethrough`

### True colors

- `#RRGGBB`
- `on_#RRGGBB`

## Dependencies

- [colored](https://crates.io/crates/colored)
- [lazy_static](https://crates.io/crates/lazy_static)
- [regex](https://crates.io/crates/regex)

## Issues

Contributions, issues and feature requests are welcome!

I am quite new to rust, so I guess many things can be improved. I mainly test on linux also, so tests on other platforms are welcome.

Feel free to check [issues page](https://github.com/sguerri/rust-colored-str/issues). You can also contact me.

## License

Copyright (C) 2023 Sebastien Guerri

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

## Contributors

- Sébastien Guerri: [@sguerri](https://github.com/sguerri)
