pub enum FlashError {
    SpiWriteError,
    SetLowError,
    SetHighError,
    ExceedSize(usize), // More than 48KB
}
