---
name: verify-translation
description: >
  Explicit command: /verify-translation <TypeName>.
  Deep verification of an existing Rust translation against the full CommonLibVR
  inheritance chain. Fixes all source-backed mismatches it finds.
---

Verify the requested translation against the full inheritance chain.

## Verification Scope

Read and verify:

1. the Rust file
2. the matching C++ header
3. the matching `.cpp` if it exists
4. every parent header and parent `.cpp` needed to understand inherited layout,
   virtual slots, and helper methods

Before editing, `python scripts/audit_translation.py <TypeName>` is a good fast
preflight when the existing Rust file may already be mostly correct.

## Required Checks

### Layout

- field order
- field types
- offset comments
- nested structs
- named nested enums, index layers, totals/default constants, and other
  source-backed helper layers from the matching same-name header/source file
- `size_of` assert
- `offset_of` asserts for mixins
- no blob substitution for named parents or mixins
- no consumer-local `*View` / stand-in substitution for real external RE
  dependencies that should live in their own matching Rust files
- no consumer-local foreign runtime-tail pointer arithmetic when the dependency
  type can expose the access honestly through owner-side helper/accessor methods
- source-backed `REX/**` dependencies are translated into `libskyrim/src/rex/*.rs`
  and imported from `crate::rex` when the RE file needs them

### Virtuals

- fresh virtual slots are represented with `virtual_method!`
- pure CommonLib `.cpp` wrappers around `RelocateVirtual(...)` use
  `relocated_virtual_method!` or an equivalent handwritten wrapper with the same
  semantics
- inherited overrides are documented with `// override (Parent)` comment blocks
- child files do not redundantly redeclare inherited parent virtuals
- vtable indices match source-backed order

### Relocations

- `.cpp` relocated methods use `relocation_func!`
- global or singleton access uses `relocation_variable!` when appropriate
- `REL::ID(...)` / `REL::Offset(...)` map to `ID` / `Offset`
- `RELOCATION_ID` maps to `RelocationID`
- true `VariantID` use is reserved for constants such as RTTI and VTABLE
- no legacy `VariantID::new(se, ae, 0)` for relocated functions
- mixed runtime shim methods use `relocate_virtual!` only in the forwarding
  branch instead of being flattened into fake virtual ownership
- source-backed smart-pointer construction helpers use the shared ABI-safe
  bridge pattern when needed instead of ad-hoc Rust allocation or permanent raw
  pointer stand-ins
- relocated, virtual, and hook-facing surfaces do not model non-trivial
  `BSPointerHandle<T>` descendants (`ActorHandle`, `ObjectRefHandle`,
  `ProjectileHandle`) by value at the raw ABI boundary; verify hidden
  out-param wrapper usage instead

### Visitors and functors

- closure-style wrappers stay closure-style in Rust when no ABI visitor is needed
- real ABI visitor interfaces are translated faithfully when the type depends on them
- safe `*_with` / `*_fn` helpers are added only when source-backed call sites
  prove synchronous, non-retained use
- Rust-side synchronous traversal helpers use `BSContainerForEachResult`
  instead of per-file custom enums
- concrete callback descendants of small ABI interfaces, such as
  `MagicCaster::PostCreationCallback`, are verified as ordinary RE layout types;
  check the abstract callback base, child layout, and any retention clues
  separately instead of treating them as generic adapters

### Events

- `BSTEventSource<T>` / `BSTEventSink<T>` use the shared translation rather than
  local stand-ins
- fixed-offset event owners use thin forwarding wrappers where source does
  likewise
- runtime-varying or runtime-exclusive event bases use cast/runtime-data
  accessors instead of fake universal fields
- low-level `BSTEventSource` APIs stay `unsafe`; safe owner-side wrappers are
  added only when source proves the sink lifetime / ownership contract
- moved mixin/base accessors use the shared runtime accessor macros rather than
  local pointer-arithmetic helpers such as `moved_base_ref` / `moved_base_mut`

### Runtime layout

- runtime-varying fields use runtime-data accessor macros where needed
- no fake single-layout struct for truly divergent runtime tails
- `ENABLE_SKYRIM_VR` branches that only change method behavior are kept as
  unified Rust methods with runtime branching instead of compile-time `#[cfg]`
- moved mixin/base accessors prefer `runtime_cast_accessor!` /
  `runtime_cast_mut_accessor!` or matching shared accessors instead of local
  one-off helpers

### Ergonomics and reuse

- extension traits exist for reusable virtual mixins
- trait blanket impls use `AsRef` / `AsMut` correctly
- public Rust-facing getters and helpers do not require stronger mutability or
  raw-pointer-only consumption unless the source-backed contract truly needs it
- when a CommonLib wrapper is non-const but semantically read-only, Rust may
  expose `&self` and companion helpers such as `*_as_str()` as long as the
  underlying low-level behavior stays honest

### Generated artifacts

- if verification required adding or removing RE files, regenerate
  `libskyrim/src/re/mod.rs` with `python scripts/generate_re_mod.py`
- if the issue is in generated offsets, fix the C++ source input and regenerate
  via `python scripts/generate_offsets.py` instead of patching generated Rust by hand

## Fix Policy

Fix all source-backed problems found during verification. Do not stop after
reporting. The goal of this skill is a corrected translation, not a read-only
review.

If verification finds a consumer-local partial stand-in for a real external RE
dependency, move that partial translation into the dependency's matching Rust
file, keep the real C++ type name there, and update the consumer to use it.

If a source-backed blocker still prevents a full fix, leave a `// TODO:` comment
at the exact compromise site describing the current stand-in, the missing
prerequisite, and the intended end state. Do not use `todo!()` /
`unimplemented!()`.

## Validation

Run:

- `python scripts/generate_re_mod.py` when the RE file set changed
- `python scripts/check_generated_staleness.py`
- `cargo fmt`
- `cargo check -p libskyrim`
- `cargo check -p libskyrim --tests`
