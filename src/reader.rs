use crate::object::*;
use std::iter::Peekable;

#[derive(Debug)]
pub enum Error {
    Eof,
    UnexpectedDelim,
}

pub struct Reader<I>
where
    I: Iterator<Item = u8>,
{
    input: Peekable<I>,
}

impl<I> From<I> for Reader<I>
where
    I: Iterator<Item = u8>,
{
    fn from(input: I) -> Self { Self {
        input: input.peekable(),
    } }
}

fn whitespace(byte: u8) -> bool {
    [b' ', b'\t', b'\n'].contains(&byte)
}

fn opening_delim(byte: u8) -> bool {
    [b'(', b'['].contains(&byte)
}

fn closing_delim(byte: u8) -> bool {
    [b')', b']'].contains(&byte)
}

fn matching_delim(byte: u8) -> u8 {
    match byte {
        b'(' => b')',
        b'[' => b']',
        _ => panic!("Invalid opening delimiter {}", byte as char),
    }
}

fn atom_constituent(byte: u8) -> bool {
    !(whitespace(byte) || opening_delim(byte) || closing_delim(byte))
}

impl<I> Reader<I>
where
    I: Iterator<Item = u8>,
{
    fn peek(&mut self) -> Result<u8, Error> {
        if let Some(&peek) = self.input.peek() {
            Ok(peek)
        } else {
            Err(Error::Eof)
        }
    }
    fn getc(&mut self) -> Result<u8, Error> {
        if let Some(byte) = self.input.next() {
            Ok(byte)
        } else {
            Err(Error::Eof)
        }
    }
    pub fn read(&mut self) -> Result<Object, Error> {
        self.consume_whitespace()?;
        let byte = self.peek()?;
        if opening_delim(byte) {
            self.read_array()
        } else if atom_constituent(byte) {
            self.read_atom()
        } else if closing_delim(byte) {
            Err(Error::UnexpectedDelim)
        } else {
            unreachable!()
        }
    }
    fn consume_whitespace(&mut self) -> Result<(), Error> {
        while whitespace(self.peek()?) {
            self.getc()?;
        }
        Ok(())
    }
    fn read_array(&mut self) -> Result<Object, Error> {
        let terminator = matching_delim(self.getc()?);
        let mut elements = Vec::new();
        loop {
            self.consume_whitespace()?;
            if self.peek()? != terminator {
                elements.push(self.read()?);
            } else {
                // remove the terminator from the stream
                self.getc()?;
                return Ok(Object::from(GcPtr::alloc_array(&elements)));
            }
        }
    }
    fn read_atom(&mut self) -> Result<Object, Error> {
        let mut chars = Vec::new();
        loop {
            if atom_constituent(self.peek()?) {
                chars.push(self.getc()?);
            } else {
                return Ok(Object::from(GcPtr::alloc_array(&chars)));
            }
        }
    }
}
