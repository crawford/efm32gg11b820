#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TRANSREQ {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `TXRQSTOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRQSTOUTR {
    #[doc = "This Message Object is not waiting for transmission."]
    FALSE,
    #[doc = "The transmission of this Message Object is requested and is not yet done."]
    TRUE,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl TXRQSTOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            TXRQSTOUTR::FALSE => 0,
            TXRQSTOUTR::TRUE => 1,
            TXRQSTOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> TXRQSTOUTR {
        match value {
            0 => TXRQSTOUTR::FALSE,
            1 => TXRQSTOUTR::TRUE,
            i => TXRQSTOUTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline]
    pub fn is_false_(&self) -> bool {
        *self == TXRQSTOUTR::FALSE
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline]
    pub fn is_true_(&self) -> bool {
        *self == TXRQSTOUTR::TRUE
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Transmission Request Bits (Of All Message Objects)"]
    #[inline]
    pub fn txrqstout(&self) -> TXRQSTOUTR {
        TXRQSTOUTR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
