# GRIDCRAFT-TILE-CONSUMER-FALSIFIER-1: Optional Excel parity after the core tile proof

**Status:** DEFERRED, documentation only. **Date:** 2026-11-05. **Owner:** GridCraft owns Excel consumer parity, not the DR routing, DT behavior or source-GUID identity laws.

## 1. Proof order

The first POC runs in lance-graph / OGAR / ndarray without GridCraft. It demonstrates a SAP- or PSCustomObject-shaped logical tile over normalized, ordinal-addressed SoA populations. DR resolves the coordinates, DT checks class and method parity, and an admitted Quack/mask-RISC fold performs the calculation.

Only afterward may a GridCraft adapter present the same tile as worksheet cells. This optional consumer test must not become a prerequisite for the foundation.

## 2. Core proof criteria, owned outside GridCraft

| Gate | Test |
|---|---|
| C1 Identity | Preserve the complete 128-bit Guid128 and explicit tenant/domain scope; no display-label identity shortcut |
| C2 Hybrid correspondence | Compare normalized AD backsync GUID to Entra id and EXO ExternalDirectoryObjectId; sourceAnchor remains an independent witness |
| C3 Ambiguity | Detect conflicting or duplicate associations; never silently choose a match |
| C4 Coordinate lifetime | On a row reorder, route change or data revision, rebind a local ordinal reference or refuse it |
| C5 DT parity | Verify ClassView field basis, WideFieldMask presence, method signature, guards, ordered effects and derived-class routing |
| C6 Execution | Compare an admitted SAP-style typed fold against a separately implemented scalar oracle |
| C7 Hydration | No per-row PSCustomObject, Excel Cell or joined DTO allocation in canonical execution |
| C8 Admission | Report unavailable foreign-lane joins and derived-mask reuse; never bypass the existing restrictions |
| C9 Measurement | Separate cold identity binding/index cost, warmed read, fold, replay checks and terminal presentation |

The foundation owns these tests. An existing cold sorted GUID index is a valid baseline. No claim that ndarray's byte-LUT helpers already implement the GUID join.

## 3. Optional GridCraft proof

A future virtual sheet may present a 30 by 12 viewport over the witnessed tile without building a copy of the entire row population. If Excel XVERWEIS is supported, independently verify match order, duplicates, not-found results, blank and empty text, exact decimals and array spills. These are Excel result semantics, not the meaning of the DT operation pointer.

The operation pointer identifies behavior; the execution can resolve a coordinate; a requested property read can return a value. Keep all three distinct. A coordinate reference must carry or be bound to scope, source snapshot and layout, rather than a naked snapshot-local ordinal.

The optional test also compares cold and warm costs, observed allocations, and output serialization. Unsupported formula modes must be explicitly reported, not silently executed through a row-materializing fallback.

## 4. Dependencies and non-dependencies

- [P0 foundation](https://github.com/AdaWorldAPI/lance-graph/pull/1960): DR coordinate routing, DT parity, and hybrid GUID tile architecture.
- [P1 ndarray](https://github.com/AdaWorldAPI/ndarray/pull/408): position-oriented execution mechanics.
- [P2 method pointer](https://github.com/AdaWorldAPI/lance-graph/pull/1961): optional coordinate-returning operation contract.
- [P3 GridCraft](https://github.com/AdaWorldAPI/gridcraft/pull/1): optional virtual worksheet consumer.
- [P7 F###](https://github.com/AdaWorldAPI/gridcraft/pull/3): optional formula-to-DT behavior-lens adapter.
- [P5 report-rs](https://github.com/AdaWorldAPI/report-rs/pull/34): separate report consumer.
- [P4 Java precedent](https://github.com/AdaWorldAPI/lance-graph-java/pull/93): non-blocking evidence for folding without host-object construction.

No GridCraft workbook import, Java runtime, DataFusion, or Codegraph adapter is required for the core POC. No runtime code, test implementation, merge or production admission is included in this PR.
