# Projection [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/brettm12345/projection/Continuous%20Integration?label=Continuous%20Integration&logo=github&style=flat-square)](https://github.com/Brettm12345/projection/actions?query=workflow%3A%22Continuous+Integration%22)

> Next generation project management cli

Projection is a project manager for the command line.
This project is still in it's beta stages.

## Todo

- [x] Remove all uses of `unwrap`. _Note:_ `unwrap` is still used inside tests and the main function.
- [x] Handle removal project directories from filesystem.
- [ ] Add custom tagging.
- [ ] Add support for a `projection` or a `.projection` file at the root of the project. Supported file formats will be `dhall`, `toml`, `yaml` and `json`
- [ ] Interactively create projects.
- [ ] Tui for searching and managing projects.
- [x] `no-confirm` option.
- [x] Fuzzy searching.
- [x] Local db to store projects and metadata.
- [x] Syntax to search through projects.
- [ ] Shell plugin generation.
- [x] Setup testing environment.
- [ ] Filter by language.
- [ ] Allow the user to define their own output format.
- [ ] Customized preview.
- [ ] Vim plugin
- [ ] Integrate with [Projectile](bbatsov/projectile)
