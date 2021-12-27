use usbd_hid::descriptor::generator_prelude::*;

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = KEYBOARD) = {
        (usage_page = KEYBOARD, usage_min = 0xE0, usage_max = 0xE7) = {
            #[packed_bits 8] #[item_settings data,variable,absolute] modifier=input;
        };
        (usage_min = 0x00, usage_max = 0xFF) = {
            #[item_settings constant,variable,absolute] reserved=input;
        };
        (usage_page = LEDS, usage_min = 0x01, usage_max = 0x05) = {
            #[packed_bits 5] #[item_settings data,variable,absolute] leds=output;
        };
        // Boot keyboards can be at most 104 keys, and ignoring the modifier keys, the
        // max usage ID is 0x65. The data is represented as an array of 6 usage ids,
        // which means the keyboard is limited to 6 key rollover.
        (usage_page = KEYBOARD, usage_min = 0x00, usage_max = 0x65) = {
            #[item_settings data,array,absolute] keycodes=input;
        };
    }
)]
/// Boot keyboard reports are exclusively for compatibility with systems that only
/// support the 6-key rollover boot-protocol
#[derive(Default)]
pub struct BootKeyboardReport {
    pub modifier: u8,
    pub reserved: u8,
    pub leds: u8,
    pub keycodes: [u8; 6],
}

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = KEYBOARD) = {
        (usage_page = KEYBOARD, usage_min = 0xE0, usage_max = 0xE7) = {
            #[packed_bits 8] #[item_settings data,variable,absolute] modifier=input;
        };
        (usage_page = LEDS, usage_min = 0x01, usage_max = 0x05) = {
            #[packed_bits 5] #[item_settings data,variable,absolute] leds=output;
        };
        // The non-modifier keys stop at usage id 0xDD=221. By using bitfields aka
        // packed bits (where idx of the bit corresponds to the usageid of the active
        // key), we can support N-key rollover.
        (usage_page = KEYBOARD, usage_min = 0x00, usage_max = 0xDD) = {
            #[packed_bits 222] #[item_settings data,variable,absolute] keycodes=input;
        };
    }
)]
#[derive(Default)]
pub struct NkroKeyboardReport {
    pub modifier: u8,
    pub leds: u8,
    pub keycodes: [u8; 28],
}
