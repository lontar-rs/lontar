# Contributing to Lontar

## Development Setup

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- Python 3.10+ (for reference document generation in Phase 0)
- LibreOffice (for visual conformance testing)

### Getting Started

```bash
git clone https://github.com/YOUR_ORG/lontar.git
cd lontar
cargo build
cargo test
```

### Project Structure

```
lontar/
├── Cargo.toml              # Workspace definition
├── crates/
│   ├── lontar-core/        # Document AST, styles, traits
│   ├── lontar-aksara/      # Text shaping, BiDi, line breaking, fonts
│   ├── lontar-docx/        # DOCX backend
│   ├── lontar-pptx/        # PPTX backend
│   ├── lontar-pdf/         # PDF backend
│   ├── lontar-md/          # Markdown backend
│   ├── lontar-html/        # HTML backend
│   ├── lontar-txt/         # Plain text backend
│   ├── lontar-template/    # Template engine
│   └── lontar-cli/         # CLI tool
├── lontar/                 # Umbrella crate (re-exports)
├── tests/
│   ├── fixtures/           # Test documents and expected outputs
│   ├── integration/        # Cross-crate tests
│   └── conformance/        # Office compatibility tests
├── tools/
│   └── gen_reference/      # Python scripts for reference doc generation
├── docs/                   # User guide (mdbook)
├── examples/               # Usage examples
├── README.md
├── ARCHITECTURE.md
├── TODO.md
├── CONTRIBUTING.md
└── CHANGELOG.md
```

## Development Workflow

### Branching

- `main` — stable, all tests pass
- `dev` — active development, may be unstable
- `feature/*` — feature branches off `dev`
- `fix/*` — bug fix branches

### Making Changes

1. Check the [TODO.md](./TODO.md) for current priorities
2. Create a feature branch: `git checkout -b feature/docx-tables`
3. Write tests first (TDD encouraged)
4. Implement the feature
5. Run the full check suite: `make check` (or commands below)
6. Submit a PR against `dev`

### Code Quality

Before submitting:

```bash
# Format
cargo fmt --all

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Test
cargo test --all

# Check for unused dependencies
cargo machete

# Benchmarks (if touching hot paths)
cargo bench
```

### Testing Philosophy

1. **Unit tests** live alongside the code in each crate
2. **Snapshot tests** compare generated output against known-good fixtures
3. **Integration tests** verify cross-crate behavior
4. **Conformance tests** verify documents open correctly in Office applications
5. **Reference comparison** — when in doubt, generate the same doc with python-docx/pptx and compare XML

### Adding a New AST Node

1. Add the variant to `Block` or `Inline` in `lontar-core`
2. Update `DocumentBuilder` with a convenience method
3. Add handling in **every** backend (even if it's just a skip-with-warning)
4. Add a test fixture
5. Update the mapping tables in `ARCHITECTURE.md`

### Adding a New Backend

1. Create a new crate: `crates/lontar-{format}/`
2. Implement `DocumentWriter` trait
3. Handle all `Block` and `Inline` variants (skip unsupported ones with `WriteReport` warnings)
4. Add snapshot tests
5. Add feature flag in the umbrella `lontar` crate
6. Update `README.md` and `ARCHITECTURE.md`

## Code Style

- Follow standard Rust conventions (rustfmt defaults)
- Document all public items with `///` doc comments
- Use `thiserror` for error types
- Prefer `&str` over `String` in function parameters where possible
- No `unwrap()` in library code — use proper error handling
- `unsafe` requires a `// SAFETY:` comment and team review
- Keep dependencies minimal — every new dep needs justification

## Commit Messages

```
type(scope): brief description

Longer explanation if needed.

Types: feat, fix, docs, test, refactor, perf, chore
Scopes: core, docx, pptx, pdf, md, html, txt, template, cli, ci
```

Examples:
```
feat(docx): implement table generation with merged cells
fix(core): correct style cascade for nested paragraphs
test(pptx): add reference comparison for chart slides
docs(readme): update roadmap status
```

## License

By contributing, you agree that your contributions will be licensed under the MIT OR Apache-2.0 dual license.
