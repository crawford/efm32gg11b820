#[doc = "Reader of register DTOGEN"]
pub type R = crate::R<u32, super::DTOGEN>;
#[doc = "Writer for register DTOGEN"]
pub type W = crate::W<u32, super::DTOGEN>;
#[doc = "Register DTOGEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DTOGEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTOGCC0EN`"]
pub type DTOGCC0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTOGCC0EN`"]
pub struct DTOGCC0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOGCC0EN_W<'a> {
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
#[doc = "Reader of field `DTOGCC1EN`"]
pub type DTOGCC1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTOGCC1EN`"]
pub struct DTOGCC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOGCC1EN_W<'a> {
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
#[doc = "Reader of field `DTOGCC2EN`"]
pub type DTOGCC2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTOGCC2EN`"]
pub struct DTOGCC2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOGCC2EN_W<'a> {
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
#[doc = "Reader of field `DTOGCDTI0EN`"]
pub type DTOGCDTI0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTOGCDTI0EN`"]
pub struct DTOGCDTI0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOGCDTI0EN_W<'a> {
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
#[doc = "Reader of field `DTOGCDTI1EN`"]
pub type DTOGCDTI1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTOGCDTI1EN`"]
pub struct DTOGCDTI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOGCDTI1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DTOGCDTI2EN`"]
pub type DTOGCDTI2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTOGCDTI2EN`"]
pub struct DTOGCDTI2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOGCDTI2EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DTI CC0 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc0en(&self) -> DTOGCC0EN_R {
        DTOGCC0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DTI CC1 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc1en(&self) -> DTOGCC1EN_R {
        DTOGCC1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DTI CC2 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc2en(&self) -> DTOGCC2EN_R {
        DTOGCC2EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DTI CDTI0 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti0en(&self) -> DTOGCDTI0EN_R {
        DTOGCDTI0EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DTI CDTI1 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti1en(&self) -> DTOGCDTI1EN_R {
        DTOGCDTI1EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DTI CDTI2 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti2en(&self) -> DTOGCDTI2EN_R {
        DTOGCDTI2EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTI CC0 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc0en(&mut self) -> DTOGCC0EN_W {
        DTOGCC0EN_W { w: self }
    }
    #[doc = "Bit 1 - DTI CC1 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc1en(&mut self) -> DTOGCC1EN_W {
        DTOGCC1EN_W { w: self }
    }
    #[doc = "Bit 2 - DTI CC2 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc2en(&mut self) -> DTOGCC2EN_W {
        DTOGCC2EN_W { w: self }
    }
    #[doc = "Bit 3 - DTI CDTI0 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti0en(&mut self) -> DTOGCDTI0EN_W {
        DTOGCDTI0EN_W { w: self }
    }
    #[doc = "Bit 4 - DTI CDTI1 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti1en(&mut self) -> DTOGCDTI1EN_W {
        DTOGCDTI1EN_W { w: self }
    }
    #[doc = "Bit 5 - DTI CDTI2 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti2en(&mut self) -> DTOGCDTI2EN_W {
        DTOGCDTI2EN_W { w: self }
    }
}
