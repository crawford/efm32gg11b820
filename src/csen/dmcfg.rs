#[doc = "Reader of register DMCFG"]
pub type R = crate::R<u32, super::DMCFG>;
#[doc = "Writer for register DMCFG"]
pub type W = crate::W<u32, super::DMCFG>;
#[doc = "Register DMCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DMCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMG`"]
pub type DMG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMG`"]
pub struct DMG_W<'a> {
    w: &'a mut W,
}
impl<'a> DMG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DMR`"]
pub type DMR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMR`"]
pub struct DMR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DMCR`"]
pub type DMCR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMCR`"]
pub struct DMCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Delta Modulator Conversion Resolution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRMODE_A {
    #[doc = "0: 10-bit delta modulator"]
    DM10 = 0,
    #[doc = "1: 12-bit delta modulator"]
    DM12 = 1,
    #[doc = "2: 14-bit delta modulator"]
    DM14 = 2,
    #[doc = "3: 16-bit delta modulator"]
    DM16 = 3,
}
impl From<CRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CRMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CRMODE`"]
pub type CRMODE_R = crate::R<u8, CRMODE_A>;
impl CRMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRMODE_A {
        match self.bits {
            0 => CRMODE_A::DM10,
            1 => CRMODE_A::DM12,
            2 => CRMODE_A::DM14,
            3 => CRMODE_A::DM16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DM10`"]
    #[inline(always)]
    pub fn is_dm10(&self) -> bool {
        *self == CRMODE_A::DM10
    }
    #[doc = "Checks if the value of the field is `DM12`"]
    #[inline(always)]
    pub fn is_dm12(&self) -> bool {
        *self == CRMODE_A::DM12
    }
    #[doc = "Checks if the value of the field is `DM14`"]
    #[inline(always)]
    pub fn is_dm14(&self) -> bool {
        *self == CRMODE_A::DM14
    }
    #[doc = "Checks if the value of the field is `DM16`"]
    #[inline(always)]
    pub fn is_dm16(&self) -> bool {
        *self == CRMODE_A::DM16
    }
}
#[doc = "Write proxy for field `CRMODE`"]
pub struct CRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "10-bit delta modulator"]
    #[inline(always)]
    pub fn dm10(self) -> &'a mut W {
        self.variant(CRMODE_A::DM10)
    }
    #[doc = "12-bit delta modulator"]
    #[inline(always)]
    pub fn dm12(self) -> &'a mut W {
        self.variant(CRMODE_A::DM12)
    }
    #[doc = "14-bit delta modulator"]
    #[inline(always)]
    pub fn dm14(self) -> &'a mut W {
        self.variant(CRMODE_A::DM14)
    }
    #[doc = "16-bit delta modulator"]
    #[inline(always)]
    pub fn dm16(self) -> &'a mut W {
        self.variant(CRMODE_A::DM16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `DMGRDIS`"]
pub type DMGRDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMGRDIS`"]
pub struct DMGRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMGRDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Delta Modulator Gain Step"]
    #[inline(always)]
    pub fn dmg(&self) -> DMG_R {
        DMG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Delta Modulator Gain Reduction Interval"]
    #[inline(always)]
    pub fn dmr(&self) -> DMR_R {
        DMR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Delta Modulator Conversion Rate"]
    #[inline(always)]
    pub fn dmcr(&self) -> DMCR_R {
        DMCR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Delta Modulator Conversion Resolution."]
    #[inline(always)]
    pub fn crmode(&self) -> CRMODE_R {
        CRMODE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Delta Modulation Gain Step Reduction Disable"]
    #[inline(always)]
    pub fn dmgrdis(&self) -> DMGRDIS_R {
        DMGRDIS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delta Modulator Gain Step"]
    #[inline(always)]
    pub fn dmg(&mut self) -> DMG_W {
        DMG_W { w: self }
    }
    #[doc = "Bits 8:11 - Delta Modulator Gain Reduction Interval"]
    #[inline(always)]
    pub fn dmr(&mut self) -> DMR_W {
        DMR_W { w: self }
    }
    #[doc = "Bits 16:19 - Delta Modulator Conversion Rate"]
    #[inline(always)]
    pub fn dmcr(&mut self) -> DMCR_W {
        DMCR_W { w: self }
    }
    #[doc = "Bits 20:21 - Delta Modulator Conversion Resolution."]
    #[inline(always)]
    pub fn crmode(&mut self) -> CRMODE_W {
        CRMODE_W { w: self }
    }
    #[doc = "Bit 28 - Delta Modulation Gain Step Reduction Disable"]
    #[inline(always)]
    pub fn dmgrdis(&mut self) -> DMGRDIS_W {
        DMGRDIS_W { w: self }
    }
}
