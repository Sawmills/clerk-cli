# Changelog

## [0.17.0](https://github.com/Sawmills/clerk-cli/compare/v0.16.0...v0.17.0) (2026-01-21)


### New Features

* **orgs:** add --id-only flag for scripting ([006a313](https://github.com/Sawmills/clerk-cli/commit/006a3134219f50eb29f8e3b079339803b5de0301))

## [0.16.0](https://github.com/Sawmills/clerk-cli/compare/v0.15.0...v0.16.0) (2026-01-21)


### New Features

* **sso:** add top-level sso list command ([a98f90a](https://github.com/Sawmills/clerk-cli/commit/a98f90a1eb4ca70e958be7260ffed525eee47ad2))

## [0.15.0](https://github.com/Sawmills/clerk-cli/compare/v0.14.0...v0.15.0) (2026-01-20)


### New Features

* **orgs:** add SSO connection management ([e4f9298](https://github.com/Sawmills/clerk-cli/commit/e4f9298fff5bb1b95131652fb1ecbcda34cee4ea))

## [0.14.0](https://github.com/Sawmills/clerk-cli/compare/v0.13.0...v0.14.0) (2026-01-16)


### Features

* **orgs:** add delete command with confirmation prompt ([a2bc6f6](https://github.com/Sawmills/clerk-cli/commit/a2bc6f6d4e460ad2d8c7804d7bb127c96d14b7f9))

## [0.13.0](https://github.com/Sawmills/clerk-cli/compare/v0.12.0...v0.13.0) (2026-01-16)


### Features

* add user management commands and improve completions ([f23d96f](https://github.com/Sawmills/clerk-cli/commit/f23d96fee8e13202279b6540eb2c6d5580d8a890))

## [0.12.0](https://github.com/Sawmills/clerk-cli/compare/v0.11.0...v0.12.0) (2026-01-15)


### Features

* **orgs:** add jwt template argument for member jwt action ([c7ba8b8](https://github.com/Sawmills/clerk-cli/commit/c7ba8b83631bb266221a2206651e83243966d7b4))

## [0.11.0](https://github.com/Sawmills/clerk-cli/compare/v0.10.0...v0.11.0) (2026-01-15)


### Features

* **orgs:** add jwt action for org members ([4177cfb](https://github.com/Sawmills/clerk-cli/commit/4177cfba9d968e1c98a05b26c1abf7ed4c695fe1))

## [0.10.0](https://github.com/Sawmills/clerk-cli/compare/v0.9.1...v0.10.0) (2026-01-15)


### Features

* **orgs:** add impersonate action for org members ([8b23ff7](https://github.com/Sawmills/clerk-cli/commit/8b23ff75d1a53ba9ac5c988c428d3a94fec39807))

## [0.9.1](https://github.com/Sawmills/clerk-cli/compare/v0.9.0...v0.9.1) (2026-01-15)


### Bug Fixes

* **completions:** show subcommands after org slug in zsh tab completion ([6b98e73](https://github.com/Sawmills/clerk-cli/commit/6b98e73f5a6ef1a472135cd720543bba4607dcf5))

## [0.9.0](https://github.com/Sawmills/clerk-cli/compare/v0.8.0...v0.9.0) (2026-01-15)


### Features

* support 'clerk orgs &lt;org&gt;' and 'clerk orgs &lt;org&gt; members' syntax ([5263f57](https://github.com/Sawmills/clerk-cli/commit/5263f57d4f14ab0dfb6bc6c7401f94135c837a3d))

## [0.8.0](https://github.com/Sawmills/clerk-cli/compare/v0.7.0...v0.8.0) (2026-01-15)


### Features

* restructure orgs command and bundle custom zsh completions ([e55b2f8](https://github.com/Sawmills/clerk-cli/commit/e55b2f8529fceabfe79e04b72ae639c70c0882f6))

## [0.7.0](https://github.com/Sawmills/clerk-cli/compare/v0.6.0...v0.7.0) (2026-01-15)


### Features

* add orgs members subcommand and remove members column ([35e11c7](https://github.com/Sawmills/clerk-cli/commit/35e11c7edcfea1b4ce8c03abb6d9b7c7cae5c51b))

## [0.6.0](https://github.com/Sawmills/clerk-cli/compare/v0.5.0...v0.6.0) (2026-01-15)


### Features

* switch to nucleo-picker and add dynamic shell completions ([9690e4c](https://github.com/Sawmills/clerk-cli/commit/9690e4c21701712a68e0c7c891fb4e2754ef17b3))

## [0.5.0](https://github.com/Sawmills/clerk-cli/compare/v0.4.0...v0.5.0) (2026-01-15)


### Features

* add interactive org picker and org-scoped impersonation ([42b1fd3](https://github.com/Sawmills/clerk-cli/commit/42b1fd3196508d4fd12a44b496df249ea5d04b36))

## [0.4.0](https://github.com/Sawmills/clerk-cli/compare/v0.3.0...v0.4.0) (2026-01-15)


### Features

* add jwt command for API testing ([8a02fc9](https://github.com/Sawmills/clerk-cli/commit/8a02fc9f21615b9ef09c317aeb114b509b93008e))


### Bug Fixes

* **ci:** check staged diff after git add for new files ([8796b9d](https://github.com/Sawmills/clerk-cli/commit/8796b9d9ef72b73389eef5d5d7783afc4ed553bb))
* **ci:** use token auth for homebrew-tap clone/push ([d1fb6b4](https://github.com/Sawmills/clerk-cli/commit/d1fb6b411f05f2595982429c2ae50c2cbab043da))

## [0.3.0](https://github.com/Sawmills/clerk-cli/compare/v0.2.0...v0.3.0) (2026-01-15)


### Features

* initial Clerk admin CLI implementation ([3651c9b](https://github.com/Sawmills/clerk-cli/commit/3651c9b59c5ede60e5622d4ce46cda590549ec4c))
* **orgs:** add --ids-only flag for plain ID output ([7d01789](https://github.com/Sawmills/clerk-cli/commit/7d01789a1508aadc9f2f0b103d03c625ecdc57df))


### Bug Fixes

* **test:** remove hardcoded version check in cli_version test ([60ebb31](https://github.com/Sawmills/clerk-cli/commit/60ebb315f326ed15ba1bb30b2e246d5b044a69cd))
* use CLERK_API_KEY env var instead of CLERK_SECRET_KEY ([8b05b04](https://github.com/Sawmills/clerk-cli/commit/8b05b047eac87aff1c87045032ae3aff0ade5f80))

## [0.2.0](https://github.com/Sawmills/clerk-cli/compare/clerk-cli-v0.1.0...clerk-cli-v0.2.0) (2026-01-15)


### Features

* initial Clerk admin CLI implementation ([3651c9b](https://github.com/Sawmills/clerk-cli/commit/3651c9b59c5ede60e5622d4ce46cda590549ec4c))
* **orgs:** add --ids-only flag for plain ID output ([7d01789](https://github.com/Sawmills/clerk-cli/commit/7d01789a1508aadc9f2f0b103d03c625ecdc57df))


### Bug Fixes

* **test:** remove hardcoded version check in cli_version test ([60ebb31](https://github.com/Sawmills/clerk-cli/commit/60ebb315f326ed15ba1bb30b2e246d5b044a69cd))
* use CLERK_API_KEY env var instead of CLERK_SECRET_KEY ([8b05b04](https://github.com/Sawmills/clerk-cli/commit/8b05b047eac87aff1c87045032ae3aff0ade5f80))
