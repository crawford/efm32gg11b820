#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `SLAVE`"]
pub type SLAVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE`"]
pub struct SLAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_W<'a> {
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
#[doc = "Reader of field `AUTOACK`"]
pub type AUTOACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOACK`"]
pub struct AUTOACK_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOACK_W<'a> {
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
#[doc = "Reader of field `AUTOSE`"]
pub type AUTOSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOSE`"]
pub struct AUTOSE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSE_W<'a> {
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
#[doc = "Reader of field `AUTOSN`"]
pub type AUTOSN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOSN`"]
pub struct AUTOSN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSN_W<'a> {
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
#[doc = "Reader of field `ARBDIS`"]
pub type ARBDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARBDIS`"]
pub struct ARBDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBDIS_W<'a> {
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
#[doc = "Reader of field `GCAMEN`"]
pub type GCAMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCAMEN`"]
pub struct GCAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GCAMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TXBIL`"]
pub type TXBIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBIL`"]
pub struct TXBIL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Clock Low High Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLHR_A {
    #[doc = "0: The ratio between low period and high period counters (Nlow:Nhigh) is 4:4"]
    STANDARD,
    #[doc = "1: The ratio between low period and high period counters (Nlow:Nhigh) is 6:3"]
    ASYMMETRIC,
    #[doc = "2: The ratio between low period and high period counters (Nlow:Nhigh) is 11:6"]
    FAST,
}
impl From<CLHR_A> for u8 {
    #[inline(always)]
    fn from(variant: CLHR_A) -> Self {
        match variant {
            CLHR_A::STANDARD => 0,
            CLHR_A::ASYMMETRIC => 1,
            CLHR_A::FAST => 2,
        }
    }
}
#[doc = "Reader of field `CLHR`"]
pub type CLHR_R = crate::R<u8, CLHR_A>;
impl CLHR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLHR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLHR_A::STANDARD),
            1 => Val(CLHR_A::ASYMMETRIC),
            2 => Val(CLHR_A::FAST),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == CLHR_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `ASYMMETRIC`"]
    #[inline(always)]
    pub fn is_asymmetric(&self) -> bool {
        *self == CLHR_A::ASYMMETRIC
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == CLHR_A::FAST
    }
}
#[doc = "Write proxy for field `CLHR`"]
pub struct CLHR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLHR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLHR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 4:4"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(CLHR_A::STANDARD)
    }
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 6:3"]
    #[inline(always)]
    pub fn asymmetric(self) -> &'a mut W {
        self.variant(CLHR_A::ASYMMETRIC)
    }
    #[doc = "The ratio between low period and high period counters (Nlow:Nhigh) is 11:6"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(CLHR_A::FAST)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Bus Idle Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITO_A {
    #[doc = "0: Timeout disabled"]
    OFF,
    #[doc = "1: Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    _40PCC,
    #[doc = "2: Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    _80PCC,
    #[doc = "3: Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    _160PCC,
}
impl From<BITO_A> for u8 {
    #[inline(always)]
    fn from(variant: BITO_A) -> Self {
        match variant {
            BITO_A::OFF => 0,
            BITO_A::_40PCC => 1,
            BITO_A::_80PCC => 2,
            BITO_A::_160PCC => 3,
        }
    }
}
#[doc = "Reader of field `BITO`"]
pub type BITO_R = crate::R<u8, BITO_A>;
impl BITO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BITO_A {
        match self.bits {
            0 => BITO_A::OFF,
            1 => BITO_A::_40PCC,
            2 => BITO_A::_80PCC,
            3 => BITO_A::_160PCC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == BITO_A::OFF
    }
    #[doc = "Checks if the value of the field is `_40PCC`"]
    #[inline(always)]
    pub fn is_40pcc(&self) -> bool {
        *self == BITO_A::_40PCC
    }
    #[doc = "Checks if the value of the field is `_80PCC`"]
    #[inline(always)]
    pub fn is_80pcc(&self) -> bool {
        *self == BITO_A::_80PCC
    }
    #[doc = "Checks if the value of the field is `_160PCC`"]
    #[inline(always)]
    pub fn is_160pcc(&self) -> bool {
        *self == BITO_A::_160PCC
    }
}
#[doc = "Write proxy for field `BITO`"]
pub struct BITO_W<'a> {
    w: &'a mut W,
}
impl<'a> BITO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BITO_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timeout disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(BITO_A::OFF)
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline(always)]
    pub fn _40pcc(self) -> &'a mut W {
        self.variant(BITO_A::_40PCC)
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline(always)]
    pub fn _80pcc(self) -> &'a mut W {
        self.variant(BITO_A::_80PCC)
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline(always)]
    pub fn _160pcc(self) -> &'a mut W {
        self.variant(BITO_A::_160PCC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `GIBITO`"]
pub type GIBITO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GIBITO`"]
pub struct GIBITO_W<'a> {
    w: &'a mut W,
}
impl<'a> GIBITO_W<'a> {
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
#[doc = "Clock Low Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLTO_A {
    #[doc = "0: Timeout disabled"]
    OFF,
    #[doc = "1: Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    _40PCC,
    #[doc = "2: Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    _80PCC,
    #[doc = "3: Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    _160PCC,
    #[doc = "4: Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout."]
    _320PCC,
    #[doc = "5: Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout."]
    _1024PCC,
}
impl From<CLTO_A> for u8 {
    #[inline(always)]
    fn from(variant: CLTO_A) -> Self {
        match variant {
            CLTO_A::OFF => 0,
            CLTO_A::_40PCC => 1,
            CLTO_A::_80PCC => 2,
            CLTO_A::_160PCC => 3,
            CLTO_A::_320PCC => 4,
            CLTO_A::_1024PCC => 5,
        }
    }
}
#[doc = "Reader of field `CLTO`"]
pub type CLTO_R = crate::R<u8, CLTO_A>;
impl CLTO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLTO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLTO_A::OFF),
            1 => Val(CLTO_A::_40PCC),
            2 => Val(CLTO_A::_80PCC),
            3 => Val(CLTO_A::_160PCC),
            4 => Val(CLTO_A::_320PCC),
            5 => Val(CLTO_A::_1024PCC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLTO_A::OFF
    }
    #[doc = "Checks if the value of the field is `_40PCC`"]
    #[inline(always)]
    pub fn is_40pcc(&self) -> bool {
        *self == CLTO_A::_40PCC
    }
    #[doc = "Checks if the value of the field is `_80PCC`"]
    #[inline(always)]
    pub fn is_80pcc(&self) -> bool {
        *self == CLTO_A::_80PCC
    }
    #[doc = "Checks if the value of the field is `_160PCC`"]
    #[inline(always)]
    pub fn is_160pcc(&self) -> bool {
        *self == CLTO_A::_160PCC
    }
    #[doc = "Checks if the value of the field is `_320PCC`"]
    #[inline(always)]
    pub fn is_320pcc(&self) -> bool {
        *self == CLTO_A::_320PCC
    }
    #[doc = "Checks if the value of the field is `_1024PCC`"]
    #[inline(always)]
    pub fn is_1024pcc(&self) -> bool {
        *self == CLTO_A::_1024PCC
    }
}
#[doc = "Write proxy for field `CLTO`"]
pub struct CLTO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLTO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timeout disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLTO_A::OFF)
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline(always)]
    pub fn _40pcc(self) -> &'a mut W {
        self.variant(CLTO_A::_40PCC)
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline(always)]
    pub fn _80pcc(self) -> &'a mut W {
        self.variant(CLTO_A::_80PCC)
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline(always)]
    pub fn _160pcc(self) -> &'a mut W {
        self.variant(CLTO_A::_160PCC)
    }
    #[doc = "Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout."]
    #[inline(always)]
    pub fn _320pcc(self) -> &'a mut W {
        self.variant(CLTO_A::_320PCC)
    }
    #[doc = "Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout."]
    #[inline(always)]
    pub fn _1024pcc(self) -> &'a mut W {
        self.variant(CLTO_A::_1024PCC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - I2C Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Addressable as Slave"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Automatic Acknowledge"]
    #[inline(always)]
    pub fn autoack(&self) -> AUTOACK_R {
        AUTOACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Automatic STOP When Empty"]
    #[inline(always)]
    pub fn autose(&self) -> AUTOSE_R {
        AUTOSE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Automatic STOP on NACK"]
    #[inline(always)]
    pub fn autosn(&self) -> AUTOSN_R {
        AUTOSN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Arbitration Disable"]
    #[inline(always)]
    pub fn arbdis(&self) -> ARBDIS_R {
        ARBDIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - General Call Address Match Enable"]
    #[inline(always)]
    pub fn gcamen(&self) -> GCAMEN_R {
        GCAMEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TX Buffer Interrupt Level"]
    #[inline(always)]
    pub fn txbil(&self) -> TXBIL_R {
        TXBIL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Clock Low High Ratio"]
    #[inline(always)]
    pub fn clhr(&self) -> CLHR_R {
        CLHR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Bus Idle Timeout"]
    #[inline(always)]
    pub fn bito(&self) -> BITO_R {
        BITO_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Go Idle on Bus Idle Timeout"]
    #[inline(always)]
    pub fn gibito(&self) -> GIBITO_R {
        GIBITO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Clock Low Timeout"]
    #[inline(always)]
    pub fn clto(&self) -> CLTO_R {
        CLTO_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Addressable as Slave"]
    #[inline(always)]
    pub fn slave(&mut self) -> SLAVE_W {
        SLAVE_W { w: self }
    }
    #[doc = "Bit 2 - Automatic Acknowledge"]
    #[inline(always)]
    pub fn autoack(&mut self) -> AUTOACK_W {
        AUTOACK_W { w: self }
    }
    #[doc = "Bit 3 - Automatic STOP When Empty"]
    #[inline(always)]
    pub fn autose(&mut self) -> AUTOSE_W {
        AUTOSE_W { w: self }
    }
    #[doc = "Bit 4 - Automatic STOP on NACK"]
    #[inline(always)]
    pub fn autosn(&mut self) -> AUTOSN_W {
        AUTOSN_W { w: self }
    }
    #[doc = "Bit 5 - Arbitration Disable"]
    #[inline(always)]
    pub fn arbdis(&mut self) -> ARBDIS_W {
        ARBDIS_W { w: self }
    }
    #[doc = "Bit 6 - General Call Address Match Enable"]
    #[inline(always)]
    pub fn gcamen(&mut self) -> GCAMEN_W {
        GCAMEN_W { w: self }
    }
    #[doc = "Bit 7 - TX Buffer Interrupt Level"]
    #[inline(always)]
    pub fn txbil(&mut self) -> TXBIL_W {
        TXBIL_W { w: self }
    }
    #[doc = "Bits 8:9 - Clock Low High Ratio"]
    #[inline(always)]
    pub fn clhr(&mut self) -> CLHR_W {
        CLHR_W { w: self }
    }
    #[doc = "Bits 12:13 - Bus Idle Timeout"]
    #[inline(always)]
    pub fn bito(&mut self) -> BITO_W {
        BITO_W { w: self }
    }
    #[doc = "Bit 15 - Go Idle on Bus Idle Timeout"]
    #[inline(always)]
    pub fn gibito(&mut self) -> GIBITO_W {
        GIBITO_W { w: self }
    }
    #[doc = "Bits 16:18 - Clock Low Timeout"]
    #[inline(always)]
    pub fn clto(&mut self) -> CLTO_W {
        CLTO_W { w: self }
    }
}
