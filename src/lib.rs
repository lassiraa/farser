use pyo3::prelude::*;

#[pyclass]
pub struct FixParser {
    buffer: Vec<u8>,
}

#[pymethods]
impl FixParser {
    #[new]
    pub fn new() -> Self {
        FixParser { buffer: Vec::new() }
    }

    pub fn add_buffer(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn get_message(&mut self) -> Option<Vec<u8>> {
        if let Some(pos) = self.buffer.windows(1).position(|w| w == &[b'\n']) {
            let message = self.buffer.drain(..=pos).collect();
            Some(message)
        } else {
            None
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn farser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FixParser>()?;
    Ok(())
}
