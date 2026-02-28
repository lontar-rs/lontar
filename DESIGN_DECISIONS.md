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

## DD-009: Workspace of Small Crates

**Decision:** Cargo workspace with one crate per backend, not a single crate with feature flags only.

**Rationale:**
- Users only compile what they use
- Each backend can have its own dependencies without bloating others
- Parallel compilation of independent crates
- Clear ownership boundaries for contributors
- The umbrella `lontar` crate re-exports everything for convenience

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

*Add new decisions using the format: DD-NNN: Title, Decision, Alternatives, Rationale, Trade-offs.*

---

## DD-012: Universal Script Support via Text Shaping Pipeline

**Decision:** Integrate a full text shaping pipeline (`rustybuzz`, `unicode-bidi`, `unicode-linebreak`) as a foundational layer rather than handling scripts individually or deferring to rendering applications.

**Alternatives considered:**
- Defer all shaping to the consuming application (Word, LibreOffice, PDF viewer)
- Handle only Latin + a few additional scripts with custom code
- Integrate HarfBuzz via C FFI instead of rustybuzz

**Rationale:** Deferring to applications works for DOCX/PPTX but fails for PDF, where the generator must produce correctly positioned glyphs. Handling scripts individually doesn't scale — there are 159+ Unicode scripts, each with their own shaping rules. By integrating `rustybuzz` (a proven, pure-Rust port of HarfBuzz), we get correct shaping for all scripts at once. This is also central to Lontar's identity — a library named after Balinese palm leaf manuscripts should be able to render Aksara Bali correctly.

**Trade-off accepted:** `rustybuzz` and font handling add to binary size and compile time. This is mitigated by making `lontar-aksara` an optional dependency — backends that don't need shaping (MD, TXT) don't pull it in. The `lontar-aksara` crate is required for DOCX (font embedding), PPTX (font embedding), and PDF (glyph positioning).

---

## DD-013: Separate lontar-aksara Crate

**Decision:** Text shaping lives in its own crate (`lontar-aksara`) rather than inside `lontar-core`.

**Rationale:** Not all backends need text shaping. Markdown and plain text output work with raw Unicode strings — they don't need `rustybuzz` or font management. Keeping `lontar-core` dependency-free means simple use cases stay lightweight. Backends that generate binary formats (DOCX, PPTX, PDF) depend on `lontar-aksara`; text-based backends depend only on `lontar-core`.
