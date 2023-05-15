# Medic Elixir

An extension pack for using [medic](https://github.com/synchronal/medic-rs)
with Elixir projects.


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
