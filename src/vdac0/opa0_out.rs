#[doc = "Reader of register OPA0_OUT"]
pub type R = crate::R<u32, super::OPA0_OUT>;
#[doc = "Writer for register OPA0_OUT"]
pub type W = crate::W<u32, super::OPA0_OUT>;
#[doc = "Register OPA0_OUT `reset()`'s with value 0x01"]
impl crate::ResetValue for super::OPA0_OUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `MAINOUTEN`"]
pub type MAINOUTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAINOUTEN`"]
pub struct MAINOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAINOUTEN_W<'a> {
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
#[doc = "Reader of field `ALTOUTEN`"]
pub type ALTOUTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALTOUTEN`"]
pub struct ALTOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTOUTEN_W<'a> {
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
#[doc = "Reader of field `APORTOUTEN`"]
pub type APORTOUTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORTOUTEN`"]
pub struct APORTOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APORTOUTEN_W<'a> {
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
#[doc = "Reader of field `SHORT`"]
pub type SHORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHORT`"]
pub struct SHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHORT_W<'a> {
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
#[doc = "OPAx Output Enable Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALTOUTPADEN_A {
    #[doc = "1: Alternate Output 0"]
    OUT0,
    #[doc = "2: Alternate Output 1"]
    OUT1,
    #[doc = "4: Alternate Output 2"]
    OUT2,
    #[doc = "8: Alternate Output 3"]
    OUT3,
    #[doc = "16: Alternate Output 4"]
    OUT4,
}
impl From<ALTOUTPADEN_A> for u8 {
    #[inline(always)]
    fn from(variant: ALTOUTPADEN_A) -> Self {
        match variant {
            ALTOUTPADEN_A::OUT0 => 1,
            ALTOUTPADEN_A::OUT1 => 2,
            ALTOUTPADEN_A::OUT2 => 4,
            ALTOUTPADEN_A::OUT3 => 8,
            ALTOUTPADEN_A::OUT4 => 16,
        }
    }
}
#[doc = "Reader of field `ALTOUTPADEN`"]
pub type ALTOUTPADEN_R = crate::R<u8, ALTOUTPADEN_A>;
impl ALTOUTPADEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ALTOUTPADEN_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(ALTOUTPADEN_A::OUT0),
            2 => Val(ALTOUTPADEN_A::OUT1),
            4 => Val(ALTOUTPADEN_A::OUT2),
            8 => Val(ALTOUTPADEN_A::OUT3),
            16 => Val(ALTOUTPADEN_A::OUT4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == ALTOUTPADEN_A::OUT0
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == ALTOUTPADEN_A::OUT1
    }
    #[doc = "Checks if the value of the field is `OUT2`"]
    #[inline(always)]
    pub fn is_out2(&self) -> bool {
        *self == ALTOUTPADEN_A::OUT2
    }
    #[doc = "Checks if the value of the field is `OUT3`"]
    #[inline(always)]
    pub fn is_out3(&self) -> bool {
        *self == ALTOUTPADEN_A::OUT3
    }
    #[doc = "Checks if the value of the field is `OUT4`"]
    #[inline(always)]
    pub fn is_out4(&self) -> bool {
        *self == ALTOUTPADEN_A::OUT4
    }
}
#[doc = "Write proxy for field `ALTOUTPADEN`"]
pub struct ALTOUTPADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTOUTPADEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALTOUTPADEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Alternate Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(ALTOUTPADEN_A::OUT0)
    }
    #[doc = "Alternate Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(ALTOUTPADEN_A::OUT1)
    }
    #[doc = "Alternate Output 2"]
    #[inline(always)]
    pub fn out2(self) -> &'a mut W {
        self.variant(ALTOUTPADEN_A::OUT2)
    }
    #[doc = "Alternate Output 3"]
    #[inline(always)]
    pub fn out3(self) -> &'a mut W {
        self.variant(ALTOUTPADEN_A::OUT3)
    }
    #[doc = "Alternate Output 4"]
    #[inline(always)]
    pub fn out4(self) -> &'a mut W {
        self.variant(ALTOUTPADEN_A::OUT4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "Reader of field `APORTOUTSEL`"]
pub type APORTOUTSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APORTOUTSEL`"]
pub struct APORTOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APORTOUTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - OPAx Main Output Enable"]
    #[inline(always)]
    pub fn mainouten(&self) -> MAINOUTEN_R {
        MAINOUTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OPAx Alternative Output Enable"]
    #[inline(always)]
    pub fn altouten(&self) -> ALTOUTEN_R {
        ALTOUTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OPAx Aport Output Enable"]
    #[inline(always)]
    pub fn aportouten(&self) -> APORTOUTEN_R {
        APORTOUTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OPAx Main and Alternative Output Short"]
    #[inline(always)]
    pub fn short(&self) -> SHORT_R {
        SHORT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:8 - OPAx Output Enable Value"]
    #[inline(always)]
    pub fn altoutpaden(&self) -> ALTOUTPADEN_R {
        ALTOUTPADEN_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - OPAx APORT Output"]
    #[inline(always)]
    pub fn aportoutsel(&self) -> APORTOUTSEL_R {
        APORTOUTSEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - OPAx Main Output Enable"]
    #[inline(always)]
    pub fn mainouten(&mut self) -> MAINOUTEN_W {
        MAINOUTEN_W { w: self }
    }
    #[doc = "Bit 1 - OPAx Alternative Output Enable"]
    #[inline(always)]
    pub fn altouten(&mut self) -> ALTOUTEN_W {
        ALTOUTEN_W { w: self }
    }
    #[doc = "Bit 2 - OPAx Aport Output Enable"]
    #[inline(always)]
    pub fn aportouten(&mut self) -> APORTOUTEN_W {
        APORTOUTEN_W { w: self }
    }
    #[doc = "Bit 3 - OPAx Main and Alternative Output Short"]
    #[inline(always)]
    pub fn short(&mut self) -> SHORT_W {
        SHORT_W { w: self }
    }
    #[doc = "Bits 4:8 - OPAx Output Enable Value"]
    #[inline(always)]
    pub fn altoutpaden(&mut self) -> ALTOUTPADEN_W {
        ALTOUTPADEN_W { w: self }
    }
    #[doc = "Bits 16:23 - OPAx APORT Output"]
    #[inline(always)]
    pub fn aportoutsel(&mut self) -> APORTOUTSEL_W {
        APORTOUTSEL_W { w: self }
    }
}
