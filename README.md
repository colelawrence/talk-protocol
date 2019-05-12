# talk-protocol
Notes on a potential Realtalk Claim/When protocol

This is an inprogress development of a Claim/When database and protocol.
It is mostly a few different pieces that are not yet completely strung together. 

## Interesting places

* [Query constraint solving `imagine/src/rtvm/db.rs`](https://github.com/colelawrence/talk-protocol/blob/500e01aea54dae4a8cfcafc046a22af5fd8e74e5/imagine/src/rtvm/db.rs#L19)
  Based on approaches presented in _The Little Schemer_ with small optimizations.

* [Macro smoke test `talk_macros/tests/smoke.rs`](https://github.com/colelawrence/talk-protocol/blob/500e01aea54dae4a8cfcafc046a22af5fd8e74e5/talk_macros/tests/smoke.rs#L12-L21)
  Based on some of the syntax used in Bret Victor's Dynamicland.
