# .vmod design

Proposal; nothing here is implemented yet.

`.vmod` is the VERA20K mod release format. It aims for the protection a Minecraft
mod jar gets: existing tools such as YR and XCC cannot open or edit a release
package, and a mod's gameplay depends on interfaces that only VERA has.

## Goals and scope

In scope:

- a signed container format;
- every INI compiled into a WASM generator;
- the core assets an author picks compiled into WASM, the rest in a plain section;
- behavior hooks, starting with one: extended AI trigger conditions;
- mod checks for multiplayer, saves and replays.

Out of scope:

- resisting memory dumps or purpose-built extraction tools (Minecraft does not
  either);
- changing the retail read paths or rules semantics;
- shipping retail files in a package; players still bring their own game files.

A mod is one more high-priority asset source plus a set of hooks. Without mods the
engine behaves exactly as it does today.

## Package format

A `.vmod` file has five sections. Only the protected section is WASM, split into
two modules: `content.wasm` runs once at mount and is then released, and
`logic.wasm` stays resident for the match, so decoded assets do not hold memory.

| Section | Contents | Stored as | How the engine reads it |
| --- | --- | --- | --- |
| Header | `VMOD` magic, format version, section offsets | plain | parsed at mount |
| Manifest | id, version, interface version, dependencies, author public key, digests of both data sections | plain TOML | validated, dependency-sorted |
| Protected | `content.wasm`: INI generator and core assets; `logic.wasm`: behavior hooks | data segments compressed, then scrambled with a key generated per build | `content.wasm` runs once to produce entries; `logic.wasm` stays resident |
| Plain | non-core assets | MIX-style index plus uncompressed body; index scrambled with the package key | archive built directly over the file; memory-mapped on desktop |
| Signature | Ed25519 over every preceding byte | plain | verified before mount |

The package stores no file names. Every entry is keyed by the MIX name hash the
engine looks files up with (`mix_hash`). The plain section is not meant to be
protected; the scrambled index only stops XCC from opening it directly.

## Asset tiers

Small, mod-specific assets go into WASM; large, generic ones go into the plain
section. INI always goes through the generator; authors can move everything else
with wildcards.

| Asset | Default section | Why |
| --- | --- | --- |
| All INI: rules, art, ai, sound, eva, theme, mode INIs | protected (generator), fixed | the mod's core design |
| VXL, HVA | protected | new vehicle models, small and mod-specific |
| SHP for new units, buildings and infantry | protected | mod-specific art |
| Cameos (SHP/PCX) and palettes used only by those assets | protected | small |
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

The protected section gets a soft limit, suggested at 64 MB compressed until the
browser build is measured. The packer warns past it but does not refuse. The limit
exists because this section is decoded in one pass at mount, and the browser build
is bound by wasm32 memory.

## INI generator

The packer compiles the INI files into `content.wasm`. At mount it emits each INI
file's complete bytes into the protected section's entry table, and the engine
reads them by file name as usual. The native readers and rules layering do not
change.

It emits INI text rather than parsed values because:

- the native readers own many details: the 511-byte line cut, `;` comments, `Key=`
  read as absent, case-sensitive keys, the first of duplicate keys winning;
- map and mode INIs override a mod's rules key by key, so those rules must enter
  the same layer stack;
- `RuleSet`'s internal layout changes between engine versions; text survives an
  engine update without repacking.

Normalization:

1. Split lines the native way first: LF ends a line, CR is dropped, NUL ends the
   line, and only the first 511 bytes are kept. The readers then see exactly the
   values of the original file.
2. Then drop comments, blank lines and leading and trailing whitespace.
3. Keep every key, including keys the engine does not read yet, so Ares and Phobos
   keys ported later still find their values.
4. Keep section and key order; registries allocate types in read order.

File names stay the same (`rulesmd.ini`, `artmd.ini`, `aimd.ini` and so on), so
`[ini] profile` and the RULESMD → LANGRULE → mode INI → map layering keep working.
A mod's `rulesmd.ini` replaces the retail one by archive priority, the same way an
INI placed in `expandmd99.mix` does today.

## Engine loading

Each vmod mounts as two archives ahead of every retail mix and `expandmd`, once, at
process start.

```
spawn.ini [Mods] -> verify signature -> sort by dependency
  |- protected: run content.wasm once -> entries -> in-memory MixArchive
  |- plain:     index -> MixArchive over a subrange of the file
  `- logic.wasm: wasmi instance, resident -> hook call sites

AssetManager archives (highest first):
  vmod protected > vmod plain > langmd.mix > ... > expandmd99..00 > ra2md.mix > ra2.mix ...
    -> existing readers, unchanged: INI layers, SHP/VXL/HVA/TMP parsers,
       atlases, simulation asset catalogs
```

- **Handoff.** The launcher adds a `[Mods]` section to `spawn.ini` with each mod's
  id, version and digest, already dependency-sorted. `open_startup_archives` mounts
  the vmods before `langmd.mix`. Archives mounted first take priority, so vmods win.
- **Two archives.**
    - The entries decoded from the protected section become an in-memory
      `MixArchive`. This needs a constructor that takes entries;
      `MixArchive::from_bytes` already builds an archive from memory.
    - The plain section becomes a `MixArchive` over a `MixData` subrange: a memory
      map of the file on desktop, the whole file read into memory in the browser.
- **Loose files.** Retail lets loose files in the game directory win over every mix
  (`CCFileClass::Open` checks disk before mix, `0x00473D10`). With vmods enabled,
  names a vmod provides win over loose files instead; otherwise an old
  `rulesmd.ini` in the player's directory would override the mod. This is a
  deliberate deviation from retail order, behind VeraX.
- **Lifetime.** The `LoadFileFromMIX` cache (`0x005B40B0`) lives for the whole
  process and keeps its first hit, so mods mount only at startup and cannot be
  swapped mid-process. Desktop already starts a new process per match
  (`vera20k -SPAWN`), and the browser build already needs a page reload before the
  next match.
- **Downstream is unchanged.** SHP/VXL/TMP parsers, atlases and the asset metadata
  the simulation uses (SHP frame counts, TMP heights and terrain types, VXL
  admission) stay as they are.

## WASM runtime and determinism

The first version uses wasmi, a pure-Rust interpreter, on desktop and in the
browser. The browser build compiles it into the engine's own wasm, so both
platforms run one implementation, which keeps determinism simple.

- **Why not wasmtime or the browser's WebAssembly.** wasmtime is large and brings a
  JIT. The browser's engine is reached through JS glue, every worker of the
  wasm-threads build would need its own instance, and two implementations may
  behave differently. The first version's calls are rare (once at mount, then
  when an AI team timer expires), so an interpreter is fast enough; a JIT gets
  evaluated if a high-frequency hook arrives.
- **Instruction checks.** The packer and the engine both reject float, SIMD
  (relaxed SIMD included) and thread/atomic instructions, and allow only imports
  from the interface list.
- **Numbers.** The interface passes only integers and `SimFixed` fixed-point
  values, matching the engine's simulation math rules.
- **Limits.** Memory pages are capped and every hook call has a fuel budget. wasmi
  counts fuel per instruction, so every client gets the same result; running out
  of fuel falls back to the hook's default and is logged.
- **State.** `logic.wasm` keeps no state in its own memory between calls; its
  memory is reset to the initial snapshot before each call. State it needs goes
  into a key-value store the engine provides. The simulation owns that store, so it
  enters save snapshots (`src/sim/snapshot.rs`) and the sync hash
  (`src/sim/world/world_hash.rs`).
- **Randomness.** Only from the match's random stream, through the interface; a
  mod has no random source of its own.

## Behavior hooks

The first version opens one hook: extended AI trigger conditions. It runs rarely
and the change sits in one function, enough to test the runtime, saves, the sync
hash and cross-platform agreement end to end.

- **Where.** `condition_holds` (`src/sim/ai_team_creation.rs`) accepts the eight
  conditions -1 to 7; any other number fails. Condition numbers of 1000 and above
  go to mods: the engine finds the `logic.wasm` export registered for that number in
  the manifest.
- **Signature.** `vx_ai_condition(code, trigger, owner, enemy) -> i32`, returning 0
  for false and 1 for true. The arguments are engine-assigned handles.
- **Retail stays.** The eight retail conditions are unchanged; without mods this
  path is exactly what it is today.

First interface set:

| Interface | Returns or does |
| --- | --- |
| `house_money(house)` | available money |
| `house_power(house)` | power output and drain |
| `house_count(house, type)` | live count of a type |
| `super_ready(house, kind)` | whether a superweapon is nearly ready |
| `type_id(name)` | type handle |
| `rand(lo, hi)` | match random number |
| `kv_get(key)` / `kv_set(key, value)` | read and write the simulation key-value store |

Later hooks follow the same pattern: defeat checks (`game_alive`), map trigger
events and actions, script actions, weapon and warhead effects. The engine's
default implementation always stays; mods take over only at points they register.

## Multiplayer, saves and replays

The mod list and its digests are match inputs: a room cannot start when they
differ, and saves and replays open only with the same mods.

- **Rooms.** The host picks mods; the room carries each mod's id, version and
  digest. A client downloads missing mods from the mod library and is ready only
  after the digest checks out.
- **Handshake.** The mod digest list joins the rules compatibility data, compared
  with the rules and map digests. Today `RulesCompatibility`
  (`src/rules/compatibility.rs`) only distinguishes compiled capability sets.
- **Rules digest.** The generator emits ordinary INI files, so the existing rules
  digest should need no change; confirm that `multiplayer_rules_digest` reads the
  files after asset resolution.
- **Saves and replays.** Their metadata records the mod list and digests. A missing
  or mismatched mod refuses the load and points to the right version.
- **Sync hash.** Hooks act through engine state and the key-value store is in
  `world_hash`, so nothing extra is needed.

## Packer: `vmod build`

`vmod build` is a command-line tool that turns a mod project directory into a
signed `.vmod`. Compilation and checks run on the author's machine; the engine
only accepts the result. It links `vera20k-core` and uses the engine's parsers.

1. Read `mod.toml`: id, version, dependencies, protection rules.
2. Check the INI files with the engine's parser and report referenced types and
   assets that do not exist.
3. Normalize the INI files: native line splitting first, then comments and
   whitespace; every key and the order stay.
4. Sort assets by the protection rules and replace file names with MIX name hashes.
5. Protected part: compress, scramble with a per-build random key, and emit
   `content.wasm` with its decoder.
6. Compile the logic sources (Rust SDK, `wasm32` target), check the instruction set
   and import list, and emit `logic.wasm`.
7. Write the plain section: MIX-style index and body, index scrambled.
8. Write the manifest with both digests and sign it with the author's private key.

A companion `vmod inspect` shows only the manifest and digests, not the contents,
for authors and the mod library to check against.

## Signing and security

Signatures answer who published a package and whether it changed; the sandbox
limits what mod code can do. Neither prevents extraction.

- **Author keys.** Authors generate an Ed25519 key pair locally and register the
  public key with the mod library, tied to their VERA20K account.
- **Verification.** The engine verifies the signature and both digests before
  mounting. Ranked play admits only public keys and digests registered with the
  mod library.
- **Reuploads.** The mod library spots duplicate uploads by package digest and
  author key.
- **Sandbox.** WASM can call only the listed imports; it has no file, network or
  clock access. Memory pages and per-call fuel are capped.
- **Automatic downloads.** A mod downloaded for a room is code too: verify the
  signature, then check the instruction set, and only then instantiate it.

## Phases and acceptance

1. **Container and mounting.** vmod format, signatures, plain-section mounting, INI
   generator and protected assets (`content.wasm`).
   Gate: the same mod packed as a vmod and as an `expandmd` gives the same rules
   digest and the same picture in a match.
2. **Runtime and the first hook.** wasmi on desktop and in the browser, instruction
   checks, fuel, key-value store; extended AI trigger conditions (numbers ≥ 1000).
   Gate: a mod with hooks stays in sync in a desktop-versus-browser match, and a
   save reloads to the same result.
3. **Multiplayer and distribution.** Mod list in rooms and the handshake, automatic
   downloads; saves and replays record mods; the mod library registers author keys.
   Gate: a player without the mod gets it on joining; mismatched versions cannot
   start.
4. **More interfaces and tools.** Defeat checks, map triggers, script actions,
   weapon effects; Rust SDK, packer docs, a sample mod.

After phase 3, vmods with hooks work in multiplayer rooms; phase 4 grows as needed.
The first two phases sit behind the VeraX capability and leave retail builds
unaffected.

## Risks and open questions

The main accepted limits: decoded INI and assets are in memory after mount, and a
GPL fork can run every vmod. Both match Minecraft's level.

Risks:

- **Memory dumps.** Decoded INI and assets can be taken after mount. That takes a
  purpose-built tool, the same level as decompiling a jar.
- **GPL forks.** A fork has every interface. Nothing technical stops that; the
  lobby, accounts, ladder and mod library are where VERA competes.
- **Interpreter speed.** A high-frequency hook such as damage resolution may be too
  slow under wasmi; it needs a 20,000-unit stress run first.
- **Name enumeration.** Features that enumerate file names, such as the map list
  and the `RULEMD*.INI` picker, do not work with hashed names. First-version mods
  carry no maps.
- **Interface versions.** INI keeps every key, so engine updates need no repack, but
  `logic.wasm` depends on the interface version, which must stay backward
  compatible.

Open questions:

- [ ] With vmods enabled, should vmod files win over loose files in the game
      directory?
- [ ] What soft limit for the protected section? Needs a browser measurement.
- [ ] Does `multiplayer_rules_digest` read the generated INI?
- [ ] Where does the mod library live: an extension of the ra20k.com map library,
      or separate?
