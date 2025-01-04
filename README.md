# Wasrex

Before starting phoenix server to bundle wasm with react:
  * Run `cd assets`
  * Run `cargo build --target wasm32-unknown-unknown`
  * Run `cargo install -f wasm-bindgen-cli`
  * Run `wasm-bindgen target/wasm32-unknown-unknown/debug/rusty_react.wasm --out-dir build`
  * Run `npm run build`
  * Start Phoenix endpoint with `mix phx.server` or inside IEx with `iex -S mix phx.server`

Now you can visit [`localhost:4000`](http://localhost:4000) from your browser.

Ready to run in production? Please [check our deployment guides](https://hexdocs.pm/phoenix/deployment.html).

## Learn more

  * Official website: https://www.phoenixframework.org/
  * Guides: https://hexdocs.pm/phoenix/overview.html
  * Docs: https://hexdocs.pm/phoenix
  * Forum: https://elixirforum.com/c/phoenix-forum
  * Source: https://github.com/phoenixframework/phoenix
