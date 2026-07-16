//! A compiler barrier against if-conversion of rare branches.

/// A do-nothing function that forces the compiler to keep the branch arm
/// calling it a real branch.
///
/// This is a kluge, and it is here for speed only. Techniques that draw further
/// words from the source on a rare path (a pool refill, a second-word
/// extension) make the next source state depend on which path was taken.
/// Without a genuine function call in the rare arm, LLVM if-converts it in
/// bulk-generation loops (observed both on Apple Silicon and 12th-gen Intel),
/// selecting the next source state with a conditional move: that puts the whole
/// conversion on the loop-carried dependency chain of the source, several times
/// slower than predicting the branch, which is taken once per ~2¹² calls. A
/// call cannot be speculated, so the arm containing it cannot be flattened;
/// branch-weight hints alone (`std::hint::cold_path`) proved insufficient. The
/// `black_box` keeps the body from being inferred side-effect-free, which would
/// let the call be optimized away.
#[cold]
#[inline(never)]
pub(crate) fn cold_barrier() {
    std::hint::black_box(());
}
