use std::{collections::VecDeque, io::Read, ops::Not};

pub struct Tokens<R> {
    reader: R,
    buf: VecDeque<u8>,
}

const READ_BUFFER_SIZE: usize = 4096;

impl<R: Read> Tokens<R> {
    pub fn from_reader(reader: R) -> Self {
        Self {
            reader,
            buf: VecDeque::new(),
        }
    }

    fn fill_buff(&mut self) -> std::io::Result<usize> {
        let mut read_buffer = [0u8; READ_BUFFER_SIZE];
        let read_bytes = self.reader.read(&mut read_buffer)?;
        self.buf.extend(read_buffer.iter().take(read_bytes));
        Ok(read_bytes)
    }

    pub fn next_token(&mut self) -> std::io::Result<String> {
        let mut wstart = 0usize;
        loop {
            if self.buf.len() <= wstart {
                let read_bytes = self.fill_buff()?;
                if read_bytes == 0 {
                    break;
                }
            }

            if !self.buf[wstart].is_ascii_whitespace() {
                break;
            }
            wstart += 1;
        }

        let mut wend = wstart;
        loop {
            if self.buf.len() <= wend {
                let read_bytes = self.fill_buff()?;
                if read_bytes == 0 {
                    break;
                }
            }

            if self.buf[wend].is_ascii_whitespace() {
                break;
            }
            wend += 1;
        }

        let word = {
            let mut result = vec![];
            if wstart < wend {
                result.extend(self.buf.range(wstart..wend));
                self.buf.drain(..wend);
            }
            String::from_utf8_lossy(&result).into_owned()
        };

        Ok(word)
    }
}

impl<R: Read> Iterator for Tokens<R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let word = self.next_token().unwrap();

        word.is_empty().not().then_some(word)
    }
}
