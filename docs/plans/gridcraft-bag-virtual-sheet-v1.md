# GRIDCRAFT-VIRTUAL-TILE-CONSUMER-1: Spreadsheet presentation over routed SoA coordinates

**Status:** DEFERRED CONSUMER STUDY / draft plan only. **Date:** 2026-11-05. **Owner:** GridCraft for formula syntax, worksheet presentation and XLSX compatibility. No role in canonical identity, DT behavior, DR routing or substrate execution.

## Scope, deliberately narrow

**The first POC is not in GridCraft.** It is a SAP / PSCustomObject-shaped one-key ordinal-addressed logical tile over AD/Entra/EXO hybrid correspondence and a typed CATS-style fold in lance-graph/OGAR/ndarray. DR teleports the computation through coordinates. DT enforces class/behavior parity. Quack lowers an admitted operation and mask-RISC/ndarray execute it. GridCraft is an **optional human-facing view** of the same already-bound coordinates.

A spreadsheet cell is a **presentation address**, not an identity. GridCraft must not mint an alternate `Guid128` mapping, workbook codebook authority, DR coordinate registry or a second DT behavior store. It must not make a local workbook engine a prerequisite for an SAP-shaped tile.

## Actual source baseline and migration seam

- [`crates/formula/src/lib.rs`](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/formula/src/lib.rs) owns Excel syntax AST/parser/printer and reference adjustments.
- [`crates/model/src/store.rs`](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/model/src/store.rs) has the existing `CellStore`; [`sheet.rs`](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/model/src/sheet.rs) owns workbook cells, style and spill metadata.
- [`crates/functions/src/lib.rs`](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/functions/src/lib.rs) dispatches already-evaluated function arguments. [`crates/calc/src/eval.rs`](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/calc/src/eval.rs) has a `Host::range_values` route that materializes `Vec<Value>`. The existing Excel engine is *not* automatically zero-hydration merely because the visible renderer is virtualized.

**Keep these baseline features intact for local Excel files.** If a borrowed SoA tile is exposed as a virtual sheet, it is a distinct read-only *provider boundary*, never a copy of all canonical fields into `CellStore`.

## Optional future provider contract

```text
GridCraft cell (sheet, display row, field column)
   → presentation order + requested viewport
   → source-qualified CoordRef { population, row ordinal, scope, frame/layout, snapshot }
   → DT MethodRef + ClassView field ordinal / WideFieldMask
   → admitted Quack/mask-RISC or lawful positional property read
   → demanded scalar / viewport cell / XLSX egress only
```

The `CoordRef` is a conceptual reference, **not** a shipped Rust ABI. A local ordinal is invalid after an unseen permutation. Binding must either revalidate or refuse when layout, source version, scope, class basis or method signature changes. Missing/error/blank/empty text have separate Excel consumer semantics and must not be inferred from DR masks. A preview is not an authorization source.

**XVERWEIS distinction:** Excel has its own return semantics; a method pointer in DT can resolve a coordinate, while Excel may request its return property. Do not encode Excel's first/last/spill rules into the shared operation pointer. See [P2](https://github.com/AdaWorldAPI/lance-graph/pull/1961).

## Optional integration acceptance (not POC prerequisite)

When and only when the core tile proof exists:

1. One mounted synthetic tile with source-qualified GUID binding, field ordinals and a 30 × 12 viewport; no one-`Cell`-per-source-row store.
2. Change the tile's row ordering; the stale viewport reference refuses or rebinds and agrees with an independent scalar source oracle.
3. Present the same two fields via virtual sheet and logical PSCustomObject-shaped view; provenance and meaning agree.
4. If adding Excel XVERWEIS, independently verify its **consumer** order, duplicate match, error, blank and spill semantics. Unsupported modes refuse. No implicit HashMap/row-cache fallback to claim a Bagop speedup.
5. Separate parser, cold bind, warm property access and XLSX egress cost. Source text and serialization necessarily materialize bytes.

**No approved work:** no ground-up GridCraft rewrite, forced universal F### formula migration, second canonical workbook engine, new SIMD kernel, new Gather or source writer. Preserve the old Excel engine for ordinary workbooks, but never treat it as authoritative over a mounted canonical tile.

**Dependencies:** [P0 routing/parity](https://github.com/AdaWorldAPI/lance-graph/pull/1960), [P1 positional mechanics](https://github.com/AdaWorldAPI/ndarray/pull/408); P2 only for a requested lookup method. P4 Java is non-blocking reference only. P7 formula integration is a separate optional research proposal. This document does not block the core SAP/Guid128 POC.
