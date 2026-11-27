---
name: translate-partial
description: >
  Explicit command: /translate-partial <TypeName>.
  Audit an existing Rust translation against CommonLibVR and fill the missing or
  incorrect pieces without rewriting correct code.
---

Audit and complete the requested Rust translation.

## Workflow

1. Run `python scripts/audit_translation.py <TypeName>` for a fast local gap
   snapshot.
2. Read the existing Rust file.
3. Read the matching C++ header.
4. Read the matching `.cpp` if it exists.
5. Resolve every base and mixin type before touching fields.
6. Produce a concrete gap list.
7. Patch only the missing or incorrect pieces.
8. Run the standard checks.

## Required Gap Classes

Check all of these:

- missing fields or wrong field types
- missing nested structs
- missing named nested enums, index layers, totals/default constants, or other
  source-backed helper layers from the matching same-name header/source file
- missing or wrong size / offset asserts
- missing `inherit!` declarations
- missing RTTI / VTABLE constants
- missing `virtual_method!` declarations for fresh virtual slots
- missing `relocated_virtual_method!` or handwritten runtime wrappers for
  CommonLib `.cpp` methods built on `RelocateVirtual(...)`
- missing override comment blocks
- missing `relocation_func!` or `relocation_variable!`
- wrong relocation type choice (`RelocationID` vs `VariantID`)
- wrong handling of `ENABLE_SKYRIM_VR` / `REL::Module::IsVR()` branches
- wrong raw-pointer or opaque stand-in where a now-supported shared smart
  pointer layer (`NiPointer`, `BSTSmartPointer`, `hkRefPtr`) should be used
- missing ABI-safe bridge usage for source-backed smart-pointer construction or
  out-param helpers
- wrong by-value ABI modeling for small non-trivial C++ wrappers such as
  `BSPointerHandle<T>` descendants (`ActorHandle`, `ObjectRefHandle`,
  `ProjectileHandle`) in relocated, virtual, or hook-facing surfaces
- wrong visitor strategy: closure wrapper vs raw ABI visitor vs safe sync adapter
- wrong event strategy: fixed event base vs runtime cast accessor vs raw sink
  pointer-only API
- duplicated local stop/continue enums that should use `BSContainerForEachResult`
- missing extension trait for reusable virtual mixins
- overly literal Rust-facing wrapper signatures or missing ergonomic helpers
  where source shows no meaningful behavioral difference, for example
  unnecessary `&mut self` on getters or missing `*_as_str()` sugar over
  read-only C-string getters
- stale `todo!()` or placeholder comments in translated logic

## Relocation Rules

- `REL::ID(id)` maps to `ID::new(id)`
- `REL::Offset(offset)` maps to `Offset::new(offset)`
- C++ `RELOCATION_ID(...)` methods must use `RelocationID`, not `VariantID`
- RTTI / VTABLE constants stay on `VariantID`
- runtime-varying indices or offsets use `VariantOffset`
- normal RE files should prefer macro-based relocation instead of explicit
  `Relocation<T>` unless a macro cannot express the case cleanly
- pure `.cpp` wrappers around `REL::RelocateVirtual(...)` should prefer
  `relocated_virtual_method!`; mixed shim methods should keep a handwritten body
  and call `relocate_virtual!` only in the forwarding branch
- if a source-backed method returns or accepts a non-trivial handle type by
  value, do not preserve that raw ABI shape in a Rust macro signature; use an
  out-param wrapper in `libskyrim/src/ffi/` and keep any ergonomic return-by-value API
  only above that boundary

## Layout Rules

- Never replace a named parent or mixin with `[u8; N]`
- If a dependency is required for layout, translate it first
- If the file currently uses a consumer-local `*View` or similarly renamed
  stand-in for a real external CommonLib RE dependency, treat that as a
  mismatch: move the minimal source-backed partial translation into the
  dependency's matching Rust file and keep the real C++ type name there.
- If the file reads runtime-tail fields from another named RE type through local
  pointer arithmetic, treat that as the same kind of mismatch: move the access
  behind owner-side helper/accessor methods on the dependency type when the
  shared runtime macros can express it honestly.
- If an RE file depends on source-backed types from `CommonLibVR/include/REX/**`,
  translate those support types into `libskyrim/src/rex/*.rs` and import them
  from `crate::rex` instead of keeping local stand-in definitions
- If a field exists only on some runtimes, use runtime-data accessor macros
  instead of inventing one fake universal layout
- If a branch only changes method implementation and not memory layout, keep one
  Rust type and translate the method as a runtime-aware wrapper instead of
  splitting the struct
- If the file uses local moved-base helper functions such as `moved_base_ref`
  / `moved_base_mut` for mixin accessors, treat that as a cleanup target when
  the shared runtime accessor macros can express the same surface. Prefer
  extending `libskyrim/src/runtime.rs` over adding another per-file helper.

## Visitor Rules

- Preserve source-backed visitor lifetime semantics. Do not invent a safe
  adapter if the engine may retain the callback object.
- If the C++ surface is already closure-style (`std::function` wrapper), prefer
  a Rust closure helper over a bespoke ABI visitor API.
- Use `BSContainerForEachResult` for Rust-side synchronous traversal helpers
  instead of local stop/continue enums.
- If the file contains a concrete callback descendant such as
  `MagicCaster::PostCreationCallback : MagicTarget::IPostCreationModification`,
  keep it in the normal RE-type bucket: verify the abstract callback base as
  ABI, then translate the concrete descendant with honest layout and
  inheritance instead of trying to fold it into a generic adapter.

## Event Rules

- Reuse the shared `BSTEventSource<T>` / `BSTEventSink<T>` layer instead of
  per-file event stand-ins.
- Fixed-offset event owners may keep thin forwarding wrappers.
- Runtime-varying event bases should use cast/runtime-data accessors rather than
  fake always-present fields.
- Do not wrap low-level `BSTEventSource` methods in safe owner helpers unless
  source proves the sink lifetime / ownership contract.

## Editing Rule

Preserve correct existing code. Patch narrowly. If a larger refactor is required
to make the file honest, do it, but keep the resulting structure idiomatic for
the current `libskyrim` architecture.

Preserve the source-backed low-level contract, but do not keep a mechanically
literal C++ signature when it only makes the Rust API worse without preserving
real behavior. If needed, keep the raw method and add a more ergonomic helper
alongside it.

If you must leave an honest compromise in place, add a source-backed `// TODO:`
comment at the exact site. Do not hide the compromise only in the final report,
and do not use `todo!()` / `unimplemented!()`.

If the task adds or removes `libskyrim/src/re/*.rs` files as part of the fix,
regenerate `libskyrim/src/re/mod.rs` with `python scripts/generate_re_mod.py`.

## Validation

Run:

- `python scripts/generate_re_mod.py` when the RE file set changed
- `python scripts/check_generated_staleness.py`
- `cargo fmt`
- `cargo check -p libskyrim`
- `cargo check -p libskyrim --tests`
