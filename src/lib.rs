pub mod message;
pub mod types;
pub use tlv::prelude::*;

pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::message::*;

    #[test]
    fn test_nas_registration_request() {
        // The provided bytes for NasRegistrationRequest
        let mut bytes = Bytes::from_static(&[0x7e, 0x0, 0x41, 0x79, 0x0, 0xc, 0x1, 0x2, 0xf8, 
            0x39, 0xf0, 0xff, 0x0, 0x0, 0x0, 0x0, 0x47, 0x78, 0x2e, 0x2, 0x80, 0x20
        ]);

        // Decode the bytes into a NasRegistrationRequest
        let decoded = NasRegistrationRequest::decode(bytes.len(), &mut bytes.clone()).expect("Failed to decode NasRegistrationRequest");
        
        println!("Decoded NasRegistrationRequest: {:#?}", decoded);

        // Create a BytesMut buffer for encoding
        let mut buffer = BytesMut::new();
        
        // Re-encode the decoded message
        let encoded = decoded.encode(&mut buffer).expect("Failed to encode NasRegistrationRequest");
        
        println!("Original bytes: {:02X?}", bytes);
        println!("Re-encoded bytes: {:02X?}", encoded);
        
        // Verify that the re-encoded message matches the original bytes
        assert_eq!(bytes.to_vec(), buffer.to_vec(), "Re-encoded message doesn't match original bytes");
    }

    #[test]
    fn test_snow_3g_f8() {
        let mut key: [u8; 16] = [
            0x2B, 0xD6, 0x45, 0x9F, 0x82, 0xC5, 0xB3, 0x00,
            0x95, 0x2C, 0x49, 0x10, 0x48, 0x81, 0xFF, 0x48,
        ];
        
        let mut data: [u8; 16] = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF,
            0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32, 0x10,
        ];
        
        let original_data = data.clone();
        
        unsafe {
            crate::bindings::snow_3g_f8(
                key.as_mut_ptr(),
                0x72A4F20F,
                0x0C,
                1,
                data.as_mut_ptr(),
                128
            );
        }
        
        println!("Original:  {:02X?}", original_data);
        println!("Encrypted: {:02X?}", data);
        
        unsafe {
            crate::bindings::snow_3g_f8(
                key.as_mut_ptr(),
                0x72A4F20F,
                0x0C,
                1,
                data.as_mut_ptr(),
                128
            );
        }
        
        println!("Decrypted: {:02X?}", data);
        
        assert_eq!(data, original_data);
    }
}
