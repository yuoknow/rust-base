use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

pub const BUFFER_SIZE: usize = 64 * 1024;

// -----------------------------------------------------------------------------
// MyBufReader
// -----------------------------------------------------------------------------

#[allow(dead_code)]
pub struct MyBufReader {
    buf: Vec<u8>,
    pos: usize,
    size: usize,
    file: File,
}

impl MyBufReader {
    fn new(file: File, capacity: usize) -> MyBufReader {
        MyBufReader {
            buf: vec![0u8; capacity],
            pos: 0,
            size: 0,
            file,
        }
    }

    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = MyBufReader::new(file, 64_000);
        return Ok(reader);
    }

    pub fn read_byte(&mut self) -> io::Result<Option<u8>> {
        if self.pos == 0 || self.pos == self.buf.len() {
            let count = self.file.read(&mut self.buf)?;
            self.size = count;
            self.pos = 0;
        }

        if self.size == 0 || self.pos == self.size {
            return Ok(None);
        }

        let b = self.buf[self.pos];
        self.pos += 1;

        return Ok(Some(b));
    }
}

// -----------------------------------------------------------------------------
// MyBufWriter
// -----------------------------------------------------------------------------

pub struct MyBufWriter {
    buf: Vec<u8>,
    pos: usize,
    file: File,
}

impl MyBufWriter {
    fn new(file: File, capacity: usize) -> MyBufWriter {
        MyBufWriter {
            buf: vec![0u8; capacity],
            pos: 0,
            file,
        }
    }

    pub fn create(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = File::create(path)?;
        let buf = MyBufWriter::new(file, 64_000);

        Ok(buf)
    }

    pub fn write_buffered(&mut self, data: &[u8]) -> io::Result<()> {
        for &byte in data {
            self.buf[self.pos] = byte;
            self.pos += 1;
            if self.pos == self.buf.iter().len() {
                self.file.write(&self.buf[0..self.pos])?;
                self.pos = 0;
            }
        }

        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.file.write(&self.buf[0..self.pos])?;
        self.pos = 0;
        Ok(())
    }

    pub fn close(mut self) -> io::Result<()> {
        self.flush()
    }
}

impl Drop for MyBufWriter {
    fn drop(&mut self) {
        // Ошибку из Drop вернуть нельзя.
        // Поэтому в реальном коде лучше явно вызывать close() или flush().
        let _ = self.flush();
    }
}

// -----------------------------------------------------------------------------
// Медленная версия
// -----------------------------------------------------------------------------

pub fn copy_slow(input: impl AsRef<Path>, output: impl AsRef<Path>) -> io::Result<u64> {
    let mut input = File::open(input)?;
    let mut output = File::create(output)?;

    let mut copied = 0;
    let mut byte = [0u8; 1];

    loop {
        let n = input.read(&mut byte)?;
        if n == 0 {
            break;
        }

        output.write_all(&byte[..n])?;
        copied += n as u64;
    }

    output.flush()?;

    Ok(copied)
}

// -----------------------------------------------------------------------------
// Быстрая версия
// -----------------------------------------------------------------------------
// copy_fast специально тоже использует побайтный API.
// Разница должна быть не в коде копирования, а в реализации MyBufReader и MyBufWriter
// эту функцию не нужно менять, она должна работать с любыми реализациями MyBufReader и MyBufWriter,
// которые вы сделаете
pub fn copy_fast(input: impl AsRef<Path>, output: impl AsRef<Path>) -> io::Result<u64> {
    let mut reader = MyBufReader::open(input)?;
    let mut writer = MyBufWriter::create(output)?;

    let mut copied = 0;

    while let Some(byte) = reader.read_byte()? {
        writer.write_buffered(&[byte])?;
        copied += 1;
    }

    writer.close()?;

    Ok(copied)
}

pub const RECORD_SIZE: usize = 10;

pub fn make_record(index: usize) -> [u8; RECORD_SIZE] {
    let mut record = [0u8; RECORD_SIZE];

    (0..RECORD_SIZE).for_each(|i| {
        record[i] = ((index + i) % 251) as u8;
    });

    record
}

pub fn generate_input_file(path: impl AsRef<Path>, records: usize) -> io::Result<()> {
    let mut file = File::create(path)?;

    for i in 0..records {
        let record = make_record(i);
        file.write_all(&record)?;
    }

    file.flush()?;

    Ok(())
}
