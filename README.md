# Crag

`crag` is a *c*ommand-line *r*ust based application for *a*cessing (*g*oogle) search results.

![crag example](docs/crag.png)

## Quickstart

If you already have rust and cargo installed, you can install `crag` directly from crates.io:

```console
cargo install crag
```

### Usage

To search with `crag`, just run `crag [SEARCH]`, and wait for the results -- it's really that simple.

```console
crag mountain-climbing
```

Of course, `crag` also supports several flags that modify the results returned by your search. The tool ships with built-in help text that describes the various options:

```console 
crag --help
...
```

## Overview

### Features

- [x] Searches things (from the command line!)
- [x] Clean search result display
- [x] Cross-platform
- [ ] Optional interactive search result navigation
- [ ] Supports multiple search engines
- [ ] ...

## Alternatives

- [googler](https://github.com/jarun/googler): The _de facto_ standard (and probably best) command line google application, but it is currently archived.
