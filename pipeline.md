# Codex Pipeline Templates

This file contains ready-to-use prompt templates for working on `libskyrim`
translations and hook ports with Codex.

## Default Rule

- Keep the main thread on `gpt-5.4` with `high` reasoning effort.
- Do not switch the model inside the same main thread unless there is a strong
  reason.
- Use subagents for read-only audits, dependency tracing, or isolated helper
  work.
- When the requested translation name matches a CommonLib file name, verify and
  carry over the full source-backed data surface from that same header/source
  file pair, not just layout-critical fields and methods. Named nested enums,
  array-slot index layers, totals/default constants, and helper nested types
  are part of the translation surface.
- If the target depends on another named CommonLib RE type whose Rust file does
  not exist yet, do not hide that dependency inside the consumer file as a
  local `*View` stand-in. Create the matching dependency file and place the
  minimal source-backed partial translation there, keeping the real C++ type
  name even when only a subset of fields or methods is translated.
- If consumer code needs runtime-tail fields from another named RE type, prefer
  owner-side helper/accessor methods on that dependency backed by the shared
  runtime macros instead of duplicating foreign pointer arithmetic in the
  consumer file.
- If an audit or verification pass finds such a consumer-local stand-in for a
  real external RE dependency, treat that as a fixable mismatch and move it to
  the correct dependency file.
- Treat `BSTEventSource<T>` / `BSTEventSink<T>` as a shared low-level event
  mixin layer. Do not invent local stand-ins in translated RE files.
- Use `bootstrap_translation.py` and `audit_translation.py` outputs to spot
  `event_bases` / `event_signals` early, because that changes how owners should
  be translated.
- For moved mixin/base accessors, prefer `runtime_cast_accessor!`,
  `runtime_cast_mut_accessor!`, and the other shared runtime accessor macros.
  Do not introduce per-file helpers like `moved_base_ref` / `moved_base_mut`
  when the same surface can be expressed through `libskyrim/src/runtime.rs`.
  If the macros are missing a needed pattern, extend the shared macro layer
  first.
- When CommonLib constructs smart pointers on the C++ side, such as
  `make_hkref<T>()`, `make_nismart<T>()`, `make_smart<T>()`, or helpers that
  fill smart-pointer out-params, prefer the shared ABI-safe bridge pattern:
  add the pure-Rust helper in `libskyrim/src/ffi/`, declare it
  in `libskyrim/src/ffi.rs`, and construct the Rust smart pointer through
  `hkRefPtr::try_construct_with`, `NiPointer::try_construct_with`, or
  `BSTSmartPointer::try_construct_with`.
- Keep source-backed low-level behavior honest, but do not worsen the public
  Rust API only to mirror a C++ signature literally when there is no real
  behavioral, ownership, layout, or ABI difference. If needed, keep the raw
  method and add a more ergonomic helper alongside it.
- If a source-backed compromise remains, leave a `// TODO:` comment at the exact
  site describing the current stand-in, the missing prerequisite, and the
  intended end state. Do not use `todo!()` / `unimplemented!()` in translated
  RE code.

## Skill Matrix

| Skill | Main or Subagent | Model | Effort |
| --- | --- | --- | --- |
| `translation-auditor` | subagent | `gpt-5.4-mini` | `medium` or `high` |
| `translate-new` | main | `gpt-5.4` | `high` |
| `translate-partial` | main | `gpt-5.4` | `high` |
| `verify-translation` | main | `gpt-5.4` | `high` |
| `translate-runtime-layout` | main | `gpt-5.4` | `high` |
| `refactor-runtime-layout` | main | `gpt-5.4` | `high` |
| `port-hooking` | main | `gpt-5.4` | `high` |
| `reconcile-api-ergonomics` | main or subagent | `gpt-5.4-mini` or `gpt-5.4` | `medium` |
| `add-extension-trait` | subagent or main | `gpt-5.4-mini` | `medium` |

Use `xhigh` only for unusually hard cases such as `SkyrimVM` or very large
multiple-inheritance hierarchies.

## Template 1: New Ordinary RE Type

Use when there is no Rust translation yet and runtime-divergent layout is not
the main concern.

```text
Use $translate-new for <TypeName>.

Keep the main thread on gpt-5.4 with high reasoning effort.

Before editing, use subagents:
- one subagent on gpt-5.4-mini medium to run a translation-auditor style read of the CommonLib header/cpp and report direct bases, nested types, relocated methods, RTTI/VTABLE presence, and likely dependencies

Then:
- translate the Rust file in libskyrim/src/re/
- if translation needs a non-opaque dependency from another CommonLib RE file,
  create or extend that dependency's matching Rust file and put the minimal
  source-backed partial translation there instead of inventing a local `*View`
  type in the consumer
- if the type needs a source-backed smart-pointer factory or out-param
  construction helper, use the shared ABI-safe bridge pattern instead of
  inventing a Rust-only allocation path
- follow the current relocation model: RelocationID for RELOCATION_ID, VariantID only for true CommonLib VariantID semantics, VariantOffset for runtime-varying indices/offsets
- if bootstrap/audit reports event_bases or event_signals, follow the event-owner rules:
  - fixed event base: thin forwarding owner helpers are fine
  - moved/runtime-exclusive event base: use runtime cast/data accessors
  - raw sink pointer-only API: keep it raw, do not invent a fake owner layer
- regenerate libskyrim/src/re/mod.rs if needed
- run cargo fmt, cargo check -p libskyrim, and cargo check -p libskyrim --tests

Report:
- what was translated
- any remaining dependency blockers or honest stubs
- any source-backed `TODO` comments left in code and why they remain
- validation results
```

## Template 2: Partial Existing Translation

Use when the Rust file already exists and needs targeted completion.

```text
Use $translate-partial for <TypeName>.

Keep the main thread on gpt-5.4 with high reasoning effort.

Use one subagent on gpt-5.4-mini high to do a read-only translation-auditor pass first and return:
- missing fields or nested structs
- missing or wrong inherit!/size_of/offset_of asserts
- missing relocation_func!/relocation_variable!
- wrong RelocationID vs VariantID usage
- extension-trait opportunities

Then patch only the confirmed gaps in the existing Rust file without rewriting correct code.

If the audit finds a consumer-local `*View` stand-in for a real external
CommonLib RE dependency:
- move that partial translation into the matching dependency Rust file
- keep the real C++ type name for the translated dependency
- update the consumer to depend on that proper file instead of the local view

If the audit reports event_bases or event_signals:
- reuse the shared BSTEvent layer from libskyrim/src/re/bst_event.rs
- keep low-level BSTEventSource mutation/dispatch methods unsafe
- add safe owner-side wrappers only when source proves the sink lifetime contract

If the file set changes, regenerate libskyrim/src/re/mod.rs.
If an honest blocker remains after the patch, leave a source-backed `// TODO:`
comment at the exact compromise site instead of hiding it in the final report
only.
Run cargo fmt, cargo check -p libskyrim, and cargo check -p libskyrim --tests.
```

## Template 3: Full Verification Pass

Use when the file already exists and you want source-backed confirmation through
the whole inheritance chain.

```text
Use $verify-translation for <TypeName>.

Keep the main thread on gpt-5.4 with high reasoning effort.

If helpful, use one subagent on gpt-5.4-mini medium for a quick audit_translation-style pass, but keep all final source-backed decisions and edits in the main thread.

Verify:
- the Rust file
- the matching CommonLib header
- the matching .cpp if it exists
- parent headers/.cpp as needed for inherited layout, virtual slots, and helper methods

Fix all confirmed mismatches you find.

If a mismatch cannot be completed honestly because a source-backed dependency,
factory bridge, delete path, or ownership layer is still missing, leave a
source-backed `// TODO:` comment at the exact compromise site and report it.

Use the current relocation/runtime model and regenerate generated files only through the scripts when needed.
Run cargo fmt, cargo check -p libskyrim, and cargo check -p libskyrim --tests.
```

## Template 4: Runtime-Divergent Layout

Use when `SE/AE/VR` layout differs or the C++ type uses
`RuntimeDataAccessors.h`-style logic.

```text
Use $translate-runtime-layout for <TypeName>.

Keep the main thread on gpt-5.4 with high reasoning effort.

Before editing:
- use one subagent on gpt-5.4-mini high to inspect the CommonLib header/cpp and report all runtime-divergent sizes, offsets, RuntimeDataAccessors usage, RelocateMember/RelocateMemberIfNewer calls, and likely common-prefix boundaries

Then:
- keep only the honest common prefix in the main repr(C) Rust type
- move divergent tails behind runtime-data accessor macros from libskyrim/src/runtime.rs
- use VariantOffset for runtime-varying non-address offsets or vtable indices
- add runtime-aware asserts where appropriate
- avoid fake universal layouts and avoid blob padding for named members
- if helper access to another named CommonLib RE type is needed, create or
  extend that dependency's matching Rust file and put the minimal
  source-backed partial translation there instead of defining a consumer-local
  `*View` type
- if a moved helper still needs a source-backed smart-pointer factory or
  out-param construction path, use the shared ABI-safe bridge pattern instead
  of adding a Rust-only allocation shortcut
- if the moved piece is an event base such as BSTEventSource<T>/BSTEventSink<T>,
  treat it like any other moved mixin and expose it through runtime cast/data
  accessors instead of a fake fixed base field

After that, run $verify-translation for <TypeName> semantics in the same main thread.
If any honest compromise remains, leave a source-backed `// TODO:` comment at
the exact site.
Run cargo fmt, cargo check -p libskyrim, and cargo check -p libskyrim --tests.
```

## Template 4B: Existing Runtime-Layout Refactor

Use when the type already exists, already has runtime-tail access of some kind,
but needs to be restructured into better raw blocks and a better runtime-facing
API.

```text
Use $refactor-runtime-layout for <TypeName>.

Keep the main thread on gpt-5.4 with high reasoning effort.

Before editing:
- run python scripts/audit_translation.py <TypeName>
- use one subagent on gpt-5.4-mini high to inspect the current Rust file and the
  matching CommonLib header/cpp and report:
  - existing whole-tail accessors
  - existing field-level accessors
  - candidate shared raw blocks across SE / AE / VR
  - naming mismatches such as universal-sounding accessors that are actually
    `_flat` or runtime-specific
  - places where tiny remaining overlap should become direct getter/setter
    helpers instead of more micro-structs

Then in the main thread:
- preserve the honest raw runtime surface
- extract meaningful shared raw sub-structures for reusable runtime overlap
- keep `_flat`, `_se`, `_ae`, `_vr` raw entrypoints truthfully named
- add or repair `runtime_view()` / `runtime_view_mut()` as the ergonomic layer
- if only a couple of overlapping fields remain, expose them through direct
  runtime-aware getter/setter helpers instead of forcing tiny structs
- replace repeated local runtime branching with owner-side helpers or view
  methods when the layout contract stays honest
- if the existing runtime accessor macros are missing a needed pattern, extend
  libskyrim/src/runtime.rs instead of keeping per-file pointer-arithmetic
  helpers

After that, run $verify-translation for <TypeName> semantics in the same main
thread.
Run cargo fmt, cargo check -p libskyrim, and cargo check -p libskyrim --tests.
```

## Template 5: Add or Repair Extension Trait

Use when a reusable mixin exists and child types should expose its API
ergonomically.

```text
Use $add-extension-trait for libskyrim/src/re/<file>.rs.

This is a bounded task. Prefer a subagent on gpt-5.4-mini medium unless the mixin is unusually complex.

Check:
- whether the file defines a reusable RE type
- whether the type is source-backed as a base or mixin in CommonLibVR and should be treated as reusable even if some child libskyrim/src/re/*.rs translations have not been added yet
- whether it exposes virtual methods or important public helpers worth surfacing
- whether a <TypeName>Ext trait already exists and is complete

Decide eligibility from the CommonLibVR inheritance surface first, not only from the set of Rust RE files that currently exist in the repository.

Implement or repair the extension trait in the same file.
Use <TypeName>Ext naming.
Keep raw virtual methods on the mixin impl itself.
Do not move logic from the mixin impl into the trait.
Do not redefine mixin methods on child types.
Use AsRef<TypeName> / AsMut<TypeName> blanket impls as appropriate.
If a safe wrapper is useful, add it to the mixin impl and expose it through the trait as well.
Ensure mod.rs re-exports the module so the trait is reachable.

Run:
- cargo fmt
- cargo check -p libskyrim
```

## Template 6: Port CommonLib Hooking

Use when converting plugin-side C++ hook code to Rust.

```text
Use $port-hooking for the attached CommonLib-style hook snippet.

Keep the main thread on gpt-5.4 with high reasoning effort.

Use one subagent on gpt-5.4-mini medium for read-only mapping of:
- RELOCATION_ID vs VariantID vs VariantOffset
- whether the hook is vtable-based or call/branch-based
- whether VR uses a distinct vtable index or runtime offset selection

Then in the main thread:
- port the hook using define_vtable_hook!, define_call_hook!, RelocationID, VariantOffset, relocate, and relocate_vr where appropriate
- keep original function invocation typed and source-backed
- preserve nullability and engine-contract comments

Run cargo fmt, cargo check -p libskyrim, and cargo check -p libskyrim --tests.
```

## Template 6B: Event-Heavy Owner Type

Use when the target owns one or more `BSTEventSource<T>` / `BSTEventSink<T>`
bases or exposes `GetEventSource/AddEventSink/RemoveEventSink/SendEvent`-style
helpers.

```text
Use $translate-new or $translate-partial for <TypeName>, depending on whether the Rust file already exists.

Keep the main thread on gpt-5.4 with high reasoning effort.

Before editing:
- run python scripts/bootstrap_translation.py <TypeName>
- use one subagent on gpt-5.4-mini high to audit header/.cpp event usage and report:
  - direct event bases
  - whether the event bases are fixed-offset, moved, or runtime-exclusive
  - whether the class exposes thin event-owner helpers
  - whether any safe wrapper would require proving sink lifetime/ownership

Then in the main thread:
- reuse libskyrim/src/re/bst_event.rs instead of local event stand-ins
- fixed event base: translate owner forwarding helpers normally
- moved/runtime-exclusive event base: use runtime cast/data accessors
- raw sink pointer-only API: keep it raw
- do not wrap low-level BSTEventSource mutation/dispatch APIs in safe helpers unless source proves the sink lifetime contract

Run cargo fmt, cargo check -p libskyrim, and cargo check -p libskyrim --tests.
```

## Template 6C: Rust-Facing API Ergonomics Reconciliation

Use when a translation is source-backed but the public Rust surface is more
literal to C++ than semantically necessary.

```text
Use $reconcile-api-ergonomics for <TypeName or path>.

Keep the main thread on gpt-5.4 with high reasoning effort for coupled public
API changes. For small isolated wrappers, a subagent on gpt-5.4-mini medium is
fine.

Before editing:
- read the existing Rust public surface
- read the matching CommonLib header
- read the matching .cpp if it exists
- classify each candidate issue as:
  - layout / ABI / ownership critical
  - behaviorally meaningful wrapper contract
  - ergonomics-only surface choice

Then:
- preserve the source-backed low-level contract
- do not worsen the Rust API just to mirror a non-const C++ wrapper or raw
  pointer return if no meaningful behavioral difference is preserved by that
  choice
- keep raw/strict methods when they are still useful, but add or repair
  ergonomic helpers alongside them
- typical fixes include:
  - semantically read-only getters: `&mut self` -> `&self`
  - read-only C-string getters: add `*_as_str()`
  - small Rust-facing helper wrappers over raw methods where the contract
    becomes clearer without hiding real edge cases

Validate with cargo fmt, cargo check -p libskyrim, and cargo check -p libskyrim --tests.
If the change affects plugin-facing public APIs and S:/Programming/RustSKSETemplate is available,
prefer a quick smoke check there too.
```

## Template 7: Parallel Batch of Independent RE Files

Use when multiple files can be edited independently without overlapping write
scope.

```text
Use multi-agent parallel work on these independent files:
- <TypeA>
- <TypeB>
- <TypeC>

Keep the main thread on gpt-5.4 high for integration.

Spawn one subagent per type:
- use gpt-5.4-mini high for simple translate-partial work
- use gpt-5.4 high for difficult files

Each subagent should own only its assigned Rust file and any directly required supporting edits with disjoint write scope.
Do not let subagents touch relocation.rs or runtime.rs unless explicitly assigned.

After subagents finish, integrate their changes in the main thread, regenerate re/mod.rs if needed, then run cargo fmt, cargo check -p libskyrim, and cargo check -p libskyrim --tests.
```

## Template 8: Read-Only Audit Before Deciding

Use when you are not sure whether to apply `translate-new`, `translate-partial`,
`verify-translation`, or `translate-runtime-layout`.

```text
Use $translation-auditor for <TypeName>.

Prefer a subagent on gpt-5.4-mini medium.

Do a read-only pass and report:
- current Rust/C++ artifact paths
- direct bases and important nested types
- event bases / event signals when present
- whether the file looks new, partial, or mostly complete
- whether runtime-divergent layout is involved
- whether extension traits are likely needed
- which next skill should be used: translate-new, translate-partial, verify-translation, or translate-runtime-layout

Do not edit yet unless a trivial source-backed fix is obviously correct.
```

## Template 9: Main-Thread-Only Central Refactor

Use for `relocation.rs`, `runtime.rs`, shared macros, panic/fatal handling, or
other highly coupled infrastructure.

```text
Work in the main thread only on <Task>.

Use gpt-5.4 with high reasoning effort.

Do not delegate code edits to subagents because the write scope is central and tightly coupled.
You may use one subagent only for read-only comparison against CommonLib source or for dependency discovery, but keep all implementation and final validation in the main thread.

Run the full relevant validation after edits.
```

## How To Ask For Multi-Agent Work

Use wording like:

```text
Use multi-agent parallel work.
One subagent should do a translation-auditor pass for <TypeName> on gpt-5.4-mini high.
Another subagent should inspect runtime-layout divergence on gpt-5.4-mini high.
Keep all final edits and integration in the main thread on gpt-5.4 high.
```

Or:

```text
Split this into subagents:
- subagent 1: verify the CommonLib header/cpp mapping for <TypeA>
- subagent 2: repair extension traits for <TypeB>
- main thread: integrate and run validation
```

## Quick Recommendations

- Default main thread: `gpt-5.4 high`
- Default audit subagent: `gpt-5.4-mini medium`
- Default partial-translation subagent: `gpt-5.4-mini high`
- Default runtime-layout-refactor subagent: `gpt-5.4-mini high`
- Default ergonomics-pass subagent: `gpt-5.4-mini medium`
- Use `xhigh` rarely and only for the hardest runtime-layout or MI cases
- For event-heavy owners, keep final translation and event-contract decisions in
  the main thread; use subagents only for read-only source classification
