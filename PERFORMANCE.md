# Performance Improvement Opportunities

## Checklist

- [x] **M** — `model_space_extents()` epoch cache + wire AABB shortcut (`src/scene/mod.rs`)
- [x] **N** — `entity_wires_arc()` returns `paper_sheet_wires_arc()` directly in model space, no Vec clone (`src/scene/mod.rs`)
- [x] **L** — ApparentIntersection snap: pre-project screen coords per wire before pair loops (`src/snap/mod.rs:483`)
- [x] **O** — Intersection snap: replace loose `r_world * 60.0` midpoint cull with tight per-segment AABB overlap check (`src/snap/mod.rs:412`)
- [x] **S** — Intersection snap: move `a0/a1` Vec3 conversion outside inner `seg_b` loop (`src/snap/mod.rs:420`)
- [x] **P** — `belongs_to_visible_block()`: precompute entity→block reverse map; replace O(B) scan with O(1) HashMap lookup (`src/scene/mod.rs:782`)
- [x] **Q** — Hit test `click_hit/box_hit/poly_hit`: eliminate per-wire `Vec<Point>` allocation; iterate lazily with NaN reset (`src/scene/hit_test.rs:35`)
- [x] **R** — `HatchModel.boundary`: change `Vec<[f32;2]>` → `Arc<Vec<[f32;2]>>` so `HatchModel::clone()` is a pointer bump (`src/scene/hatch_model.rs`)

---

## Details

### L — ApparentIntersection snap: redundant world_to_screen per pair
**File:** `src/snap/mod.rs:483–506`

For every segment pair tested, 4 `world_to_screen()` calls (matrix multiply + perspective divide)
are made just to get 2D coordinates for an intersection test. If no intersection is found, all 4
projections are wasted. Dense drawings → O(N²M²) matrix multiplies per snap event.

**Fix:** Pre-project all segment endpoints for each in-range wire into a `Vec<Point>` before
entering the pair loops. Each endpoint is then projected once regardless of how many pairs it
participates in.

---

### M — model_space_extents() re-tessellates everything
**File:** `src/scene/mod.rs:823`

`model_space_extents()` called tessellate_one() for every entity on every ZOOM E / auto-fit.
No cached result was used — geometry rebuilt from scratch each call.

**Fix:** Epoch-keyed cache (`model_extents_cache`). On cache hit: O(1). On miss with wire cache
available: union wire AABBs (no tessellation). Fallback to tessellate_one() only on first call
or when wire cache is not for model space.

---

### N — paper_canvas_wires() defeats Arc cache with full Vec clone
**File:** `src/scene/mod.rs:582`

In model space, `entity_wires_arc()` cloned the entire `paper_sheet_wires_arc()` Vec just to
wrap it in a new Arc. The Arc caching was bypassed on every cache miss.

**Fix:** For model space, return `paper_sheet_wires_arc()` directly — the two caches share the
same Arc. Clone only needed in paper space where viewport-content wires must be appended.

---

### O — Intersection snap O(N²·M²) segment pair testing
**File:** `src/snap/mod.rs:412–441`

The segment-level cull inside the intersection pair loop uses `r_world * 60.0` midpoint
distance, which is extremely loose and passes most segments. Every pair then runs
`seg_intersect_xy()` (cross-product math).

**Fix:** Replace midpoint cull with tight AABB overlap check per segment pair: compute
`[min_x, max_x, min_y, max_y]` for each segment and skip if the two AABBs don't overlap.
Combined with the fix in S (pre-converted Vec3), this eliminates most cross-product calls.

---

### P — belongs_to_visible_block() O(B) scan repeated per entity
**File:** `src/scene/mod.rs:782–787`

The fallback path (null owner_handle, entity_handles not populated) iterates all `block_records`
to confirm an entity is not in any other block. Called once per entity per epoch rebuild.
O(E × B) total where E = entity count, B = block count.

**Fix:** Epoch-keyed `entity_block_map_cache: HashMap<Handle, Handle>` built from
`block_records[*].entity_handles`. Fallback becomes a single `HashMap::contains_key` lookup.

---

### Q — Hit testing: linear scan, no spatial index
**File:** `src/scene/hit_test.rs:35`

`click_hit()`, `box_hit()`, `poly_hit()` allocate a `Vec<Point>` for every wire by eagerly
projecting all points to screen space, even for wires far from the cursor/box. O(E × S) with
MB-scale allocation churn on large drawings.

**Fix:** Iterate lazily — project one point at a time, maintain a `prev: Option<Point>`,
test segment, advance. NaN points reset `prev`. Eliminates the per-wire Vec allocation entirely.

---

### R — synced_hatch_models(): unconditional HatchModel clone per epoch
**File:** `src/scene/hatch_model.rs`, `src/scene/mod.rs:1572`

Every visible hatch is cloned in full (boundary `Vec<[f32;2]>` + pattern enum + name String)
on every geometry epoch rebuild, even when only `angle_offset`, `scale`, or `color` changes.

**Fix:** Change `HatchModel.boundary` to `Arc<Vec<[f32;2]>>`. `HatchModel::clone()` then
copies only the Arc pointer (8 bytes) for the boundary instead of heap-allocating a new Vec.

---

### S — Intersection snap: repeated Vec3 endpoint copies in inner loop
**File:** `src/snap/mod.rs:420–423`

`a0 = Vec3::from(seg_a[0])` and `a1 = Vec3::from(seg_a[1])` are recomputed for every
iteration of the inner `seg_b` loop.

**Fix:** Move `a0` and `a1` conversions outside the inner loop.
