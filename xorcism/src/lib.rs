use std::borrow::Borrow;
use std::io;

/// A munger which XORs a key with some data.
#[derive(Clone)]
pub struct Xorcism<'k> {
    /// The symetric cryptographic key.
    key: &'k [u8],
    /// Index in `key` representing the current rotation start.
    start: usize,
}

impl<'k> Xorcism<'k> {
    /// Creates a new munger from a `key`.
    pub fn new<K>(key: &'k K) -> Self
    where
        K: AsRef<[u8]> + ?Sized,
    {
        Self {
            key: key.as_ref(),
            start: 0,
        }
    }

    /// XORs each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce
    /// different results, even with identical inputs, as the key is rotated.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        // Do the XOR-ing with rotation taken into account.
        self.key
            .iter()
            .cycle()
            .skip(self.start)
            .zip(data.iter_mut())
            .for_each(|(&k, d)| *d ^= k);
        // Update rotation only once: length is known.
        self.start = (self.start + data.len()) % self.key.len();
    }

    /// XORs each byte of the input data iterator with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce
    /// different results, even with identical inputs, as the key is rotated.
    pub fn munge<'d, I, D>(&'d mut self, data: I) -> impl Iterator<Item = u8> + 'd
    where
        D: Borrow<u8>,
        I: IntoIterator<Item = D>,
        <I as IntoIterator>::IntoIter: 'd,
        'k: 'd,
    {
        // Do the XOR-ing with rotation taken into account.
        self.key
            .iter()
            .cycle()
            .skip(self.start)
            .zip(data)
            .map(|(&k, d)| {
                // Update rotation at each iteration: length is unknown.
                self.start = (self.start + 1) % self.key.len();
                k ^ *d.borrow()
            })
    }

    /// Returns a value implementing [`io::Read`] with XOR-ing in between the
    /// source `reader` and the destination buffer.
    ///
    /// The current XOR-er is used as the streaming adaptor.
    pub fn reader<R: io::Read>(self, reader: R) -> Reader<'k, R> {
        Reader {
            xorer: self,
            reader,
        }
    }

    /// Returns a value implementing [`io::Write`] with XOR-ing in between the
    /// source buffer and the destination `writer`.
    ///
    /// The current XOR-er is used as the streaming adaptor.
    pub fn writer<W: io::Write>(self, writer: W) -> Writer<'k, W> {
        Writer {
            xorer: self,
            writer,
        }
    }
}

/// Implements [`io::Read`] with XOR-ing in between the source reader and the
/// destination buffer.
pub struct Reader<'x, R: io::Read> {
    xorer: Xorcism<'x>,
    reader: R,
}

impl<'x, R: io::Read> io::Read for Reader<'x, R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let read = self.reader.read(buf)?;
        self.xorer.munge_in_place(&mut buf[..read]);
        Ok(read)
    }
}

/// Implements [`io::Write`] with XOR-ing in between the source buffer and the
/// destination writer.
pub struct Writer<'x, W: io::Write> {
    xorer: Xorcism<'x>,
    writer: W,
}

impl<'x, W: io::Write> io::Write for Writer<'x, W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // As `buf` is read-only and `writer.write` requires a buffer as well,
        // allocation seems unavoidable here.
        let mut buf = buf.to_vec();
        self.xorer.munge_in_place(&mut buf);
        self.writer.write(&buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}
