use blake2::{Blake2b, Digest};
use generic_array::typenum::*;
use generic_array::GenericArray;
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
fn kstretch(_py: Python, m: &PyModule) -> PyResult<()>
{
    m.add_wrapped(wrap_pyfunction!(process))
}

pub fn hash(input: &[u8]) -> GenericArray<u8, U64>
{
    let mut b = Blake2b::default();
    b.update(input);
    b.finalize()
}

///
/// # Example
/// ```
/// assert_eq!(kstretch::hash_with_salt(b"123", b"456"),
///            kstretch::hash(b"123456"));
/// ```
pub fn hash_with_salt(input: &[u8], salt: &[u8]) -> GenericArray<u8, U64>
{
    let mut b = Blake2b::default();
    b.update(input);
    b.update(salt);
    b.finalize()
}

fn digest(input: &[u8]) -> String
{
    let hashed = hash(input);

    let mut blake = Blake2b::default();

    (0..128u16)
        .into_par_iter()
        .map(|index| hash_with_salt(&hashed, &index.to_be_bytes()))
        .map(|input| compute(input))
        .collect::<Vec<_>>()
        .iter()
        .for_each(|it| blake.update(&it));

    let blake_final = blake.finalize();

    hex::encode(&blake_final[0..blake_final.len() / 2])
}

fn compute(input: GenericArray<u8, U64>) -> GenericArray<u8, U64>
{
    (0..2u32.pow(25)).fold(input, |a, _| hash(&a))
}
