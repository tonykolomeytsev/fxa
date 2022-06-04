# fxa

<img src="images/gh-logo.png"/><br/>

A multi-platform tool for exporting resources from Figma to an Android project.

## Features

- **The util works on macOS, Windows, Linux systems, etc.**

    You can build it for any platform supported by the Rust compiler.

- **No external dependencies.**

    You don't need to install the webp package to convert images to webp. You don't need to install java and run vd-tool to convert svg to xml.

- **Figma-export-like api**

    Similar YAML config file, similar CLI arguments.

- **Good error description**

    There will always be a clear explanation after the inscription Error.

- **Just one small executable**

- **One error does not interrupt the entire export process**

## How to use?

> Installation via package managers will be available later.

### Building

Build project with `cargo` or `rustc`:

```bash
cargo build --release
```

And then take the compiled program `{project_root}/target/release/fxn`

### Running

Use `fxn --help` for help :)

#### Create yaml config file:

```bash
fxn config for_images.yaml
```

And then fill out the `for_images.yaml` file with your data (figma file id, frame names etc.).

#### Export images (PNG, WEBP, SVG)

```bash
fxn images -c for_images.yaml img_lol img_kek ...
```

#### Export icons (Vector Drawable XML, SVG)

```bash
fxn icons -c for_icons.yaml ic_24/icon1 ic_16/icon2 ...
```

### What about figma personal token?

Yes it's needed. Pass it as argument `-t TOKEN` or just create environment variable `FIGMA_PERSONAL_TOKEN` and the util will take it on its own.

Check `fxn --help`.

## Project status

The project is in progress and is being developed just for fun. Additional features, such as svg to xml conversion, will be added in the future.