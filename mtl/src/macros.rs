use super::*;

macro_rules! declare {
    ($name:ident) => {
        declare!($name: AnyObject);
    };
    ($name:ident: $parent:ident) => {
        #[derive(Debug)]
        #[repr(C)]
        pub struct $name($parent);
        unsafe impl RefEncode for $name {
            const ENCODING_REF: Encoding = $parent::ENCODING_REF;
        }
        unsafe impl Message for $name {}
        impl ::std::ops::Deref for $name {
            type Target = $parent;
            #[inline]
            fn deref(&self) -> &$parent {
                &self.0
            }
        }
    };
}

pub type TryNewCatch<T> = Result<Retained<T>, Retained<NSString>>;
macro_rules! try_new_catch {
    ($raw_error:ident<$T:ty> => $body:expr) => {
        unsafe {
            let mut $raw_error: *mut NSError = std::ptr::null_mut();
            let raw_t: *mut $T = $body;
            if $raw_error.is_null() {
                Ok(Retained::from_raw(raw_t).expect(ID_NEW_FAILURE))
            } else {
                let error: Retained<NSError> =
                    Retained::retain_autoreleased($raw_error).expect(ID_RETAIN_AUTO_FAILURE);
                Err(error.localizedDescription())
            }
        }
    };
}

pub type TryCatch = Result<(), Retained<NSString>>;
macro_rules! try_catch {
    ($raw_error:ident => $body:expr) => {
        unsafe {
            let mut $raw_error: *mut NSError = std::ptr::null_mut();
            let () = $body;
            if $raw_error.is_null() {
                Ok(())
            } else {
                let error: Retained<NSError> =
                    Retained::retain_autoreleased($raw_error).expect(ID_RETAIN_AUTO_FAILURE);
                Err(error.localizedDescription())
                // Err(NSString::from_str("oops"))
            }
        }
    };
}

macro_rules! impl_encode_for_type {
    ($name:ident: $type_name:ty) => {
        unsafe impl Encode for $name {
            const ENCODING: Encoding = <$type_name>::ENCODING;
        }
    };
}

pub(crate) use {declare, impl_encode_for_type, try_catch, try_new_catch};
