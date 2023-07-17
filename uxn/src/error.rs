#[derive(Debug, PartialEq)]
pub enum Error {
    Underflow = 0x01,
    Overflow = 0x02,
    DivisionByZero = 0x03,
    FailedToLoadRom = 0x04,
    EndOfExecution = 0x05,
}
