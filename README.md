# i3blocks-cpu

This is CPU block for i3blocks written as beginner project in Rust.
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

You clould also use cargo to test the project.

```
cargo run
```

## License

i3blocks-cpu - A CPU block for i3blocks<br/>
Copyright (c) 2020  Michael Sasser <Michael@MichaelSasser.org><br/>
<br/>
This program is free software: you can redistribute it and/or modify<br/>
it under the terms of the GNU General Public License as published by<br/>
the Free Software Foundation, either version 3 of the License, or<br/>
(at your option) any later version.<br/>
<br/>
This program is distributed in the hope that it will be useful,<br/>
but WITHOUT ANY WARRANTY; without even the implied warranty of<br/>
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the<br/>
GNU General Public License for more details.<br/>
<br/>
You should have received a copy of the GNU General Public License<br/>
along with this program.  If not, see <http://www.gnu.org/licenses/>.


