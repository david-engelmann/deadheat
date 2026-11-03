//! The deadheat binary, a thin shell over the library.
//!
//! The arguments are taken as the operating system hands them
//! over, byte for byte, and decoded the way the Python runtime
//! decodes them, as PORT-7 states. The outcome of the library
//! dispatch is written out under the closed pipe rule PORT-12
//! states, amending PORT-10. The output the Python command
//! line accumulates waits in its writer and reaches the raw
//! stream in one write at the end, unless the accumulation
//! reaches 8192 bytes during the run, when the whole
//! accumulation is handed down and the raw write is attempted
//! while the command runs. On a pipe whose reader is already
//! gone the failure therefore lands in one of three bands. An
//! output of 8192 bytes or more fails during the run and
//! exits with status 1. An output from 4097 to 8191 bytes is
//! attempted only at the end and the failure is absorbed, the
//! failed attempt leaving nothing behind it, and the command
//! exits with its own status. An output of 4096 bytes or
//! fewer is attempted only at the end and the failure stands,
//! and the command exits with status 120. A failed write of
//! the error stream exits with status 120 at every size
//! probed. The traceback bytes of the Python failure stand
//! outside the reproduced behavior.

use std::io::Write;

/// The size the accumulation of the Python command line has
/// to reach for its write to be attempted during the run,
/// probed against the Python surface on the build machine,
/// where an output of exactly this many bytes is the smallest
/// whose write is attempted during the run.
const OUTPUT_BUFFER: usize = 8192;

/// The largest output whose failure at the end flush still
/// stands, probed against the Python surface on the build
/// machine. An output one byte larger is absorbed by its
/// failed end flush and the command keeps its own status.
const END_FLUSH_STANDS: usize = 4096;

#[cfg(unix)]
fn arg_bytes(arg: &std::ffi::OsString) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;
    arg.as_os_str().as_bytes().to_vec()
}

#[cfg(not(unix))]
fn arg_bytes(arg: &std::ffi::OsString) -> Vec<u8> {
    arg.to_string_lossy().into_owned().into_bytes()
}

/// Write the standard output of one run the way the writer
/// of the Python surface writes it, under the three bands
/// PORT-12 states. An output that reaches the buffer size is
/// attempted during the run, and any failure in writing it
/// out is the failure of the run, exit status 1. A smaller
/// output is attempted only at the end. There the failure of
/// an output past the standing size is absorbed, the write
/// is made and its failure let go, and the run keeps its own
/// status, while the failure of an output at the standing
/// size or under stands, exit status 120.
fn write_stdout(bytes: &[u8]) -> Result<(), i32> {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    if bytes.len() >= OUTPUT_BUFFER {
        let mut offset = 0;
        while offset < bytes.len() {
            let step = OUTPUT_BUFFER.min(bytes.len() - offset);
            if lock.write_all(&bytes[offset..offset + step]).is_err() {
                return Err(1);
            }
            offset += step;
        }
        if lock.flush().is_err() {
            return Err(1);
        }
        return Ok(());
    }
    if bytes.len() > END_FLUSH_STANDS {
        let _ = lock.write_all(bytes);
        let _ = lock.flush();
        return Ok(());
    }
    if lock.write_all(bytes).is_err() || lock.flush().is_err() {
        return Err(120);
    }
    Ok(())
}

fn main() {
    let args: Vec<deadheat::pytext::PText> = std::env::args_os()
        .skip(1)
        .map(|arg| deadheat::cli::decode_arg(&arg_bytes(&arg)))
        .collect();
    let columns = deadheat::cli::terminal_columns();
    let outcome = deadheat::cli::run(&args, columns);
    if let Err(code) = write_stdout(&outcome.stdout) {
        std::process::exit(code);
    }
    let stderr = std::io::stderr();
    let mut err_lock = stderr.lock();
    if err_lock.write_all(&outcome.stderr).is_err() || err_lock.flush().is_err() {
        std::process::exit(120);
    }
    std::process::exit(outcome.code);
}
