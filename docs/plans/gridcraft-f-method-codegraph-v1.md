# GRIDCRAFT-METHOD-LENS-CONSUMER-1: Formula names bind to existing DT64 behavior, not a second codegraph

**Status:** DEFERRED CONSUMER STUDY / plan only. **Date:** 2026-11-05. **Owners:** DT64 and its codebook define immutable behavior and method identity; Quack/relalg/mask-RISC/ndarray own lawful execution. GridCraft owns formula parsing and Excel-specific presentation. Kythe/R2IL/Codegraph remain independent analysis models.

## 1. Fix the operation-pointer confusion

**XVERWEIS as an operation pointer is not Excel's return value, and not a stored or materialized result row.** The method reference names the *behavior* to invoke. Its admitted execution may produce a witnessed coordinate reference, a selection, a fold, or a terminal scalar. A later consumer dereferences a requested field or renders an Excel-compatible value.

```text
GridCraft parser / Excel spelling / other consumer call site
       → DT64 MethodRef (class, MethodId, SigRev, guard)
       → AlgebraRef behavior-lens word + operands, when bound
       → DR coordinate route / source-layout witness
       → admitted Quack/relalg/mask-RISC/ndarray operation
       → coordinate or result, depending on method contract
       → demanded property or Excel-visible value, if any
```

This is a **consumer adapter** for a more general SAP / PSCustomObject-shaped one-key ordinal SoA substrate. It never defines DR as "a Bag" or requires Excel formulas for the first GUID128 tile POC.

## 2. Reuse real DT64 contracts, not invented F IDs

The existing [DT64 implementation](https://github.com/AdaWorldAPI/lance-graph/blob/main/crates/lance-graph-dt64/src/lib.rs) already defines:

- `MethodId` independent of method name; `MethodRef` binds `class`, `method`, `sig_rev` and a guard witness. Renaming an Excel spelling does not mint a new operation identity.
- `AlgebraRef { lens, word }`: a polyfill word is a field **position** under a codebook's behavior lens. DT64 recognizes the `0xF###` right-half behavior-lens convention and uses `ClassView` to resolve its basis. **This does not mean that every Excel function has a deployed F### number.**
- `MethodBinding` with `on`, `reads`, ordered `effects`, `population`, `truth`, `demand`, `SigRev`. `WideFieldMask` selects field positions and footprints; it is not a business-value table.
- `BindStore::register` refuses altered behavior for an existing classid. To change meaning, use a derived class and versioned reroute. Historical binding interpretation is not historical data-value replay.

The [ClassView field basis](https://github.com/AdaWorldAPI/lance-graph/blob/main/crates/lance-graph-contract/src/class_view.rs) resolves field names *late*, without copying labels into the hot lanes. DR's [routing contract](https://github.com/AdaWorldAPI/lance-graph/blob/main/crates/lance-graph-dt64/src/dr.rs) maps coordinates to classes; it does not execute the method. Do not introduce a GridCraft-local canonical registry, a global per-formula F-number table, or a parallel codebook.

**Before assigning any Excel spelling to a method:** audit the owning `ClassView`/behavior-lens basis, `MethodId`, `SigRev`, operands, guard, result shape, and existing admission. If absent, request a new owner-approved method, not an invented ordinal.

## 3. What GridCraft can preserve and later adapt

Source facts: [Formula parser/AST](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/formula/src/lib.rs); [FnSpec name table](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/functions/src/lib.rs); [evaluator](https://github.com/AdaWorldAPI/gridcraft/blob/main/crates/calc/src/eval.rs) and its materializing `Host::range_values` route.

A future GridCraft binding can map **Excel syntax** to a verified DT64 method pointer. It must preserve source spans, reference kind, user-visible error/default, volatile and ordering semantics, and distinguish absent, empty, blank, formula cache and spill. None of these are excuses to mutate method identity or turn a `Vec<Value>` / `CellStore` cache into a canonical runtime.

| Formula family | Optional adapter concern | Shared law |
|---|---|---|
| XVERWEIS | Exact/first/last/approx/wildcard Excel match/return semantics and explicit ordering | Resolve a DT-bound method; coordinate result is not the method pointer |
| SUM/COUNT/MIN/MAX | Numeric/error/NULL compatibility | Admitted typed fold, not necessarily a new F### word |
| FILTER/GROUPBY/PIVOTBY | Excel shape, order and spill | Admitted selection/group fold with witness and multiplicity |
| IF/LET/LAMBDA | Lazy branches, evaluation limits, reference-valued closures | No universal "everything is one fold" claim |
| RAND/NOW/INDIRECT | Time/external effects and dynamic references | Admit effects and permissions explicitly, or refuse |

If no lawful polyfill exists, **do not silently route a mounted canonical tile through GridCraft's materializing interpreter**. Ordinary independent local Excel workbooks may still use their existing engine; this proposal does not authorize a global rewrite.

## 4. Kythe / R2IL / Codegraph non-dilution

- **Kythe and Codegraph:** existing code and symbol/provenance models remain authoritative only for what their own schemas can express. A formula→method→native symbol trace, if actually supported, is **derived analysis evidence**, never a second runtime or method-identity registry.
- **R2IL:** preserve machine/behavioral instruction identity and typed semantics; **do not alias Excel SUM/XVERWEIS into R2IL machine opcodes**. R2IL is not a mandatory execution hop for a spreadsheet or SAP tile.
- Existing [ruff R2IL plan](https://github.com/AdaWorldAPI/ruff/blob/main/.claude/plans/r2il-behavioral-ir-v1.md) says to preserve typed IR rather than stringify and rediscover it. The exact adapter for formula provenance is **not verified**; there is no justification to invent a parallel graph schema in GridCraft.

## 5. Deferred independent test gates

1. Same verified `MethodRef` plus schema and signature yields identical operation meaning across label rename; wrong basis/guard refuses.
2. Changed method effect order under same classid is refused; derived class/reroute changes behavior at a new version only.
3. One XVERWEIS-named operation invoked without Excel may produce a witnessed coordinate and support two property dereferences without row-object hydration; a separate Excel adapter tests its own return value.
4. Independently test duplicate keys, source order, exact decimal precision, missing/blank/error/spill, volatile and lazy control flow for **each supported Excel mode**.
5. Check no per-row `Vec<Value>`/PSCustomObject/Cell materialization during canonical tile execution, and count cold bind versus hot fold versus terminal output.
6. R2IL opcode vocabulary and Kythe/Codegraph schemas do not change to accommodate formula evaluation; only owner-approved read-only provenance traces are allowed.

**POC status:** **NOT REQUIRED** to prove AD↔Entra↔EXO GUID128 chained strides and SAP-style property folds. P0/P1 and current OGAR/dir-sim/SAP code own that proof; P2 is optional operation-pointer integration. This PR creates no new production behavior, IDs, formula backend or Codegraph dependency.
