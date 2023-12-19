use ark_scale::ArkScale;
use ark_serialize::CanonicalDeserialize;
use bandersnatch_vrfs::ring::StaticVerifierKey;
use bounded_collections::{BoundedVec, ConstU32};
use parity_scale_codec::{Decode, Encode};
use wasm_bindgen::prelude::*;
use crate::ring_vrf_impl::BandersnatchVrfVerifiable;
use crate::{GenerateVerifiable, Entropy};
use js_sys::{Object, Uint8Array};
#[cfg(feature = "small-ring")]
const ONCHAIN_VK: &[u8] = include_bytes!("ring-data/zcash-9.vk");
#[cfg(not(feature = "small-ring"))]
const ONCHAIN_VK: &[u8] = include_bytes!("ring-data/zcash-16.vk");

#[wasm_bindgen]
pub fn one_shot(entropy: Uint8Array, members: Uint8Array, context:Uint8Array, message:Uint8Array) -> Object {
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
    let mut members:BoundedVec<<BandersnatchVrfVerifiable as GenerateVerifiable>::Member, ConstU32<{u32::MAX}>> = Decode::decode(&mut &raw_members[..]).unwrap();

    //TODO Add Self to Members, decide if we want to add here or before.
    members.try_push(member.clone()).unwrap();
    let members_encoded = Encode::encode(&members);

	// Open
	let commitment = BandersnatchVrfVerifiable::open(&member, members.into_iter()).expect("Error during open");

	// Create
    let context = &context.to_vec()[..];
    let message = &message.to_vec()[..];
	let (proof,alias) = BandersnatchVrfVerifiable::create(commitment, &secret, context, message).expect("Error during open");

    // Return Results
	let obj = Object::new();
	js_sys::Reflect::set(&obj, &"member".into(), &Uint8Array::from(&member_encoded[..])).unwrap();
    js_sys::Reflect::set(&obj, &"members".into(), &Uint8Array::from(&members_encoded[..])).unwrap();
    js_sys::Reflect::set(&obj, &"proof".into(), &Uint8Array::from(&proof[..])).unwrap();
    js_sys::Reflect::set(&obj, &"alias".into(), &Uint8Array::from(&alias[..])).unwrap();
    js_sys::Reflect::set(&obj, &"message".into(), &Uint8Array::from(&message[..])).unwrap();
    js_sys::Reflect::set(&obj, &"context".into(), &Uint8Array::from(&context[..])).unwrap();
	obj
}

#[wasm_bindgen]
pub fn validate(proof: Uint8Array, members: Uint8Array, context: Uint8Array, message: Uint8Array)-> Uint8Array {
    let proof = proof.to_vec();
    let proof: <BandersnatchVrfVerifiable as GenerateVerifiable>::Proof = Decode::decode(&mut &proof[..]).unwrap();

    let members = members.to_vec();
    let members:BoundedVec<<BandersnatchVrfVerifiable as GenerateVerifiable>::Member, ConstU32<{u32::MAX}>> = Decode::decode(&mut &members[..]).unwrap();

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

