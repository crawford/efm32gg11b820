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
#[doc = "Reader of field `EM2BLOCK`"]
pub type EM2BLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM2BLOCK`"]
pub struct EM2BLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> EM2BLOCK_W<'a> {
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
#[doc = "Reader of field `EM2BODDIS`"]
pub type EM2BODDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM2BODDIS`"]
pub struct EM2BODDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EM2BODDIS_W<'a> {
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
#[doc = "Reader of field `EM01LD`"]
pub type EM01LD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM01LD`"]
pub struct EM01LD_W<'a> {
    w: &'a mut W,
}
impl<'a> EM01LD_W<'a> {
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
#[doc = "Reader of field `EM23VSCALEAUTOWSEN`"]
pub type EM23VSCALEAUTOWSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM23VSCALEAUTOWSEN`"]
pub struct EM23VSCALEAUTOWSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EM23VSCALEAUTOWSEN_W<'a> {
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
#[doc = "EM23 Voltage Scale\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EM23VSCALE_A {
    #[doc = "0: Voltage Scale Level 2"]
    VSCALE2 = 0,
    #[doc = "2: Voltage Scale Level 0"]
    VSCALE0 = 2,
    #[doc = "3: RESV"]
    RESV = 3,
}
impl From<EM23VSCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: EM23VSCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EM23VSCALE`"]
pub type EM23VSCALE_R = crate::R<u8, EM23VSCALE_A>;
impl EM23VSCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EM23VSCALE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EM23VSCALE_A::VSCALE2),
            2 => Val(EM23VSCALE_A::VSCALE0),
            3 => Val(EM23VSCALE_A::RESV),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VSCALE2`"]
    #[inline(always)]
    pub fn is_vscale2(&self) -> bool {
        *self == EM23VSCALE_A::VSCALE2
    }
    #[doc = "Checks if the value of the field is `VSCALE0`"]
    #[inline(always)]
    pub fn is_vscale0(&self) -> bool {
        *self == EM23VSCALE_A::VSCALE0
    }
    #[doc = "Checks if the value of the field is `RESV`"]
    #[inline(always)]
    pub fn is_resv(&self) -> bool {
        *self == EM23VSCALE_A::RESV
    }
}
#[doc = "Write proxy for field `EM23VSCALE`"]
pub struct EM23VSCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> EM23VSCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM23VSCALE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Voltage Scale Level 2"]
    #[inline(always)]
    pub fn vscale2(self) -> &'a mut W {
        self.variant(EM23VSCALE_A::VSCALE2)
    }
    #[doc = "Voltage Scale Level 0"]
    #[inline(always)]
    pub fn vscale0(self) -> &'a mut W {
        self.variant(EM23VSCALE_A::VSCALE0)
    }
    #[doc = "RESV"]
    #[inline(always)]
    pub fn resv(self) -> &'a mut W {
        self.variant(EM23VSCALE_A::RESV)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "EM4H Voltage Scale\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EM4HVSCALE_A {
    #[doc = "0: Voltage Scale Level 2"]
    VSCALE2 = 0,
    #[doc = "2: Voltage Scale Level 0"]
    VSCALE0 = 2,
    #[doc = "3: RESV"]
    RESV = 3,
}
impl From<EM4HVSCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: EM4HVSCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EM4HVSCALE`"]
pub type EM4HVSCALE_R = crate::R<u8, EM4HVSCALE_A>;
impl EM4HVSCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EM4HVSCALE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EM4HVSCALE_A::VSCALE2),
            2 => Val(EM4HVSCALE_A::VSCALE0),
            3 => Val(EM4HVSCALE_A::RESV),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VSCALE2`"]
    #[inline(always)]
    pub fn is_vscale2(&self) -> bool {
        *self == EM4HVSCALE_A::VSCALE2
    }
    #[doc = "Checks if the value of the field is `VSCALE0`"]
    #[inline(always)]
    pub fn is_vscale0(&self) -> bool {
        *self == EM4HVSCALE_A::VSCALE0
    }
    #[doc = "Checks if the value of the field is `RESV`"]
    #[inline(always)]
    pub fn is_resv(&self) -> bool {
        *self == EM4HVSCALE_A::RESV
    }
}
#[doc = "Write proxy for field `EM4HVSCALE`"]
pub struct EM4HVSCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4HVSCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM4HVSCALE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Voltage Scale Level 2"]
    #[inline(always)]
    pub fn vscale2(self) -> &'a mut W {
        self.variant(EM4HVSCALE_A::VSCALE2)
    }
    #[doc = "Voltage Scale Level 0"]
    #[inline(always)]
    pub fn vscale0(self) -> &'a mut W {
        self.variant(EM4HVSCALE_A::VSCALE0)
    }
    #[doc = "RESV"]
    #[inline(always)]
    pub fn resv(self) -> &'a mut W {
        self.variant(EM4HVSCALE_A::RESV)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline(always)]
    pub fn em2block(&self) -> EM2BLOCK_R {
        EM2BLOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Disable BOD in EM2"]
    #[inline(always)]
    pub fn em2boddis(&self) -> EM2BODDIS_R {
        EM2BODDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn em01ld(&self) -> EM01LD_R {
        EM01LD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Automatically Configures Flash and Frequency to Wakeup From EM2 or EM3 at Low Voltage"]
    #[inline(always)]
    pub fn em23vscaleautowsen(&self) -> EM23VSCALEAUTOWSEN_R {
        EM23VSCALEAUTOWSEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - EM23 Voltage Scale"]
    #[inline(always)]
    pub fn em23vscale(&self) -> EM23VSCALE_R {
        EM23VSCALE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - EM4H Voltage Scale"]
    #[inline(always)]
    pub fn em4hvscale(&self) -> EM4HVSCALE_R {
        EM4HVSCALE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline(always)]
    pub fn em2block(&mut self) -> EM2BLOCK_W {
        EM2BLOCK_W { w: self }
    }
    #[doc = "Bit 2 - Disable BOD in EM2"]
    #[inline(always)]
    pub fn em2boddis(&mut self) -> EM2BODDIS_W {
        EM2BODDIS_W { w: self }
    }
    #[doc = "Bit 3 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn em01ld(&mut self) -> EM01LD_W {
        EM01LD_W { w: self }
    }
    #[doc = "Bit 4 - Automatically Configures Flash and Frequency to Wakeup From EM2 or EM3 at Low Voltage"]
    #[inline(always)]
    pub fn em23vscaleautowsen(&mut self) -> EM23VSCALEAUTOWSEN_W {
        EM23VSCALEAUTOWSEN_W { w: self }
    }
    #[doc = "Bits 8:9 - EM23 Voltage Scale"]
    #[inline(always)]
    pub fn em23vscale(&mut self) -> EM23VSCALE_W {
        EM23VSCALE_W { w: self }
    }
    #[doc = "Bits 16:17 - EM4H Voltage Scale"]
    #[inline(always)]
    pub fn em4hvscale(&mut self) -> EM4HVSCALE_W {
        EM4HVSCALE_W { w: self }
    }
}
