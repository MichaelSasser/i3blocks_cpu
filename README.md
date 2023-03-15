# i3blocks_cpu

This is a CPU block for i3blocks written as beginner project in Rust.
It shows your CPU in percent and the temperature in a fixed length:

The output looks like:
```
__5.31% 48°C
_95.10% 85°C
100.00% 89°C
```

The underlines "\_" in the example are spaces if no character is displayed.

### Building

Use cargo to build the project.

```
cargo build --release
```

The executable is located in `./target/release/i3blocks_cpu`

## Semantic Versioning

This repository uses [SemVer](https://semver.org/) for its release
cycle.

## License
Copyright &copy; 2020-2023 Michael Sasser <Info@MichaelSasser.org>. 
Released under the MIT license.
