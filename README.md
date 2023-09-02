<div align="center">
  <img src=".github/logo.gif" width="90px"/>
</div>
<h1 align="center">Yew Template</h1>
<p align="center">A custom template for Yew.rs</p>

## What's Yew.rs
`Yew.rs` is a WebAssembly(wasm) framework based on Rust.

Official Site: [https://yew.rs/](https://yew.rs/)

## Notice
`wasm-bindgen` is required when you run `trunk serve`，if you are in trouble downloading this module, please download it manually.

### Manually install `wasm-bindgen`
1. Download: [https://github.com/rustwasm/wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
2. Unzip it and rename the folder: wasm-bindgen-{version}
3. Place it into cache folder
   - Windows:
     ~~~
     Place wasm-bindgen-{version} into
     C:\Users\{username}\AppData\Local\trunkrs\trunk\cache
     ~~~
   - macOS:
     ~~~shell
      # Place wasm-bindgen-{version} into
      # /Users/{username}/Library/Caches/dev.trunkrs.trunk
      # You may encounter a warning when trunk try to invoke wasm-bindgen, please run the following command.
      xattr -d com.apple.quarantine wasm-bindgen--{version}
     ~~~
4. Run `trunk serve`

## What I use this to do ?
😺I wanna try to write web app frontend without `JavaScript` or `TypeScript`.😺

## You need to install
1. Rust programming environment.
2. Trunk
~~~shell
cargo install trunk
~~~

3. Install rustup target: `wasm-unknown-unknown`
~~~shell
rustup target install wasm-unknown-unknown
~~~

## Features
* ✅ Stylesheet: scss 
* ✅ 😁You won't even write one line of JavaScript. 😁 
* ✅ Pre-configuration of some general tools:
    * ✅ CSS/SCSS compiler
    * ✅ features of `Cargo.toml` dependencies
    * ✅ auto copy static assets


## Development
Yew.rs is friendly if you can use `React.js` or `Solid.js`, I'd like to call this framework: `React.rs`. 😻

Development docs: [https://yew.rs/docs/getting-started/introduction](https://yew.rs/docs/getting-started/introduction)

## Deploy
Deploy a wasm app is same as you deploy a webpage. You may use `Github Pages` , `Node.js + Express`，`nginx` and etc.

### Build
You can set custom `public_url` for Github Pages in `Trunk.toml`.
Remember change `release = false` to `true`
~~~shell
trunk build --release
~~~

Or directly use shell command
~~~shell
trunk build --release --public-url /your-github-repo-name/
~~~
