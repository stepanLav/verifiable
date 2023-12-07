
use wasm_bindgen::prelude::*;

use crate::demo_impls::Trivial;
use crate::GenerateVerifiable;
use js_sys::{Array, Object, Uint8Array};



#[wasm_bindgen]
pub fn new_array() -> Uint8Array {
    Uint8Array::new(&JsValue::from("Hello Array!"))
}

#[wasm_bindgen]
pub fn new_secret(entropy_input: Uint8Array) -> Uint8Array {
    let mut entropy = [0u8; 32];
    entropy.copy_from_slice(&entropy_input.to_vec());

    let secret = Trivial::new_secret(entropy);

    // create Uint8array from secret

    let mut secret_bytes = [0u8; 32];
    secret_bytes.copy_from_slice(&secret.to_vec()[..]);

    Uint8Array::from(&secret_bytes[..])
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
    let mut member = [0u8; 32];

    member.copy_from_slice(&member_input.to_vec());

    let raw_members = members_input.to_vec();

    let members_iter =
        raw_members.chunks(32).map(|chunk| {
            let mut m = [0u8; 32];
            m.copy_from_slice(&chunk);
            m
        });

    let result = Trivial::open(&member, members_iter);
    let output = match result {
        Ok((me, members)) => {
            let obj = Object::new();

            let mut me_tmp = [0u8; 32];
            me_tmp.copy_from_slice(&me.to_vec()[..]);
            let me = Uint8Array::from(&me_tmp[..]);

            let array = Array::new();
            for member in members.iter() {
                let mut member_tmp = [0u8; 32];
                member_tmp.copy_from_slice(&member.to_vec()[..]);
                let member = Uint8Array::from(&member_tmp[..]);
                array.push(&member);
            }

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
