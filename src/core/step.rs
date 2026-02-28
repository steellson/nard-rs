use super::throw::Throw;

#[derive(Debug)]
pub struct Step<'a> {
    pub throw: &'a Throw,
    pub is_from_head: bool,
    pub is_finally: bool,
}

impl<'a> Step<'a> {
    pub fn new(
    throw: &'a Throw,
        is_from_head: bool,
        is_finally: bool
    ) -> Self {
        Self {
            throw: throw, 
            is_from_head: is_from_head,
            is_finally: is_finally 
        }
    }
}