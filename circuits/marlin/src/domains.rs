use algebra::PrimeField;
use ff_fft::EvaluationDomain;

#[derive(Debug, Clone, Copy)]
pub struct EvaluationDomains<F : PrimeField> {
    pub h: EvaluationDomain<F>,
    pub k: EvaluationDomain<F>,
    pub b: EvaluationDomain<F>,
    pub x: EvaluationDomain<F>,
}

impl<F : PrimeField> EvaluationDomains<F> {
    pub fn create(
        variables : usize,
        public_inputs: usize,
        nonzero_entries: usize) -> Option<Self> {

        let h_group_size = 
            EvaluationDomain::<F>::compute_size_of_domain(variables)?;
        let x_group_size =
            EvaluationDomain::<F>::compute_size_of_domain(public_inputs)?;
        let k_group_size =
            EvaluationDomain::<F>::compute_size_of_domain(nonzero_entries)?;

        let h = EvaluationDomain::<F>::new(h_group_size)?;
        let k = EvaluationDomain::<F>::new(k_group_size)?;
        let b = EvaluationDomain::<F>::new(k_group_size * 3 - 3)?;
        let x = EvaluationDomain::<F>::new(x_group_size)?;

        Some (EvaluationDomains { h, k, b, x })
    }
}
