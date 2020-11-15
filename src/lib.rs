mod roller;

/// Type-erased errors.
pub type BoxError = std::boxed::Box<dyn
    std::error::Error   // must implement Error to satisfy `?`
    + std::marker::Send // needed for threads
    + std::marker::Sync // needed for threads
>;

#[derive(Clone, Debug, PartialEq)]
pub enum RollerOp {}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Roller {
    pub count: u32,
    pub range: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
}
