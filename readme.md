# CommonLib-SkyrimLib (pure-Rust fork)

A Rust library for writing SKSE plugins for Skyrim SE / AE / VR, with no C++
code or FFI bridge.

- `libskyrim`: game types (`re`), Address Library lookup, relocation,
  trampoline hooks (`write_call` / `write_vfunc`), SKSE interfaces and a
  higher-level `sdk` layer
- `rex_com`: COM interfaces (IUnknown / DXGI / D3D11) usable without libskyrim
- `core-util`, `cstd`, `libskyrim-macros`: support crates

The `re` layouts and logic are translated from CommonLibSSE-NG and kept in sync
with it (Skyrim 1.7.x supported).

## References

- [CommonLibSSE-NG](https://github.com/alandtse/CommonLibSSE-NG): source of the
  translated layouts and logic
- [Newrite/CommonLib-SkyrimLib](https://github.com/Newrite/CommonLib-SkyrimLib):
  upstream of this fork
- [SkyrimAEUncapper-Rust](https://github.com/TheDreadedAndy/SkyrimAEUncapper-Rust):
  where the library originated
- [commonlibsse_ng](https://github.com/SARDONYX-sard/commonlibsse_ng): another
  Rust port of CommonLibSSE-NG, partly borrowed from

## License

[GPL-3.0-or-later](./LICENSE), because the code ports logic from
CommonLibSSE-NG, which is licensed
[GPL-3.0-or-later](https://github.com/alandtse/CommonLibSSE-NG/blob/ng/COPYING.txt)
(with [its own exceptions](https://github.com/alandtse/CommonLibSSE-NG/blob/ng/EXCEPTIONS.md)).

The original libskyrim is licensed under BSD-2-Clause:
[LICENSES/LICENSE.txt](./LICENSES/LICENSE.txt)
([upstream](https://github.com/Newrite/CommonLib-SkyrimLib/blob/CommonLib-LibSkyrim/LICENSE.txt)).
