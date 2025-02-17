use std::io::Write;

pub struct VecWriter<'a>(&'a mut Vec<u8>);

impl<'a> VecWriter<'a> {
    pub fn new(vec: &mut Vec<u8>) -> VecWriter<'_> {
        VecWriter(vec)
    }
}

impl<'a> Write for VecWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}