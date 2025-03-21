# Javascript/Typescript Installation

The JS/TS SDK is delivered as a `.tgz` compressed NPM package file, ready for inclusion into any project using a NPM compatible package manager. Currently, the package is designed for being compatible with
popular bundlers such as [`webpack`](https://webpack.js.org/) and [`vite`](https://vitejs.dev/)[^wasm-package] , and comes with TypesScript type definitions as well as ergonomic JavaScript wrappers.
These instructions assume that you already have a project setup using one of the supported bundlers.

## Installing the JS/TS SDK

To install the WASM SDK, simply place the compressed file next to your project and use `npm`/`pnpm` to install it from your project directory (the one containing your `project.json` file):

```bash
npm install <path-to-the-tgz-file>
```

The package can then be used within your application like so (example using `webpack`):

```typescript
import { ETOPaySdk, Environment, Level } from "@eto/etopay-sdk-wasm";

const sdk = await new ETOPaySdk();
sdk.initLogger(Level.Info);
await sdk.setEnvironment(Environment.Development);
await sdk.validateConfig();
```

or using a named import:

```typescript

import * as ETOPay from "@eto/etopay-sdk-wasm";

const sdk = new ETOPay.ETOPaySdk();
sdk.initLogger(ETOPay.Level.Info);
await sdk.setEnvironment(ETOPay.Environment.Development);
await sdk.validateConfig();

```

See the API reference for more information about the available functions.

## Updating the JS/TS SDK

Updating the SDK is simply replacing the file and performing the steps above again.

## Future releases of the JS/TS SDK

Future releases of the SDK for JS/TS will be published to the NPM registry, for even easier installation.

## Minimum supported version

The bindings have been tested to work with `webpack` v4.47.0.

[^wasm-package]: If the current package does not work for you, and you for example need a package for inclusion directly on a web page as a `<script>` tag, please reach out to the development team.
