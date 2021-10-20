pub type Pairing = blst::Pairing;
pub type Fp = blst::blst_fp;
pub type Fp12 = blst::blst_fp12;
pub type Fp6 = blst::blst_fp6;
pub type Fr = blst::blst_fr;
pub type P1 = blst::blst_p1;
pub type P1Affine = blst::blst_p1_affine;
pub type P2 = blst::blst_p2;
pub type P2Affine = blst::blst_p2_affine;
pub type Scalar = blst::blst_scalar;
pub type Uniq = blst::blst_uniq;

pub mod finite;
pub mod utils;
pub mod poly;
pub mod kzg;

pub type BlsScalar = bls12_381::Scalar;
pub use crate::kzg::{Commitment, Kzg, Proof};
pub use poly::Poly;

// pub trait ZkFr: Clone {
    // fn default() -> Self;

    // fn from_u64(u: u64) -> Self;

    // fn destroy(&self);
// }

