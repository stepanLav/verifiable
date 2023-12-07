
use ark_scale::ArkScale;
use bandersnatch_vrfs::PublicKey;
use wasm_bindgen::prelude::*;

use crate::{demo_impls::Trivial, ring_vrf_impl::BandersnatchVrfVerifiable};
use crate::GenerateVerifiable;
use js_sys::{Array, Object, Uint8Array, Boolean};


fn to_u8_32(input:Uint8Array) -> [u8; 32] {
	let mut entropy = [0u8; 32];
	entropy.copy_from_slice(&input.to_vec());
	entropy
}

// fn to_u8_32_chunks(input: Uint8Array) -> impl Iterator<Item = [u8; 32]> {
//     input.clone().to_vec().clone().chunks(32).map(|chunk| {
//             let mut m = [0u8; 32];
//             m.copy_from_slice(&chunk);
//             m
//         }).clone().to_owned()
// }

#[wasm_bindgen]
pub fn all(entropy: Uint8Array, members:Uint8Array) -> Object {
	let entropy = to_u8_32(entropy);

	// Get Secret from Entropy
	let secret = BandersnatchVrfVerifiable::new_secret(entropy);

	// Get Member from Secret
	let member = BandersnatchVrfVerifiable::member_from_secret(&secret);

	let raw_members = members.to_vec();
    let members_iter =
        raw_members.chunks(32).map(|chunk| {
			let reader = &mut &chunk[..];
			// this fails for now :S
			// ArkScale::from(PublicKey::deserialize(reader).expect("Invalid public key"))
			1
		});



	// // Get Commitment from Member
	// let commitment = BandersnatchVrfVerifiable::open(&member, members_iter).expect("Error during open");

	// // Get Proof & Alias from Commitment
	// let (proof,alias) = BandersnatchVrfVerifiable::create(commitment, &secret, &[23u8], &[42u8]).expect("Error during open");

	let obj = Object::new();
	js_sys::Reflect::set(&obj, &"proof".into(), &Boolean::from(true)).unwrap();
	// js_sys::Reflect::set(&obj, &"proof".into(), &Uint8Array::from(&proof[..])).unwrap();
	// js_sys::Reflect::set(&obj, &"alias".into(), &Uint8Array::from(&alias[..])).unwrap();
	obj
}

#[wasm_bindgen]
pub fn new_secret(entropy_input: Uint8Array) -> Uint8Array {
    let mut entropy = [0u8; 32];
    entropy.copy_from_slice(&entropy_input.to_vec());

    let secret = Trivial::new_secret(entropy);

	let secret_snatch = BandersnatchVrfVerifiable::new_secret(entropy);

	let public = secret_snatch.as_publickey();
	if public.0.is_on_curve() {
		let mut secret_bytes = [0u8; 32];
		secret_bytes.copy_from_slice(&secret.to_vec()[..]);

		Uint8Array::from(&secret_bytes[..])
	}else{
		Uint8Array::new(&JsValue::from("value is not on curve"))
	}
}

#[wasm_bindgen]
pub fn member_from_secret(secret_input: Uint8Array) -> Uint8Array {
    let mut secret = [0u8; 32];
    secret.copy_from_slice(&secret_input.to_vec());

    let member = Trivial::member_from_secret(&secret);
    // create Uint8array from secret

    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&member.to_vec()[..]);

    Uint8Array::from(&bytes[..])
}

#[wasm_bindgen]
pub fn open(member_input: Uint8Array, members_input: Uint8Array) -> Object {
	// Handle Input
    let mut member = [0u8; 32];

    member.copy_from_slice(&member_input.to_vec());

    let raw_members = members_input.to_vec();
    let members_iter =
        raw_members.chunks(32).map(|chunk| {
            let mut m = [0u8; 32];
            m.copy_from_slice(&chunk);
            m
        });

	// Actual Function Call
    let result = Trivial::open(&member, members_iter);

	// Handle Output
    let output = match result {
        Ok((me, members)) => {



            let me = Uint8Array::from(&me[..]);

            let array = Array::new();
            for member in members.iter() {
                let member = Uint8Array::from(&member[..]);
                array.push(&member);
            }

			let obj = Object::new();
            js_sys::Reflect::set(&obj, &"me".into(), &me).unwrap();
            js_sys::Reflect::set(&obj, &"members".into(), &array).unwrap();
            obj
        }
        Err(_) => Object::new(),
    };

    output
}

#[wasm_bindgen]
pub fn create(
    commitment: Uint8Array,
    secret: Uint8Array,
    context: Uint8Array,
    message: Uint8Array,
) {
    todo!()
}

#[wasm_bindgen]
pub fn validate(proof: Uint8Array, members: Uint8Array, context: Uint8Array, message: Uint8Array) {
    todo!()
}

// impl TryFrom<Entropy> for Uint8Array {
//     type Error = ();

//     fn try_from(input: Entropy) -> Result<Self, Self::Error> {
//         Ok(Uint8Array::from(&input[..]))
//     }
// }
