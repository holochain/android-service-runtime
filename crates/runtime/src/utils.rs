use crate::{RuntimeError, RuntimeResult};
use sodoken::{BufRead, BufWrite};
use zeroize::Zeroize;

/// Copy bytes into locked memory, then zeroize the bytes
/// For use in receiving passphrase input
pub fn move_to_locked_mem(mut bytes_tmp: Vec<u8>) -> RuntimeResult<BufRead> {
    match BufWrite::new_mem_locked(bytes_tmp.len()) {
        Err(e) => {
            bytes_tmp.zeroize();
            Err(RuntimeError::MoveToLockedMem(e))
        }
        Ok(p) => {
            {
                let mut lock = p.write_lock();
                lock.copy_from_slice(&bytes_tmp);
                bytes_tmp.zeroize();
            }
            Ok(p.to_read())
        }
    }
}
