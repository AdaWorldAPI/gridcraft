# GRIDCRAFT-F-METHOD-CODEGRAPH-1: Formula F### Bagop polyfills without Kythe/R2IL/Codegraph dilution

## Scope reset: formula polyfills deferred (2026-11-05)

F### formula polyfill/Codegraph hygiene is a **future consumer problem**, not a dependency or first milestone of the SAP-shaped GUID-tile POC. No Excel formula registry, Kythe/R2IL integration or Codegraph adapter must be built for that POC. The core work is the existing dir-sim/OGAR 128-bit scoped GUID correspondence + lance-graph-sap typed lane fold + ndarray positional discipline, returning a logical PSCustomObject-shaped tile projection without per-row object allocation. Keep this plan parked until a formula consumer requires its admission. Existing R2IL semantics and code-analysis vocabularies remain unchanged.

**Status:** PROPOSED / architecture only. **Date:** 2026-11-05. **Owner:** GridCraft for Excel formula grammar and presentation. **Method identity, Bagop semantics and admissibility:** existing canonical owning registries in lance-graph / OGAR, to be established by source audit. **Evidence/provenance owners:** existing Kythe, R2IL, and Codegraph components, each under its own contract.

**Depends on:** [P0 lance-graph #1960](https://github.com/AdaWorldAPI/lance-graph/pull/1960), [P1 ndarray #408](https://github.com/AdaWorldAPI/ndarray/pull/408), [P2 Bagop #1961](https://github.com/AdaWorldAPI/lance-graph/pull/1961), [P3 virtual GridCraft #1](https://github.com/AdaWorldAPI/gridcraft/pull/1). **Downstream:** P5 report-rs and P6 falsifier.

## 0. Operator ruling / scope

**Every GridCraft formula with executable meaning must be resolved to a canonical, typed F### method polyfill, or be explicitly classified as NOT ADMITTED.** F### denotes the user's method-vocabulary convention. **Do not assume a particular F### numbering, width, global registry or deployed implementation exists merely from the convention.** Source-audit the owning registry first; never invent local F### ordinals in GridCraft.

**DR64 only holographically represents the Bag; Bagop interprets it; ndarray supplies positional execution.** GridCraft retains Excel syntax, names, formatting and editing, not a second semantics-bearing workbook interpreter. An F### method is a *typed, witnessed semantic binding*, not a new per-cell executable heap object or a new general DR64 instruction set.

**Kythe / R2IL / Codegraph must not be diluted.** Reuse their established source/behavioral facts as evidence or graph projections. Do not rebrand Excel formula AST nodes as machine-level R2IL opcodes; do not create a second Kythe graph schema, a second behavioral IR, or a GridCraft-local Codegraph truth registry. Formula-specific meaning belongs to the F### / Bagop method contract, not a fork of code-analysis semantics.

## 1. Current GridCraft facts (source-verified)

- [crates/formula/src/lib.rs](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/formula/src/lib.rs) already separates Excel lexer/parser/AST/printer and reference adjustment. Retain this syntax layer.
- [crates/functions/src/lib.rs](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/functions/src/lib.rs) describes functions with FnSpec and currently evaluates already-evaluated arguments via function pointers; range arguments may arrive as Value::Array. The lookup-by-name table is **not** an authoritative F### registry.
- [crates/calc/src/eval.rs](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/calc/src/eval.rs) has Ev::V(Value), Ev::R(Vec<Area>) and Ev::L(Arc<Lambda>); Host::range_values defaults to collecting a Vec<Value>, with a visit_range sparse route. These are concrete materialization hotspots. Simply changing CellStore physical layout does not eliminate them.
- [crates/calc/src/recalc.rs](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/calc/src/recalc.rs) maintains a derived dependency graph and recalculation lifecycle. Its graph cannot become a second canonical Bag state.
- [crates/model/src/sheet.rs](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/model/src/sheet.rs) keeps spill cells and ranges. Spills should resolve as witnessed anchor + shape + terminal presentation when an F### method permits it.

**Next source audit before code:** identify existing F### method registry and numbering, OGAR vocabulary/alias rules, Codegraph/Kythe source-symbol and edge schemas, and the **exact** R2IL behavioral contract in its owners. The current source inspection has *not* verified a canonical F### registry, Kythe adapter for GridCraft or a direct R2IL formula mapping. Treat these as OPEN, never as implemented facts.

## 2. Required formula lowering (normative target, not present implementation)

    Excel source string and workbook names
      → GridCraft syntax AST (preserve location, ref kind, formula text)
      → typed formula semantic binding (source spans, field refs, argument kinds)
      → canonical MethodRef / F### resolution under an owning versioned registry
      → Bagop method application over witnessed Bag population
      → **only admitted** Quack / RelAlg / mask-risc operators
      → ndarray::simd position/mask/fold execution
      → demanded scalar, lazy spill window or terminal serialization

**F### is a semantic method handle**, not an Excel function-name hash, synthetic customer key, local ordinal dictionary, or direct SIMD opcode. If a method already exists in the canonical vocabulary, alias Excel syntax to that existing meaning; if no equivalent exists, request an *owner-level* method proposal and show independent Excel-compatible semantics. No ad hoc F-number assignment.

A formula can compose F### calls as a method DAG of references. The DAG is a *derived* witness/provenance view; evaluation must not expand referenced ranges to row objects or Vec<Value> merely to pass between calls. **No cross-program derived-mask reuse, Gather or cross-table lane-derived address without existing admission**; a plausible-looking formula does not confer new execution rights.

## 3. Separate the four graphs (no dilution)

| View | Owns | Forbidden cross-contamination |
|---|---|---|
| GridCraft formula AST | Surface syntax, relative/absolute references, source spans, operator spelling, Excel compatibility | Must not own canonical Bag identity or mask execution |
| F### / Bagop method DAG | Typed method meaning, argument shape, call edges, witness requirements, admitted lowering | Must not redefine Kythe, R2IL or Java methods |
| Kythe / Codegraph projection | Source facts, symbol/definition/use/call provenance and impact analysis **to the extent their actual source contracts support them** | Not a second formula evaluator; graph is derived/read-only, not a source of canonical data values |
| R2IL | Machine/behavioral semantics and their owner-declared instruction vocabulary, when legitimately relevant to an implementation proof | Do not invent Excel-to-R2IL instruction aliases, overwrite R2IL opcode meanings, or preconvert live R2IL into lossy static shadow for formula execution |

Preserve exact source identity and lineage: original formula location → parsed AST span → method identity/version → admitted operator/plan identity → native symbol/impl → result witness. A Codegraph/Kythe export can *describe* this chain without owning its execution or inventing a parallel canonical record population. If the existing codegraph fork has no such interface, record a missing adapter; do not manufacture a competing schema under GridCraft.

See [ruff R2IL plan](https://github.com/AdaWorldAPI/ruff/blob/main/.claude/plans/r2il-behavioral-ir-v1.md), which demands preserving typed IR rather than stringify-and-reparse; [OGAR ogar-r2il](https://github.com/AdaWorldAPI/OGAR/blob/main/crates/ogar-r2il/src/lib.rs), whose machine vocabulary is a distinct addressable reading, not a generic spreadsheet opcode set; and [Codegraph](https://github.com/AdaWorldAPI/codegraph), whose current repository implementation must be separately audited for applicable graph ingestion.

## 4. Formula families and explicit stop cases

| Excel syntax family | Required F### / Bagop shape | Admission policy |
|---|---|---|
| SUM, COUNT, MIN/MAX, numeric aggregates | Typed masked fold over admitted Bag population with correct Excel numeric/error semantics | Cite existing kernel and owner law |
| XVERWEIS | Ordered first/last/approximate/wildcard modes as *separately admitted* Bagop fold methods | First/last order witness mandatory; other modes refuse until proved |
| FILTER / GROUPBY / PIVOTBY | Bag selection, grouped fold and lazily addressed result | Multiplicity, shape, ordering and legal mask reuse |
| Arithmetic/comparison | Typed per-position method, state handling, no lossy decimal conversion | No unsafe u8/i8 mix; scalar value is not always Bag fold |
| INDEX / OFFSET / INDIRECT / structured references | Reference-valued lens and dynamic binding, not full range hydration | Reject unadmitted cross-table Gather, volatile dependencies |
| IF / IFERROR / LET / LAMBDA / MAP / REDUCE | Control-flow, closure and lazy method composition with explicit evaluation/fuel semantics | Never evaluate forbidden branches just to build a mask; no Java/PowerShell object interpreter |
| RAND / NOW / external/data-linked functions | Source/time/effect/volatility witness | May be not expressible; typed refusal and explicit effect authority |
| Unsupported function | No invented implicit function-pointer fallback | Keep imported formula text intact and show unimplemented/status appropriately |

This plan does not assert every Excel operation is mathematically a fold. Every executable formula needs a typed method/polyfill contract; some methods are elementwise, control-flow, effect-bearing or reference-valued. **The canonical path does not silently route these back through GridCraft's old materializing evaluator.**

## 5. Falsifier and cost gates

**F0 formula corpus:** exact syntax and source spans for ordinary references, 2-row header mapping, named ranges, XVERWEIS duplicates, 65,536 identities, 30×12 viewport, blanks/missing/empty text, exact decimals, a volatile expression, a conditional with a forbidden branch, and recursive LAMBDA fuel. Two differently scoped populations share external labels. Independent Excel-style scalar oracle is separate from runtime F### map.

1. **No numbering split:** resolving the same formula under same registry yields identical MethodRef; unknown F### registry refuses. Disable the version/namespace check and trigger a wrong binding.
2. **No value hydration:** 65,536 logical rows with SUM and XVERWEIS do not create a row object or Vec<Value> per element in canonical execution. Count allocations and method crossings. Disable lazy range routing; catch population-sized hydration.
3. **No codegraph dilution:** codegraph view contains only source/provenance mappings allowed by its real contract; fingerprinting the formula AST and F### plan does not mint a new method identity. A changed source span updates provenance without changing Bag data.
4. **No R2IL alias:** verify machine-domain R2IL vocabulary is unchanged and Excel SUM/XVERWEIS are not reinterpreted as R2IL opcodes. Contradicting instructions fail vocabulary tests.
5. **Semantics:** compare error, array-lift, reference, closure, volatile and first/last ordering cases against independent oracle; unsupported cases refuse loudly rather than silently fall back to legacy execution.
6. **Witnesses:** foreign frame, edited value, stale method registry, revoked OGAR scope all refuse; disable each witness individually and demonstrate red checks.
7. **Cost:** report parse/bind, method DAG build, cold prepared Bag binding, warm method invocation, viewport resolution, spill access, provenance extraction and export independently. Do not count codegraph graph construction as kernel execution.

## 6. Ownership, decisions and implementation sequence

| Open decision | Owner | Rule |
|---|---|---|
| Is F### a deployed canonical ID namespace or a requested naming pattern? | Canonical vocabulary owner (lance-graph/OGAR), verify source | Do not mint until confirmed |
| How can a formula bind to a versioned MethodRef? | Bagop + DT64, P2 | No independent function-name-to-ordinal authority |
| Which formula families have admitted execution? | Quack/RelAlg/mask-risc + ndarray | No unadmitted Gather, composition or mask reuse |
| How are source/behavior graphs exported? | Existing Kythe/R2IL/Codegraph owners | Read-only projection/adapter; no forked graph semantics |
| How does GridCraft retire Value::Array hydration? | GridCraft P3 | One long-term Bagop core, compatibility only at boundary |
| How are time/external/volatile effects authorized? | Method owner / scope owner | No silent host-local effects in canonical Bag execution |

**Decision log 2026-11-05:** formula F### polyfills and Kythe/R2IL/Codegraph non-dilution are new explicit operator constraints. Existing F### numeric assignments and a ready-made formula→R2IL/Codegraph adapter were **not verified** and must not be asserted.

**Next implementation PR (BLOCKED):** one selected, verified existing F### (or owner-approved method) for a non-materializing SUM/COUNT with an independent oracle, preserving a formula AST/Codegraph provenance link. Expand to XVERWEIS only after P2 ordered selection admission. **No code changes in this PR.**
