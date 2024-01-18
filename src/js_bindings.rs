use crate::ring_vrf_impl::BandersnatchVrfVerifiable;
use crate::{Entropy, GenerateVerifiable};
use ark_scale::ArkScale;
use ark_serialize::CanonicalDeserialize;
use bandersnatch_vrfs::ring::StaticVerifierKey;
use bounded_collections::{BoundedVec, ConstU32};
use js_sys::{Object, Uint8Array};
use parity_scale_codec::{Decode, Encode};
use wasm_bindgen::prelude::*;
#[cfg(feature = "small-ring")]
const ONCHAIN_VK: &[u8] = include_bytes!("ring-data/zcash-9.vk");
#[cfg(not(feature = "small-ring"))]
const ONCHAIN_VK: &[u8] = include_bytes!("ring-data/zcash-16.vk");

#[wasm_bindgen]
pub fn one_shot(
	entropy: Uint8Array,
	members: Uint8Array,
	context: Uint8Array,
	message: Uint8Array,
) -> Object {
	// store entropy instead of key is fine
	let entropy_vec = entropy.to_vec();
	let entropy = Entropy::decode(&mut &entropy_vec[..]).unwrap();

	// Secret
	let secret = BandersnatchVrfVerifiable::new_secret(entropy);

	// Member
	let member = BandersnatchVrfVerifiable::member_from_secret(&secret);
	let member_encoded = Encode::encode(&member);

	// All Members
	let raw_members = members.to_vec();
	let members: BoundedVec<
		<BandersnatchVrfVerifiable as GenerateVerifiable>::Member,
		ConstU32<{ u32::MAX }>,
	> = Decode::decode(&mut &raw_members[..]).unwrap();

	let members_encoded = Encode::encode(&members);

	// Open
	let commitment =
		BandersnatchVrfVerifiable::open(&member, members.into_iter()).expect("Error during open");

	// TODO I find this quite fishy!, Why use the commitment as the secret?
	let secret = BandersnatchVrfVerifiable::new_secret([commitment.0 as u8; 32]);

	// Create
	let context = &context.to_vec()[..];
	let message = &message.to_vec()[..];
	let (proof, alias) = BandersnatchVrfVerifiable::create(commitment, &secret, context, message)
		.expect("Error during open");

	// Return Results
	let obj = Object::new();
	js_sys::Reflect::set(
		&obj,
		&"member".into(),
		&Uint8Array::from(&member_encoded[..]),
	)
	.unwrap();
	js_sys::Reflect::set(
		&obj,
		&"members".into(),
		&Uint8Array::from(&members_encoded[..]),
	)
	.unwrap();
	js_sys::Reflect::set(&obj, &"proof".into(), &Uint8Array::from(&proof[..])).unwrap();
	js_sys::Reflect::set(&obj, &"alias".into(), &Uint8Array::from(&alias[..])).unwrap();
	js_sys::Reflect::set(&obj, &"message".into(), &Uint8Array::from(&message[..])).unwrap();
	js_sys::Reflect::set(&obj, &"context".into(), &Uint8Array::from(&context[..])).unwrap();
	obj
}

#[wasm_bindgen]
pub fn validate(
	proof: Uint8Array,
	members: Uint8Array,
	context: Uint8Array,
	message: Uint8Array,
) -> Uint8Array {
	let proof = proof.to_vec();
	let proof: <BandersnatchVrfVerifiable as GenerateVerifiable>::Proof =
		Decode::decode(&mut &proof[..]).unwrap();

	let members = members.to_vec();
	let members: BoundedVec<
		<BandersnatchVrfVerifiable as GenerateVerifiable>::Member,
		ConstU32<{ u32::MAX }>,
	> = Decode::decode(&mut &members[..]).unwrap();

	// TODO ok as validation is only happening on chain right? Otherwise expose vk?
	let vk = StaticVerifierKey::deserialize_uncompressed_unchecked(ONCHAIN_VK).unwrap();
	let get_one = |i| Ok(ArkScale(vk.lag_g1[i]));

	let mut inter = BandersnatchVrfVerifiable::start_members();
	members.iter().for_each(|member| {
		BandersnatchVrfVerifiable::push_member(&mut inter, member.clone(), get_one).unwrap();
	});
	let members_commitment = BandersnatchVrfVerifiable::finish_members(inter);

	let context = &context.to_vec()[..];
	let message = &message.to_vec()[..];
	let alias =
		BandersnatchVrfVerifiable::validate(&proof, &members_commitment, context, message).unwrap();

	Uint8Array::from(&Encode::encode(&alias)[..])
}

#[wasm_bindgen]
pub fn new_secret(entropy_input: Uint8Array) -> Uint8Array {
	todo!()
}

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

use wasm_bindgen_test::*;
#[wasm_bindgen_test]
fn equal_members() {
	let entropy = [0u8; 32];
	let alice_secret = BandersnatchVrfVerifiable::new_secret(entropy);
	let rust_member = BandersnatchVrfVerifiable::member_from_secret(&alice_secret);

	let js_member = member_from_entropy(Uint8Array::from(&entropy[..]));

	assert_eq!(rust_member.encode().len(), js_member.to_vec().len());
	assert_eq!(rust_member.encode().len(), 33);
	assert_eq!(js_member.to_vec().len(), 33);
	assert_eq!(rust_member.encode(), js_member.to_vec());
}

#[wasm_bindgen_test]
fn equal_proofs() {
	let get_secret_and_member = |entropy: &[u8; 32]| {
		let secret = BandersnatchVrfVerifiable::new_secret(entropy.clone());
		let member = BandersnatchVrfVerifiable::member_from_secret(&secret);
		(secret, member)
	};

	let alice_entropy = [0u8; 32];
	let bob_entropy = [1u8; 32];
	let (alice_secret, alice_member) = get_secret_and_member(&alice_entropy);
	let (bob_secret, bob_member) = get_secret_and_member(&bob_entropy);

	let members = vec![alice_member.clone(), bob_member.clone()];

	let context = b"Context";
	let message = b"FooBar";

	let commitment =
		BandersnatchVrfVerifiable::open(&alice_member, members.clone().into_iter()).unwrap();
	let secret = BandersnatchVrfVerifiable::new_secret([commitment.0 as u8; 32]);
	let (proof, alias) =
		BandersnatchVrfVerifiable::create(commitment, &secret, context, message).unwrap();

	let result = one_shot(
		Uint8Array::from(&alice_entropy[..]),
		Uint8Array::from(&members.encode().to_vec()[..]),
		Uint8Array::from(&context[..]),
		Uint8Array::from(&message[..]),
	);

	let js_alias =
		js_sys::Reflect::get(&result, &JsValue::from_str("alias")).expect("alias should exist");
	let js_alias = Uint8Array::new(&js_alias);
	assert_eq!(js_alias.to_vec(), alias.to_vec());

	let js_member =
		js_sys::Reflect::get(&result, &JsValue::from_str("member")).expect("members should exist");
	let js_member = Uint8Array::new(&js_member);
	assert_eq!(js_member.to_vec(), alice_member.encode().to_vec());

	let js_members =
		js_sys::Reflect::get(&result, &JsValue::from_str("members")).expect("members should exist");
	let js_members = Uint8Array::new(&js_members);
	assert_eq!(js_members.to_vec(), members.encode().to_vec());

	let js_proof =
		js_sys::Reflect::get(&result, &JsValue::from_str("proof")).expect("proof should exist");
	let js_proof = Uint8Array::new(&js_proof);

	assert_eq!(js_proof.to_vec().len(), proof.len());
	assert_eq!(js_proof.to_vec(), proof.to_vec());
}
