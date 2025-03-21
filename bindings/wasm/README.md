# ETOPay SDK WASM bindings

In order to use the etopay sdk in a webapp the browser, this crate provides `wasm` bindings and generation of TypeScript type definitions for all the data type objects used.
It also has functionality for publishing the bindings together with the generated `wasm` binary to a NPM registry.

## Development

### Build with `wasm-pack build`

```
wasm-pack build
```

### Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### Publish to NPM with `wasm-pack publish` after building

```
wasm-pack publish
```

### Executing the example scripts
1. Navigate to examples directory: `cd bindings/wasm/examples`.
2. Add a `.env` file as shown below, with the corresponding values:
```
MNEMONIC=
MNEMONIC_ALICE=
KC_URL=
KC_REALM=
KC_CLIENT_ID=
KC_CLIENT_SECRET=
PASSWORD=
EXAMPLE_BACKEND_URL=
```
3. Install dependencies: `bun install`.
4. Build the package with `nodejs` as target: `wasm-pack build --target nodejs`.
5. Run individual examples, ex: `bun 07-get_balance.ts`.

In case you want to run all the examples after installing the dependencies and building the package:
1. Navigate back to sdk directory: `cd sdk`.
2. Run: `make run_wasm_node_examples`. This will execute the `run_all_examples.sh` script on `bindings/wasm/examples`, which runs all the examples.

### Executing the webapp example

1. Follow step 1 above to make sure you have a `.env` file in the root with a valid access token for either `satoshi`, `archiveme` or `alice` depending on example which you want to run.(access token can be obtained with the `hello.http` API request in `docs/api-requests/hello.http`).
2. Go into the webapp folder and install the dependencies:
  ```bash
  cd examples/webapp
  pnpm install
  ```
3. Start the development server with `pnpm start`. This will start a server on port `8080`, which you can forward to the host (if vscode does not do this automatically)
   by going to the `PORTS` tab in the vscode terminal and adding port `8080`. Then you can view the webapp at `http://localhost:8080/`. Open the developer console (F12)
   to view the sdk logs. Click the button of the example you want to run.
