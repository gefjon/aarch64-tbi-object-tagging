use crate::object::*;
use std::{borrow::Borrow, convert::TryInto, io::{self, Write}};

pub struct Printer<W>
where
    W: Write,
{
    output: W,
}

impl<W> From<W> for Printer<W>
where
    W: Write,
{
    fn from(w: W) -> Self { Self { output: w } }
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self { Error::Io(err) }
}

impl<W> Printer<W>
where
    W: Write,
{
    pub fn print(&mut self, obj: Object) -> Result<(), Error> {
        self.print_one(obj)?;
        writeln!(self.output)?;
        self.output.flush()?;
        Ok(())
    }
    fn print_one(&mut self, obj: Object) -> Result<(), Error> {
        match extract_tag(obj.clone()) {
            TypeId::Fixnum => self.print_fixnum(obj.try_into().unwrap()),
            TypeId::ObjArray => self.print_objarr(obj.try_into().unwrap()),
            TypeId::String => self.print_string(obj.try_into().unwrap()),
        }
    }
    fn print_fixnum(&mut self, fix: Fixnum) -> Result<(), Error> {
        write!(self.output, "{}", fix.to_i64())?;
        Ok(())
    }
    fn print_objarr(&mut self, arr: GcPtr<Array<Object>>) -> Result<(), Error> {
        write!(self.output, "(")?;
        let mut first_time = true;
        for i in 0..(arr.len().to_u64() as usize) {
            if !first_time {
                write!(self.output, " ")?;
            } else {
                first_time = false;
            }
            self.print_one(arr[i].clone())?;
        }
        write!(self.output, ")")?;
        Ok(())
    }
    fn print_string(&mut self, arr: GcPtr<Array<u8>>) -> Result<(), Error> {
        Ok(self.output.write_all((*arr).borrow())?)
    }
}
