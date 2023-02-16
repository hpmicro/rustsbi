use crate::memory_range::Physical;
use spec::binary::SbiRet;

/// Debug Console Extension
///
/// The debug console extension defines a generic mechanism for debugging
/// and boot-time early prints from supervisor-mode software.
///
/// Kernel developers should switch to driver based console implementation
/// instead of using this extension to prevent possible race conditions between
/// the firmware and the kernel when drivers are ready.
///
/// If the underlying physical console has extra bits for error checking
/// (or correction) then these extra bits should be handled by the SBI
/// implementation.
///
/// *NOTE:* It is recommended that bytes sent/received using the debug
/// console extension follow UTF-8 character encoding.
pub trait Console {
    /// Write bytes to the debug console from input memory.
    ///
    /// # Parameters
    ///
    /// The `bytes` parameter specifies the input memory, including its length
    /// and memory physical base address (both lower and upper bits).
    ///
    /// # Non-blocking function
    ///
    /// This is a non-blocking SBI call and it may do partial/no writes if
    /// the debug console is not able to accept more bytes.
    ///
    /// # Return value
    ///
    /// The number of bytes written is returned in `SbiRet.value` and the
    /// possible return error codes returned in `SbiRet.error` are shown in
    /// the table below:
    ///
    /// | Return code               | Description
    /// |:--------------------------|:----------------------------------------------
    /// | `SbiRet::success()`       | Bytes written successfully.
    /// | `SbiRet::invalid_param()` | The memory pointed to by `bytes` does not satisfy the [requirements](struct.Physical.html#Requirements) described in shared memory physical address range.
    /// | `SbiRet::failed()`        | Failed to write due to I/O errors.
    fn write(&self, bytes: Physical<&[u8]>) -> SbiRet;
    /// Read bytes from the debug console into an output memory.
    ///
    /// # Parameters
    ///
    /// The `bytes` parameter specifies the output memory, including the maximum
    /// bytes which can be written, and its memory physical base address
    /// (both lower and upper bits).
    ///
    /// # Non-blocking function
    ///
    /// This is a non-blocking SBI call and it will not write anything
    /// into the output memory if there are no bytes to be read in the
    /// debug console.
    ///
    /// # Return value
    ///
    /// The number of bytes read is returned in `SbiRet.value` and the
    /// possible return error codes returned in `SbiRet.error` are shown in
    /// the table below:
    ///
    /// | Return code               | Description
    /// |:--------------------------|:----------------------------------------------
    /// | `SbiRet::success()`       | Bytes read successfully.
    /// | `SbiRet::invalid_param()` | The memory pointed to by `bytes` does not satisfy the [requirements](struct.Physical.html#Requirements) described in shared memory physical address range.
    /// | `SbiRet::failed()`        | Failed to read due to I/O errors.
    fn read(&self, bytes: Physical<&mut [u8]>) -> SbiRet;
    /// Write a single byte to the debug console.
    ///
    /// # Blocking function
    ///
    /// This is a blocking SBI call and it will only return after writing
    /// the specified byte to the debug console. It will also return, with
    /// `SbiRet::failed()`, if there are I/O errors.
    /// # Return value
    ///
    /// The `SbiRet.value` is set to zero and` and the possible return error
    /// codes returned in `SbiRet.error` are shown in the table below:
    ///
    /// | Return code               | Description
    /// |:--------------------------|:----------------------------------------------
    /// | `SbiRet::success()`       | Byte written successfully.
    /// | `SbiRet::failed()`        | Failed to write the byte due to I/O errors.
    fn write_byte(&self, byte: u8) -> SbiRet;
}
