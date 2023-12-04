# Medic Elixir

An extension pack for using [medic](https://github.com/synchronal/medic-rs)
with Elixir projects.

## Installation

```shell
brew tap synchronal/tap
brew install medic-ext-elixir
```

Example `Brewfile`:

```shell
tap 'synchronal/tap'

brew  'synchronal/tap/medic'
brew  'synchronal/tap/medic-ext-elixir'
```

## Usage

```toml
[doctor]
checks = [
  { check = "homebrew" },
  { check = "asdf", command = "plugin-installed", args = { plugin = "erlang" } },
  { check = "asdf", command = "plugin-installed", args = { plugin = "elixir" } },
  { check = "asdf", command = "package-installed", args = { plugin = "erlang" } },
  { check = "asdf", command = "package-installed", args = { plugin = "elixir" } },
  { check = "hex", command = "local-hex" },
  { check = "hex", command = "local-rebar" },
  { check = "hex", command = "packages-installed" },
]

[test]
checks = [
  { name = "Check for warnings", shell = "mix compile --force --warnings-as-errors" },
  { name = "Elixir tests", shell = "mix test --color --warnings-as-errors", verbose = true },
]

[audit]
checks = [
  { step = "elixir", command = "credo" },
  { step = "elixir", command = "dialyzer" },
  { step = "elixir", command = "audit-deps" },
  { check = "elixir", command = "unused-deps" },
]

[outdated]
checks = [
  { check = "elixir" },
]

[update]
steps = [
  { step = "git", command = "pull" },
  { step = "elixir", command = "get-deps" },
  { step = "elixir", command = "compile-deps", args = { mix-env = "dev" } },
  { step = "elixir", command = "compile-deps", args = { mix-env = "test" } },
  { doctor = {} },
  { name = "Migrate", shell = "mix ecto.migrate" },
  { name = "Build docs", shell = "mix docs" },
]

[shipit]
steps = [
  { name = "Check formatting", shell = "mix format --check-formatted" },
  { audit = {} },
  { update = {} },
  { test = {} },
  { step = "git", command = "push" },
  { step = "github", command = "link-to-actions", verbose = true },
]
```


## medic-check-elixir

Checks for whether an Elixir project is configured.

### archive installed?

Is the given package installed as a mix archive?

```shell
medic-check-elixir archive-installed --name <name>
```

### local hex installed?

Is hex installed locally?

```shell
medic-check-elixir local-hex
```

### local rebar installed?

Is rebar installed locally?

```shell
medic-check-elixir local-rebar
```

### packages compiled?

Are all mix deps compiled for a project?

```shell
medic-check-elixir packages-compiled
medic-check-elixir packages-compiled --cd path/to/project
```

### packages installed?

Are all mix deps installed for a project?

```shell
medic-check-elixir packages-installed
medic-check-elixir packages-installed --cd path/to/project
```

### unused deps?

Are there any hex deps listed in `mix.lock` that are not explicitly
or implicitly listed in `mix.exs`?

```shell
medic-check-elixir unused-deps
medic-check-elixir unused-deps --cd path/to/project
```


## medic-outdated-elixir

Check for outdated and updatable hex dependencies.


## medic-step-elixir

Steps for updating an Elixir project or for shipping changes to one.

### audit-deps

Runs `mix deps.audit`, assuming that the `mix_audit` package has been added
to a project.

```shell
medic-step-elixir audit-deps
medic-step-elixir audit-deps --cd path/to/project
```

### compile-deps

Compiles any dependencies that are not yet compiled. Deps for which mix
returns output indicating that the dep is compiled will be skipped.

```shell
medic-step-elixir compile-deps
medic-step-elixir compile-deps --cd path/to/project
medic-step-elixir compile-deps --mix-env test
```

### credo

Runs `mix credo --strict`.

```shell
medic-step-elixir credo
medic-step-elixir credo --cd path/to/project
```

### dialyzer

Runs `mix dialyzer`, assuming that the `dialyxir` package has been added

```shell
medic-step-elixir dialyzer
medic-step-elixir dialyzer --cd path/to/project
```
o a project.

### get-deps

Downloads any missing dependencies.

```shell
medic-step-elixir get-deps
medic-step-elixir get-deps --cd path/to/project
```
