# Color Transform

# Usage examples
```bash
cargo run -- --help
cargo run -- -o rgb "#123456" # hex without alpha
cargo run -- -o hex "rgba(0, 128, 255, 1)" # rgb with alpha
cargo run -- -o percent "#1234" # Hex with alpha
```

# Full docs
```bash
color_transform 1.0.0
Transforms colors into other formats

USAGE:
    color_transform [OPTIONS] <INPUT>

ARGS:
    <INPUT>    The input, can be a hex "#123456", "rgb(1,2,3)" or "%0.1,0.2,0.3"

OPTIONS:
    -h, --help               Print help information
    -o, --output <OUTPUT>    The transformed output format. [default: hex] [possible values: hex,
                             rgb, percent]
    -V, --version            Print version information
```