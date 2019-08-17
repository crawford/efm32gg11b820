#[doc = "Reader of register BOOTLOADERCTRL"]
pub type R = crate::R<u32, super::BOOTLOADERCTRL>;
#[doc = "Writer for register BOOTLOADERCTRL"]
pub type W = crate::W<u32, super::BOOTLOADERCTRL>;
#[doc = "Register BOOTLOADERCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::BOOTLOADERCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLRDIS`"]
pub type BLRDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLRDIS`"]
pub struct BLRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BLRDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `BLWDIS`"]
pub type BLWDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLWDIS`"]
pub struct BLWDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BLWDIS_W<'a> {
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
    #[doc = "Bit 0 - Flash Bootloader Read Disable"]
    #[inline(always)]
    pub fn blrdis(&self) -> BLRDIS_R {
        BLRDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Flash Bootloader Write/Erase Disable"]
    #[inline(always)]
    pub fn blwdis(&self) -> BLWDIS_R {
        BLWDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Bootloader Read Disable"]
    #[inline(always)]
    pub fn blrdis(&mut self) -> BLRDIS_W {
        BLRDIS_W { w: self }
    }
    #[doc = "Bit 1 - Flash Bootloader Write/Erase Disable"]
    #[inline(always)]
    pub fn blwdis(&mut self) -> BLWDIS_W {
        BLWDIS_W { w: self }
    }
}
