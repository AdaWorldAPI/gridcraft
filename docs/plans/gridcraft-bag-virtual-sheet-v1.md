# GRIDCRAFT-BAGSHEET-1: Virtual worksheet over the holographic Bag

**Status:** PROPOSED / plan-only. **Date:** 2026-11-05. **Owner:** GridCraft. **Depends on:** lance-graph P0/P2 and ndarray P1.

**Intent:** Reuse GridCraft as the familiar spreadsheet façade, but mount an authorized holographic Bag so cells, XVERWEIS and PSCustomObject-style properties are semantic projections. DR64 exists only for Bag representation. GridCraft must not create a second canonical workbook row/value authority.

## Existing GridCraft seam

- [CellStore](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/model/src/store.rs) holds 64-row copy-on-write bands with nested BTreeMap row/column cells. Good for normal standalone workbooks, inappropriate as an authoritative mirrored Bag.
- [Sheet](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/model/src/sheet.rs) owns CellStore, spill map, spill ranges, styles/metadata and a transient SheetUid.
- [Cell](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/model/src/cell.rs) owns value/formula/style payloads; [Calc](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/calc/src/recalc.rs) owns an independently derived dependency graph.
- [XLSX](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/xlsx/src/lib.rs) currently reads into a Workbook and emits from one. The entire model need not be replaced in one PR; an alternate **mounted BagSheet** backing is proposed.
- GridCraft currently has a virtualized *renderer*, which does not by itself prove a virtualized semantic cell store.

## Proposed boundary

    UI cell (sheet, row, column)
      → virtual sheet descriptor (ordering, visible window, DT64 property binding)
      → authorized witnessed Bagop selection
      → ndarray/mask-risc under exclusive holographic DR64 Bag
      → value or terminal viewport cell only on demand

Presentation coordinates are not global identity and must not be stored as a second identity registry. View holds a source receipt, scope, frame/layout digest, value revision, field/method bindings and view order. Revalidation at each terminal operation prevents stale data or revocation leaks. The Bag's owner performs canonical writes; if D4/S-INGRESS is blocked, mounted cells are read-only and editing refuses explicitly.

**No GridCraft-owned Bag rows, no separate CustomerStore, no per-population Cell allocations, no app-specific SIMD or Gather.** Ordinary unmounted local workbooks may keep their current CellStore; the new backing's data source must be authoritative only in Bag.

## Obligations by feature

| Feature | BagSheet interpretation | Refusal / open gate |
|---|---|---|
| Viewport | Resolve only visible cells from authorized selection | Scope/frame mismatch invalidates cached window |
| XVERWEIS | P2 ordered Bagop fold | Unsupported mode refuses, not HashMap fallback |
| FILTER | Admitted selection mask | Derived mask reuse requires witness/admission |
| SORT | Order descriptor over addressed occurrences | Never assume tile order equals sheet order |
| Spilled arrays | Anchor, shape, value projection; hydrate visible children only | Exact spill behavior requires independent oracle |
| Pivot | Grouped Bag aggregation/fold | Bag multiplicity and ordering preserved |
| Formulas | Syntax AST + dependency descriptors | No second full result table for Bag-backed cells |
| Styles | Sparse local presentation metadata | No authority over Bag value or identity |
| Undo/edit | Versioned write intent and read-after-write witness | D4/S-INGRESS writer unresolved, refuse writes |
| XLSX ingress | Parse bytes, check cached formulas/spills, lower by canonical owner | No rehydrated canonical workbook duplicate |
| XLSX/CSV egress | Stream authorized terminal view where possible | Serialization necessarily materializes bytes |
| PSCustomObject | Named Bag property projection | Physical object only at explicit external membrane |

Excel compatibility is a semantic gate: Absent, Blank, EmptyText, Missing, Formula, Spilled, error, date/text, and exact decimals must not silently collapse. A cached formula is not proof of live recalculation.

## Smallest executable successor (not this PR)

Mount one synthetic DR256×256 Bag (65,536 identities, 24 fields) into a virtual sheet. Show a 30×12 viewport, one exact XVERWEIS, one spilled FILTER and one property projection. Compare against independent scalar Excel oracle; require no persistent GridCraft Cell per Bag row, zero lookup rerouting after prepared binding and bounded allocations by visible working set. Test colliding labels across two tenant scopes and scope revocation. Do not declare zero-byte output, zero file decoding or zero cost; separate all boundaries.

**Disable runs:** remove frame/layout witness, scope witness, value revision, order witness, and a spill-state check, each must fail its targeted test. Benchmark cold binding, warm lookup, viewport and serialization separately.

## Decisions and next PR

| Requirement | Owner | Gate |
|---|---|---|
| One Bag substrate | P0 lance-graph | Owner reconciliation |
| XVERWEIS ordering/fold | P2 lance-graph | Admitted operator or typed refusal |
| FACET STRIDE | P1 ndarray | Backend parity and positional access |
| Cell provider seam | GridCraft | Can preserve standalone mode? |
| Scope | OGAR | Authorized tenant-qualified projection |
| Write authority | D4/S-INGRESS | Read-only until approved |
| Cross-repo oracle | P6 GridCraft | Falsifiers before implementation |

**Decision log 2026-11-05:** propose an additive Bag-backed virtual mode first, not an all-at-once replacement of GridCraft. **Implementation successor:** only after P0/P1/P2 approval, isolated provider trait + one read-only viewport, no production edits here.
