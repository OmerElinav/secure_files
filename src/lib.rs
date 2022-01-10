use std::fs;
use std::path::{PathBuf};
use pyo3::prelude::*;

fn simple_encrypt(message: String) -> String {
    let bytes = &message
        .as_bytes()
        .iter()
        .map(|x| (x ^ 1))
        .collect::<Vec<u8>>()[..];
    std::str::from_utf8(bytes).unwrap().to_string()
}

fn simple_decrypt(message: String) -> String {
    let bytes = &message
        .as_bytes()
        .iter()
        .map(|x| (x ^ 1))
        .collect::<Vec<u8>>()[..];
    std::str::from_utf8(bytes).unwrap().to_string()
}

#[pyclass]
struct SecureFile {
    path: PathBuf,
}

#[pymethods]
impl SecureFile {
    #[new]
    fn new(path: PathBuf) -> Self {
        SecureFile { path }
    }

    fn read(&self) -> PyResult<String> {
        let encrypted = &simple_encrypt(fs::read_to_string(&self.path)?);
        Ok(encrypted.to_string())
    }

    fn write(&self, contents: &str) -> std::io::Result<()> {
        let content = simple_decrypt(contents.to_string());
        fs::write(&self.path, content)
    }

    fn __enter__<'p>(this: PyRef<'p, Self>, _py: Python<'p>) -> PyResult<PyRef<'p, Self>> {
        Ok(this)
    }

    fn __exit__<'p>(&self, exc_type: &'p PyAny, exc_value: &'p PyAny, exc_traceback: &'p PyAny)
                    -> (&'p PyAny, &'p PyAny, &'p PyAny) {
        (exc_type, exc_value, exc_traceback)
    }
}

#[pyfunction]
fn open(path: PathBuf) -> PyResult<SecureFile> {
    Ok(SecureFile { path })
}

#[pymodule]
fn secure_files(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SecureFile>()?;
    m.add_function(wrap_pyfunction!(open, m)?)?;
    Ok(())
}