pub struct Error(NTSTATUS);

impl Error {
    pub const UNSUCCESSFUL: Error = Error(STATUS_UNSUCCESSFUL);
    pub const NOT_IMPLEMENTED: Error = Error(STATUS_NOT_IMPLEMENTED);

    pub fn from_kernel_errno(status: NTSTATUS) -> Error {
        Error(status)
    }

    pub fn to_kernel_errno(&self) -> NTSTATUS {
        self.0
    }
}