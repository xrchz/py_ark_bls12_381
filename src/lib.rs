use num_bigint::BigInt;
use pyo3::prelude::*;
use ark_bls12_381;
use ark_ff::MontConfig;
use ark_ff::BigInteger;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

// #[pyfunction]
// fn multiply(base: &G1Affine, scalar: u64) -> PyResult<(BigInt, BigInt, BigInt)> {
// }

/// A Python module implemented in Rust.
#[pymodule]
fn py_ark_bls12_381(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("curve_order", BigInt::from_signed_bytes_le(&ark_bls12_381::fr::FrConfig::MODULUS.to_bytes_le()))?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    // m.add_function(wrap_pyfunction!(multiply, m)?)?;
    Ok(())
}
