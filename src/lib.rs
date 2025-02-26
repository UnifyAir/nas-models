pub mod message;
pub mod types;

pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}


#[cfg(test)]
mod tests {

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
