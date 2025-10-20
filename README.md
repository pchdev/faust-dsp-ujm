# Build and run the presentation - Linux

```sh
# Install the rust toolchain:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone this repository:
git clone https://github.com/pchdev/faust-dsp-ujm

# Build and run the presentation:
cd faust-dsp-ujm
cargo run --release
```

# Build and run the presentation - OSX

```sh
# Install the rust toolchain:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Add Rust to Your PATH
export PATH="$HOME/.cargo/bin:$PATH"
# Then reload your shell:
source ~/.zshrc

# Clone this repository:
git clone https://github.com/pchdev/faust-dsp-ujm

# Build and run the presentation:
cd faust-dsp-ujm
cargo run --release
```

> **Note**: only tested on *Linux* and  **macOS**, **Windows** would be appreciated :-) 

# Keyboard shortcuts:

- `Right arrow`: Next slide;
- `Left arrow`: Previous slide;
- `F4`: jump to specific slide;
- `Up/Down` arrows for navigating the slide paragraphs;
- `Enter` on a paragraph to see the animation (if there's one);
- `Ctrl+Shift+Left or Right`: switch between left-side and right-side windows;
- `Ctrl+W`: quit the presentation.

# Resources

> **Note:** the slide captures will be **updated after each class** (hopefully in a timely fashion...).

- **Class examples**: in the `examples/` directory.
- **Slide captures**: [slides.pdf](slides.pdf)
- **Faust IDE**: [https://faustide.grame.fr](https://faustide.grame.fr)
- **Faust Playground**: [https://faustplayground.grame.fr/](https://faustplayground.grame.fr/)
- **Faust Manual**: [https://faustdoc.grame.fr/manual/introduction/](https://faustdoc.grame.fr/manual/introduction/)
- **Faust Syntax Documentation**: [https://faustdoc.grame.fr/manual/syntax/#faust-syntax](https://faustdoc.grame.fr/manual/syntax/#faust-syntax)
- **Faust Libraries Documentation**: [https://faustlibraries.grame.fr/libs/](https://faustlibraries.grame.fr/libs/)



## License

Copyright (c) Pierre Cochard <pierre.cochard@inria.fr>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
