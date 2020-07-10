/**
    Author : Niharika Gauraha
            KTH
            Email : niharika@kth.se
    function:  rust_normalize
            compute normalized weights using log sum trick to avoid numerical underflow
**/

use pyo3::prelude::*;



#[pyfunction]
fn rust_normalize(x: Vec<f64>) -> PyResult<Vec<f64>>{
    let n = x.len();
    let logweights: Vec<f64> = x.iter().map(|x| x.ln()).collect();
    let mut max_weight: f64;

    max_weight = logweights[0];
    for i in 0..n
    {
        if max_weight < logweights[i] {
            max_weight = logweights[i];
        }
    }

    let weights:Vec<f64> = logweights.iter().map(|x| (x-max_weight).exp()).collect();

    let w_sum:f64 = weights.iter().sum();

    let normalised_weights:Vec<f64> = weights.iter().map(|x| x /w_sum).collect();

    Ok(normalised_weights)
}

#[pymodule]
fn rust_library(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "rust_normalize")]
    fn normalize_py(_py: Python, x: Vec<f64>) -> PyResult<Vec<f64>> {
        let out = rust_normalize(x);
        out
    }

    Ok(())
}


