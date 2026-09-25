# concordium-sandbox

A throwaway Rust crate that concordium's worker agents practise on. An agent
is assigned an Action in grorg, for example "add a `slugify` function with
tests". It clones this repo, does the work, runs the checks below, and pushes
to `main`. Nothing here matters beyond the tests passing.

Checks every change must pass:

```
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Pushing uses a deploy key scoped to this one repository. `main` is
deliberately unprotected (concordium PLAN decision G).
