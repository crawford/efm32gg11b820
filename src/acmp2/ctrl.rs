#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x0700_0000"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0700_0000
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Reader of field `INACTVAL`"]
pub type INACTVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INACTVAL`"]
pub struct INACTVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> INACTVAL_W<'a> {
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
#[doc = "Reader of field `GPIOINV`"]
pub type GPIOINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIOINV`"]
pub struct GPIOINV_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOINV_W<'a> {
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
#[doc = "Reader of field `APORTXMASTERDIS`"]
pub type APORTXMASTERDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORTXMASTERDIS`"]
pub struct APORTXMASTERDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> APORTXMASTERDIS_W<'a> {
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
#[doc = "Reader of field `APORTYMASTERDIS`"]
pub type APORTYMASTERDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORTYMASTERDIS`"]
pub struct APORTYMASTERDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> APORTYMASTERDIS_W<'a> {
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
#[doc = "Reader of field `APORTVMASTERDIS`"]
pub type APORTVMASTERDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORTVMASTERDIS`"]
pub struct APORTVMASTERDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> APORTVMASTERDIS_W<'a> {
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
#[doc = "Power Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRSEL_A {
    #[doc = "0: AVDD supply"]
    AVDD,
    #[doc = "1: DVDD supply"]
    DVDD,
    #[doc = "2: IOVDD/IOVDD0 supply"]
    IOVDD0,
    #[doc = "4: IOVDD1 supply (if part has two I/O voltages)"]
    IOVDD1,
}
impl From<PWRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRSEL_A) -> Self {
        match variant {
            PWRSEL_A::AVDD => 0,
            PWRSEL_A::DVDD => 1,
            PWRSEL_A::IOVDD0 => 2,
            PWRSEL_A::IOVDD1 => 4,
        }
    }
}
#[doc = "Reader of field `PWRSEL`"]
pub type PWRSEL_R = crate::R<u8, PWRSEL_A>;
impl PWRSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWRSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PWRSEL_A::AVDD),
            1 => Val(PWRSEL_A::DVDD),
            2 => Val(PWRSEL_A::IOVDD0),
            4 => Val(PWRSEL_A::IOVDD1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AVDD`"]
    #[inline(always)]
    pub fn is_avdd(&self) -> bool {
        *self == PWRSEL_A::AVDD
    }
    #[doc = "Checks if the value of the field is `DVDD`"]
    #[inline(always)]
    pub fn is_dvdd(&self) -> bool {
        *self == PWRSEL_A::DVDD
    }
    #[doc = "Checks if the value of the field is `IOVDD0`"]
    #[inline(always)]
    pub fn is_iovdd0(&self) -> bool {
        *self == PWRSEL_A::IOVDD0
    }
    #[doc = "Checks if the value of the field is `IOVDD1`"]
    #[inline(always)]
    pub fn is_iovdd1(&self) -> bool {
        *self == PWRSEL_A::IOVDD1
    }
}
#[doc = "Write proxy for field `PWRSEL`"]
pub struct PWRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "AVDD supply"]
    #[inline(always)]
    pub fn avdd(self) -> &'a mut W {
        self.variant(PWRSEL_A::AVDD)
    }
    #[doc = "DVDD supply"]
    #[inline(always)]
    pub fn dvdd(self) -> &'a mut W {
        self.variant(PWRSEL_A::DVDD)
    }
    #[doc = "IOVDD/IOVDD0 supply"]
    #[inline(always)]
    pub fn iovdd0(self) -> &'a mut W {
        self.variant(PWRSEL_A::IOVDD0)
    }
    #[doc = "IOVDD1 supply (if part has two I/O voltages)"]
    #[inline(always)]
    pub fn iovdd1(self) -> &'a mut W {
        self.variant(PWRSEL_A::IOVDD1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `ACCURACY`"]
pub type ACCURACY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACCURACY`"]
pub struct ACCURACY_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCURACY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Input Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUTRANGE_A {
    #[doc = "0: Setting when the input can be from 0 to ACMPVDD."]
    FULL,
    #[doc = "1: Setting when the input will always be greater than ACMPVDD/2."]
    GTVDDDIV2,
    #[doc = "2: Setting when the input will always be less than ACMPVDD/2."]
    LTVDDDIV2,
}
impl From<INPUTRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUTRANGE_A) -> Self {
        match variant {
            INPUTRANGE_A::FULL => 0,
            INPUTRANGE_A::GTVDDDIV2 => 1,
            INPUTRANGE_A::LTVDDDIV2 => 2,
        }
    }
}
#[doc = "Reader of field `INPUTRANGE`"]
pub type INPUTRANGE_R = crate::R<u8, INPUTRANGE_A>;
impl INPUTRANGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INPUTRANGE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INPUTRANGE_A::FULL),
            1 => Val(INPUTRANGE_A::GTVDDDIV2),
            2 => Val(INPUTRANGE_A::LTVDDDIV2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == INPUTRANGE_A::FULL
    }
    #[doc = "Checks if the value of the field is `GTVDDDIV2`"]
    #[inline(always)]
    pub fn is_gtvdddiv2(&self) -> bool {
        *self == INPUTRANGE_A::GTVDDDIV2
    }
    #[doc = "Checks if the value of the field is `LTVDDDIV2`"]
    #[inline(always)]
    pub fn is_ltvdddiv2(&self) -> bool {
        *self == INPUTRANGE_A::LTVDDDIV2
    }
}
#[doc = "Write proxy for field `INPUTRANGE`"]
pub struct INPUTRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUTRANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUTRANGE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Setting when the input can be from 0 to ACMPVDD."]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(INPUTRANGE_A::FULL)
    }
    #[doc = "Setting when the input will always be greater than ACMPVDD/2."]
    #[inline(always)]
    pub fn gtvdddiv2(self) -> &'a mut W {
        self.variant(INPUTRANGE_A::GTVDDDIV2)
    }
    #[doc = "Setting when the input will always be less than ACMPVDD/2."]
    #[inline(always)]
    pub fn ltvdddiv2(self) -> &'a mut W {
        self.variant(INPUTRANGE_A::LTVDDDIV2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `IRISE`"]
pub type IRISE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRISE`"]
pub struct IRISE_W<'a> {
    w: &'a mut W,
}
impl<'a> IRISE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `IFALL`"]
pub type IFALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IFALL`"]
pub struct IFALL_W<'a> {
    w: &'a mut W,
}
impl<'a> IFALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `BIASPROG`"]
pub type BIASPROG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BIASPROG`"]
pub struct BIASPROG_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASPROG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `FULLBIAS`"]
pub type FULLBIAS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FULLBIAS`"]
pub struct FULLBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> FULLBIAS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    pub fn inactval(&self) -> INACTVAL_R {
        INACTVAL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comparator GPIO Output Invert"]
    #[inline(always)]
    pub fn gpioinv(&self) -> GPIOINV_R {
        GPIOINV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - APORT Bus X Master Disable"]
    #[inline(always)]
    pub fn aportxmasterdis(&self) -> APORTXMASTERDIS_R {
        APORTXMASTERDIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - APORT Bus Y Master Disable"]
    #[inline(always)]
    pub fn aportymasterdis(&self) -> APORTYMASTERDIS_R {
        APORTYMASTERDIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - APORT Bus Master Disable for Bus Selected By VASEL"]
    #[inline(always)]
    pub fn aportvmasterdis(&self) -> APORTVMASTERDIS_R {
        APORTVMASTERDIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Power Select"]
    #[inline(always)]
    pub fn pwrsel(&self) -> PWRSEL_R {
        PWRSEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - ACMP Accuracy Mode"]
    #[inline(always)]
    pub fn accuracy(&self) -> ACCURACY_R {
        ACCURACY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Input Range"]
    #[inline(always)]
    pub fn inputrange(&self) -> INPUTRANGE_R {
        INPUTRANGE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Rising Edge Interrupt Sense"]
    #[inline(always)]
    pub fn irise(&self) -> IRISE_R {
        IRISE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Falling Edge Interrupt Sense"]
    #[inline(always)]
    pub fn ifall(&self) -> IFALL_R {
        IFALL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 24:29 - Bias Configuration"]
    #[inline(always)]
    pub fn biasprog(&self) -> BIASPROG_R {
        BIASPROG_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Full Bias Current"]
    #[inline(always)]
    pub fn fullbias(&self) -> FULLBIAS_R {
        FULLBIAS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    pub fn inactval(&mut self) -> INACTVAL_W {
        INACTVAL_W { w: self }
    }
    #[doc = "Bit 3 - Comparator GPIO Output Invert"]
    #[inline(always)]
    pub fn gpioinv(&mut self) -> GPIOINV_W {
        GPIOINV_W { w: self }
    }
    #[doc = "Bit 8 - APORT Bus X Master Disable"]
    #[inline(always)]
    pub fn aportxmasterdis(&mut self) -> APORTXMASTERDIS_W {
        APORTXMASTERDIS_W { w: self }
    }
    #[doc = "Bit 9 - APORT Bus Y Master Disable"]
    #[inline(always)]
    pub fn aportymasterdis(&mut self) -> APORTYMASTERDIS_W {
        APORTYMASTERDIS_W { w: self }
    }
    #[doc = "Bit 10 - APORT Bus Master Disable for Bus Selected By VASEL"]
    #[inline(always)]
    pub fn aportvmasterdis(&mut self) -> APORTVMASTERDIS_W {
        APORTVMASTERDIS_W { w: self }
    }
    #[doc = "Bits 12:14 - Power Select"]
    #[inline(always)]
    pub fn pwrsel(&mut self) -> PWRSEL_W {
        PWRSEL_W { w: self }
    }
    #[doc = "Bit 15 - ACMP Accuracy Mode"]
    #[inline(always)]
    pub fn accuracy(&mut self) -> ACCURACY_W {
        ACCURACY_W { w: self }
    }
    #[doc = "Bits 18:19 - Input Range"]
    #[inline(always)]
    pub fn inputrange(&mut self) -> INPUTRANGE_W {
        INPUTRANGE_W { w: self }
    }
    #[doc = "Bit 20 - Rising Edge Interrupt Sense"]
    #[inline(always)]
    pub fn irise(&mut self) -> IRISE_W {
        IRISE_W { w: self }
    }
    #[doc = "Bit 21 - Falling Edge Interrupt Sense"]
    #[inline(always)]
    pub fn ifall(&mut self) -> IFALL_W {
        IFALL_W { w: self }
    }
    #[doc = "Bits 24:29 - Bias Configuration"]
    #[inline(always)]
    pub fn biasprog(&mut self) -> BIASPROG_W {
        BIASPROG_W { w: self }
    }
    #[doc = "Bit 31 - Full Bias Current"]
    #[inline(always)]
    pub fn fullbias(&mut self) -> FULLBIAS_W {
        FULLBIAS_W { w: self }
    }
}
