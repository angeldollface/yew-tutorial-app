# YEW TUTORIAL APP :spider_web: :lock: :crab:

***A small tutorial app for Yew.rs. :spider_web: :lock: :crab:***

![GitHub CI](https://github.com/angeldollface/yew-tutorial-app/actions/workflows/yew.yml/badge.svg)

## ABOUT :books:

Some time ago I wrote a tutorial on how to make 3D graphics using Javascript in the browser on my blog. This repository contains the soruce code for a tutorial on how to get started with Rust's web framework, *Yew.rs*. Here's [the link to the blog post](https://angeldollface.art/). Enjoy. :heart_on_fire:

## DEPLOYED PROJECT ON GITHUB PAGES :rocket:

To view a live deployed version of this project, click here: [VIEW](https://angeldollface.art/yew-tutorial-app)

## USAGE :hammer:

- 1.) Visit [this link](https://angeldollface.art/yew-tutorial-app).
- 2.) Type in the desired length of the password you want to generate.
- 3.) Press the `GENERATE` button.
- 4.) Use your generated password by pasting it somewhere with `Ctrl+V`.
- 5.) Enjoy. :heart:

## TRY THE CODE FOR YOURSELF! :inbox_tray:

You should have the following tools installed and available from the command line:

- Rust
- Git

To try *Ceramic.rs* on your own machine, follow these steps:

- 1.) Install `trunk` from [crates.io](https://crates.io/crates/trunk):

```bash
cargo install trunk
```

- 2.) Clone this project's source code:

```bash
git clone https://github.com/angeldollface/yew-tutorial-app
```

- 3.) Change directory into the source code's root directory:

```bash
cd yew-tutorial-app
```

- 4.) Serve the app locally (This will serve the app locally on [`http://127.0.0.1:8080/yew-tutorial-app/`](http://127.0.0.1:8080/yew-tutorial-app/).):

```bash
trunk --config trunk.toml serve --release
```

- 5.) If you want to build the app into a bundle to deploy to a server, run the command below. This will produce a directory called `dist` with the bundle inside it.

```bash
trunk --config trunk.toml build --release
```

- 5.) Enjoy! :heart_on_fire:


## CHANGELOG :black_nib:

### Version 1.0.0

- Initial release.
- Upload to GitHub.
- Deployment to GitHub Pages.

## NOTE :scroll:

- *Yew Tutorial App :spider_web: :lock: :crab:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.