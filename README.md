# imggen

[![Latest version](https://img.shields.io/crates/v/imggen?color=mediumvioletred)](https://crates.io/crates/imggen)
[![Documentation](https://docs.rs/imggen/badge.svg)](https://docs.rs/imggen)
![MSRV](https://img.shields.io/badge/rustc-1.63+-blue.svg)
![GitHub License](https://img.shields.io/github/license/ChurchTao/imggen)

imggen is a test image generation tool that can generate images of various sizes and colors.

## Usage

If you have Rust installed, you can install imggen with the following command:

```shell
cargo install imggen
```

<!-- If you don't have Rust installed, you can install with Homebrew:

```shell
brew install ChurchTao/tap/imggen
``` -->

```shell
# sample
imggen -w 1024 -h 768 -f png -n myimage -o ~/Desktop
```

```log
# or

Usage: imggen [OPTIONS]

Options:
  -w, --width <WIDTH>        Sets the width of the image [default: 800]
  -h, --height <HEIGHT>      Sets the height of the image [default: 600]
  -f, --format <FORMAT>      Sets the output format (png, jpg, etc.) [default: png]
  -n, --filename <FILENAME>  Sets the output filename [default: output]
  -o, --outdir <OUTDIR>      Sets the output directory [default: .]
  -h, --help                 Print help
  -V, --version              Print version
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
