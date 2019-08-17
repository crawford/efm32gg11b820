#[doc = "Reader of register TXBDCTRL"]
pub type R = crate::R<u32, super::TXBDCTRL>;
#[doc = "Writer for register TXBDCTRL"]
pub type W = crate::W<u32, super::TXBDCTRL>;
#[doc = "Register TXBDCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TXBDCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXBDTSMODE`"]
pub type TXBDTSMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXBDTSMODE`"]
pub struct TXBDTSMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBDTSMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5 - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    pub fn txbdtsmode(&self) -> TXBDTSMODE_R {
        TXBDTSMODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - TX Descriptor Timestamp Insertion mode, 00: TS insertion disable, 01: TS inserted for PTP Event Frames only, 10: TS inserted for All PTP Frames only, 11: TS insertion for All Frames"]
    #[inline(always)]
    pub fn txbdtsmode(&mut self) -> TXBDTSMODE_W {
        TXBDTSMODE_W { w: self }
    }
}
