# Change log

## Unreleased

- Fix compilation warnings from newer Rust updates.

## 0.2.1

- Remove redundant `cd .` from check remedies.

## 0.2.0

- Adds `medic-outdated-elixir`.

## 0.1.0

- Moved out of `medic-rs` to separate project.
- Adds `medic-check-elixir`:
  - `archive-installed`
  - `local-hex`
  - `local-rebar`
  - `packages-compiled`
  - `packages-installed`
  - `unused-deps`
- Adds `medic-step-elixir`:
  - `audit-deps`
  - `compile-deps`
  - `credo`
  - `dialyzer`
  - `get-deps`
