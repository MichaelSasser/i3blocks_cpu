# i3blocks_cpu

This is a CPU block for i3blocks written as beginner project in Rust.
It shows your CPU in percent and the temperature in a fixed length:

The output looks like:
```
__5.31% 48°C
_95.10% 85°C
100.00% 89°C
```

The "\_" are spaces if they are not used.

### Building

Use cargo to build the project.

```
cargo build --release
```

The executable is now located in `./target/release/i3blocks_cpu`

## Semantic Versioning

This repository uses [SemVer](https://semver.org/) for its release
cycle.

## Branching Model

This repository uses the
[git-flow](https://danielkummer.github.io/git-flow-cheatsheet/index.html)
branching model by [Vincent Driessen](https://nvie.com/about/).
It has two branches with infinite lifetime:

* [master](https://github.com/MichaelSasser/i3blocks_cpu/tree/master)
* [develop](https://github.com/MichaelSasser/i3blocks_cpu/tree/develop)

The master branch gets updated on every release. The develop branch is the
merging branch.

## License
Copyright &copy; 2020-2021 Michael Sasser <Info@MichaelSasser.org>. 
Released under the GPLv3 license.
