# Evidence and public correspondence

## Local, exact-base evidence

- Before these verification artifacts were added, the isolated worktree was clean at `3e3e77ed57cad1a61e5d0657a78207b9425533f7`, with `origin` set to `https://github.com/aws/s2n-quic.git`.
- `git ls-remote origin refs/heads/main` returned that same commit during revalidation.
- The bounded counterexample search was confined to `dc/s2n-quic-dc/src/packet/stream/id.rs`, its direct queue-ID uses, and its boundary/round-trip tests. No counterexample exists on this base: constructors reject `queue_id >= 2^60`, packing is bounded below `2^62`, and tests cover the boundary and round trips.
- The historical predecessor is publicly visible as commit `f83bbcd52bf0bc266d841dc508459a005f72026e` (`fix(s2n-quic-dc): Validate stream IDs fit into maximum encodable size`). Its diff shows the prior unchecked shift/panic path and the added guard. It is already contained in the chosen base, so no production change is warranted or made here.

## Focused public source correspondence

- Repository ownership/default branch: <https://api.github.com/repos/aws/s2n-quic> identifies the public `aws/s2n-quic` repository and `main` default branch.
- Exact production source: <https://github.com/aws/s2n-quic/blob/3e3e77ed57cad1a61e5d0657a78207b9425533f7/dc/s2n-quic-dc/src/packet/stream/id.rs>.
- Historical fix correspondence: <https://github.com/aws/s2n-quic/commit/f83bbcd52bf0bc266d841dc508459a005f72026e>.

## Admission gate (outside proof scope)

The public AWS disclosure route was revalidated locally from <https://aws.amazon.com/security/vulnerability-reporting/>. It directs AWS open-source vulnerability reports to the AWS HackerOne VDP. This records the experiment's admission gate only; it is not evidence about stream-ID behavior. No contact or report was made.

## Reproduction

Run `./verification/stream_id/check.sh`. The script derives its own directory and defaults to a `verus` executable beside `check.sh`; supply `VERUS` when the pinned executable is stored elsewhere. The script requires version `0.2025.03.28.892067f` and SHA-256 `562588748e3587043fa23361448b934ad88b32f7be3d6457938a7a417ec775f8`, so a non-identical binary is rejected before proof execution.
