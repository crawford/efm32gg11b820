#[doc = "Reader of register ROUTEPEN"]
pub type R = crate::R<u32, super::ROUTEPEN>;
#[doc = "Writer for register ROUTEPEN"]
pub type W = crate::W<u32, super::ROUTEPEN>;
#[doc = "Register ROUTEPEN `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTEPEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CC0PEN`"]
pub type CC0PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC0PEN`"]
pub struct CC0PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CC0PEN_W<'a> {
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
#[doc = "Reader of field `CC1PEN`"]
pub type CC1PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC1PEN`"]
pub struct CC1PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1PEN_W<'a> {
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
#[doc = "Reader of field `CC2PEN`"]
pub type CC2PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC2PEN`"]
pub struct CC2PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CC3PEN`"]
pub type CC3PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC3PEN`"]
pub struct CC3PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CDTI0PEN`"]
pub type CDTI0PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDTI0PEN`"]
pub struct CDTI0PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CDTI0PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CDTI1PEN`"]
pub type CDTI1PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDTI1PEN`"]
pub struct CDTI1PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CDTI1PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CDTI2PEN`"]
pub type CDTI2PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDTI2PEN`"]
pub struct CDTI2PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CDTI2PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CC Channel 0 Pin Enable"]
    #[inline(always)]
    pub fn cc0pen(&self) -> CC0PEN_R {
        CC0PEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CC Channel 1 Pin Enable"]
    #[inline(always)]
    pub fn cc1pen(&self) -> CC1PEN_R {
        CC1PEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CC Channel 2 Pin Enable"]
    #[inline(always)]
    pub fn cc2pen(&self) -> CC2PEN_R {
        CC2PEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CC Channel 3 Pin Enable"]
    #[inline(always)]
    pub fn cc3pen(&self) -> CC3PEN_R {
        CC3PEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti0pen(&self) -> CDTI0PEN_R {
        CDTI0PEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti1pen(&self) -> CDTI1PEN_R {
        CDTI1PEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti2pen(&self) -> CDTI2PEN_R {
        CDTI2PEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC Channel 0 Pin Enable"]
    #[inline(always)]
    pub fn cc0pen(&mut self) -> CC0PEN_W {
        CC0PEN_W { w: self }
    }
    #[doc = "Bit 1 - CC Channel 1 Pin Enable"]
    #[inline(always)]
    pub fn cc1pen(&mut self) -> CC1PEN_W {
        CC1PEN_W { w: self }
    }
    #[doc = "Bit 2 - CC Channel 2 Pin Enable"]
    #[inline(always)]
    pub fn cc2pen(&mut self) -> CC2PEN_W {
        CC2PEN_W { w: self }
    }
    #[doc = "Bit 3 - CC Channel 3 Pin Enable"]
    #[inline(always)]
    pub fn cc3pen(&mut self) -> CC3PEN_W {
        CC3PEN_W { w: self }
    }
    #[doc = "Bit 8 - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti0pen(&mut self) -> CDTI0PEN_W {
        CDTI0PEN_W { w: self }
    }
    #[doc = "Bit 9 - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti1pen(&mut self) -> CDTI1PEN_W {
        CDTI1PEN_W { w: self }
    }
    #[doc = "Bit 10 - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti2pen(&mut self) -> CDTI2PEN_W {
        CDTI2PEN_W { w: self }
    }
}
