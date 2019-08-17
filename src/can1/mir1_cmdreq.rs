#[doc = "Reader of register MIR1_CMDREQ"]
pub type R = crate::R<u32, super::MIR1_CMDREQ>;
#[doc = "Writer for register MIR1_CMDREQ"]
pub type W = crate::W<u32, super::MIR1_CMDREQ>;
#[doc = "Register MIR1_CMDREQ `reset()`'s with value 0x01"]
impl crate::ResetValue for super::MIR1_CMDREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `MSGNUM`"]
pub type MSGNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MSGNUM`"]
pub struct MSGNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> MSGNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:5 - Message Number"]
    #[inline(always)]
    pub fn msgnum(&self) -> MSGNUM_R {
        MSGNUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Message Number"]
    #[inline(always)]
    pub fn msgnum(&mut self) -> MSGNUM_W {
        MSGNUM_W { w: self }
    }
}
