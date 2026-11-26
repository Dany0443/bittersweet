# Shader and playable-mission implementation plan

The required outcome is the first mission under player control at sustained
20 to 30 rendered frames per second, with visible 3D and UI. The opening
intro must also advance correctly. Visible frames alone do not meet this goal.

Current evidence: simple scaling now presents changing 3D and overlaid
credits under GE-Proton11-14. World frame times remain measured in seconds.
Memory pressure, title-memory copies and incomplete shader coverage remain.
No playable mission or target frame rate has been verified.

The new user archive has SHA-256
`9c2d6976f01b5abe0c4ad3723967f18adffc16a9ea991b73a3364e8734f4ad39`.
Its changed shader files match DoubleGhosty/cool_fox main at
`c3e5c375bad6447bbd99057c828eb14925b0e503`. Existing loop normalization
and restored wave64 masks must be retained rather than duplicated.

Port behavior with focused tests and current primary-source provenance.
Keep host bindings, shader metadata and cache identities consistent. Keep
uncertain paths disabled until synthetic GPU tests and title runs verify
them. Do not replace the shader source tree wholesale or import title data.

The main dependency is raw constant-buffer translation: root views can use
the existing bounded root-SRV path, while table and heap views require
matching raw descriptor slots. Those slots must preserve bounds and remain
inside host heap capacity. CPU and GPU memory visibility must be measured
before using imported memory. A successful heap-open return is insufficient.

Use both ecosystem trackers during update cycles, then inspect main
branches and unmerged PRs in the repositories themselves. Trackers and
README performance claims are leads, not local execution evidence.

Keep private helpers, captures, caches and title files outside Git. Work on
main with the configured maintainer identity and separate verified commits.
Do not push without explicit authorization for the pending work.
