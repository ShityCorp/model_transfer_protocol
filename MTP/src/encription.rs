// This file manage all of the encryption keys for the network communication
use getrandom::{SysRng, rand_core::UnwrapErr};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

/// Generate a Private and Public Key use for encryption and recognition between hosts
fn generate_private_key() -> SigningKey {    
    return SigningKey ::generate(&mut UnwrapErr(SysRng));
}

/// Generate a public key for a private key
fn generate_public_key(key:SigningKey ) -> VerifyingKey{
    return key.verifying_key();
}

/// Return a signed message from a private key
fn sign_message(key:SigningKey, message:&[u8]) -> Signature{
    let msg :Signature= key.sign(message);
    assert!(key.verify(message, &msg).is_ok());
    return msg;
}

/// Verify the signature come from a specific key
fn is_proper_signature(public_key:VerifyingKey, nonce:i32, signed_nonce:Signature) -> bool{
    return public_key.verify(&nonce.to_le_bytes(), &signed_nonce).is_ok();
}


/// change bytes to signature
fn byte_to_signature(bytes:&[u8]) -> Signature{
    match Signature::from_slice(bytes) {
        Ok(s) => return s,
        Err(_) => panic!("Was not a 64 byte packet") // TODO: find a better way of handling it
    }
}

/// Save specific key to the key ring
fn save_key_to_keyring(){

}