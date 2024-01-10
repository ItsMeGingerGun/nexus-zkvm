use std::ops::{Add, AddAssign, Mul, MulAssign};

use ark_ec::PrimeGroup;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

/// Defines basic operations on commitments.
pub trait CommitmentOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output> + AddAssign<Rhs>
{
}

impl<T, Rhs, Output> CommitmentOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output> + AddAssign<Rhs>
{
}

/// A helper trait for types implementing a multiplication of a commitment with a scalar.
pub trait ScalarMul<Rhs, Output = Self>: Mul<Rhs, Output = Output> + MulAssign<Rhs> {}

impl<T, Rhs, Output> ScalarMul<Rhs, Output> for T where T: Mul<Rhs, Output = Output> + MulAssign<Rhs>
{}

pub trait Commitment<G: PrimeGroup>:
    Default
    + PartialEq
    + Copy
    + Clone
    + Send
    + Sync
    + CommitmentOps
    + ScalarMul<G::ScalarField>
    + CanonicalSerialize
    + CanonicalDeserialize
{
}
impl<G: PrimeGroup, T> Commitment<G> for T where
    T: Default
        + PartialEq
        + Copy
        + Clone
        + Send
        + Sync
        + CommitmentOps
        + ScalarMul<G::ScalarField>
        + CanonicalSerialize
        + CanonicalDeserialize
{
}

pub trait CommitmentScheme<G: PrimeGroup>: Send + Sync {
    /// Commitment scheme public parameters.
    type PP: CanonicalSerialize + CanonicalDeserialize + Sync;

    /// Auxiliary data used for setup (such as an SRS)
    type SetupAux;

    /// Commitment type.
    type Commitment: Commitment<G>;

    /// Samples new public parameters of a specified size.
    fn setup(n: usize, aux: &Self::SetupAux) -> Self::PP;

    /// Commits to the given vector using provided public parameters.
    fn commit(pp: &Self::PP, x: &[G::ScalarField]) -> Self::Commitment;
}