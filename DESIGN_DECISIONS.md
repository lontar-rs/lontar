# Design Decisions

This document captures key architectural and design choices made during Lontar's development, along with the reasoning behind each decision. Future contributors should read this before proposing changes to core architecture.

---

## DD-001: Unified AST vs Per-Format Models

**Decision:** Single document AST consumed by all format backends.

**Alternatives considered:**
- Separate models per format (like how python-docx and python-pptx are completely independent)
- Shared base with format-specific extensions via composition

**Rationale:** The whole point of Lontar is "write once, emit many formats." If we allow per-format models, we become just a collection of unrelated libraries in a monorepo. The unified AST forces us to think about document semantics (what the content *means*) rather than format mechanics (how it's stored).

**Trade-off accepted:** Some format-specific features (slide transitions, tracked changes) fit awkwardly in a unified model. We handle this with the `Raw` block variant as an escape hatch and format-specific extension traits.

---

## DD-002: Enum-Based AST vs Trait Objects

**Decision:** Use enums (`Block`, `Inline`) for the AST, not `dyn Node` trait objects.

**Rationale:**
- Enums give exhaustive pattern matching — the compiler tells us if a backend doesn't handle a new node type
- No heap allocation per node (enums are stack-sized to their largest variant)
- Easier serialization with serde
- More idiomatic Rust

**Trade-off accepted:** Adding a new AST variant is a breaking change (all backends must update). This is acceptable because we *want* backends to consciously handle every node type, even if the handler is just "skip and warn."

---

## DD-003: Builder API as Primary Interface

**Decision:** `DocumentBuilder` is the primary way to construct documents, not direct AST manipulation.

**Rationale:**
- Builders enforce construction order and invariants
- Method chaining reads naturally: `doc.heading(1, "Title").paragraph("text")`
- We can validate during construction rather than at write time
- Direct AST manipulation remains possible for advanced users

---

## DD-004: Option<T> for Style Fields

**Decision:** All style fields are `Option<T>`, not `T` with defaults.

**Rationale:** Enables the cascade. `None` means "inherit from parent style." A concrete value means "override." This mirrors how CSS and OOXML styling actually work. If we used defaults (e.g., `bold: false`), we couldn't distinguish "explicitly not bold" from "inherit boldness from parent."

---

## DD-005: Feature Degradation over Failure

**Decision:** If a backend can't represent a feature, it degrades gracefully (with warnings) rather than returning an error.

**Rationale:**
- A Markdown file with a missing chart is more useful than no file at all
- Users can check `WriteReport` to see what was degraded/skipped
- Strict mode can be opted into via `WriterOptions` for CI pipelines that want to fail on degradation

**Degradation hierarchy:** full support → approximation (e.g., chart → table) → omission with warning

---

## DD-006: ZIP + quick-xml for OOXML

**Decision:** Use `zip` and `quick-xml` crates directly for OOXML output, rather than building on existing Rust Office libraries.

**Rationale:**
- Existing Rust OOXML libraries have different design philosophies and coverage priorities
- We need full control over XML output to match our AST model
- `zip` and `quick-xml` are mature, well-maintained, and minimal
- This approach is exactly what rust_xlsxwriter does successfully

---

## DD-007: No Document Reading (Initially)

**Decision:** Lontar is write-only for v0.x. No parsing of existing documents.

**Rationale:**
- Reading OOXML is significantly harder than writing it (must handle malformed input, legacy features, compatibility modes)
- It doubles the API surface and testing burden
- The primary use case (programmatic document generation) doesn't require reading
- Can be added in v1.x as a separate initiative

---

## DD-008: Dual MIT/Apache-2.0 License

**Decision:** Standard Rust ecosystem dual license.

**Rationale:** Maximum compatibility. MIT for permissive use. Apache-2.0 for patent protection. This is the de facto standard for Rust crates and removes licensing as a barrier to adoption.

---

## DD-009: Single Crate with Feature-Flagged Modules

**Decision:** Lontar is published as a single `lontar` crate on crates.io, with backends and optional subsystems gated behind Cargo feature flags. A minimal two-member workspace exists only because the CLI binary cannot live inside a library crate.

**Previous decision (reverted):** Cargo workspace of one crate per backend. Reverted at v0.1.0 before any real users existed.

**Alternatives considered:**
- Workspace of small crates, one per backend (original DD-009)
- Single flat crate with no feature flags (everything always compiled)

**Rationale for reversal:** The original rationale held that separate crates give users compile-time savings by not pulling in unused backends. In practice, Cargo feature flags achieve the same result within a single crate. The multi-crate approach added real costs that outweighed the theoretical benefits:

- **Publishing friction.** crates.io rate-limits publishes. A 12-crate workspace requires coordinated releases and version synchronization on every change. A single crate publishes once.
- **Dependency graph complexity.** Intra-workspace path dependencies need to become versioned crates.io dependencies on every release, with tight version coupling across all members.
- **User-facing surface area.** Users have to discover which of 12 crates they need and keep their versions in sync. One crate with documented feature flags is simpler.
- **Cross-cutting changes.** Modifying a core type (e.g., adding a `Block` variant) required touching `lontar-core` plus every backend crate in the same PR. With modules, it's one crate, one version bump.

The workspace-of-crates approach genuinely pays off when backends have separate maintainers, separate release cadences, or are designed to be used independently. Lontar's backends are tightly coupled to the core AST by design — they are not independent libraries. That coupling belongs in a single crate.

**Current structure:**
```toml
lontar = { version = "0.1", features = ["docx", "pdf"] }  # typical server use
lontar = { version = "0.1", features = ["full"] }          # everything
lontar = { version = "0.1" }                               # core AST only
```

**Feature flag topology:**

| Feature    | Deps enabled                              |
|------------|-------------------------------------------|
| `aksara`   | rustybuzz, unicode-bidi, unicode-linebreak, unicode-script |
| `diagram`  | petgraph                                  |
| `docx`     | aksara + zip + quick-xml                  |
| `pptx`     | aksara + zip + quick-xml                  |
| `pdf`      | aksara                                    |
| `xlsx`     | rust_xlsxwriter                           |
| `md`       | (none)                                    |
| `html`     | (none)                                    |
| `txt`      | (none)                                    |
| `template` | tera                                      |
| `full`     | all of the above                          |

**Trade-off accepted:** The `lontar` crate name is now a hard dependency for anyone who wants any part of Lontar. This is the correct trade-off — it matches how users think about the library and how the project presents itself.

**Yanked crates:** `lontar-core`, `lontar-aksara`, `lontar-cli`, `lontar-diagram`, `lontar-docx` v0.1.0 were yanked. No production users existed at the time of the reversal.

---

## DD-010: Slide as a Block Variant

**Decision:** `Block::Slide` is part of the core AST, not a pptx-specific concept.

**Rationale:** Slides are a semantic unit ("present this content as a visual page") that maps to other formats too — PDF pages, HTML sections, Markdown `---` separators. Making it a first-class AST node allows the DOCX backend to render slides as sections, the MD backend as horizontal rules, etc. A document *without* explicit Slide nodes is treated as a continuous flow; the PPTX backend auto-segments it.

---

## DD-011: Resource Store for Binary Assets

**Decision:** Images and other binary content are stored in a central `ResourceStore` and referenced by ID, not embedded inline in the AST.

**Rationale:**
- Same image can be referenced multiple times without duplication
- Binary data doesn't bloat the AST during manipulation
- Clean separation between document structure and binary content
- Each backend handles embedding differently (OOXML puts images in `media/`, PDF embeds inline, HTML uses data URIs or paths)

---

## DD-012: Universal Script Support via Text Shaping Pipeline

**Decision:** Integrate a full text shaping pipeline (`rustybuzz`, `unicode-bidi`, `unicode-linebreak`) as a foundational layer rather than handling scripts individually or deferring to rendering applications.

**Alternatives considered:**
- Defer all shaping to the consuming application (Word, LibreOffice, PDF viewer)
- Handle only Latin + a few additional scripts with custom code
- Integrate HarfBuzz via C FFI instead of rustybuzz

**Rationale:** Deferring to applications works for DOCX/PPTX but fails for PDF, where the generator must produce correctly positioned glyphs. Handling scripts individually doesn't scale — there are 159+ Unicode scripts, each with their own shaping rules. By integrating `rustybuzz` (a proven, pure-Rust port of HarfBuzz), we get correct shaping for all scripts at once. This is also central to Lontar's identity — a library named after Balinese palm leaf manuscripts should be able to render Aksara Bali correctly.

**Trade-off accepted:** `rustybuzz` and font handling add to binary size and compile time. This is mitigated by making `lontar-aksara` an optional dependency — backends that don't need shaping (MD, TXT, LaTeX) don't pull it in. The `lontar-aksara` crate is required for DOCX (font embedding), PPTX (font embedding), and PDF (glyph positioning).

---


---

## DD-013: MSRV 1.85 / Edition 2024

**Decision:** Set MSRV to 1.85 (Rust 2024 edition floor) for all crates. LontarOffice Desktop tracks stable channel.

**Alternatives considered:**
- Keep 1.75 / edition 2021 (original scaffold value)
- Pin to latest stable (1.93) as MSRV for library crates too
- Use nightly for async features

**Rationale:** The projects were scaffolded with 1.75 during the design phase but have zero published releases and zero external users. Edition 2024 provides async closures (needed for collaboration and sync layers), refined lifetime capture (cleaner trait returns), and MSRV-aware dependency resolution. 1.85 is the natural floor as the first release supporting edition 2024. For the Desktop binary, there is no reason to pin — tracking stable avoids maintenance overhead with no downside since no one depends on it as a library.

**Trade-off accepted:** Any user on Rust < 1.85 cannot compile Lontar. This excludes distros shipping very old Rust toolchains (e.g., Debian oldstable). Acceptable because our primary targets (Alpine/musl, direct source builds) always have current Rust available, and edition 2024 features materially improve the codebase.

---

## DD-014: Wrap rust_xlsxwriter Instead of Building Native XLSX

**Decision:** The `lontar-xlsx` crate is a thin integration wrapper around `rust_xlsxwriter`, not a native XLSX implementation.

**Alternatives considered:**
- Build native XLSX generation from OOXML SpreadsheetML spec
- Exclude XLSX entirely and tell users to use rust_xlsxwriter directly
- Fork rust_xlsxwriter and integrate it into the workspace

**Rationale:** `rust_xlsxwriter` is mature, actively maintained, and built by the same author who created the Python and C equivalents over a decade. Reimplementing it would duplicate years of work for no practical benefit. However, excluding XLSX breaks Lontar's "one document, every format" promise. A thin wrapper that maps `Block::Table` to worksheets and `Block::Chart` to chart objects gives users the convenience of the unified API for common cases while respecting the existing library's quality.

**Trade-off accepted:** The mapping from document AST to spreadsheet grid is inherently lossy — paragraphs, headings, and flow-based content don't map cleanly to cells. The wrapper handles common cases (tables, charts, images) and gracefully degrades the rest. Users with advanced spreadsheet needs should use `rust_xlsxwriter` directly.

---

## DD-015: Native Diagram Engine Instead of External Tool Dependency

**Decision:** Lontar includes a built-in diagram engine that defines, lays out, and renders diagrams using each format's native primitives.

**Alternatives considered:**
- Embed Mermaid/Graphviz output as rasterized images
- Generate Mermaid code only and rely on external renderers
- Exclude diagrams entirely and tell users to insert images

**Rationale:** Every technical document needs diagrams, yet no document generation library in any language renders diagrams natively across output formats. Embedding rasterized images produces non-editable, resolution-dependent output. Generating Mermaid code only works for Markdown viewers. By computing layout internally and rendering to each format's native primitives (DrawingML shapes in Office, SVG paths in HTML/PDF, TikZ in LaTeX, Mermaid in Markdown, ASCII in plain text), we produce output that is editable, scalable, and format-appropriate everywhere.

**Trade-off accepted:** Automatic graph layout is a complex problem. We start with layered (Sugiyama) layout for flowcharts and tree layout for hierarchies — these cover the majority of documentation diagrams. Force-directed layout for arbitrary graphs is Phase 2 of the diagram engine. Manual coordinate mode is the escape hatch for layouts the algorithms can't handle.

---

## DD-016: LaTeX as a First-Class Backend

**Decision:** LaTeX output (`lontar-latex`) is a mainline backend in the build pipeline, not a stretch goal or community add-on.

**Alternatives considered:**
- Keep LaTeX as a stretch goal after all other backends
- Generate LaTeX only via an intermediate format (e.g., AST → Markdown → pandoc → LaTeX)
- Support only PDF output and skip LaTeX source entirely

**Rationale:** Academic publishing — particularly in medicine, biology, and clinical research — runs on LaTeX. Journals require it. Grant agencies expect it. Collaborators need DOCX alongside it. The entire value proposition of Lontar ("write once, emit every format") breaks for a massive user base if LaTeX is an afterthought. Moreover, LaTeX is *technically simple* to emit (it's string concatenation like Markdown, with more structure) — the hard part is getting the preamble management and citation integration right, which we solve once and all backends benefit.

**Trade-off accepted:** Supporting LaTeX well requires the `Citation`/`Bibliography` AST additions (DD-017) and cross-reference system, which add complexity to `lontar-core`. This is acceptable because these features benefit *every* backend — DOCX and PDF also need citations and cross-references. LaTeX is the forcing function that makes us build the right abstractions.

---

## DD-017: Citations and Bibliography in Core AST

**Decision:** `Inline::Citation`, `Block::Bibliography`, and `BibliographyStore` are part of `lontar-core`, not a format-specific extension.

**Alternatives considered:**
- LaTeX-only citation support via `Raw` blocks
- Separate `lontar-bib` crate that post-processes the document
- Store citations as plain text and let each backend parse them

**Rationale:** Citations are a *semantic* document feature, not a format-specific one. A citation says "this claim is supported by this source" — that meaning is independent of whether the output is `\cite{key}`, a DOCX field code, an HTML hyperlink, or `[1]` in plain text. By modeling citations in the AST, every backend can render them correctly according to its format's conventions. The `BibliographyStore` follows the same pattern as `ResourceStore` — centralized data, referenced by ID, rendered differently per backend.

**Trade-off accepted:** The `BibEntry` struct must be expressive enough for academic references (which have many fields — DOI, volume, issue, pages, etc.) without becoming a full CSL-JSON reimplementation. We start with fields sufficient for common entry types (article, book, proceedings, thesis, report) and use `custom: HashMap<String, String>` as an escape hatch for rare fields.

---

## DD-018: Cross-Reference System via Labels

**Decision:** Blocks that can be referenced (tables, figures, equations, code listings, diagrams) carry an optional `label: Option<String>` field. `Inline::CrossRef` resolves against these labels. Headings use their existing `id: Option<String>` field.

**Alternatives considered:**
- Automatic numbering only (no user-defined labels)
- A separate cross-reference registry outside the AST
- Format-specific cross-referencing (LaTeX `\label`/`\ref`, DOCX bookmarks) with no unified model

**Rationale:** Labels are the universal mechanism. LaTeX has `\label{fig:results}`/`\ref{fig:results}`. DOCX has bookmarks. HTML has `id`/`href`. By putting labels in the AST, every backend maps to its native mechanism. The numbering (e.g., "Figure 3") is computed during write time by each backend — the AST stores the label, not the number.

**Trade-off accepted:** Label uniqueness must be validated at document build time, not at write time. This is a builder responsibility — `DocumentBuilder` should reject duplicate labels.

---

## DD-019: XeLaTeX/LuaLaTeX Only, No pdfLaTeX

**Decision:** The LaTeX backend targets XeLaTeX and LuaLaTeX exclusively. pdfLaTeX is not supported.

**Alternatives considered:**
- Target pdfLaTeX with `inputenc`/`fontenc` for basic Latin support
- Emit different preambles depending on engine selection
- Target all three engines with a compatibility layer

**Rationale:** Lontar's core value includes universal script support. pdfLaTeX's 8-bit encoding model fundamentally cannot handle Unicode — it requires font encoding packages, special input encodings, and breaks entirely on scripts like Balinese, Devanagari, or Arabic without extreme workarounds. XeLaTeX and LuaLaTeX use Unicode natively and access system fonts via `fontspec`. Since Lontar guarantees multi-script output, targeting pdfLaTeX would mean either (a) silently producing broken output for non-Latin text or (b) maintaining two completely different preamble generation strategies. Neither is acceptable.

**Trade-off accepted:** Users with legacy pdfLaTeX-only workflows cannot use Lontar's LaTeX output directly. This is mitigated by the fact that XeLaTeX/LuaLaTeX are the current standard for new LaTeX installations, and most journal submission systems accept XeLaTeX. For journals that truly require pdfLaTeX, Lontar's DOCX or PDF output is the better submission path.

---

## DD-020: BibLaTeX over BibTeX for LaTeX Citation Backend

**Decision:** The LaTeX backend emits BibLaTeX commands (`\parencite`, `\textcite`, `\addbibresource`) rather than legacy BibTeX (`\cite`, `\bibliography`).

**Alternatives considered:**
- Legacy BibTeX for maximum compatibility
- Configurable: user selects BibTeX or BibLaTeX
- Custom citation rendering (bypass both, emit raw text)

**Rationale:** BibLaTeX + Biber is the modern standard for citation management in LaTeX. It supports all citation modes in our `CitationMode` enum (parenthetical, narrative, year-only, suppress-author, full), offers proper Unicode support (critical for non-English references), and handles all bibliography styles through backend selection rather than `.bst` files. Legacy BibTeX cannot distinguish `\parencite` from `\textcite`, doesn't handle Unicode well, and has a more limited type system.

**Trade-off accepted:** Some older journals or submission systems may still expect legacy BibTeX. For these cases, users can either (a) add `extra_preamble` overrides to switch to BibTeX or (b) use Lontar's PDF output instead. A future `LatexOptions::citation_backend` setting could offer a BibTeX fallback mode if demand warrants it.

---

*Add new decisions using the format: DD-NNN: Title, Decision, Alternatives, Rationale, Trade-offs.*