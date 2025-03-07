use pyo3::prelude::*;

#[pyclass]
pub struct FixParser {
    buffer: Vec<u8>,
    equal_byte: u8,
    soh_byte: u8,
    end_tag: u8,
    start_tag: u8,
}

#[pymethods]
impl FixParser {
    #[new]
    pub fn new(equal_byte: u8, soh_byte: u8, end_tag: u8, start_tag: u8) -> Self {
        FixParser {
            buffer: Vec::new(),
            equal_byte,
            soh_byte,
            end_tag,
            start_tag,
        }
    }

    pub fn add_buffer(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn get_message(&mut self) -> Option<Vec<u8>> {
        let mut pos: usize = 0;
        let buffer_length: usize = self.buffer.len();
        let mut in_tag: bool = true;

        while pos < buffer_length {
            // Assume we have TAG=VALUE.
            // We are currently in TAG
            if in_tag {
                // Find next position that contains b"="
                let equals_pos: usize =
                    match self.buffer.windows(1).position(|w| w == &[self.equal_byte]) {
                        // Found b"="
                        Some(b) => b,
                        // Did not find b"=", so we can early exit.
                        None => return None,
                    };
            } else {
            }
        }

        let message: Vec<u8> = self.buffer.drain(..=pos).collect();
        Some(message)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn farser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FixParser>()?;
    Ok(())
}
