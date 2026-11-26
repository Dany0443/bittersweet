# Implementation tasks

## 1. Complete the memory-import probe verification

Acceptance: GPU writes through the probe helper are read correctly by the
CPU; private and mapped memory results are distinguished. Verify with the
focused Windows-target GPU test and the runtime probe. Dependencies: none.
Scope: small, inplace.rs and provenance. Helper fix is committed; runtime
import coherence must be rechecked with the rebuilt executable.

## 2. Port packed-byte shader arithmetic

Acceptance: signed packed dot-product accumulation matches synthetic CPU
results, including negative operands and wrapping. Verify portable lowering
and Windows-target GPU readback. Dependencies: none. Scope: small, lower.rs
and focused tests. Attribute the new cool_fox source identity.

## 3. Translate root constant buffers as raw bytes

Acceptance: the opt-in path preserves constant values and root bindings;
disabled output remains unchanged; cache keys distinguish the setting.
Verify aligned and offset synthetic reads on the GPU. Dependencies: none.
Scope: medium, shader state, lowering, cache and tests.

## 4. Supply bounded raw descriptor twins

Acceptance: table and heap raw reads see the original view bytes, preserve
bounds and use a safe fallback when host heap capacity is insufficient.
Verify descriptor creation and synthetic GPU reads before enabling title
execution. Dependencies: task 3. Scope: medium, descriptor host, shader
handle creation and tests.

## 5. Complete raw constant-buffer integration

Acceptance: table and heap constant reads match the original path; runtime
views and cache identity follow the shader setting; a title comparison
reports copy counts, memory use and frame times. Dependencies: tasks 3 and
4. Scope: medium, lowering, draw bindings, descriptor host and tests.

## 6. Evaluate indexed write-watch coverage

Acceptance: coverage answers match the existing linear reference for
overlapping and touching original queries; local measurements establish
lookup cost and title behavior. Inspect unmerged cool_fox PR 5 at its
current head before porting. Dependencies: none. Scope: small, watch.rs and
focused tests. Do not infer frame-rate gains from a microbenchmark alone.

## 7. Verify memory placement before enabling it

Acceptance: imported private memory preserves CPU/GPU writes and lifetime
rules; unsupported mapped sections retain copies; bounded title tests show
no GPU timeout or stale data. Dependencies: task 1. Scope: medium, placement,
copy policy and tests. Measure paging and copy costs after each change.

## 8. Adapt video synchronization from the new source

Acceptance: decoded frames are submitted in order and remain synchronized
without scripted input during the cutscene. Inspect the new archive's
video-copy and recording dependencies before porting. Dependencies: none.
Scope: split into small runtime changes as needed; preserve timing policy.

## 9. Verify the requested end state

Acceptance: the first mission is visible and responds to player input;
rendered frame times demonstrate sustained 20 to 30 fps during control;
UI and intro timing are verified in the same supported configuration.
Dependencies: the relevant rendering and performance tasks above. Run
focused tests, policy, workspace tests, Windows release build and relevant
format checks. Record remaining inherited check failures accurately.

## Checkpoints

- [ ] Shader/runtime/cache changes are reviewed together after each slice.
- [ ] Every completed feature has a verified maintainer-authored commit.
- [ ] Local title evidence supports every Linux status claim.
- [ ] The full playable-mission goal remains open until task 9 is proven.
