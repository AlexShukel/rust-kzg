#[cfg(test)]
pub mod fft_g1_tests {
    use kzg_bench::tests::fft_g1::*;
    use mcl_rust::data_types::fr::Fr;
    use mcl_rust::data_types::g1::G1;
    use mcl_rust::fk20_fft::{FFTSettings, make_data};
    use mcl_rust::mcl_methods::init;
    use mcl_rust::CurveType;

    #[test]
    fn roundtrip_fft_g1_() {
        assert!(init(CurveType::BLS12_381));
        roundtrip_fft::<Fr, G1, FFTSettings>(&make_data);
    }

    #[test]
    fn stride_fft_g1_() {
        assert!(init(CurveType::BLS12_381));
        stride_fft::<Fr, G1, FFTSettings>(&make_data);
    }
}
