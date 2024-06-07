use blake2::Blake2b512;
use blake2::{Blake2b, Digest};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

/// # Usage
/// Process `input` and give out the hex hash as `String
/// # Parameter
/// `input`: `String` form input
/// # Return Value
/// `String`: hexed output
#[pyfunction]
pub fn process(input: &str) -> PyResult<String>
{
    Ok(digest(input.as_bytes()))
}

#[pymodule]
fn kstretch(m: &Bound<'_, PyModule>) -> PyResult<()>
{
    m.add_function(wrap_pyfunction!(process, m)?)
}

pub fn hash(input: &[u8]) -> [u8; 64]
{
    let mut b = Blake2b512::default();
    b.update(input);
    b.finalize().into()
}

///
/// # Example
/// ```
/// assert_eq!(kstretch::hash_with_salt(b"123", b"456"),
///            kstretch::hash(b"123456"));
/// ```
pub fn hash_with_salt(input: &[u8], salt: &[u8]) -> [u8; 64]
{
    let mut b = Blake2b::default();
    b.update(input);
    b.update(salt);
    b.finalize().into()
}

fn digest(input: &[u8]) -> String
{
    let hashed = hash(input);

    let mut blake = Blake2b512::default();

    (0..128u16)
        .into_par_iter()
        .map(|index| hash_with_salt(&hashed, &index.to_be_bytes()))
        .map(compute)
        .collect::<Vec<_>>()
        .iter()
        .for_each(|it| blake.update(it));

    let blake_final = blake.finalize();

    hex::encode(&blake_final[0..blake_final.len() / 2])
}

fn compute(input: [u8; 64]) -> [u8; 64]
{
    (0..2u32.pow(25)).fold(input, |a, _| hash(&a))
}
