# plugin.random

Seeded random generation for [GAMS](https://github.com/kkgams/gams).

| Unit | Source | Original work | License |
| --- | --- | --- | --- |
| Random provider | `plugins/random.comp/component.c` and `wit/package.wit` | GAMS contributors; WASI specification contributors | [Apache-2.0](LICENSE), [original dependency terms](LICENSES/), [NOTICE](NOTICE) |

## Development

Use the pinned toolchain with `nix develop --command make <target>`.

| Command | Purpose |
| --- | --- |
| `make test` | Run native C and stateful Wasmtime component assertions. |
| `make build` | Compile the C/WASI component with embedded complete terms. |
| `make package` | Build `build.nosync/plugin.random.wasm`, `LICENSES.zip` and `SHA256SUMS`. |
| `make open` | Build and open the local Project in the released macOS Apple Silicon Host. |
| `make docs` | Build the pinned wiki in `build.nosync/site/`. |
| `make serve-docs` | Serve docs; independent `DOCS_PORT` defaults to 8080. |
| `make validate-docs` | Validate wiki content. |
| `make clean` | Remove all of `build.nosync/`, `tools/` and `gams_modules/`, including unmarked contents; refuse tracked files and symlinks before deleting anything. |

The local Files View browses the Project; it is not a random-generation demo. First launch downloads pinned supporting Units. An existing Host is never updated implicitly. `make test` checks deterministic seed replay, returned byte length and seed readback both directly in C and through the component ABI using a native Rust/Wasmtime test with one persistent store/instance. These bounded tests do not establish Host integration coverage.

## API

World `gams:random/random-provider@1.0.0` exports `wasi:random/random@0.2.0` and seeded controls (`set-seed`, `get-seed`, `get-generation`). Consumers use it as a WASI random provider. Complete types are in `plugins/random.comp/wit/package.wit`.
