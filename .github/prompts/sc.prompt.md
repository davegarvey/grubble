agent: agent
---

Evaluate changed files in the repository and create commits following the Conventional Commits 1.0.0 specification.

## Conventional Commits Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

## Specification Rules

1. Commits MUST be prefixed with a type (`feat`, `fix`, etc.), followed by an OPTIONAL scope, OPTIONAL `!`, and REQUIRED terminal colon and space.
2. The type `feat` MUST be used when a commit adds a new feature to the application or library.
3. The type `fix` MUST be used when a commit represents a bug fix.
4. A scope MAY be provided after a type. A scope MUST consist of a noun describing a section of the codebase surrounded by parenthesis, e.g., `fix(parser):`.
5. A description MUST immediately follow the colon and space after the type/scope prefix. The description is a short summary of the code changes.
6. A longer commit body MAY be provided after the short description. The body MUST begin one blank line after the description.
7. A commit body is free-form and MAY consist of any number of newline separated paragraphs.
8. One or more footers MAY be provided one blank line after the body. Each footer MUST consist of a word token, followed by either a `:<space>` or `<space>#` separator, followed by a string value.
9. A footer's token MUST use `-` in place of whitespace characters, e.g., `Acked-by`. An exception is made for `BREAKING CHANGE`, which MAY also be used as a token.
10. Breaking changes MUST be indicated in the type/scope prefix of a commit, or as an entry in the footer.
11. If included as a footer, a breaking change MUST consist of the uppercase text `BREAKING CHANGE:`, followed by a space and description.
12. If included in the type/scope prefix, breaking changes MUST be indicated by a `!` immediately before the `:`. If `!` is used, `BREAKING CHANGE:` MAY be omitted from the footer section.
13. Types other than `feat` and `fix` MAY be used, e.g., `build`, `ci`, `docs`, `perf`, `refactor`, `chore` `test`:
  - build: Changes that affect the build system or external dependencies
  - ci: Changes to CI configuration files and scripts
  - docs: Documentation only changes
  - perf: A code change that improves performance
  - refactor: A code change that neither fixes a bug nor adds a feature
  - chore: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
  - test: Adding missing tests or correcting existing tests

## SemVer Correlation

- `fix` type commits translate to PATCH releases.
- `feat` type commits translate to MINOR releases.
- Commits with `BREAKING CHANGE` translate to MAJOR releases.

## Workflow

1. **Detect changes:**
   - Identify changed files in the working directory using `git status` or similar.

2. **Analyse changes:**
   - Identify logical groupings of changes by type.
   - Separate unrelated changes into distinct commit groups.
   - If a commit conforms to more than one type, split into multiple commits.

3. **Prepare commits:**
   - For each group, stage only the files belonging to that group.
   - Generate a commit message following the format above.

4. **Perform commits:**
   - Stage the files for each group.
   - Commit with the generated message.

## Constraints

- Project must pass linting and tests before staging and committing.
- Do not push changes to the remote repository.
- Do not combine unrelated changes into a single commit.
- Do not modify the content of the changes; only group, stage and commit.
- Do not use git hunks or partial staging; stage entire files only.
