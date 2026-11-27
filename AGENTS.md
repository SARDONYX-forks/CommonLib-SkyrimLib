# Codex Repo Policy

This repository contains multiple agent ecosystems. For Codex, the only
authoritative repository-local instruction sources are:

- `.codex/AGENTS.md`
- `.codex/skills/*`

Ignore these paths unless the user explicitly asks to inspect or edit them:

- `.agents/**`
- `.qwen/**`
- agent metadata bundled inside vendored dependencies such as `CommonLibVR/**`

If the same topic is described in multiple places, prefer `.codex/**`.

Note: external Codex runtime configuration still has higher priority than files
inside the repository. This file defines the repository-local policy only.

Useful repository-local helper scripts live under `scripts/`, especially:

- `generate_offsets.py`
- `generate_re_mod.py`
- `audit_translation.py`
- `bootstrap_translation.py`
- `find_runtime_layout_candidates.py`
- `find_extension_trait_candidates.py`
- `check_generated_staleness.py`

When translating `libskyrim/src/re/*.rs`, remember that some RE types depend on
source-backed helper structs, enums, aliases, or ABI types declared under
`CommonLibVR/include/REX/**`. Translate those dependencies into
`libskyrim/src/rex/*.rs` and import them via `crate::rex`; do not keep them as
one-off local definitions inside the RE file.

When a Rust RE translation corresponds to a same-name CommonLib header/source
file pair, carry over the full source-backed data surface from that file pair,
not only the layout-critical fields and methods. This includes nested enums,
named index layers for arrays, totals/default constants, nested helper types,
and private/source-only helpers that belong to the same translated type.

When CommonLib constructs smart pointers on the C++ side, such as
`make_hkref<T>()`, `make_nismart<T>()`, `make_smart<T>()`, or helper functions
that fill smart-pointer out-params, prefer the ABI-safe bridge pattern through
the pure-Rust `libskyrim/src/ffi/` layer (no C++ is linked), and the Rust-side
`try_construct_with(...)` helpers on `hkRefPtr`, `NiPointer`, and
`BSTSmartPointer`. Do not fake nontrivial C++ construction with raw Rust
allocation alone.

Do not worsen the Rust-facing API only to mirror a C++ wrapper literally when
that literal shape carries no real behavioral, layout, ownership, or ABI
difference. Preserve source-backed low-level behavior where it matters, but
prefer more idiomatic Rust helpers such as `&self` getters or `*_as_str()`
wrappers when they do not hide a real contract.

Do not use executable placeholders such as `todo!()` or `unimplemented!()` in
translated RE code. If a translation keeps an honest compromise such as a raw
pointer stand-in, opaque stub, missing helper, or skipped factory/delete path,
leave a source-backed `// TODO:` comment at the exact site describing the
current compromise, the missing prerequisite, and the intended end state.

Treat `rex` as "RE Extensions": a home for shared extension/support types used
by RE translations, not as a wholesale mirror of the Windows SDK.

When CommonLib uses `ENABLE_SKYRIM_VR`, `REL::Module::IsVR()`, or
`REL::RelocateVirtual(...)`, do not translate that literally into Rust `#[cfg]`
by default. In the cross-runtime `libskyrim` layer, prefer:

- runtime-data accessor macros for layout or field-offset divergence
- `VariantOffset`, `relocate`, and `relocate_vr` for runtime-varying non-address
  values such as field offsets or vtable indices
- `ID::new(...)` and `Offset::new(...)` for raw `REL::ID(...)` /
  `REL::Offset(...)` selectors
- `relocated_virtual_method!` or `relocate_virtual!` for `.cpp` wrapper methods
  backed by `REL::RelocateVirtual(...)`
- ordinary Rust wrapper methods with runtime branching when flat runtimes call a
  vtable slot but VR uses a custom shim implementation

For moved mixin/base accessors, prefer the shared runtime accessor macros such
as `runtime_cast_accessor!` / `runtime_cast_mut_accessor!` and
`runtime_data_accessor!`. Do not add local helper methods like
`moved_base_ref` / `moved_base_mut` when the macro layer can express the same
surface; if it cannot, extend the shared macro layer in `libskyrim/src/runtime.rs`.

For visitor-style APIs:

- translate `RE::BSContainer::ForEachResult` into the shared Rust-side
  `BSContainerForEachResult` for synchronous traversal helpers
- if the C++ API already exposes a `std::function` convenience wrapper, prefer
  a Rust closure-based `for_each_*` / `visit_*_fn` helper instead of inventing
  an ABI visitor layer
- only add safe Rust visitor adapters when source-backed code proves the engine
  uses the visitor synchronously and does not retain it after the call returns
- if retention or lifetime is unclear, expose only the raw ABI visitor entrypoint
- stateful concrete visitor classes with their own fields are ordinary RE
  types, not macro-generated adapters
- concrete callback descendants of small ABI interfaces, such as
  `MagicCaster::PostCreationCallback : MagicTarget::IPostCreationModification`,
  are also ordinary RE types. Translate the abstract callback base as an ABI
  interface and the concrete descendant as a normal nested RE type with honest
  layout and inheritance. Do not add a safe Rust closure adapter unless
  source-backed code proves the callback is synchronous and non-retained.


For event-style APIs built on `BSTEvent.h`:

- treat `BSTEventSource<T>` and `BSTEventSink<T>` as reusable ABI mixins, not
  per-file local stand-ins
- fixed-offset event owners may expose thin forwarding helpers such as
  `GetEventSource/AddEventSink/RemoveEventSink/SendEvent`
- if an event base moves or disappears across runtimes, use cast/runtime-data
  accessors instead of pretending the base is always present at one offset
- low-level `BSTEventSource` mutation and dispatch methods stay `unsafe`; only
  add safe owner-side wrappers when source proves the sink lifetime and
  ownership contract
