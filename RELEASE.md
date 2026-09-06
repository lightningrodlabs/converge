# Releasing Converge

Converge is distributed as a `.webhapp` referenced by the
[weave-tool-curation](https://github.com/lightningrodlabs/weave-tool-curation)
list. A release is **UI-only**: it bundles the current UI with the *exact same*
frozen happ as every previous release on the same line, so all installs stay on
the same DNA / network and existing users' data is preserved.

## The 0.7 line is a new network

This branch targets Holochain 0.7, which has no data migration path from 0.6:
the DNA hash changed, 0.6 and 0.7 conductors cannot read each other's databases,
and 0.6 and 0.7 agents form disjoint networks. Deliberations, criteria, proposals
and outcomes created on the 0.6 line (Converge 0.3.x, published as `0.3.2`) do
**not** carry over, and a group has to move across together — a 0.7 agent cannot
see 0.6 peers at all.

**Converge has no export/import feature**, so there is no way to carry content
across. Anyone upgrading starts from an empty deliberation list, and the 0.6
instance has to be kept installed if the old content still matters. Say so in the
curation changelog.

## Why the happ is frozen (never rebuilt)

The zome wasm embeds the builder's absolute paths (`~/.cargo/...` and source
paths via the HDK macros). That makes the happ **non-reproducible** on a
different machine/user or in CI — a rebuild produces a different DNA hash, i.e. a
different network. The happ is built once and those exact bytes are reused
forever.

The canonical bytes live as the `happ-v<dnaVersion>` GitHub release (tag in
`.happ-version`); their sha256 is recorded in `.happ-sha256` and checked by both
`scripts/release-happ.sh` and the release workflow.

> ⚠️ Do **not** release by uploading the output of `npm run package`. That
> rebuilds the happ locally (your paths → wrong DNA → a forked network).
> Releases must go through the tag-triggered workflow below.

## One-time per DNA version: publish the canonical happ

```bash
nix develop --command bash -c "npm run build:happ:release"
nix develop --command bash scripts/release-happ.sh
```

`build:happ:release` is the canonical build: it runs `scripts/wasm-opt-zomes.mjs`
over every zome named in `dnas/converge/workdir/dna.yaml` (`wasm-opt -Oz
--strip-debug --strip-producers`) before packing. Plain `build:happ` skips that
and therefore produces a **different DNA hash** — a different network. That is
intentional: dev builds must never join the canonical network. Only
`build:happ:release` output may be published.

The script verifies `workdir/converge.happ` against `.happ-sha256` and
creates/updates the `happ-v<dnaVersion>` release. It also accepts the URL of an
already-published `.webhapp` as its first argument, to recover the exact bytes of
a line that is already live.

This must be done **before the first 0.7 webhapp release**. If the local build
does not match `.happ-sha256` (a different machine, a `cargo clean`, a dependency
bump), the script refuses and tells you what to do: that mismatch is the tripwire
that stops a silent network fork. Deliberately starting a new DNA line means
bumping `dnaVersion` in `ui/package.json`, updating `.happ-version`, and writing
the new sha into `.happ-sha256`.

## Each release: cut a webhapp

1. Bump `version` in `ui/package.json` (must be higher than the installed
   version for Moss to offer it as an upgrade).
2. Commit, then:

   ```bash
   npm run release:webhapp        # tags v<version> and pushes
   ```

   Note the `v` prefix. Converge's pre-0.7 releases were tagged **without** it
   (`0.3.2`, `0.3.1`, …), and there are also branches literally named `0.4` and
   `0.4.0`. `release-webhapp.yaml` triggers on `v*`, so from the 0.7 line on the
   tags are `v0.4.0`, `v0.4.1`, … — a deliberate change of convention, and the
   reason the old unprefixed tags do not fire the workflow.

3. The [`release-webhapp`](.github/workflows/release-webhapp.yaml) workflow then:
   - downloads the frozen happ from `happ-v<dnaVersion>` and checks its sha256
     against `.happ-sha256`,
   - builds the UI and packs `converge.webhapp` (no `--recursive`, so the happ
     is embedded verbatim — never rebuilt),
   - re-verifies the embedded happ still equals the frozen DNA,
   - prints the three curation hashes to the run summary,
   - publishes a **prerelease** GitHub release with `converge.webhapp` attached.
      It is deliberately not a draft: draft assets are not served at the public
      `releases/download/<tag>/...` URL Moss fetches, so they 404.
4. Nothing is live yet — updating the curation list below is the go-live gate.

## Update the curation list

The workflow run summary (and the release body) contains:

```json
"hashes": {
  "happSha256": "<frozen>",         // always the frozen DNA, from .happ-sha256
  "webhappSha256": "<new>",
  "uiSha256": "<new>"
}
```

Add a new `versions[]` entry for `converge` in the 0.16 curation list with the
new `version`, the release's `converge.webhapp` `url`, and these hashes. Because
`happSha256` is unchanged, Moss treats it as an in-place upgrade on the same
network. To get the hashes for an artifact locally: `npm run weave-hash`.
