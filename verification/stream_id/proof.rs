// Verification model for dc/s2n-quic-dc/src/packet/stream/id.rs at
// 3e3e77ed57cad1a61e5d0657a78207b9425533f7.
//
// This is a mathematical model, not linked into the production crate.
// It covers only the queue-id/flag packing performed by Id::into_varint and
// the corresponding arithmetic used by Id::from_varint.

use vstd::prelude::*;

verus! {

spec fn max_queue_id() -> nat { 1152921504606846976u64 as nat } // 2^60
spec fn max_varint() -> nat { 4611686018427387904u64 as nat } // 2^62

spec fn flags(reliable: bool, bidirectional: bool) -> nat {
    (if reliable { 2nat } else { 0nat }) +
    (if bidirectional { 1nat } else { 0nat })
}

spec fn encode(queue_id: nat, reliable: bool, bidirectional: bool) -> nat {
    queue_id * 4 + flags(reliable, bidirectional)
}

// These model `value & 0b10 == 0b10` and `value & 0b01 == 0b01` from
// Id::from_varint after reducing the wire value to its low two bits.
spec fn decode_reliable(wire: nat) -> bool {
    wire % 4 >= 2
}

spec fn decode_bidirectional(wire: nat) -> bool {
    wire % 2 == 1
}

proof fn flags_fit(reliable: bool, bidirectional: bool)
    ensures flags(reliable, bidirectional) < 4
{
    if reliable && bidirectional {
        assert(flags(reliable, bidirectional) == 3);
    } else if reliable {
        assert(flags(reliable, bidirectional) == 2);
    } else if bidirectional {
        assert(flags(reliable, bidirectional) == 1);
    } else {
        assert(flags(reliable, bidirectional) == 0);
    }
}

// Corresponds to the construction guards in normal,
// unreliable_unidirectional, and with_queue_id, plus into_varint.
proof fn encodable_and_lossless(queue_id: nat, reliable: bool, bidirectional: bool)
    requires queue_id < max_queue_id(),
    ensures
        encode(queue_id, reliable, bidirectional) < max_varint(),
        encode(queue_id, reliable, bidirectional) / 4 == queue_id,
        encode(queue_id, reliable, bidirectional) % 4 == flags(reliable, bidirectional),
        decode_reliable(encode(queue_id, reliable, bidirectional)) == reliable,
        decode_bidirectional(encode(queue_id, reliable, bidirectional)) == bidirectional,
{
    flags_fit(reliable, bidirectional);
    assert(encode(queue_id, reliable, bidirectional) < queue_id * 4 + 4);
    assert(queue_id * 4 + 4 <= max_queue_id() * 4);
    assert(max_varint() == max_queue_id() * 4);
    assert(encode(queue_id, reliable, bidirectional) < max_varint());
    assert((queue_id * 4 + flags(reliable, bidirectional)) / 4 == queue_id) by (nonlinear_arith);
    assert((queue_id * 4 + flags(reliable, bidirectional)) % 4 == flags(reliable, bidirectional)) by (nonlinear_arith);
    if reliable && bidirectional {
        assert(flags(reliable, bidirectional) == 3);
        assert((queue_id * 4 + 3) % 4 == 3) by (nonlinear_arith);
    } else if reliable {
        assert(flags(reliable, bidirectional) == 2);
        assert((queue_id * 4 + 2) % 4 == 2) by (nonlinear_arith);
    } else if bidirectional {
        assert(flags(reliable, bidirectional) == 1);
        assert((queue_id * 4 + 1) % 4 == 1) by (nonlinear_arith);
    } else {
        assert(flags(reliable, bidirectional) == 0);
        assert((queue_id * 4) % 4 == 0) by (nonlinear_arith);
    }
}

// A wire VarInt is strictly below 2^62. Therefore from_varint's quotient
// always satisfies the same queue-id bound on this modeled input domain.
proof fn decoded_queue_id_is_encodable(wire: nat)
    requires wire < max_varint(),
    ensures wire / 4 < max_queue_id(),
{
    assert(wire == (wire / 4) * 4 + wire % 4);
    assert((wire / 4) * 4 <= wire);
    assert(max_varint() == max_queue_id() * 4);
    if wire / 4 >= max_queue_id() {
        assert(max_queue_id() * 4 <= (wire / 4) * 4);
        assert(max_varint() <= wire);
        assert(false);
    }
}

} // verus!

fn main() {}
