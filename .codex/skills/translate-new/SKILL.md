---
name: translate-new
description: >
  Explicit command: /translate-new <TypeName>.
  Translate a new CommonLibVR RE type into libskyrim with correct layout,
  inheritance, relocation mapping, and module registration.
---

Translate the requested type from CommonLibVR into `libskyrim/src/re/`.

## Workflow

1. Confirm the Rust file does not already exist.
2. Run `python scripts/bootstrap_translation.py <TypeName>` to gather the main
   source paths and likely workflow.
3. Read the full C++ header.
4. Read the matching `.cpp` if it exists.
5. Resolve the inheritance chain before writing fields.
6. Translate nested structs before the parent type.
7. Write the Rust file with layout asserts, inheritance metadata, and required
   traits.
8. Regenerate `libskyrim/src/re/mod.rs` with `python scripts/generate_re_mod.py`.
9. Run `python scripts/check_generated_staleness.py`.
10. Run the standard checks.

## Mandatory Source Checks

The `.cpp` is required whenever it exists. Extract all of the following:

- nested enums, named index layers, totals/default constants, and helper types
  from the matching same-name header/source file pair, even when they only name
  array slots or preset values
- `RELOCATION_ID(...)` methods
- `REL::ID(...)` / `REL::Offset(...)`
- static methods not visible in the header
- private helpers defined only in the `.cpp`
- singleton/global accessors
- `ENABLE_SKYRIM_VR` / `REL::Module::IsVR()` runtime branches
- `RelocateVirtual(...)` wrappers and vtable index clues
- visitor or functor call sites, including whether the code uses
  `std::function`, stack-local adapter objects, or passes a visitor across API
  boundaries
- smart-pointer ownership and construction surfaces such as `make_hkref`,
  `make_nismart`, `make_smart`, smart-pointer out-params, attach/adopt helpers,
  and delete/destructor behavior that constrains Rust-side ownership helpers
- whether any source-backed relocated or virtual surface returns or accepts a
  small non-trivial C++ class by value, especially `BSPointerHandle<T>`
  descendants such as `ActorHandle`, `ObjectRefHandle`, or `ProjectileHandle`

## Dependency Rules

For each parent or embedded field type:

- If the Rust type already exists, use it.
- If the layout is required and the Rust type does not exist, stop and translate
  that dependency first.
- If only part of the dependency is needed right now, create the matching Rust
  file for that dependency and place a minimal source-backed partial
  translation there. Keep the real C++ type name; do not invent a
  consumer-local `*View` stand-in for a named external RE type.
- If the type is pointer-only or reference-only, an `abstract_type!` stub is
  acceptable.
- If the dependency comes from `CommonLibVR/include/REX/**` and is source-backed
  for the RE translation, place it under `libskyrim/src/rex/*.rs` and import it
  from `crate::rex`.
- If the translated type needs a source-backed smart-pointer construction path,
  prefer the shared out-param construction pattern in `libskyrim/src/ffi/` (pure Rust)
  and `libskyrim/src/ffi.rs`, then construct the Rust smart pointer through
  `hkRefPtr::try_construct_with(...)`, `NiPointer::try_construct_with(...)`, or
  `BSTSmartPointer::try_construct_with(...)`.
- If a source-backed function returns or accepts a non-trivial C++ handle type
  such as `BSPointerHandle<T>` by value, do not model that raw ABI literally in
  Rust macro signatures even if the type is only 4 bytes wide. Use an honest
  hidden out-param wrapper in `libskyrim/src/ffi/`, then expose a Rust-friendly helper
  above it.

Never replace a named parent or mixin with `[u8; N]`.

## Output Rules

Write files in this order:

1. imports
2. nested structs
3. main `#[repr(C)]` struct with offset comments
4. `size_of` assert
5. `offset_of` asserts for mixins
6. RTTI / VTABLE constants and `RttiType`
7. `inherit!` declarations
8. `impl` blocks with:
   - `virtual_method!` for fresh virtual slots
   - `relocated_virtual_method!` for pure `.cpp` wrappers backed by `RelocateVirtual(...)`
   - `relocation_func!` for `.cpp` relocated methods
   - `relocation_variable!` for globals/singletons
   - non-public helpers from `.cpp`
9. ergonomic Rust-facing helpers when they do not hide a real contract, for
   example `*_as_str()` wrappers or semantically read-only `&self` getters
10. extension trait if the type is a reusable virtual mixin

If an honest blocker remains, do not use `todo!()` or `unimplemented!()`.
Instead, leave a source-backed `// TODO:` comment at the exact compromise site
that names the current stand-in, the missing prerequisite, and the intended end
state.

Do not hand-maintain `libskyrim/src/re/mod.rs` if the generator script is
available. Regenerate it.

## Relocation Rules

- `REL::ID(id)` -> `ID::new(id)`
- `REL::Offset(offset)` -> `Offset::new(offset)`
- `RELOCATION_ID(se, ae)` -> `RelocationID::new(se, ae)`
- `RELOCATION_ID(se, ae, vr)` -> `RelocationID::with_vr(se, ae, vr)` only if the
  VR ID is real and source-backed
- true CommonLib `VariantID` -> `VariantID::new(se, ae, vr_offset)`
- runtime-varying non-address values -> `VariantOffset`

Do not use `VariantID::new(se, ae, 0)` for relocated methods.
Do not use by-value `ActorHandle` / `ObjectRefHandle` / `ProjectileHandle`
directly in relocated, virtual, or hook signatures.

## Cross-Runtime Branching Rules

- Do not translate `ENABLE_SKYRIM_VR` literally into Rust `#[cfg]` by default.
- If the branch changes layout or field offsets, use runtime-data accessors.
- If the branch changes a scalar value, vtable index, or similar non-address
  selection, use `VariantOffset`, `relocate`, or `relocate_vr`.
- If the branch is a pure `.cpp` wrapper around `REL::RelocateVirtual(...)`,
  prefer `relocated_virtual_method!`.
- If the method mixes runtime branching with only part of the body forwarding to
  a vtable slot, write a normal Rust method and use `relocate_virtual!` inside
  the relevant branch.
- If the type needs moved base/mixin accessors, use
  `runtime_cast_accessor!` / `runtime_cast_mut_accessor!` or the matching
  shared runtime accessor macros. Do not invent local helpers such as
  `moved_base_ref` / `moved_base_mut`.

## Visitor Rules

- If the C++ API is already a `std::function` convenience wrapper, prefer a
  Rust closure helper instead of translating an ABI visitor first.
- If the class owns a real ABI visitor interface and source-backed call sites
  prove synchronous use, you may expose:
  - a raw ABI entrypoint
  - a typed `*_with(&mut visitor)` helper
  - a closure `*_fn(...)` helper
- If the visitor lifetime is unclear, expose only the raw ABI entrypoint.
- Use `BSContainerForEachResult` for Rust-side synchronous traversal helpers.
- Concrete stateful visitor descendants with fields are normal RE types, not
  generic adapters.
- Concrete callback descendants of small ABI interfaces, such as
  `MagicCaster::PostCreationCallback : MagicTarget::IPostCreationModification`,
  are also normal RE types. Translate the abstract callback base as ABI and the
  concrete descendant with its real fields and inheritance; only add a safe
  callback adapter if source-backed call sites prove synchronous, non-retained
  use.

## Event Rules

- `BSTEventSource<T>` and `BSTEventSink<T>` are reusable ABI mixins. Reuse the
  shared translation instead of inventing local stand-ins.
- If the owner has a fixed-offset event base, translate owner helpers as thin
  forwarding methods to that base.
- If the event base moves or only exists on some runtimes, use cast/runtime
  accessors instead of pretending the base is universally present.
- Keep low-level `BSTEventSource` mutation / dispatch APIs `unsafe`; only add
  safe owner-side wrappers when source proves the sink lifetime contract.

## Runtime Layout Rules

If SE/AE/VR layout differs:

- keep the common struct prefix in the main type
- move divergent tails behind runtime-data accessor macros
- do not fake multiple runtime sizes in one Rust type

## Extension Trait Rule

If the translated type is used as a non-zero-offset mixin and exposes virtual or
important public helper methods, add `<TypeName>Ext` in the same file.

## Validation

Run:

- `python scripts/generate_re_mod.py`
- `python scripts/check_generated_staleness.py`
- `cargo fmt`
- `cargo check -p libskyrim`
- `cargo check -p libskyrim --tests`
