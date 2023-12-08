use bounded_collections::{BoundedVec, ConstU32};
use parity_scale_codec::{Decode, Encode};
use wasm_bindgen::prelude::*;

use crate::demo_impls::Trivial;
use crate::{GenerateVerifiable, Entropy};
use js_sys::{Object, Uint8Array};



#[wasm_bindgen]
pub fn one_shot_trivial(entropy: Uint8Array, members: Uint8Array) -> Object {
    let entropy_vec = entropy.to_vec();
    let entropy = Entropy::decode(&mut &entropy_vec[..]).unwrap();


	// Get Secret from Entropy
	let secret = Trivial::new_secret(entropy);

	// Get Member from Secret
	let member = Trivial::member_from_secret(&secret);
    let member_encoded = Encode::encode(&member);

	let raw_members = members.to_vec();
    let mut members:BoundedVec<<Trivial as GenerateVerifiable>::Member, ConstU32<{u32::MAX}>> = Decode::decode(&mut &raw_members[..]).unwrap();
    members.try_push(member.clone()).unwrap();
    let members_encoded = Encode::encode(&members);

	// // Get Commitment from Member
	let commitment = Trivial::open(&member, members.into_iter()).expect("Error during open");

	// // Get Proof & Alias from Commitment
	let (proof,alias) = Trivial::create(commitment, &secret, &[23u8], &[42u8]).expect("Error during open");

	let obj = Object::new();
	js_sys::Reflect::set(&obj, &"member".into(), &Uint8Array::from(&member_encoded[..])).unwrap();
    js_sys::Reflect::set(&obj, &"members".into(), &Uint8Array::from(&members_encoded[..])).unwrap();
    js_sys::Reflect::set(&obj, &"proof".into(), &Uint8Array::from(&proof[..])).unwrap();
    js_sys::Reflect::set(&obj, &"alias".into(), &Uint8Array::from(&alias[..])).unwrap();

	obj
}


#[wasm_bindgen]
pub fn new_secret_trivial(entropy_input: Uint8Array) -> Uint8Array {
    todo!()
}

#[wasm_bindgen]
pub fn member_from_secret_trivial(secret_input: Uint8Array) -> Uint8Array {
    todo!()
}

#[wasm_bindgen]
pub fn open_trivial(member_input: Uint8Array, members_input: Uint8Array) -> Object {
	todo!()
}

#[wasm_bindgen]
pub fn create_trivial(
    commitment: Uint8Array,
    secret: Uint8Array,
    context: Uint8Array,
    message: Uint8Array,
) {
    todo!()
}

#[wasm_bindgen]
pub fn validate_trivial(proof: Uint8Array, members: Uint8Array, context: Uint8Array, message: Uint8Array) {
    todo!()
}


