use std::fs;
use std::path::{PathBuf};
use pyo3::prelude::*;


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

    fn read(&self) -> std::io::Result<String> {
        fs::read_to_string(&self.path)
    }

    fn write(&self, contents: &str) -> std::io::Result<()> {
        fs::write(&self.path, contents)
    }

    fn __enter__<'p>(this: PyRef<'p, Self>, _py: Python<'p>) -> PyResult<PyRef<'p, Self>> {
        Ok(this)
    }

    fn __exit__<'p>(&self, exc_type: &'p PyAny, exc_value: &'p PyAny, exc_traceback: &'p PyAny)
                -> (&'p PyAny, &'p PyAny, &'p PyAny) {
        (exc_type, exc_value, exc_traceback)
    }
}

// #[pyfunction]
// fn open<'p>(path: PathBuf) -> PyResult<&'p SecureFile> {
//     Ok(&SecureFile { path })
// }

#[pymodule]
fn secure_files(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SecureFile>()?;
    Ok(())
}