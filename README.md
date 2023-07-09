<p style="width: fit-content; margin: 0 auto;">
  <img src=".github/logo.gif" width="90px"/>
</p>
<h1 align="center">Yew Template</h1>
<p align="center">A custom template for Yew.rs</p>

## What's Yew.rs
`Yew.rs` is a WebAssembly(wasm) framework based on Rust.

Official Site: [https://yew.rs/](https://yew.rs/)

## What I use this to do ?
😺I wanna try to write web app frontend without `JavaScript` or `TypeScript`.😺


## What's Required
1. Rust programming environment.
2. Trunk
~~~shell
cargo install trunk
~~~
3.Install rustup target: `wasm-unknown-unknown`
~~~shell
rustup target install wasm-unknown-unknown
~~~

## Features
* Stylesheet: scss ✅
* 😁You won't even write one line of JavaScript. 😁 ✅
* Pre-configuration of some general tools:
    * css/scss compiler
    * features of `Cargo.toml` dependencies
    * auto copy static assets


## Development
Yew.rs is friendly if you can use `React.js` or `Solid.js`, I'd like to call this framework: `React.rs`. 😻

Development docs: [https://yew.rs/docs/getting-started/introduction](https://yew.rs/docs/getting-started/introduction)

## Deploy
Deploy a wasm app is same as you deploy a webpage. You may use `Github Pages` , `Node.js + Express`，`nginx` and etc.

### Build
You can set custom `public_url` for Github Pages in `Trunk.toml`.
~~~shell
trunk build --release
~~~