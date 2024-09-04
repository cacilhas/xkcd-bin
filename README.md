# xkcd-bin

Just a CLI to show [Xkcd.com][] comics in [Kitty Terminal][].


## Installation

```sh
cargo +nightly install xkcd-bin
```


## Usage

Get the latest comic:

```sh
xkcd latest
```

Or simply:

```sh
xkcd
```

Get a random comic:

```sh
xkcd random
```

Get the comic number 162:

```sh
xkcd 162
```

Open the comic number 162 in the default web browser:

```sh
xkcd --browser 162
```

Get help:

```sh
xkcd --help
```


## Note

If you’re looking for a library to build Xkcd into your project, please head to
[Rusty Xkcd][] or [xkcd-rs][].


## License

- [The 3-Clause BSD License][]
- [Copyright 2024 Rodrigo Cacilhας &lt;montegasppa@cacilhas.info&gt;][]

[Copyright 2024 Rodrigo Cacilhας &lt;montegasppa@cacilhas.info&gt;]: https://github.com/cacilhas/xkcd-bin/blob/master/COPYING
[Kitty Terminal]: https://sw.kovidgoyal.net/kitty/
[Rusty Xkcd]: https://crates.io/crates/rusty_xkcd
[Xkcd.com]: https://xkcd.com/
[xkcd-rs]: https://crates.io/crates/xkcd
[The 3-Clause BSD License]: https://opensource.org/license/BSD-3-Clause
