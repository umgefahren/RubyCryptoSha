#[macro_use]
extern crate rutie;

use sha2::{Sha256, Digest};
use rutie::{Array, Class, Object, VM, AnyException, Exception, Fixnum};

fn internalHash(input: Array) -> Array {
    let mut hasher = Sha256::new();
    let mut result = Array::new();

    let enumerate = input.into_iter();
    for element in enumerate {
        if !element.is_fixnum() {
            VM::raise_ex(AnyException::new("StandardError", Some("Element in Array is not integer")));
        } else {
            let fixNumResult = element.try_convert_to::<Fixnum>();
            let fixNum = match fixNumResult {
                Ok(d) => d,
                Err(e) => {
                    VM::raise_ex(e);
                    panic!();
                }
            };
            let val: i64 = fixNum.to_i64();
            let valBytes = val.to_be_bytes();
            hasher.update(&valBytes);
        }
    }
    let hashResult = hasher.finalize();
    hashResult.into_iter().for_each(|e| {
        result.push(Fixnum::new(e.into()));
    });
    return result;
}

class!(RubyCryptoSha256);

methods!(
    RubyCryptoSha256,
    _rtself,
    fn hash(input: Array) -> Array {
        let inp = input.map_err(|e| VM::raise_ex(e)).unwrap();
        return internalHash(inp);
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_ruby_crypto_sha() {
    Class::new("RubyCryptoSha256", None).define(|klass| {
        klass.def_self("hash", hash);
    });
}
