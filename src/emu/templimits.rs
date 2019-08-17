#[doc = "Reader of register TEMPLIMITS"]
pub type R = crate::R<u32, super::TEMPLIMITS>;
#[doc = "Writer for register TEMPLIMITS"]
pub type W = crate::W<u32, super::TEMPLIMITS>;
#[doc = "Register TEMPLIMITS `reset()`'s with value 0xff00"]
impl crate::ResetValue for super::TEMPLIMITS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff00
    }
}
#[doc = "Reader of field `TEMPLOW`"]
pub type TEMPLOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEMPLOW`"]
pub struct TEMPLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMPLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TEMPHIGH`"]
pub type TEMPHIGH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEMPHIGH`"]
pub struct TEMPHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMPHIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `EM4WUEN`"]
pub type EM4WUEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM4WUEN`"]
pub struct EM4WUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4WUEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Temperature Low Limit"]
    #[inline(always)]
    pub fn templow(&self) -> TEMPLOW_R {
        TEMPLOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Temperature High Limit"]
    #[inline(always)]
    pub fn temphigh(&self) -> TEMPHIGH_R {
        TEMPHIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enable EM4 Wakeup Due to Low/high Temperature"]
    #[inline(always)]
    pub fn em4wuen(&self) -> EM4WUEN_R {
        EM4WUEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Temperature Low Limit"]
    #[inline(always)]
    pub fn templow(&mut self) -> TEMPLOW_W {
        TEMPLOW_W { w: self }
    }
    #[doc = "Bits 8:15 - Temperature High Limit"]
    #[inline(always)]
    pub fn temphigh(&mut self) -> TEMPHIGH_W {
        TEMPHIGH_W { w: self }
    }
    #[doc = "Bit 16 - Enable EM4 Wakeup Due to Low/high Temperature"]
    #[inline(always)]
    pub fn em4wuen(&mut self) -> EM4WUEN_W {
        EM4WUEN_W { w: self }
    }
}
