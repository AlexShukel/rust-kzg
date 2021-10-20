use bls12_381::{G1Affine, G1Projective, G2Affine, G2Projective};
//use poly::blsScalar as Fr;
use bls12_381::Scalar as Fr;
use bls12_381::*;
use std::fmt;
use super::*;

use crate::poly::Poly;

use pairing::Engine;

pub struct polydata {
    coeffs: Vec<u64>,
}


pub fn blst_poly_into_zk_poly(pd: polydata) -> Result<Poly, fmt::Error> {
	//use bls12_381::Scalar as Fr;
    let mut poly = Vec::new();
    for x in pd.coeffs {
        poly.push(Fr::from(x))
    }

    let p = super::Poly(poly);
    Ok(p)
}

pub fn blst_fr_into_zk_fr(fr: super::Fr) -> Fr { 
	Fr::from_raw(fr.l) 
}