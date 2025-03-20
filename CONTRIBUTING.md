# Contributing to JanusLens

Thank you for considering contributing to JanusLens! This document provides guidelines and instructions for contributing to make the process smooth for everyone.

## Getting Started

1. **Fork the repository**
2. **Clone your fork**
   ```bash
   git clone https://github.com/YOUR-USERNAME/januslens.git
   cd januslens
   ```
3. **Set up the development environment**
   - Follow the [Setup Guide](docs/SETUP.md) for detailed instructions
   - Make sure all prerequisites are installed

## Development Workflow

1. **Create a branch for your feature/fix**
   ```bash
   git checkout -b feature/your-feature-name
   ```
   Use prefixes like `feature/`, `bugfix/`, `docs/`, or `refactor/` to categorize your work.

2. **Make your changes**
   - Follow the code style and conventions (see below)
   - Write tests for new features
   - Ensure all tests pass
   - Keep your changes focused on a single issue/feature

3. **Commit your changes**
   - Use clear, descriptive commit messages
   - Follow the [Conventional Commits](https://www.conventionalcommits.org/) format:
     ```
     type(scope): short description
     
     Longer description if necessary
     
     Fixes #issue-number
     ```
   - Types include: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`

4. **Submit a Pull Request**
   - Push your branch to your fork
   - Open a pull request against the main repository's `main` branch
   - Provide a clear description of the changes
   - Link to any relevant issues
   - Be prepared to discuss your changes and make adjustments

## Code Style and Guidelines

### Rust Code

- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html)
- Use `rustfmt` to format your code: `cargo fmt`
- Run `cargo clippy` to catch common mistakes
- Document public API with rustdoc comments

### Frontend Code (Svelte/TypeScript)

- Use TypeScript for type safety
- Follow the [Svelte Style Guide](https://github.com/sveltejs/eslint-plugin-svelte3)
- Use Prettier for code formatting
- Follow component naming conventions:
  - PascalCase for component files and names
  - camelCase for variables and functions
  - Use descriptive names for components and functions

### Testing

- Write unit tests for all new features
- Ensure existing tests pass
- Aim for good test coverage, especially for critical functionality
- Structure tests clearly with descriptive names

## Documentation

- Update documentation for any changed or new features
- Document API changes
- Add comments for complex logic
- Use clear, concise language
- Include examples where appropriate

## Pull Request Process

1. Ensure your code passes all tests and linting checks
2. Update the README.md or documentation with details of changes if appropriate
3. Add your changes to the CHANGELOG.md file under the "Unreleased" section
4. The PR will be reviewed by maintainers who may request changes
5. Once approved, a maintainer will merge your PR

## Performance Considerations

JanusLens focuses heavily on performance. When implementing features:

- Consider memory usage and optimize where possible
- Be mindful of UI responsiveness
- Test with large repositories to ensure scalability
- Profile your code when working on performance-critical features

## Reporting Bugs

When reporting bugs, please include:

1. A clear, descriptive title
2. Steps to reproduce the issue
3. Expected behavior
4. Actual behavior
5. Screenshots or logs if applicable
6. Your environment (OS, Rust version, Node version, etc.)

## Suggesting Features

Feature suggestions are welcome! Please provide:

1. A clear, descriptive title
2. Detailed description of the proposed feature
3. Any relevant mock-ups or examples
4. Explanation of why this feature would be useful to most users

## Code of Conduct

Please be respectful and considerate of others. We strive to create a welcoming and inclusive environment for everyone.

- Be respectful and inclusive
- Be patient and welcoming
- Be thoughtful
- Be collaborative
- When disagreeing, try to understand why

## License

By contributing to JanusLens, you agree that your contributions will be licensed under the project's license.

## Questions?

If you have questions or need help, please:

1. Check the [Developer Guide](docs/DEVELOPER.md)
2. Open an issue with your question
3. Reach out to the maintainers

---

Thank you for contributing to JanusLens! 