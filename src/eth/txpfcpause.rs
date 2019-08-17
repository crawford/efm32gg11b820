#[doc = "Reader of register TXPFCPAUSE"]
pub type R = crate::R<u32, super::TXPFCPAUSE>;
#[doc = "Writer for register TXPFCPAUSE"]
pub type W = crate::W<u32, super::TXPFCPAUSE>;
#[doc = "Register TXPFCPAUSE `reset()`'s with value 0"]
impl crate::ResetValue for super::TXPFCPAUSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VECTORENB`"]
pub type VECTORENB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VECTORENB`"]
pub struct VECTORENB_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTORENB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `VECTOR`"]
pub type VECTOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VECTOR`"]
pub struct VECTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
    #[inline(always)]
    pub fn vectorenb(&self) -> VECTORENB_R {
        VECTORENB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
    #[inline(always)]
    pub fn vector(&self) -> VECTOR_R {
        VECTOR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
    #[inline(always)]
    pub fn vectorenb(&mut self) -> VECTORENB_W {
        VECTORENB_W { w: self }
    }
    #[doc = "Bits 8:15 - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
    #[inline(always)]
    pub fn vector(&mut self) -> VECTOR_W {
        VECTOR_W { w: self }
    }
}
