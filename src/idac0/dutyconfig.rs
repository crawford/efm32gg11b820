#[doc = "Reader of register DUTYCONFIG"]
pub type R = crate::R<u32, super::DUTYCONFIG>;
#[doc = "Writer for register DUTYCONFIG"]
pub type W = crate::W<u32, super::DUTYCONFIG>;
#[doc = "Register DUTYCONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::DUTYCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EM2DUTYCYCLEDIS`"]
pub type EM2DUTYCYCLEDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM2DUTYCYCLEDIS`"]
pub struct EM2DUTYCYCLEDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EM2DUTYCYCLEDIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Duty Cycle Enable"]
    #[inline(always)]
    pub fn em2dutycycledis(&self) -> EM2DUTYCYCLEDIS_R {
        EM2DUTYCYCLEDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Duty Cycle Enable"]
    #[inline(always)]
    pub fn em2dutycycledis(&mut self) -> EM2DUTYCYCLEDIS_W {
        EM2DUTYCYCLEDIS_W { w: self }
    }
}
