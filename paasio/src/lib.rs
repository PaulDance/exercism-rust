use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    throughput: usize,
    operations: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(reader: R) -> ReadStats<R> {
        Self {
            reader,
            throughput: 0,
            operations: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.throughput
    }

    pub fn reads(&self) -> usize {
        self.operations
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_read = self.reader.read(buf)?;
        self.throughput += bytes_read;
        self.operations += 1;
        Ok(bytes_read)
    }
}

pub struct WriteStats<W> {
    writer: W,
    throughput: usize,
    operations: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(writer: W) -> WriteStats<W> {
        Self {
            writer,
            throughput: 0,
            operations: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.throughput
    }

    pub fn writes(&self) -> usize {
        self.operations
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_written = self.writer.write(buf)?;
        self.throughput += bytes_written;
        self.operations += 1;
        Ok(bytes_written)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
