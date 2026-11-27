# Implementation tasks

## 1. Complete the memory-import probe verification

Acceptance: GPU writes through the probe helper are read correctly by the
CPU; private and mapped memory results are distinguished. Verify with the
focused Windows-target GPU test and the runtime probe. Dependencies: none.
Scope: small, inplace.rs and provenance. Helper fix is committed. The rebuilt runtime probe verifies private
memory CPU/GPU reads and writes; mapped sections fail data-coherence
checks and must retain copies.

## 2. Port packed-byte shader arithmetic

Acceptance: signed packed dot-product accumulation matches synthetic CPU
results, including negative operands and wrapping. Verify portable lowering
and Windows-target GPU readback. Dependencies: none. Scope: small, lower.rs
and focused tests. Attribute the new cool_fox source identity.

Completed: synthetic GPU readback reproduced the unsupported instruction
before the port and passes afterward, including signed bytes and overflow.
The portable shader suite and policy tests pass. Title effects remain to
be measured after rebuilding.

The follow-up VOP3P byte-dot port adds signed and unsigned full-width
accumulation and saturation, with focused GPU readback. Captured programs
previously stopped at this instruction after the first dot port; complete
upscaler coverage must still be measured.

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
lookup cost and title behavior. PR 5 was inspected at head
`bbcb014787a3454c4361ecce486de44816463ee8`: closed and unmerged after
upstream adopted a larger journal refactor. This tree retains its existing
journal. The coverage index and seven focused tests pass under Proton;
the local lookup benchmark improves. Title measurement remains pending. Dependencies: none. Scope: small, watch.rs and
focused tests. Do not infer frame-rate gains from a microbenchmark alone.

## 7. Verify memory placement before enabling it

Acceptance: imported private memory preserves CPU/GPU writes and lifetime
rules; unsupported mapped sections retain copies; bounded title tests show
no GPU timeout or stale data. Dependencies: task 1. Scope: medium, placement,
copy policy and tests. Measure paging and copy costs after each change.

The open klbf-lab PR 34 guarded-copy path is ported behind
`VICTORY_DIRECT_COPY=1`. All 23 focused host tests pass with it enabled
and disabled. Title runs still show heavy upload and paging costs; the
copy benchmark does not establish a playable frame rate. File-mapping
imports must stay disabled after failed coherence checks.

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

## 10. Prebuild observed pipelines before the title loads

Acceptance: completed entries are reusable only with the same translation
settings and retained library; an opt-in run selects only previously used
pipelines, with no whole-title fallback for an empty observed list. Run
the prebuild and title with matching settings, then compare cold pipeline
waits, intro timing and memory pressure. Keep derived caches outside Git.
Dependencies: current shader ports. Scope: small independent cache-identity
and prebuild-selection changes, committed after focused verification.

The `--early-only` CLI boundary passes focused tests. A manual integration
check with an empty observed list scanned the local shader pack but
scheduled and created zero pipelines. The normal observed-list prebuild
and title comparison have also run. A warmup loaded 1,992 observed
pipelines from the host library. The subsequent title run reached visible
world rendering with no pipelines pending, but remained below 1 fps.
Prebuilding has not established the requested world frame rate or intro
timing.
