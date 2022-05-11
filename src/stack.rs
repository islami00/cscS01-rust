use std::fmt::Debug;

pub mod simple; // Can come below

#[derive(Debug,PartialEq,Eq)]
pub enum Error {
    Overflow,
    Underflow,
}
// We need a stack
pub struct Stack<'st> {
    // That should be able to store different kinds of items
    elements: Vec<Box<dyn 'st + Debug>>,
    top: Option<u16>,
}

// It should contain elements

impl<'st> Stack<'st> {
    pub fn new(size: u16) -> Self {
        Stack {
            elements: Vec::with_capacity(size.into()),
            top: None,
        }
    }
    // The stack should have the following methods:
    //   push
    pub fn push<T>(&mut self, el: T) -> Result<(), Error>
    where
        T: 'st + Debug,
    {
        if self.is_full() {
            return Err(Error::Overflow);
        }
        // Overflow
        match self.top {
            Some(t) => {
                self.top = Some(t.checked_add(1).unwrap());
                self.elements.push(Box::new(el));
            }
            None => {
                // Push.
                self.top = Some(self.top.unwrap_or_default());
                self.elements.push(Box::new(el));
            }
        }
            Ok(())
    }
    //   pop
    pub fn pop(&mut self) -> Result<Option<Box<(dyn Debug + 'st)>>,Error> {
        if self.is_empty() {
            return Err(Error::Underflow);
        }
        Ok(self.elements.pop())
    }
    //   peak
    pub fn peak(&self) -> Option<&Box<(dyn Debug + 'st)>> {
        // self.elements.
        self.elements.last()
    }
    //   isEmpty
    pub fn is_empty(&self) -> bool {
        match self.top {
            Some(_) => false,
            None => true,
        }
    }
    // Convenience methods:
    //   isFull
    pub fn is_full(&self) -> bool {
        match self.top {
            Some(t) => {
                let max = t + 1;
                self.elements.capacity() == max.into()
            }
            _ => false,
        }
    }
}

