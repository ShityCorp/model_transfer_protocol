use std::sync::mpsc::RecvTimeoutError;

use getrandom::{SysRng, rand_core::UnwrapErr};
use rand_core::TryRng;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey, ed25519::Error};

fn generate_private_key() -> SigningKey {
    // Generate a Private and Public Key use for encryption and recognition between hosts
    return SigningKey ::generate(&mut UnwrapErr(SysRng));
}

fn generate_public_key(key:SigningKey ) -> VerifyingKey{
    // Generate a public key for a private key
    return key.verifying_key();
}

fn sign_message(key:SigningKey, message:&[u8]) -> Signature{
    // Return a signed message from a private key
    let msg :Signature= key.sign(message);
    assert!(key.verify(message, &msg).is_ok());
    return msg;
}

fn is_proper_signature(public_key:VerifyingKey, nonce:i32, signed_nonce:Signature) -> bool{
    return public_key.verify(&nonce.to_le_bytes(), &signed_nonce).is_ok();
}

fn byte_to_signature(bytes:&[u8]) -> Signature{
    match Signature::from_slice(bytes) {
        Ok(s) => return s,
        Err(_) => panic!("Was not a 64 byte packet") // TODO: find a better way of handling it
    }
}