# fxa

<img src="images/gh-logo.png"/><br/>

![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/tonykolomeytsev/fxa?label=version) 
![GitHub license](https://img.shields.io/github/license/tonykolomeytsev/fxa)

A multi-platform tool for exporting resources from Figma to an Android project.

## Features

- 🥑 **fxn** convert images to WEBP and icons to Android Vector Drawable XML.

- 🥰 **fxn** works on macOS, Windows and Linux. One small executable for each platform.

- 🚀 No external dependencies. **No webp** package, **no java** and **no vd-tool** required.

- 🤖 [Figma-export](https://github.com/RedMadRobot/figma-export)-like api. Similar YAML config file, similar CLI arguments.

- 🧭 Good error description. There will always be a clear explanation after the inscription Error.

<img src="images/gh-demo.png"/><br/>

## How to install?

> Installation via package managers will be available later.

### Installation on MacOS

Just run on terminal:

```bash
curl -o- https://raw.githubusercontent.com/tonykolomeytsev/fxa/master/install/macos.sh | bash
```

### Installation on Ubuntu

Just run on terminal:

```bash
curl -o- https://raw.githubusercontent.com/tonykolomeytsev/fxa/master/install/linux.sh | bash
```

### Installation on Windows

So far, there is no simple installation guide. Download the [zip archive from here](https://github.com/tonykolomeytsev/fxa/releases/latest/download/fxa-v0.1.0-x86_64-pc-windows-msvc.zip) and unzip it. Inside there will be a `fxa.exe` program, you can run it from the terminal.

### Build source code

Install Rust compiler: https://www.rust-lang.org/tools/install

And then clone and build the project with `cargo`:

```bash
cargo build --release
```

And then take the compiled program `{project_root}/target/release/fxn`

## How to use?

Use `fxn --help` for help :)

#### Create yaml config file:

```bash
fxn config for_images.yaml
```

And then fill out the `config.yaml` file with your data (figma file id, frame names etc.).

#### Export images (PNG, WEBP, SVG)

```bash
fxn images -c config.yaml img_lol "img_kek" ...
```

#### Export icons (Vector Drawable XML, SVG)

```bash
fxn icons -c config.yaml ic_24/icon1 ic_16/icon2 ...
```

### What about figma personal token?

Yes it's needed. Pass it as argument `-t TOKEN` or just create environment variable `FIGMA_PERSONAL_TOKEN` and the util will take it on its own.

Check `fxn --help`.

## Project status

The project is in progress and is being developed just for fun. Additional features will be added in the future.