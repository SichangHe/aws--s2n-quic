# DC stream-ID packing proof scope

Base: `3e3e77ed57cad1a61e5d0657a78207b9425533f7` (`origin/main` when revalidated on 2026-07-24).

Production correspondence: `dc/s2n-quic-dc/src/packet/stream/id.rs`, specifically the construction guards, `Id::into_varint`, and the quotient used in `Id::from_varint`.

The proof establishes for every `queue_id < 2^60` and each pair of flag booleans:

- the packed value `queue_id * 4 + flags` is below the QUIC VarInt upper bound `2^62`;
- division by four recovers the original queue ID; and
- the modeled `& 0b10` and `& 0b01` tests recover the two input flags.

Exclusions: packet framing, codec buffers, Rust/Verus compiler correctness, concurrency, callers of `Id`, malformed byte decoding, transport behavior, and every DC behavior other than this arithmetic. This proof makes no vulnerability claim and does not modify production code.
