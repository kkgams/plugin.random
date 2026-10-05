# plugin.random

GAMS WASM Project Unit for seeded random generation.
Distribution and WIT interface versions are independent.

## API

The `gams:random/random-provider@1.0.0` world exports
`wasi:random/random@0.2.0` plus `seeded-random` controls for setting/reading
the seed and reading the generation counter. Consumers resolve this component
as a provider of the WASI random interface.

The contract is in `plugins/random.comp/wit/package.wit`.
Plugin Manager resolves imports; distribution filenames do not rename WIT identities.

## Build and test

All source/build inputs are owned by this repository; no sibling checkout is
required for its supported build/test commands. Use the pinned Nix environment:

```sh
nix develop --command make build
nix develop --command make test
```

Output: `dist/plugin.random.wasm`. Install it as
`plugins/random.comp.wasm` in an external GAMS Project. Configure that Project separately.

The flake and lockfiles pin the toolchain. Network access may be required to
fetch tools/modules. `.envrc` supports `direnv allow`.

`make test` runs offline integrity/publication regression tests, builds the
components, validates WASM, extracts WIT, and checks the exact input inventory.
It also transpiles the built components with jco and runs standalone WASM
runtime assertions. These are bounded smoke checks, not full API or Host
integration coverage.

## Licensing and releases

GAMS-authored contributions are Apache-2.0. Third-party code retains its own
terms; see `LICENSING.md`, `THIRD-PARTY-REVIEW.md`, and `LICENSES/`.
Source inventory and notice evidence are checked before candidate packaging.
Changed inputs require refreshed evidence and review of the resulting digests;
checksum consistency alone is not legal or publication approval.

Distribution is through GitHub Releases, not npm or OCI. Before publishing:

1. Review the current source, third-party evidence, and final linked artifact.
   Hosted build/candidate review and runtime validation remain outstanding until
   independently recorded; preparation or local tests do not clear them.
2. Set repository-scoped `LICENSE_SHA256` and `NOTICE_SHA256` Actions variables
   to the exact reviewed texts. Local rehearsal uses
   `APPROVED_LICENSE_SHA256` and `APPROVED_NOTICE_SHA256`.
3. Push the reviewed `release` branch and inspect its hosted candidate. Without
   approved digests, branch checks do not distribute a candidate. Manual release
   workflow dispatch verifies but does not publish.
4. Tag that reviewed release-branch commit as `v<version.txt>`. Never reuse or
   move a published or failed tag. Publication requires immutable-release policy,
   canonical repository/version identity, exact artifacts and checksums, embedded
   notices, and complete readable legal assets. An existing release blocks creation.

The publication workflow rehearses a private draft's bytes before publication
and checks the anonymous public bytes afterward. Keep LICENSE, NOTICE, evidence,
review documents, and applicable LICENSES with distributed artifacts. See the
repository's release workflow and scripts for the enforced gates.
