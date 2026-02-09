# Contributing Guidelines

To keep our workflow consistent, please follow these conventions when creating branches, commits, and pull requests.

## Branch Naming

Branches must follow this pattern:

```bash
<type>/<ticket-id>-<short-summary>
```

or if no ticket exists:

```bash
<type>/<short-summary>
```

### Types

- **feature** → new functionality
- **fix** → bug fix
- **chore** → maintenance or tooling
- **docs** → documentation only
- **refactor** → code restructuring without changing behavior
- **test** → adding or updating tests

### Examples

- `feature/HLAVI-50-Add-Storage-Interface`
- `fix/HLAVI-72-Handle-Async-Error`
- `docs/Update-API-Documentation`
- `feature/Add-SQLite-Storage`

### Notes:

- Include the ticket ID if available (future: HLAVI-XXX, currently optional)
- Use Pascal-Case or hyphenated words for the summary
- Keep summaries concise but descriptive
- All feature branches merge into `main` via Pull Request

## Commit Message Convention

Commit messages must follow this style:

```bash
<type>: <short summary>
```

or with ticket ID:

```bash
<type>(TICKET-ID): <short summary>
```

### Types

- **feature** → new feature
- **fix** → bug fix
- **chore** → maintenance or tooling
- **docs** → documentation
- **refactor** → code restructuring
- **test** → adding or updating tests
- **ci** → CI/CD changes
- **release** → version bumps and releases

### Examples

- `feature: add SQLite storage backend`
- `fix(HLAVI-72): handle async errors in file operations`
- `docs: update storage interface documentation`
- `refactor: split ticket logic into separate modules`
- `ci: add multi-platform build workflow`

### Notes:

- Use imperative mood in the summary (add, fix, update)
- Keep summaries concise (< 72 characters)
- Ticket ID optional until tracking system is finalized

## Pull Request Titles

PR titles must follow this pattern:

```bash
[XX] <ticket-id>: <short summary>
```

or without ticket:

```bash
[XX] <short summary>
```

### Where:

- **XX** → Your initials (first 2 characters of first name + surname)
- **ticket-id** → Optional ticket identifier (e.g., HLAVI-50)
- **short summary** → Description of the completed work

### Examples

- `[MaMu] HLAVI-50: Added SQLite Storage Backend`
- `[MaMu] Fix async error handling`
- `[JoDo] Add storage interface documentation`

### Notes:

- Use past tense for the summary (describes what was done)
- Keep summaries under 70 characters
- Link to ticket in PR description when available

## Dependency Management

**hlavi-core is a library dependency** used by other Hlavi projects. Changes here affect downstream projects (hlavi-cli, hlavi-api, hlavi-agent).

### Important Workflow

1. **Make changes in hlavi-core**
2. **Push and ensure CI passes**
3. **Create a versioned release** using the version-bump workflow
4. **Update dependent projects** to reference the new version

### Dependent Projects

Projects that depend on hlavi-core:
- **hlavi-cli** - Command-line interface
- **hlavi-api** - HTTP API server
- **hlavi-agent** - AI agent orchestration

These projects reference hlavi-core via git dependency:

```toml
[dependencies]
hlavi-core = { git = "https://github.com/mmuhlariholdings/hlavi-core", branch = "main" }
```

Or with specific tag:

```toml
hlavi-core = { git = "https://github.com/mmuhlariholdings/hlavi-core", tag = "v0.1.14" }
```

### Versioning Guidelines

- **Patch (0.0.x)**: Bug fixes, documentation, minor tweaks
- **Minor (0.x.0)**: New features, non-breaking API additions
- **Major (x.0.0)**: Breaking API changes

Use the GitHub Actions "Tag and Release" workflow to create releases:
1. Go to Actions tab
2. Select "Tag and Release" workflow
3. Click "Run workflow"
4. Choose version type (patch/minor/major)

## Workflow

### 1. Create Feature Branch

```bash
# Start from main
git checkout main
git pull origin main

# Create feature branch
git checkout -b feature/HLAVI-50-Add-SQLite-Storage
```

### 2. Make Changes and Commit

```bash
git add .
git commit -m "feature(HLAVI-50): add SQLite storage backend with migrations"
```

### 3. Push and Create PR

```bash
git push origin feature/HLAVI-50-Add-SQLite-Storage
```

Then open a Pull Request on GitHub with title:
```
[MaMu] HLAVI-50: Added SQLite Storage Backend
```

### 4. Code Review

- Request reviews from maintainers
- Address feedback and push updates
- Ensure CI passes (tests, clippy, formatting)

### 5. Merge and Release

- Maintainer merges via GitHub after approval
- Feature branch is automatically deleted
- **Run version-bump workflow** to create tagged release
- Dependent projects can now update to new version

## Protected Branch Rules

The `main` branch is protected:

- ✅ Requires pull request reviews before merging
- ✅ Requires status checks to pass (CI)
- ✅ Requires branches to be up to date before merging
- ✅ Requires conversation resolution before merging
- ❌ Direct pushes not allowed
- ❌ Force pushes not allowed

## Code Quality Standards

Before submitting a PR, ensure:

- [ ] Code passes `cargo fmt`
- [ ] Code passes `cargo clippy`
- [ ] All tests pass `cargo test`
- [ ] New features have tests
- [ ] Documentation is updated
- [ ] Public API changes are documented
- [ ] Consider impact on dependent projects

## Questions?

If you have questions about contributing, please:
- Check existing issues and PRs
- Ask in GitHub Discussions
- Contact maintainers

Thank you for contributing to Hlavi! 🚀
