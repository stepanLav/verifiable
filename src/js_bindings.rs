use bounded_collections::{BoundedVec, ConstU32};
use parity_scale_codec::{Decode, Encode};
use wasm_bindgen::prelude::*;
use crate::ring_vrf_impl::BandersnatchVrfVerifiable;
use crate::{GenerateVerifiable, Entropy};
use js_sys::{Object, Uint8Array};

#[wasm_bindgen]
pub fn one_shot(entropy: Uint8Array, members: Uint8Array) -> Object {
    let entropy_vec = entropy.to_vec();
    let entropy = Entropy::decode(&mut &entropy_vec[..]).unwrap();

    // Secret
	let secret = BandersnatchVrfVerifiable::new_secret(entropy);

    // Member
	let member = BandersnatchVrfVerifiable::member_from_secret(&secret);
    let member_encoded = Encode::encode(&member);

    // All Members
	let raw_members = members.to_vec();
    let mut members:BoundedVec<<BandersnatchVrfVerifiable as GenerateVerifiable>::Member, ConstU32<{u32::MAX}>> = Decode::decode(&mut &raw_members[..]).unwrap();
    // Add Self to Members
    members.try_push(member.clone()).unwrap();
    let members_encoded = Encode::encode(&members);

	// Open
	let commitment = BandersnatchVrfVerifiable::open(&member, members.into_iter()).expect("Error during open");

	// Create
	let (proof,alias) = BandersnatchVrfVerifiable::create(commitment, &secret, &[23u8], &[42u8]).expect("Error during open");

    // Return Results
	let obj = Object::new();
	js_sys::Reflect::set(&obj, &"member".into(), &Uint8Array::from(&member_encoded[..])).unwrap();
    js_sys::Reflect::set(&obj, &"members".into(), &Uint8Array::from(&members_encoded[..])).unwrap();
    js_sys::Reflect::set(&obj, &"proof".into(), &Uint8Array::from(&proof[..])).unwrap();
    js_sys::Reflect::set(&obj, &"alias".into(), &Uint8Array::from(&alias[..])).unwrap();
	obj
}

#[wasm_bindgen]
pub fn new_secret(entropy_input: Uint8Array) -> Uint8Array {
    todo!()
}

#[wasm_bindgen]
pub fn member_from_entropy(entropy: Uint8Array) -> Uint8Array {
    let entropy_vec = entropy.to_vec();
    let entropy = Entropy::decode(&mut &entropy_vec[..]).unwrap();

    // Secret
	let secret = BandersnatchVrfVerifiable::new_secret(entropy);

    // Member
	let member = BandersnatchVrfVerifiable::member_from_secret(&secret);
    let member_encoded = Encode::encode(&member);

    Uint8Array::from(&member_encoded[..])
}

#[wasm_bindgen]
pub fn member_from_secret(secret_input: Uint8Array) -> Uint8Array {
    todo!()
}

#[wasm_bindgen]
pub fn open(member_input: Uint8Array, members_input: Uint8Array) -> Object {
	todo!()
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
