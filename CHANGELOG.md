## [0.4.0] - 2025-08-27

### 🚀 Features

- Use Duration for the queue polling timeout, not a simple number
- A panic in a job should not bring down the whole worker

### ⚙️ Miscellaneous Tasks

- Update dependencies
- Run rustfmt
- Add cargo-release config
- Recreate changelog
- Recreate changelog
## [0.2.6] - 2023-04-11

### 🐛 Bug Fixes

- *(deps)* Make cargo audit happy by using only needed chrono features and updated cron_parser
## [0.2.5] - 2023-04-07

### 🐛 Bug Fixes

- Fix example not to ignore a result

### ⚙️ Miscellaneous Tasks

- Run Rustfmt
- Rustfmt the examples after the fix
## [0.2.4] - 2023-03-18

### 🐛 Bug Fixes

- Replace the library we use for cron parsing, it relied on a quite old nom, which was throwing clippy future incompatibility warnings

### ⚙️ Miscellaneous Tasks

- Run rustfmt
## [0.2.3] - 2023-01-17

### ⚙️ Miscellaneous Tasks

- Bump dependencies
## [0.2.2] - 2022-11-03

### 💼 Other

- Bump dependencies

### ⚙️ Miscellaneous Tasks

- Bump dependencies
- Bump dependencies
## [0.2.1] - 2022-07-23

### 🚀 Features

- Re-export typetag so that the users don't have to depend on it

### ⚙️ Miscellaneous Tasks

- Format changelog
## [0.2.0] - 2022-07-13

### ⚙️ Miscellaneous Tasks

- [**breaking**] Update dependencies especially type_tag
- Version bump
## [0.1.2] - 2021-11-29
