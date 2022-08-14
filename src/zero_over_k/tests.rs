#[cfg(test)]
mod test {
    use crate::{
        zero_over_k::commitment::{HomomorphicPolynomialCommitment, KZG10},
    //     error::{to_pc_error, Error},
        label_polynomial,
        virtual_oracle::add_vo::AddVO,
        zero_over_k::ZeroOverK,
    };
    use ark_bls12_381::{Bls12_381, Fr};
    use ark_ff::One;
    use ark_ff::{Field, UniformRand};
    use ark_poly::{
        univariate::DensePolynomial, EvaluationDomain, Evaluations, GeneralEvaluationDomain,
        UVPolynomial,
    };
    use crate::{Marlin, SimpleHashFiatShamirRng};
    use rand_chacha::ChaChaRng;
    use ark_poly_commit::{
        LabeledCommitment, LabeledPolynomial, PCRandomness, PolynomialCommitment,
    };
    use ark_std::test_rng;
    use blake2::Blake2s;
    // use rand_core::OsRng;
    use std::iter;

    type F = Fr;
    type PC = KZG10<Bls12_381>;
    type D = Blake2s;

    type FS = SimpleHashFiatShamirRng<Blake2s, ChaChaRng>;


    #[test]
    fn test_zero_over_k_add_vo() {
        let n = 8;
        let mut rng = test_rng();
        let domain = GeneralEvaluationDomain::<F>::new(n).unwrap();

        let max_degree = 20;
        let pp = PC::setup(max_degree, None, &mut rng).unwrap();
        let (ck, vk) = PC::trim(&pp, max_degree, 0, None).unwrap();

        let f_evals: Vec<F> = vec![
            F::from(1u64),
            F::from(2u64),
            F::from(3u64),
            F::from(4u64),
            F::from(5u64),
            F::from(6u64),
            F::from(7u64),
            F::from(8u64),
        ];

        let f = DensePolynomial::<F>::from_coefficients_slice(&domain.ifft(&f_evals));
        let f = label_polynomial!(f);

        let g_evals: Vec<F> = vec![
            -F::from(1u64),
            -F::from(2u64),
            -F::from(3u64),
            -F::from(4u64),
            -F::from(5u64),
            -F::from(6u64),
            -F::from(7u64),
            -F::from(8u64),
        ];

        let g = DensePolynomial::<F>::from_coefficients_slice(&domain.ifft(&g_evals));
        let g = LabeledPolynomial::new(String::from("g"), g.clone(), None, None);

        let concrete_oracles = [f, g];
        let alphas = vec![F::one(), F::one()];
        let (commitments, rands) = PC::commit(&ck, &concrete_oracles, None).unwrap();

        let zero_over_k_vo = AddVO {};

        let zero_over_k_proof = ZeroOverK::<F, PC, FS>::prove(
            &concrete_oracles,
            &commitments,
            &rands,
            &zero_over_k_vo,
            &alphas,
            &domain,
            &ck,
            &mut rng,
        );

        assert!(zero_over_k_proof.is_ok());

        let is_valid = ZeroOverK::<F, PC, FS>::verify(
            &zero_over_k_proof.unwrap(),
            &commitments,
            &zero_over_k_vo,
            &domain,
            &alphas,
            &vk,
            &mut rng
        );

        println!("{:?}", is_valid);

        // assert!(is_valid.is_err());

        // Test for a specific error
        // assert_eq!(is_valid.err().unwrap(), Error::Check2Failed);
    }
}
