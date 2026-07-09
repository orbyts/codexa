---
schema: codexa.document@1
id: workflows.playbooks.lureva-lightroom-handoff
title: Lureva Lightroom Handoff Manual
description: Daily Lightroom Classic handoff workflow for switching between quasar and eclipse during the Lureva 960 review.
kind: playbook
status: active
visibility: private

tags:
  - lureva
  - lightroom
  - nazariya
  - runbook

distribution:
  notion: true
  web: private

notion:
  workspace: codexa
  data_source: documents
---


# Lureva Lightroom Workflow Manual

## Quick steps instructions

This is the fast daily workflow for switching between **quasar** and **eclipse** while working on the Lureva 960 Lightroom review.

### 1. Before leaving the current Mac

Quit Lightroom Classic completely.

Then pack the current `Lr_Proetus` catalog database to Desktop:

```bash
src="$HOME/Pictures/Lightroom/Proetus/Lr_Proetus"
out="$HOME/Desktop/Lr_Proetus_catalog_transfer_$(hostname -s)_$(date +%Y%m%d-%H%M%S).tar.gz"

tar \
  --exclude='*.lrcat-lock' \
  --exclude='*.lrcat-wal' \
  --exclude='*.lrcat-shm' \
  --exclude='* Previews.lrdata' \
  --exclude='* Smart Previews.lrdata' \
  --exclude='* Helper.lrdata' \
  -czf "$out" \
  -C "$(dirname "$src")" \
  "$(basename "$src")"

shasum -a 256 "$out" > "$out.sha256"

echo "$out"
```

Wait until the `.tar.gz` and `.sha256` files have synced to Desktop on the other Mac.

### 2. On the next Mac

Verify the archive checksum and unpack it:

```bash
cd "$HOME/Desktop"

latest="$(ls -t Lr_Proetus_catalog_transfer_*.tar.gz | head -1)"
shasum -a 256 -c "$latest.sha256"

mkdir -p "$HOME/Pictures/Lightroom/Proetus"
rm -rf "$HOME/Pictures/Lightroom/Proetus/Lr_Proetus"
tar -xzf "$latest" -C "$HOME/Pictures/Lightroom/Proetus"
```

Open this catalog in Lightroom Classic:

```text
~/Pictures/Lightroom/Proetus/Lr_Proetus/Lr_Proetus.lrcat
```

### 3. Sanity-check paths

Open a fresh terminal and run:

```bash
echo "$WHISK"
echo "$WHISK_ML_DATASETS"
echo "$PROETUS_IMAGES_ROOT"
echo "$LUREVA_DATA_ROOT"

test -d "$PROETUS_IMAGES_ROOT" && echo "images ok"
test -d "$LUREVA_DATA_ROOT" && echo "lureva data ok"
```

Expected image root:

```text
/Volumes/whisk/work/ml/datasets/proetus/images
```

or the equivalent Whisk mount path on that Mac.

### 4. Make sure the Nazariya plugin is installed

One-time setup on each Mac:

```bash
mkdir -p "$HOME/Library/Application Support/Adobe/Lightroom/Modules"

ln -sfn \
  "$MATRIX/packages/nazariya/lightroom/nazariya.lrplugin" \
  "$HOME/Library/Application Support/Adobe/Lightroom/Modules/nazariya.lrplugin"
```

In Lightroom Classic:

```text
File → Plug-in Manager… → reload or enable Nazariya
```

The menu items should appear under:

```text
Library → Plug-in Extras
```

### 5. Run the next batch

From the Nazariya repo:

```bash
cd "$MATRIX/packages/nazariya"
```

For batch 2:

```bash
rm -rf data/lureva/runs/image-batch-02-v0.1.0

./scripts/nazariya lureva process-image-batch \
  --sample-run sampled-pools-v0.1.0 \
  --batch 2 \
  --primary-count 20 \
  --alternate-count 20 \
  --close-reference-count 5 \
  --run-id image-batch-02-v0.1.0
```

Then in Lightroom:

```text
Library → Plug-in Extras → Lureva: Apply Batch Assignments
```

Select:

```text
data/lureva/runs/image-batch-02-v0.1.0/batch_02_assignments.csv
```

Expected Lightroom structure for batch 2:

```text
projects
└── lureva
    └── Lureva 960 Review v0.1.0
        ├── Batches
        │   └── batch_02
        │       └── batch_02 - All
        └── Groups
            ├── u009
            ├── u010
            ├── ...
            └── u016
```

### 6. Review keyboard shortcuts

```text
P = Pick / final keep
U = Unflag / remove from final but keep visible as alternate
X = Reject / definitely bad
G = Grid view
E = Loupe view
N = Survey view
C = Compare view
Space = zoom in/out
Arrow keys = move through images
```

Review rule:

```text
Each u### group must have exactly 20 Picked images.
```

The group collection is the source of truth:

```text
Groups → u001
Groups → u002
...
Groups → u048
```

The batch collection is only a convenience view.

---

## Full setup instructions

### 1. What is shared and what is local

Shared/synced:

```text
Nazariya repo/plugin     → Dropbox / $MATRIX/packages/nazariya
Lureva run CSVs/manifests → repo data/lureva/runs
RAW archive              → Whisk
```

Local/handoff:

```text
Lightroom catalog database → Lr_Proetus.lrcat / Lr_Proetus.lrcat-data
Lightroom previews         → local and disposable
```

Do not copy RAW files between Macs. Both Macs should see the RAW archive from Whisk.

Do not sync active Lightroom catalogs through iCloud/Dropbox while Lightroom is open. Treat the catalog as a handoff file.

### 2. Why the working catalog is `Lr_Proetus`

For the Lureva 960 selection workflow, use:

```text
Lr_Proetus.lrcat
```

Reason: `Lr_Proetus` sees the full RAW archive, which is needed for selecting new 960 images.

The smaller `Lr_lureva` catalog only sees the DNG versions of the 560 edited reference images and is not enough for selection of the new RAW candidates.

### 3. Standard catalog location

Use the same local path on both Macs:

```text
~/Pictures/Lightroom/Proetus/Lr_Proetus/Lr_Proetus.lrcat
```

Folder:

```text
~/Pictures/Lightroom/Proetus/Lr_Proetus
```

Files that matter:

```text
Lr_Proetus.lrcat
Lr_Proetus.lrcat-data
```

Files/folders that can be skipped during handoff:

```text
*.lrcat-lock
*.lrcat-wal
*.lrcat-shm
* Previews.lrdata
* Smart Previews.lrdata
* Helper.lrdata
```

Lightroom can rebuild previews locally.

### 4. Apogee environment expectations

The Proetus image root should now point to Whisk:

```bash
echo "$PROETUS_IMAGES_ROOT"
```

Expected:

```text
/Volumes/whisk/work/ml/datasets/proetus/images
```

Relevant environment variables:

```text
WHISK
WHISK_ML_DATASETS
PROETUS_IMAGES_ROOT
LUREVA_DATA_ROOT
MATRIX
```

Typical checks:

```bash
echo "$WHISK"
echo "$WHISK_ML_DATASETS"
echo "$PROETUS_IMAGES_ROOT"
echo "$LUREVA_DATA_ROOT"

test -d "$PROETUS_IMAGES_ROOT" && echo "images ok"
test -d "$LUREVA_DATA_ROOT" && echo "lureva data ok"
```

### 5. Apogee Proetus config shape

The Proetus module should have only one `[modules.cloud.proetus.emit.env]` table and only one quasar host override.

Expected shape:

```toml
[modules.cloud.proetus.emit.env]
PROETUS_REPO = "{detect.path}"
PROETUS_SCRATCH_ROOT = "{home}/Documents/Scratch/tmp/Proetus"
PROETUS_IMAGES_ROOT  = "$WHISK_ML_DATASETS/proetus/images"

[modules.cloud.proetus.emit.env_hosts.quasar]
PROETUS_SCRATCH_ROOT = "{home}/Documents/Scratch/tmp/Proetus"
PROETUS_IMAGES_ROOT  = "$WHISK_ML_DATASETS/proetus/images"

[modules.cloud.proetus.emit.env_hosts.eclipse]
PROETUS_SCRATCH_ROOT = "{home}/Documents/Scratch/tmp/Proetus"
PROETUS_IMAGES_ROOT  = "$WHISK_ML_DATASETS/proetus/images"
```

Duplicate TOML tables will break apogee startup.

Check duplicates:

```bash
cd ~/.config
rg -n '^\[modules\.cloud\.proetus\.emit\.env\]' apogee/config.toml
rg -n '^\[modules\.cloud\.proetus\.emit\.env_hosts\.quasar\]' apogee/config.toml
rg -n '^\[modules\.cloud\.proetus\.emit\.env_hosts\.eclipse\]' apogee/config.toml
```

Each should print exactly one line.

---

## Lightroom collection organization

The canonical review structure is:

```text
Collections
└── projects
    └── lureva
        └── Lureva 960 Review v0.1.0
            ├── Batches
            │   ├── batch_01
            │   │   └── batch_01 - All
            │   ├── batch_02
            │   │   └── batch_02 - All
            │   └── ...
            └── Groups
                ├── u001
                ├── u002
                ├── ...
                └── u048
```

Meaning:

```text
Batches → convenience for loading/reviewing chunks
Groups  → source of truth for final picks
```

A manually added image counts if it is in the correct `u###` group collection and marked Pick.

The final rule remains:

```text
48 groups × 20 Picked images = 960 final images
```

---

## Batch commands reference

### Batch 1

```bash
cd "$MATRIX/packages/nazariya"

./scripts/nazariya lureva process-image-batch \
  --sample-run sampled-pools-v0.1.0 \
  --batch 1 \
  --primary-count 20 \
  --alternate-count 20 \
  --close-reference-count 5 \
  --run-id image-batch-01-v0.1.0
```

Apply in Lightroom:

```text
data/lureva/runs/image-batch-01-v0.1.0/batch_01_assignments.csv
```

### Batch 2

```bash
cd "$MATRIX/packages/nazariya"

./scripts/nazariya lureva process-image-batch \
  --sample-run sampled-pools-v0.1.0 \
  --batch 2 \
  --primary-count 20 \
  --alternate-count 20 \
  --close-reference-count 5 \
  --run-id image-batch-02-v0.1.0
```

Apply in Lightroom:

```text
data/lureva/runs/image-batch-02-v0.1.0/batch_02_assignments.csv
```

### Batch number to group mapping

With batch size 8:

```text
batch_01 → u001–u008
batch_02 → u009–u016
batch_03 → u017–u024
batch_04 → u025–u032
batch_05 → u033–u040
batch_06 → u041–u048
```

---

## Re-sampling reference

The current preferred sampling approach is 80 sampled candidates per group, then 40 Lightroom assignments per group:

```text
20 primaries + 20 alternates = 40 images per group in Lightroom
```

Sample pool command:

```bash
cd "$MATRIX/packages/nazariya"

./scripts/nazariya lureva sample-image-pools \
  --pool-run image-pools-v0.1.0 \
  --max-per-group 80 \
  --strategy temporal-spread \
  --seed 42 \
  --batch-size 8 \
  --run-id sampled-pools-v0.1.0
```

Only rerun this if you intentionally want to rebuild the sampled pools.

---

## Optional shell helpers

Add these later to apogee for faster daily handoff.

### Pack current catalog

```bash
lureva_pack_catalog() {
  local src="$HOME/Pictures/Lightroom/Proetus/Lr_Proetus"
  local out="$HOME/Desktop/Lr_Proetus_catalog_transfer_$(hostname -s)_$(date +%Y%m%d-%H%M%S).tar.gz"

  tar \
    --exclude='*.lrcat-lock' \
    --exclude='*.lrcat-wal' \
    --exclude='*.lrcat-shm' \
    --exclude='* Previews.lrdata' \
    --exclude='* Smart Previews.lrdata' \
    --exclude='* Helper.lrdata' \
    -czf "$out" \
    -C "$(dirname "$src")" \
    "$(basename "$src")"

  shasum -a 256 "$out" > "$out.sha256"
  echo "$out"
}
```

### Unpack latest synced catalog

```bash
lureva_unpack_latest_catalog() {
  local latest
  cd "$HOME/Desktop" || return 1

  latest="$(ls -t Lr_Proetus_catalog_transfer_*.tar.gz | head -1)"
  test -n "$latest" || { echo "No catalog transfer archive found"; return 1; }

  shasum -a 256 -c "$latest.sha256" || return 1

  mkdir -p "$HOME/Pictures/Lightroom/Proetus"
  rm -rf "$HOME/Pictures/Lightroom/Proetus/Lr_Proetus"
  tar -xzf "$latest" -C "$HOME/Pictures/Lightroom/Proetus"

  echo "$HOME/Pictures/Lightroom/Proetus/Lr_Proetus/Lr_Proetus.lrcat"
}
```

---

## Safety rules

1. Never open/edit the same Lightroom catalog on both Macs at once.
2. Always quit Lightroom before packing the catalog.
3. Only transfer `.lrcat` and `.lrcat-data`; skip previews.
4. Keep RAW files on Whisk, not copied between Macs.
5. Keep the Nazariya plugin loaded from the repo symlink.
6. The `u###` group collections are the source of truth.
7. Final target is exactly 20 Picks per group, 960 Picks total.

