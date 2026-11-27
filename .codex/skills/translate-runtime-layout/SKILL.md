---
name: translate-runtime-layout
description: >
  Handle SE/AE/VR-divergent struct layouts by splitting common prefixes from
  runtime-specific tails and expressing access through runtime-data accessor
  macros instead of fake universal layouts.
---

Use this skill when a CommonLib type has runtime-dependent field offsets, sizes,
or helper accessors.

## Use Cases

- `RuntimeDataAccessors.h` style patterns
- structs whose `sizeof` differs across SE / AE / VR
- fields that exist only on some runtimes
- plugin code that currently uses manual `if is_vr()` memory arithmetic

Do not use this skill just because the header has `ENABLE_SKYRIM_VR`. Some
CommonLib cross-VR branches only change method implementation and belong in the
normal translation flow with runtime-aware wrapper methods instead of split
layout work.

## Helper Scripts

- Run `python scripts/find_runtime_layout_candidates.py --query <TypeName>` to
  confirm that the type really belongs in this workflow.
- Run `python scripts/audit_translation.py <TypeName>` if an existing Rust file
  already exists and you want a fast local snapshot before restructuring it.

## Workflow

1. Read the C++ header and `.cpp`.
2. Record every runtime-dependent offset and size.
3. Identify the maximal common prefix that is honest across all runtimes.
4. Keep only that common prefix in the main Rust `repr(C)` type.
5. Move divergent tails behind runtime-data accessor macros from
   `libskyrim/src/runtime.rs`.
6. Add runtime-aware layout asserts where appropriate.
7. Verify that no fake single-layout tail remains in the struct body.

## Rules

- Do not encode multiple runtime sizes into one Rust type.
- Do not use `[u8; N]` blobs to paper over runtime divergence.
- If the translated RE layout relies on source-backed helper types from
  `CommonLibVR/include/REX/**`, place those helper translations in
  `libskyrim/src/rex/*.rs` and import them from `crate::rex`.
- If the translated file needs to read fields from another named CommonLib RE
  type whose Rust file is missing, do not introduce a consumer-local `*View`
  stand-in for that external type. Create or extend the matching dependency
  Rust file and place the minimal source-backed partial translation there,
  keeping the real C++ type name.
- Prefer `VariantOffset` for runtime-varying non-address offsets or indices.
- Prefer strict accessors for fields guaranteed in the active runtime, and
  optional accessors only when the field is truly absent on some runtimes.
- If a field offset is version-gated inside flat runtimes, model that separately
  from VR branching.
- For moved bases or mixins, prefer `runtime_cast_accessor!` /
  `runtime_cast_mut_accessor!` over per-file pointer-arithmetic helpers.
- If the shared accessor macros cannot express the required pattern yet, extend
  `libskyrim/src/runtime.rs` instead of adding a local `moved_base_*` helper.
- If a runtime-varying piece is really a moved `BSTEventSource<T>` /
  `BSTEventSink<T>` mixin base, treat it like any other moved mixin and expose
  it through runtime cast/data accessors instead of a fake fixed base field.
- If the `.cpp` branch is really a wrapper around `REL::RelocateVirtual(...)`,
  keep the layout honest and translate the method layer with
  `relocated_virtual_method!` or `relocate_virtual!` instead of forcing the type
  into fake runtime tails.
- If a runtime-specific helper still needs source-backed smart-pointer
  construction, use the shared ABI-safe bridge pattern in
  the pure-Rust `libskyrim/src/ffi/` layer instead of a Rust-
  only allocation shortcut.
- If you must leave an honest compromise, add a source-backed `// TODO:` comment
  at the exact site. Do not use `todo!()` / `unimplemented!()` in translated RE
  code.

## Validation

Run:

- `cargo fmt`
- `python scripts/check_generated_staleness.py`
- `cargo check -p libskyrim`
- `cargo check -p libskyrim --tests`
