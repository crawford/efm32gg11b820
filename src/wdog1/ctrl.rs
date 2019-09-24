#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x0f00"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f00
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
#[doc = "Reader of field `DEBUGRUN`"]
pub type DEBUGRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEBUGRUN`"]
pub struct DEBUGRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGRUN_W<'a> {
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
#[doc = "Reader of field `EM2RUN`"]
pub type EM2RUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM2RUN`"]
pub struct EM2RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> EM2RUN_W<'a> {
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
#[doc = "Reader of field `EM3RUN`"]
pub type EM3RUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM3RUN`"]
pub struct EM3RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> EM3RUN_W<'a> {
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
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
#[doc = "Reader of field `EM4BLOCK`"]
pub type EM4BLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM4BLOCK`"]
pub struct EM4BLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4BLOCK_W<'a> {
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
#[doc = "Reader of field `SWOSCBLOCK`"]
pub type SWOSCBLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWOSCBLOCK`"]
pub struct SWOSCBLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SWOSCBLOCK_W<'a> {
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
#[doc = "Reader of field `PERSEL`"]
pub type PERSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PERSEL`"]
pub struct PERSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PERSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Watchdog Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSEL_A {
    #[doc = "0: ULFRCO"]
    ULFRCO,
    #[doc = "1: LFRCO"]
    LFRCO,
    #[doc = "2: LFXO"]
    LFXO,
    #[doc = "3: HFCORECLK"]
    HFCORECLK,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        match variant {
            CLKSEL_A::ULFRCO => 0,
            CLKSEL_A::LFRCO => 1,
            CLKSEL_A::LFXO => 2,
            CLKSEL_A::HFCORECLK => 3,
        }
    }
}
#[doc = "Reader of field `CLKSEL`"]
pub type CLKSEL_R = crate::R<u8, CLKSEL_A>;
impl CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSEL_A {
        match self.bits {
            0 => CLKSEL_A::ULFRCO,
            1 => CLKSEL_A::LFRCO,
            2 => CLKSEL_A::LFXO,
            3 => CLKSEL_A::HFCORECLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKSEL_A::ULFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFCORECLK`"]
    #[inline(always)]
    pub fn is_hfcoreclk(&self) -> bool {
        *self == CLKSEL_A::HFCORECLK
    }
}
#[doc = "Write proxy for field `CLKSEL`"]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ULFRCO"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKSEL_A::ULFRCO)
    }
    #[doc = "LFRCO"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKSEL_A::LFRCO)
    }
    #[doc = "LFXO"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKSEL_A::LFXO)
    }
    #[doc = "HFCORECLK"]
    #[inline(always)]
    pub fn hfcoreclk(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFCORECLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `WARNSEL`"]
pub type WARNSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WARNSEL`"]
pub struct WARNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WARNSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `WINSEL`"]
pub type WINSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WINSEL`"]
pub struct WINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WINSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `CLRSRC`"]
pub type CLRSRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRSRC`"]
pub struct CLRSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRSRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `WDOGRSTDIS`"]
pub type WDOGRSTDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDOGRSTDIS`"]
pub struct WDOGRSTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOGRSTDIS_W<'a> {
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
    #[doc = "Bit 0 - Watchdog Timer Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Energy Mode 2 Run Enable"]
    #[inline(always)]
    pub fn em2run(&self) -> EM2RUN_R {
        EM2RUN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Energy Mode 3 Run Enable"]
    #[inline(always)]
    pub fn em3run(&self) -> EM3RUN_R {
        EM3RUN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Configuration Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Energy Mode 4 Block"]
    #[inline(always)]
    pub fn em4block(&self) -> EM4BLOCK_R {
        EM4BLOCK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Software Oscillator Disable Block"]
    #[inline(always)]
    pub fn swoscblock(&self) -> SWOSCBLOCK_R {
        SWOSCBLOCK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Watchdog Timeout Period Select"]
    #[inline(always)]
    pub fn persel(&self) -> PERSEL_R {
        PERSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Watchdog Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Watchdog Timeout Period Select"]
    #[inline(always)]
    pub fn warnsel(&self) -> WARNSEL_R {
        WARNSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - Watchdog Illegal Window Select"]
    #[inline(always)]
    pub fn winsel(&self) -> WINSEL_R {
        WINSEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 30 - Watchdog Clear Source"]
    #[inline(always)]
    pub fn clrsrc(&self) -> CLRSRC_R {
        CLRSRC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Watchdog Reset Disable"]
    #[inline(always)]
    pub fn wdogrstdis(&self) -> WDOGRSTDIS_R {
        WDOGRSTDIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DEBUGRUN_W {
        DEBUGRUN_W { w: self }
    }
    #[doc = "Bit 2 - Energy Mode 2 Run Enable"]
    #[inline(always)]
    pub fn em2run(&mut self) -> EM2RUN_W {
        EM2RUN_W { w: self }
    }
    #[doc = "Bit 3 - Energy Mode 3 Run Enable"]
    #[inline(always)]
    pub fn em3run(&mut self) -> EM3RUN_W {
        EM3RUN_W { w: self }
    }
    #[doc = "Bit 4 - Configuration Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Bit 5 - Energy Mode 4 Block"]
    #[inline(always)]
    pub fn em4block(&mut self) -> EM4BLOCK_W {
        EM4BLOCK_W { w: self }
    }
    #[doc = "Bit 6 - Software Oscillator Disable Block"]
    #[inline(always)]
    pub fn swoscblock(&mut self) -> SWOSCBLOCK_W {
        SWOSCBLOCK_W { w: self }
    }
    #[doc = "Bits 8:11 - Watchdog Timeout Period Select"]
    #[inline(always)]
    pub fn persel(&mut self) -> PERSEL_W {
        PERSEL_W { w: self }
    }
    #[doc = "Bits 12:13 - Watchdog Clock Select"]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - Watchdog Timeout Period Select"]
    #[inline(always)]
    pub fn warnsel(&mut self) -> WARNSEL_W {
        WARNSEL_W { w: self }
    }
    #[doc = "Bits 24:26 - Watchdog Illegal Window Select"]
    #[inline(always)]
    pub fn winsel(&mut self) -> WINSEL_W {
        WINSEL_W { w: self }
    }
    #[doc = "Bit 30 - Watchdog Clear Source"]
    #[inline(always)]
    pub fn clrsrc(&mut self) -> CLRSRC_W {
        CLRSRC_W { w: self }
    }
    #[doc = "Bit 31 - Watchdog Reset Disable"]
    #[inline(always)]
    pub fn wdogrstdis(&mut self) -> WDOGRSTDIS_W {
        WDOGRSTDIS_W { w: self }
    }
}
