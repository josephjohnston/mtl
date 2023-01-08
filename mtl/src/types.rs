use super::*;

// declare!(NSString);
// impl NSString {
//     const UTF8_ENCODING_NUM: usize = 4;
//     fn class() -> &'static Class {
//         class!(NSString)
//     }
//     pub fn to_str(&self) -> &str {
//         let bytes = unsafe {
//             let data: *const u8 = msg_send![self, cStringUsingEncoding: Self::UTF8_ENCODING_NUM];
//             let len: usize = msg_send![self, lengthOfBytesUsingEncoding: Self::UTF8_ENCODING_NUM];
//             std::slice::from_raw_parts(data, len)
//         };
//         std::str::from_utf8(bytes).unwrap()
//     }
//     pub fn from_str(string: &str) -> Id<NSString, Shared> {
//         let bytes: *const c_void = string.as_ptr().cast();
//         unsafe {
//             let allocated_object: *mut Object = msg_send![Self::class(), alloc];
//             let initialized_object = msg_send![
//                 allocated_object,
//                 initWithBytes: bytes,
//                 length: string.len(),
//                 encoding: Self::UTF8_ENCODING_NUM
//             ];
//             Id::new(initialized_object).expect(ID_NEW_FAILURE)
//         }
//     }
// }

// // declare!(NSArray);
// #[repr(C)]
// pub struct NSArray<T>(T);
// unsafe impl<T> RefEncode for NSArray<T> {
//     const ENCODING_REF: Encoding = Object::ENCODING_REF;
// }
// unsafe impl<T> Message for NSArray<T> {}
// impl<T> NSArray<T> {
//     fn class() -> &'static Class {
//         class!(NSArray)
//     }
//     fn count(&self) -> usize {
//         unsafe { msg_send![self, count] }
//     }
//     fn add(t: &T) {}
//     // fn map
// }

declare!(NSURL);
impl NSURL {
    // Creating an NSURL Object
    pub fn url_with_string(url_string: &NSString) -> Id<NSURL> {
        // let string = NSString::from_str(url_string);
        // unsafe {
        //     let raw_url: *mut Self = msg_send![class!(NSURL), URLWithString: string.as_ref()];
        //     Id::retain_autoreleased(raw_url).unwrap()
        // }
        unsafe { msg_send_id![class!(NSURL), URLWithString: url_string] }
    }
    pub fn url_with_string_relative_to_url(url_string: &NSString, base_url: &NSURL) -> Id<NSURL> {
        unsafe {
            // let raw_url: *mut Self = msg_send![
            //     class!(NSURL),
            //     URLWithString: url_string,
            //     relativeToURL: base_url
            // ];
            // Id::retain_autoreleased(raw_url).unwrap()
            msg_send_id![
                class!(NSURL),
                URLWithString: url_string,
                relativeToURL: base_url
            ]
        }
    }
    // Accessing the Parts of the URL
    pub fn absolute_string(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, absoluteString] }
    }
    pub fn relative_path(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, relativeString] }
    }
    pub fn relative_string(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, relativePath] }
    }
}

// declare!(NSError);
// impl NSError {
//     pub fn localized_description(&self) -> Id<NSString, Shared> {
//         unsafe {
//             let desc = msg_send![self, localizedDescription];
//             Id::new(desc).expect(ID_NEW_FAILURE)
//         }
//     }
// }

// #[repr(C)]
// pub struct NSRange {
//     location: usize,
//     length: usize,
// }
// unsafe impl Encode for NSRange {
//     const ENCODING: Encoding = Encoding::Struct("NSRange", &[usize::ENCODING, usize::ENCODING]);
// }
