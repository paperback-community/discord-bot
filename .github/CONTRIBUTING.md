# Contributing Guidelines <!-- omit in toc -->

Before checking the contributing guideline, please read through our [Code of Conduct](https://github.com/paperback-community/discord-bot?tab=coc-ov-file#readme).

We encourage and value all types of contributions. Refer to the [Table of Contents](#table-of-contents) for guidance on how to start contributing to this project. Following the guidelines will make the experience smoother for everyone involved, and the community looks forward to your contributions!

## Table of Contents <!-- omit in toc -->

-   [Development Contributions](#development-contributions)
    -   [Project Conventions](#project-conventions)
    -   [Development and Build Process](#development-and-build-process)
        -   [Commands](#commands)
        -   [.env File Structure](#env-file-structure)
    -   [Open Tasks](#open-tasks)
-   [Documentation Contributions](#documentation-contributions)
-   [Support Contributions](#support-contributions)
    -   [Answering Questions](#answering-questions)
    -   [Validating Bug Reports](#validating-bug-reports)
    -   [Contributing to Enhancement Suggestions](#contributing-to-enhancement-suggestions)

## Development Contributions

### Project Conventions

-   **Layout:** The layout is based on [these conventions](https://doc.rust-lang.org/cargo/guide/project-layout.html). Please create an issue before making any changes to it.

-   **Branching:** The project follows [these branching conventions](https://gist.github.com/digitaljhelms/4287848). Ensure branches are named and managed accordingly.

-   **Versioning:** The project uses [Semantic Versioning 2.0.0](https://semver.org).

-   **Commit Messages:** Ensure your commit messages are clear and descriptive.

-   **Pull Requests:** Each pull request must include a detailed description of the changes and relevant links to other issues or pull requests, rather than just using the commit message as the title.

### Development and Build Process

#### Commands

Check the [`Makefile`](MakeFile) for available commands and [`Cargo.toml`](Cargo.toml) for configurable features.

#### .env File Structure

The default [`Makefile`](Makefile) command uses the `dotenv` feature, allowing you to configure environment variables during development with a `.env` file:

```env
# The stdout log level (OFF, ERROR, WARN, INFO, DEBUG, TRACE)
LOG_STDOUT_LEVEL=INFO
# The files log level (OFF, ERROR, WARN, INFO, DEBUG, TRACE)
LOG_FILES_LEVEL=INFO
# The Discord token of your bot, will be moved to the YAML config down the line
DISCORD_TOKEN=
```

### Open Tasks

You can work on bug fixes or approved enhancement suggestions that haven't been picked up yet. The README will also list planned major features. Check in with us via our [Discord server](https://discord.gg/netsky-s-basement-965890377896845352) if you want to work on those.

## Documentation Contributions

We welcome contributions that improve, update, or correct the documentation.

## Support Contributions

### Answering Questions

Assist by answering questions from other community members. Questions can be asked in our [Discord server](https://discord.gg/netsky-s-basement-965890377896845352) or in the [Q&A Discussions](https://github.com/paperback-community/discord-bot/discussions/categories/q-a). Your insights are always appreciated!

### Validating Bug Reports

Help ensure bugs can be addressed effectively by attempting to replicate reported issues and confirming their existence.

### Contributing to Enhancement Suggestions

Enhancement suggestions often need feedback and refinement. Contribute by reviewing and discussing these suggestions, offering your perspective, and proposing improvements. Your input helps shape the future of the project.
