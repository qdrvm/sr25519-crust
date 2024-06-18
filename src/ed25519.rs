use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::{
    Keypair, PublicKey, SecretKey, Signer, Verifier, KEYPAIR_LENGTH, PUBLIC_KEY_LENGTH,
    SIGNATURE_LENGTH,
};
use std::os::raw::c_ulong;
use std::{ptr, slice};

use crate::constants::ED25519_SEED_LENGTH;

/// Status code of a function call
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Ed25519Result {
    /// Success
    Ok = 0,
    /// a pointer argument passed into function is null
    NullArgument,
    /// decoding a keypair from bytes failed
    KeypairFromBytesFailed,
    /// decoding a public key from bytes failed
    PublicKeyFromBytesFailed,
    /// decoding a signature from bytes failed
    SignatureFromBytesFailed,
    /// sign operation failed
    SignFailed,
    /// signature verification failed
    VerificationFailed,
}

/**
 * Generate a keypair using the provided seed
 * @arg seed_ptr - the seed that will be used as a secret key
 */
#[no_mangle]
pub unsafe extern "C" fn ed25519_keypair_from_seed(keypair_out: *mut u8, seed_ptr: *const u8) {
    if keypair_out.is_null() || seed_ptr.is_null() {
        return;
    }

    let seed = slice::from_raw_parts(seed_ptr, ED25519_SEED_LENGTH as usize);
    let secret_key = SecretKey::from_bytes(seed)
        .expect("SecretKey::from_bytes may throw only if the size of the supplied seed is wrong.");
    let public_key: PublicKey = (&secret_key).into();
    let keypair = Keypair {
        public: public_key,
        secret: secret_key,
    };
    ptr::copy_nonoverlapping(keypair.to_bytes().as_ptr(), keypair_out, KEYPAIR_LENGTH);
}

/**
 * Sign the message using the provided keypair
 * @returns a status code as the function return value, a signature as an output parameter
 */
#[no_mangle]
pub unsafe extern "C" fn ed25519_sign(
    signature_out: *mut u8,
    keypair_ptr: *const u8,
    message_ptr: *const u8,
    message_size: c_ulong,
) -> Ed25519Result {
    if keypair_ptr.is_null() || message_ptr.is_null() {
        return Ed25519Result::NullArgument;
    }
    let message = slice::from_raw_parts(message_ptr, message_size as usize);
    let keypair_bytes = slice::from_raw_parts(keypair_ptr, KEYPAIR_LENGTH);
    let keypair = match Keypair::from_bytes(keypair_bytes) {
        Ok(kp) => kp,
        Err(_) => return Ed25519Result::KeypairFromBytesFailed,
    };
    let signature = match keypair.try_sign(message) {
        Ok(sign) => sign,
        Err(e) => return Ed25519Result::SignFailed,
    };
    if keypair.verify(message, &signature).is_ok() {
        ptr::copy_nonoverlapping(
            signature.to_bytes().as_ptr(),
            signature_out,
            SIGNATURE_LENGTH,
        );
        Ed25519Result::Ok
    } else {
        Ed25519Result::VerificationFailed
    }
}

/**
 * Verify a signature of a message using provided public key
 */
#[no_mangle]
pub unsafe extern "C" fn ed25519_verify(
    signature_ptr: *const u8,
    public_key_ptr: *const u8,
    message_ptr: *const u8,
    message_size: c_ulong,
) -> Ed25519Result {
    if public_key_ptr.is_null() || signature_ptr.is_null() || message_ptr.is_null() {
        return Ed25519Result::NullArgument;
    }
    let public_key_bytes = slice::from_raw_parts(public_key_ptr, PUBLIC_KEY_LENGTH);
    let signature_bytes = slice::from_raw_parts(signature_ptr, SIGNATURE_LENGTH);
    let message_bytes = slice::from_raw_parts(message_ptr, message_size as usize);
    let public_key = match PublicKey::from_bytes(public_key_bytes) {
        Ok(pk) => pk,
        Err(_) => return Ed25519Result::PublicKeyFromBytesFailed,
    };
    let signature = match Signature::from_bytes(signature_bytes) {
        Ok(sign) => sign,
        Err(_) => return Ed25519Result::SignatureFromBytesFailed,
    };
    if public_key.verify(message_bytes, &signature).is_ok() {
        Ed25519Result::Ok
    } else {
        Ed25519Result::VerificationFailed
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::{ED25519_KEYPAIR_LENGTH, ED25519_SIGNATURE_LENGTH};

    use super::*;
    use rand_chacha::rand_core::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    fn generate_random_seed() -> Vec<u8> {
        (0..32).map(|_| rand::random::<u8>()).collect()
    }

    #[test]
    fn can_create_keypair() {
        let seed = generate_random_seed();
        let mut keypair = [0u8; ED25519_KEYPAIR_LENGTH as usize];
        unsafe { ed25519_keypair_from_seed(keypair.as_mut_ptr(), seed.as_ptr()) };

        assert_eq!(keypair.len(), KEYPAIR_LENGTH);
    }

    #[test]
    fn dalek_sign_verify() {
        let mut csprng = ChaCha20Rng::from_entropy();
        let kp = Keypair::generate(&mut csprng);
        let message = "Hello, world!\n";
        let s = kp.try_sign(message.as_ref()).unwrap();
        kp.public.verify(message.as_ref(), &s).unwrap();
    }

    #[test]
    fn sign_verify() {
        let seed = generate_random_seed();
        let mut keypair = [0u8; ED25519_KEYPAIR_LENGTH as usize];
        unsafe { ed25519_keypair_from_seed(keypair.as_mut_ptr(), seed.as_ptr()) };

        let mut signature = [0u8; ED25519_SIGNATURE_LENGTH as usize];
        let message = "Hello, world!\n";
        unsafe {
            let status = ed25519_sign(
                signature.as_mut_ptr(),
                keypair.as_ptr(),
                message.as_ptr(),
                message.len() as u64,
            );
            assert_eq!(status, Ed25519Result::Ok);
        }
        unsafe {
            let status = ed25519_verify(
                signature.as_mut_ptr(),
                keypair[32..].as_ptr(),
                message.as_ptr(),
                message.len() as u64,
            );
            assert_eq!(status, Ed25519Result::Ok);
        }
    }
}
