use std::convert::TryInto;

use crate::stack::Error;
const MAX: usize = 3;
// We need a stack
pub struct SimpleStack {
    elements: [Option<u32>; MAX],
    top: Option<u16>,
}

impl SimpleStack {
    pub fn new() -> Self{
        SimpleStack{
            elements: Default::default(),
            top: None
        }
    }
    // The stack should have the following methods:
    //   push
    pub fn push(&mut self, element: u32 ) -> Result<(),Error>{
        if self.is_full() {
           return Err(Error::Overflow)
        }
        match self.top {
            Some(t) => {
                let new_t = t+1;
                self.elements[new_t as usize] = Some(element);
                self.top = Some(new_t);
            },
            None => {
                self.top = Some(0);
                self.elements[0] = Some(element);
            }
        }
        Ok(())
    }
    //   pop
    pub fn pop(&mut self) -> Result<Option<u32>,Error>{
        if self.is_empty() {
            return Err(Error::Underflow);
        }
        let t =  self.top.unwrap(); // top must be Some(_)
        self.top =  t.checked_sub(1); // if < 0, errs out.
        return Ok(self.elements[t as usize].take());
    }
    //   peak
    pub fn peak(&self) -> Option<u32>{
        match self.top {
            Some( t) => {
                self.elements[t as usize]
            },
            None => None
        }
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
            Some(t) => (t + 1) == MAX.try_into().unwrap(),
            _ => false,
        }
    }
}


#[test]
pub fn should_work(){
    let mut st1 = SimpleStack::new();
    st1.push(22).unwrap();
    st1.push(123).ok();
    st1.push(23).ok();
    st1.push(1111).ok(); // Should err for next two
    match st1.push(12345) {
        Ok(_) => {

            let _ = st1.pop();
            println!("Yo {:?}", st1.peak());
        }
        Err(error) => {
            assert_eq!(Error::Overflow,error);
            assert_eq!(23,st1.peak().unwrap());
        },
    };
}