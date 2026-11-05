# GRIDCRAFT-BAGOP-FALSIFIER-1: Cross-repository proof and admission program

**Status:** PROPOSED / planning-only. **Date:** 2026-11-05. **Owner:** GridCraft as integration harness; all semantic and kernel decisions remain with their owners. **Depends on:** P0–P3 and P5. **P4 (lance-graph-java) is a non-blocking historical/empirical precedent solely for folding without per-row materialization, not an implementation, runtime or semantic dependency.**

**Research question:** Can the exact same witnessed holographic Bag answer Excel XVERWEIS, a PSCustomObject-shaped named property and direct Bagop folding without materializing one row/cell/object per logical identity, without fabricating a second authority, and while preserving Excel semantics and OGAR scopes?

## Explicit non-goals

No production code, dependency update, merge, full GridCraft rewrite, new SIMD engine, new Bagop interpreter, custom Gather, independent identity dictionary, or claim that file decoding/serialization costs zero bytes. Do not promote a lab to a production admission.

## Fixture F0: independently specified

Create a *future implementation* fixture with exactly DR256 × 256 tiles (65,536 local positions), deliberately permuted local order and at least two distinct OGAR-granted tenant populations whose external labels collide. Each population includes duplicate lookup keys, unique keys, one not-found key, multiple typed columns, reversed and sorted presentations, and invalid/foreign source identifiers.

The independent oracle carries canonical source-qualified identities, Bag occurrence numbers, typed values and a separate **logical row order**. It does **not** share DR local positions, Bagop execution plans, a candidate index or the production codebook implementation.

Required values: zero, negative number, exact decimal text 9007199254740993, null/absent, blank, empty text, explicit missing, formula anchor and spill cached child, error state, Unicode text, and a changed value between versions. Do not pretend all these states are equivalent or always native to one codec.

## Paths under test

A. Direct source-qualified Bagop fold via admitted Quack/RelAlg/mask-risc/ndarray.

B. GridCraft XVERWEIS expression over mounted virtual BagSheet, using the same Bagop contract. Cover exact first and last; unadmitted wildcard/approximate/binary must give a typed refusal.

C. PSCustomObject-shaped named property projection over the same Bag, with no nested object tree in execution.

D. Independent scalar Excel-style oracle for exact matching and order, with distinct Bag multiplicity oracle. Optional real Excel/XLSX terminal roundtrip must be reported as external compatibility evidence, not as an internal oracle.

## Acceptance and disable matrix

| Gate | Positive oracle | Disable/falsifier |
|---|---|---|
| G0 semantic authority | Delete every derived index then rebuild: same IDs, multiplicities and results | Substitute workbook-minted ordinal: fail |
| G1 exact first/last | Excel-style independent source order matches | Remove order witness: red |
| G2 duplicates | Bag fold counts all occurrences, XVERWEIS chooses specified one | Implicit DISTINCT: red |
| G3 field binding | DT64 property name binds same meaning after input column reorder | Mutable basis without witness: red |
| G4 epistemic states | Blank, EmptyText, Absent, Missing, Formula/Spilled and error distinct as admitted | Conflate two states: red |
| G5 precision | Exact decimal string roundtrips or refuses lossless conversion | f64 conversion: red |
| G6 frame correctness | Permuted tile/frame refused or correctly rebound | Remove digest: red |
| G7 tenant isolation | Equal labels in two scopes do not alias | Bypass scope check: red |
| G8 value revision | Value-bearing cached method refuses stale revision; map COUNT valid | Remove value witness: red |
| G9 amortization | 256 repeated methods after one bind: zero additional global→local lookups | Re-route each method: counter red |
| G10 no row hydration | 65,536 Bag rows, zero persistent GridCraft Cell/PSCustomObject instances per Bag row | Force per-row Cell creation: allocation test red |
| G11 viewport | 30×12 window memory bounded by visible rows/cells and descriptors | Eager entire 65K×24 render: red |
| G12 dynamic arrays | Visible spill children have correct anchor, shape and order | Collapse spilled cached child: red |
| G13 admission law | Unadmitted Gather/composition/mask reuse refuses | Permit value-derived cross-table address: red |
| G14 backend parity | Scalar, AVX2, AVX512, NEON, WASM where runnable | Tail/stride corruption: red |
| G15 egress scope | Export only authorized values; no cross-scope cache | Reuse view after permission revocation: red |

A negative control must be run in a disposable test mutation and restored. A red disable proves the test can catch the violation; it is not evidence of production readiness.

## Performance experimental matrix

Record CPU model, ISA, Rust version, build flags, source revision, fixture seed, warmup, repetitions, distribution and resident/peak bytes. Separate:

1. XLSX parse/decompression and eTag/snapshot validation.
2. Canonical source bind and DR global→local preparation (map count).
3. One exact-match fold.
4. 256 warm repeated folds under the same prepared descriptor.
5. 30×12 viewport presentation and all-cell scrolling.
6. Cold text hydration and exact numeric conversion.
7. Terminal CSV/XLSX/PowerShell serialization and total bytes.
8. Cache invalidation/rebind after scope, frame or value change.

Compare to GridCraft CellStore and report-rs #33 **only where work and witness obligations match**. The ~0.39ns warmed lookup is not an end-to-end XVERWEIS comparator. Record p50/p95 and allocation deltas; do not cherry-pick backend/shape.

## PR/program gate order

| Phase | Prerequisite | Pass requirement |
|---|---|---|
| R0 | P0 owner decision | One-authority law with no hidden contradictory DR interpretation |
| R1 | P1, P2 owners | Exact-match ordered fold admitted or explicit STOP |
| R2 | P3 review | GridCraft virtual presentation and object-materialization bounds; P4 reference optional |
| R3 | P5 convergence | report-rs does not create a second source |
| R4 | DRISA M1/M2, OGAR | Frame, route and scope attestations |
| R5 | D4/S-INGRESS if writing requested | Canonical writer admission; otherwise read-only |
| R6 | This falsifier plan | Independent oracle and negative controls defined **before** code |

**Decision log 2026-11-05:** proposed test program, not proof. **Next implementation PR:** one read-only lab vertical slice with fixture and independent scalar oracle, not production. Stop if the required Bagop operator or layout/scope witness is not admitted.

## References / companion plans

- [P0/P2 source repository](https://github.com/AdaWorldAPI/lance-graph), [DRISA lab #1955](https://github.com/AdaWorldAPI/lance-graph/pull/1955)
- [P1 ndarray FACET STRIDE](https://github.com/AdaWorldAPI/ndarray/blob/master/src/simd_stride.rs)
- [P3 GridCraft CellStore](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/model/src/store.rs)
- [P4 non-blocking evidence for non-materializing native folds](https://github.com/AdaWorldAPI/lance-graph-java/blob/main/java/README.md)
- [P5 report-rs PR #33](https://github.com/AdaWorldAPI/report-rs/pull/33)
