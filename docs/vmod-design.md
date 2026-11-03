# .vmod design

Proposal; nothing here is implemented yet.

`.vmod` is the VERA20K mod release format. A mod is written as ordinary INI and assets, or
partly as WASM code, and released as one signed package. The package has two aims:

- **No direct modification.** A changed package fails its signature, and a changed mod
  cannot join rooms, ranked play, saves or replays made with the original.
- **A higher extraction bar.** Existing tools such as YR and XCC cannot open a package, and
  a mod's gameplay logic ships as compiled, obfuscated WASM instead of readable INI.

WASM is VERA's extension runtime, the role Lua plays in other games: mod code calls engine
interfaces to query state, run actions and react to events.

## Goals and scope

In scope:

- a signed container format;
- data INI and core assets encrypted inside the package;
- the logic sections of INI compiled to WASM automatically by the packer, so authors keep
  writing INI;
- hand-written WASM extensions calling the same engine interfaces, reachable from INI;
- mod checks for multiplayer, saves and replays.

Out of scope:

- stopping extraction from a modified engine. The engine is open source, so a one-line
  patch can dump anything it decodes; see [Threat model](#threat-model);
- changing the retail read paths or rules semantics;
- shipping retail files in a package; players still bring their own game files.

A mod is one more high-priority asset source plus a logic module. Without mods the engine
behaves exactly as it does today.

## Threat model

| Attacker | What stops them |
| --- | --- |
| Opens the package with XCC or a hex editor | encrypted sections, hashed names |
| Edits the package and plays it | signature check before mount |
| Plays an edited mod with others | mod digests in rooms, ranked play, saves and replays |
| Re-uploads someone else's mod | mod library content fingerprints and author keys |
| Wants to read the gameplay logic | compiled, obfuscated WASM: decompiling or probing takes real work |
| Patches the engine to dump decoded data | nothing technical; data INI and assets are readable this way |

Data that the simulation reads in hot loops cannot be hidden. `src/sim` reads rules values
over 4,000 times, many of them per unit per tick. Routing each read through WASM would not
fit the 20,000-unit target. Handing the values over once puts them in plaintext engine
memory. A black-box value function can also be enumerated over the type list the engine
already holds. Logic is different: the engine only needs its inputs and effects, so its
structure and constants can stay inside WASM.

## Package format

A `.vmod` file has six sections.

| Section | Contents | Stored as | How the engine reads it |
| --- | --- | --- | --- |
| Header | `VMOD` magic, format version, section offsets | plain | parsed at mount |
| Manifest | id, version, interface version, required capabilities, dependencies, author public key, section digests, logic exports | plain TOML | validated, dependency-sorted |
| Data | data INI and core assets | MIX-style entries, compressed, encrypted with a per-build key | decrypted once at mount into an in-memory archive |
| Logic | `logic.wasm`: compiled INI logic and hand-written extensions | WASM, obfuscated | instantiated at mount, resident for the match |
| Plain | non-core assets | MIX-style index plus uncompressed body; index encrypted with the package key | archive built directly over the file; memory-mapped on desktop |
| Signature | Ed25519 over every preceding byte | plain | verified before mount |

The package stores no file names. Every entry is keyed by the MIX name hash the engine
looks files up with (`mix_hash`). The data-section key travels in the package, so its
encryption stops off-the-shelf tools, not a determined reader.

## Asset tiers

Small, mod-specific assets go into the data section; large, generic ones go into the plain
section. Authors can move everything except INI with wildcards.

| Asset | Default section | Why |
| --- | --- | --- |
| Data INI: rules, art, ai (skeleton), sound, eva, theme, mode INIs | data, fixed | the mod's core design |
| Logic INI sections | compiled into logic | see [Logic compilation](#logic-compilation) |
| VXL, HVA | data | new vehicle models, small and mod-specific |
| SHP for new units, buildings and infantry | data | mod-specific art |
| Cameos (SHP/PCX) and palettes used only by those assets | data | small |
| CSF strings | plain | low value |
| TMP tiles and other theater assets | plain | many files, mostly derived from retail |
| Audio (bag/idx, wav, aud) and music | plain | large |
| BIK movies, loading screens, large UI art | plain | large |
| Retail files | not packaged | players bring their own |

Authors override the defaults in the project file `mod.toml`:

```toml
[protect]
include = ["*.vxl", "*.hva", "units/*.shp", "cameo/*"]
exclude = ["*.wav"]
```

The data section gets a soft limit, suggested at 64 MB compressed until the browser build is
measured. The packer warns past it but does not refuse. The limit exists because the section
is decoded in one pass at mount, and the browser build is bound by wasm32 memory.

## Data INI

Data INI reaches the engine as INI text, and the native readers and rules layering do not
change. Text rather than parsed values, because:

- the native readers own many details: the 511-byte line cut, `;` comments, `Key=` read as
  absent, case-sensitive keys, the first of duplicate keys winning;
- map and mode INIs override a mod's rules key by key, so those rules must enter the same
  layer stack;
- `RuleSet`'s internal layout changes between engine versions; text survives an engine
  update without repacking.

Normalization:

1. Split lines the native way first: LF ends a line, CR is dropped, NUL ends the line, and
   only the first 511 bytes are kept. The readers then see exactly the values of the
   original file.
2. Then drop comments, blank lines and leading and trailing whitespace.
3. Keep every key, including keys the engine does not read yet, so Ares and Phobos keys
   ported later still find their values.
4. Keep section and key order; registries allocate types in read order.
5. Replace each compiled logic entry with its skeleton (next section).

File names stay the same (`rulesmd.ini`, `artmd.ini`, `aimd.ini` and so on), so
`[ini] profile` and the RULESMD → LANGRULE → mode INI → map layering keep working. A mod's
`rulesmd.ini` replaces the retail one by archive priority, the same way an INI placed in
`expandmd99.mix` does today.

## Logic compilation

### Two sources, one module

- **INI.** Authors write the usual INI. `vmod build` compiles its logic sections into
  `logic.wasm`; nothing extra is written by hand.
- **Hand-written WASM.** Authors who need more write WASM against the same interfaces (Rust
  SDK first). INI reaches these functions through reserved numbers:
  - AI trigger condition numbers 1000 and above call a registered condition export;
  - a reserved range of script action numbers calls a registered action export;
  - map trigger events and actions follow the same pattern later.

Both sources link into one `logic.wasm` per mod and use one ABI.

### Compilation specializes the engine's interpreters

The engine already interprets INI-defined logic:

| Interpreter | INI it runs |
| --- | --- |
| `condition_holds` (`src/sim/ai_team_creation.rs`) | `[AITriggerTypes]` conditions in `aimd.ini` |
| `team_script_vm` (`src/sim/team_script_vm.rs` and its `actions`, `super_actions` modules) | `[ScriptTypes]` action lists |
| `trigger_runtime` (`src/sim/trigger_runtime.rs`, `evaluate_event` and the action dispatch) | map Triggers, Events and Actions |

Compiling an entry folds the interpreter's dispatch and the entry's constants into code. Take
"when a > b, do c": the WASM reads a through an import, compares it with the folded b, and
calls the import for c. The engine sees only the interface calls.

The leaves stay where they are. Counting owned objects, reading available money or running a
team action remains the engine's single Rust port of that native function, exposed as an
import. WASM holds only the composition and the constants, so no native function gains a
second implementation.

```
INI --(engine readers)--> definitions --> logic IR --> obfuscation --> wasm-encoder --> logic.wasm
                              |
                              `--> skeleton (ids, references, scheduling state) --> data INI
```

- **Readers.** The packer parses INI with the engine's own readers into the engine's
  definition types, such as `TeamAiTriggerDefinition`, so every lexical detail matches the
  runtime.
- **IR.** A small expression and sequence IR, for example
  `Compare(op, Call(house_money, enemy), Const(5000))`. Hand-written WASM skips it.
- **Skeleton and logic.** The engine keeps what its schedulers read and write; the decision
  moves to WASM. For `TeamAiTriggerDefinition` (`src/sim/team_script_vm.rs`):
  - skeleton: id, `primary_team_type` and the other team references, `weights` (they change
    during play and are saved), `enabled`, `side`, `multiplayer`, `owner`;
  - compiled: `condition`, `comparison_mask`, `object_type`, `threshold`.
- **Equivalence.** Every compiled entry must decide exactly as the native interpreter does.
  `vmod build` runs both on a set of constructed states and refuses to pack on any
  difference.
- **Obfuscation.** Retail logic has a fixed shape: a plain compile decompiles to
  `if house_money(e) > 5000`. Since compilation is automatic, each build applies seeded
  constant splitting, opaque predicates and control-flow flattening at no cost to authors.

### Layering

Maps extend `aimd.ini` and may replace a mod's triggers or scripts. Resolution is per id: an
entry the map layer replaces runs on the native interpreter with the map's definition;
otherwise the compiled export runs. An `[AITriggerTypes]` value is one line of 18 tokens, so a
replacement is whole and never half compiled.

### Protection by kind

- **AI trigger conditions** hide the condition, comparator, amount and counted type.
  Recovering them takes decompiling obfuscated WASM or probing with constructed game states.
- **Scripts and map trigger actions** are mostly ordered engine actions. Step state stays in
  the engine, so stepping a script reveals its sequence; compiling hides conditional
  arguments and branches, not the order.

## Extension runtime

Desktop runs `logic.wasm` on wasmtime (Cranelift); the browser build hands it to the
browser's own WebAssembly engine through `js-sys`. Both compile to machine code, so a hook
costs about a native call plus the interface crossing. The two engines must decide
identically, so everything that could differ between them is removed before a module
reaches either one.

- **Compilation and cache.** Desktop compiles a module when its mod is installed or first
  mounted and caches the result locally, keyed by the logic digest and the wasmtime version.
  The engine never loads a precompiled artifact it did not produce itself: wasmtime's
  `Module::deserialize` trusts its input, so a downloaded artifact would be arbitrary native
  code. The browser compiles at mount with `WebAssembly.compile`.
- **Instruction checks.** The packer and the engine both reject float, SIMD (relaxed SIMD
  included), thread and atomic instructions, and allow only imports from the interface list.
  What remains is integer code that every conforming engine runs the same way.
- **Fuel and stack depth.** The packer instruments the limits into the module instead of
  relying on an engine's own metering: each basic block subtracts its cost from a fuel
  global, and each call checks a depth global. Both engines run the same instrumented code,
  so a mod runs out at the same instruction on every client. Running out falls back to the
  native default for that entry and is logged. The engine rejects modules without the
  instrumentation.
- **Numbers.** Integers only: `i32`, `i64`, engine handles and `SimFixed` raw bits, matching
  the engine's simulation math rules.
- **When it runs.** Hooks run only in serial tick phases (Triggers, TeamScript, AI team
  creation), never inside rayon-parallel phases, so one instance per mod is enough and the
  wasm-threads build needs no per-worker instance. Calls are batched: one call evaluates
  every registered entry of a phase for one house, so the crossing count per tick follows
  houses, not entries. This matters most in the browser, where each call passes JS glue
  between the engine's wasm and the mod's instance.
- **Memory and state.** Memory is small, capped in the manifest, and reset to the initial
  snapshot before each batch, so nothing persists inside the module. State goes into a
  key-value store the simulation owns, so it enters save snapshots (`src/sim/snapshot.rs`)
  and the sync hash (`src/sim/world/world_hash.rs`).
- **Randomness.** Only from the match's random stream, through the interface; a mod has no
  random source of its own.
- **Languages.** Any toolchain that emits wasm32 without float or SIMD can target the
  interface; a Rust SDK comes first. Languages whose runtimes use floats internally (most
  garbage-collected ones) fail the instruction check.

An interpreter such as wasmi would run one implementation everywhere, but it is several
times slower than compiled code. With the instruction checks and packer-side fuel, two
compiled engines give the same results, so the interpreter is not needed.

### Interface (`vx1`)

- **Imports** live in the import module `vx1`, named after the interface version. A new
  version adds a module and never changes an existing one.
- **Types.** `i32` and `i64` only. Objects are handles: house index (`i32`), type handle
  (`i32`, the registry index after rules load) and object stable id (`i64`). A stale handle
  returns an error code; it never traps.
- **Required exports.**
    - `vx_abi_version() -> i32`;
    - `vx_init()`, run once at mount;
    - `vx_io_buffer() -> i32`, the address of the buffer for variable-length arguments and
      results;
    - the entry tables, starting with `vx_ai_conditions(house, first, count)`, which writes
      one result per registered condition into the buffer.
- **Static registration.** The manifest lists every export and the INI id or reserved number
  it serves; the engine checks the list against the module's exports at mount. Hooks cannot
  be added at runtime, so the hook set is covered by the mod digest.
- **Queries are immediate** and read current state through the owning module's read API.
- **Actions are buffered.** A batch's actions queue in order, and the engine commits them
  through their owners after the batch returns. A batch that traps or runs out of fuel
  commits nothing.
- **Code location.** The host side lives in `src/sim/logic/` in `vera20k-core`, beside the
  interpreters it specializes, with no `render`, `ui`, `audio` or `net` dependency.

| Kind | Examples | Rules |
| --- | --- | --- |
| Queries | `house_money`, `house_power`, `house_count`, `super_ready`, `type_id`, `unit_health`, `cell_owner` | read-only, immediate |
| Actions | `team_create`, `script_set_mission`, `trigger_fire`, `grant_super` | buffered; each goes through the engine owner of that state; no direct writes |
| Events | `on_unit_created`, `on_destroyed`, `on_timer` | a mod registers only what it uses; the engine calls back at a fixed tick phase |
| State and randomness | `kv_get` / `kv_set`, `rand(lo, hi)` | saved and hashed; random numbers from the match stream |

Interfaces open only at low-frequency points first: AI triggers, scripts, map triggers, unit
creation and death. Per-unit, per-tick points such as damage resolution and movement wait for
a 20,000-unit, 30-house stress run. The engine's native behavior always stays; WASM takes over
only the entries and events a mod registers.

### Native capabilities stay native

Ares, Phobos and the other [capabilities](capabilities.md) are not WASM plugins, and an
`ares.wasm` is not planned. They change native functions from inside: they hook the middle of
gamemd functions, add state to objects and run per unit per tick. A plugin would need an
interface at almost every hook site and would become a second owner of behavior the engine
already ports. They stay compile-time features (`cfg(feature = "ares")`), ported into each
native owner. A mod's manifest declares the capabilities it needs, and a build without them
refuses to mount it.

Their documentation sets the scope: which features exist and which keys they read. It does
not give defaults, parse details, hook positions, RNG draws, timers or feature interactions,
and mods depend on all of these, Ares bugs included. Behavior therefore comes from the DLL's
instructions plus native ports of the gamemd functions it hooks, as [ares.md](ares.md)
describes.

## Engine loading

Each vmod mounts once, at process start, ahead of every retail mix and `expandmd`.

```
spawn.ini [Mods] -> verify signature -> sort by dependency
  |- data:       decrypt -> entries -> in-memory MixArchive
  |- plain:      index -> MixArchive over a subrange of the file
  `- logic.wasm: instruction check -> compile (cached on desktop), resident -> registered entries and events

AssetManager archives (highest first):
  vmod data > vmod plain > langmd.mix > ... > expandmd99..00 > ra2md.mix > ra2.mix ...
    -> existing readers, unchanged: INI layers, SHP/VXL/HVA/TMP parsers,
       atlases, simulation asset catalogs
```

- **Handoff.** The launcher adds a `[Mods]` section to `spawn.ini` with each mod's id,
  version and digest, already dependency-sorted. `open_startup_archives` mounts the vmods
  before `langmd.mix`. Archives mounted first take priority, so vmods win.
- **Archives.**
    - The decrypted data section becomes an in-memory `MixArchive`. This needs a constructor
      that takes entries; `MixArchive::from_bytes` already builds an archive from memory.
    - The plain section becomes a `MixArchive` over a `MixData` subrange: a memory map of the
      file on desktop, the whole file read into memory in the browser.
- **Logic registration.** The manifest lists each compiled or hand-written export and the
  INI id or reserved number it serves. When the rules load, each registry entry records
  whether it runs natively or through an export, after map layering is applied.
- **Loose files.** Retail lets loose files in the game directory win over every mix
  (`CCFileClass::Open` checks disk before mix, `0x00473D10`). With vmods enabled, names a vmod
  provides win over loose files instead; otherwise an old `rulesmd.ini` in the player's
  directory would override the mod. This is a deliberate deviation from retail order, behind
  VeraX.
- **Lifetime.** The `LoadFileFromMIX` cache (`0x005B40B0`) lives for the whole process and
  keeps its first hit, so mods mount only at startup and cannot be swapped mid-process.
  Desktop already starts a new process per match (`vera20k -SPAWN`), and the browser build
  already needs a page reload before the next match.
- **Downstream is unchanged.** SHP/VXL/TMP parsers, atlases and the asset metadata the
  simulation uses (SHP frame counts, TMP heights and terrain types, VXL admission) stay as
  they are.

## Multiplayer, saves and replays

The mod list and its digests are match inputs: a room cannot start when they differ, and
saves and replays open only with the same mods.

- **Rooms.** The host picks mods; the room carries each mod's id, version and digest. A
  client downloads missing mods from the mod library and is ready only after the digest
  checks out.
- **Handshake.** The mod digest list joins the rules compatibility data, compared with the
  rules and map digests. Today `RulesCompatibility` (`src/rules/compatibility.rs`) only
  distinguishes compiled capability sets.
- **Rules digest.** Data INI stays ordinary INI, so the existing rules digest should need no
  change; confirm that `multiplayer_rules_digest` reads the files after asset resolution. The
  logic digest joins it, because compiled entries no longer appear in the INI.
- **Saves and replays.** Their metadata records the mod list and digests. A missing or
  mismatched mod refuses the load and points to the right version.
- **Sync hash.** Logic acts through engine state and the key-value store is in `world_hash`,
  so nothing extra is needed.

## Packer: `vmod build`

`vmod build` is a command-line tool that turns a mod project directory into a signed
`.vmod`. Compilation and checks run on the author's machine; the engine only accepts the
result. It links `vera20k-core` and uses the engine's readers and interpreters.

1. Read `mod.toml`: id, version, dependencies, protection rules.
2. Check the INI files with the engine's parser and report referenced types and assets that
   do not exist.
3. Normalize the INI files: native line splitting first, then comments and whitespace;
   every key and the order stay.
4. Compile logic sections to IR, split off their skeletons, and run the equivalence check
   against the native interpreters.
5. Compile hand-written logic (Rust SDK, `wasm32` target), link it with the compiled IR,
   obfuscate with a per-build seed, check the instruction set and import list, and emit
   `logic.wasm`.
6. Sort assets by the protection rules and replace file names with MIX name hashes.
7. Data section: compress and encrypt with a per-build key.
8. Write the plain section: MIX-style index and body, index encrypted.
9. Write the manifest with section digests and logic exports, and sign it with the author's
   private key.

A companion `vmod inspect` shows only the manifest and digests, not the contents, for
authors and the mod library to check against.

## Signing and security

Signatures answer who published a package and whether it changed; the sandbox limits what
mod code can do; content fingerprints catch re-uploads.

- **Author keys.** Authors generate an Ed25519 key pair locally and register the public key
  with the mod library, tied to their VERA20K account.
- **Verification.** The engine verifies the signature and every section digest before
  mounting. Ranked play admits only public keys and digests registered with the mod library.
- **Re-uploads.** A repacked extraction has a new package digest, so the mod library
  compares content instead: hashes of each normalized INI section, each asset and each logic
  export, scored by overlap with registered mods.
- **Sandbox.** WASM can call only the listed imports; it has no file, network or clock
  access. Memory pages and per-call fuel are capped.
- **Automatic downloads.** A mod downloaded for a room is code too: verify the signature,
  then check the instruction set, and only then instantiate it.

## Phases and acceptance

1. **Container and mounting.** vmod format, signatures, encrypted data section, plain
   section, data INI.
   Gate: the same mod packed as a vmod and as an `expandmd` gives the same rules digest and
   the same picture in a match.
2. **Runtime and AI trigger compilation.** wasmtime on desktop and the browser's WebAssembly,
   instruction checks, packer-side fuel, key-value store; `[AITriggerTypes]` conditions compiled from INI with the
   equivalence check and obfuscation; condition numbers ≥ 1000 for hand-written exports.
   Gate: a mod with compiled triggers stays in sync in a desktop-versus-browser match, picks
   the same teams as its INI form, and a save reloads to the same result.
3. **Multiplayer and distribution.** Mod list in rooms and the handshake, automatic
   downloads; saves and replays record mods; the mod library registers author keys and
   fingerprints content.
   Gate: a player without the mod gets it on joining; mismatched versions cannot start; a
   repacked extraction is flagged.
4. **More logic and interfaces.** `[ScriptTypes]`, then map triggers; events; Rust SDK,
   packer docs, a sample mod.

After phase 3, vmods with compiled logic work in multiplayer rooms; phase 4 grows as needed.
The first two phases sit behind the VeraX capability and leave retail builds unaffected.

## Risks and open questions

The main accepted limits: a modified engine can dump data INI and assets after mount, and a
GPL fork can run every vmod.

Risks:

- **Patched engines.** Decoded data INI and assets can be written out with a one-line engine
  patch. Only compiled logic resists this, and only to the degree its obfuscation holds.
- **GPL forks.** A fork has every interface. Nothing technical stops that; the lobby,
  accounts, ladder and mod library are where VERA competes.
- **Equivalence drift.** A change to a native interpreter changes what compiled logic must
  do. Engine updates that touch an interpreter rerun the equivalence check on registered
  mods, and the logic interface version moves when results differ.
- **Interface crossings.** A high-frequency hook such as damage resolution may cost too
  much in crossings, most of all through the browser's JS glue; it needs a 20,000-unit
  stress run first.
- **Engine divergence.** wasmtime and browser engines must decide identically. The
  instruction checks and packer-side fuel remove the known sources; the phase 2 gate plays
  desktop against browser to catch the rest.
- **Name enumeration.** Features that enumerate file names, such as the map list and the
  `RULEMD*.INI` picker, do not work with hashed names. First-version mods carry no maps.
- **Interface versions.** Data INI keeps every key, so engine updates need no repack, but
  `logic.wasm` depends on the interface version, which must stay backward compatible.

Open questions:

- [ ] With vmods enabled, should vmod files win over loose files in the game directory?
- [ ] What soft limit for the data section? Needs a browser measurement.
- [ ] Does `multiplayer_rules_digest` read the data INI after asset resolution?
- [ ] Which constructed states make the equivalence check cover every branch of
      `condition_holds`?
- [ ] Where does the mod library live: an extension of the ra20k.com map library, or
      separate?
