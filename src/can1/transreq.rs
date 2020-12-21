#[doc = "Reader of register TRANSREQ"]
pub type R = crate::R<u32, super::TRANSREQ>;
#[doc = "Transmission Request Bits (Of All Message Objects)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum TXRQSTOUT_A {
    #[doc = "0: This Message Object is not waiting for transmission."]
    FALSE = 0,
    #[doc = "1: The transmission of this Message Object is requested and is not yet done."]
    TRUE = 1,
}
impl From<TXRQSTOUT_A> for u32 {
    #[inline(always)]
    fn from(variant: TXRQSTOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXRQSTOUT`"]
pub type TXRQSTOUT_R = crate::R<u32, TXRQSTOUT_A>;
impl TXRQSTOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, TXRQSTOUT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXRQSTOUT_A::FALSE),
            1 => Val(TXRQSTOUT_A::TRUE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline(always)]
    pub fn is_false_(&self) -> bool {
        *self == TXRQSTOUT_A::FALSE
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true_(&self) -> bool {
        *self == TXRQSTOUT_A::TRUE
    }
}
impl R {
    #[doc = "Bits 0:31 - Transmission Request Bits (Of All Message Objects)"]
    #[inline(always)]
    pub fn txrqstout(&self) -> TXRQSTOUT_R {
        TXRQSTOUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
