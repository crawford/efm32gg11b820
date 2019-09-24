#[doc = "Reader of register SDIOCTRL"]
pub type R = crate::R<u32, super::SDIOCTRL>;
#[doc = "Writer for register SDIOCTRL"]
pub type W = crate::W<u32, super::SDIOCTRL>;
#[doc = "Register SDIOCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SDIOCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SDIO Reference Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIOCLKSEL_A {
    #[doc = "0: HFRCO clock is used to clock SDIO"]
    HFRCO,
    #[doc = "1: HFXO clock is used to clock SDIO"]
    HFXO,
    #[doc = "2: AUXHFRCO is used to clock SDIO"]
    AUXHFRCO,
    #[doc = "3: USHFRCO is used to clock SDIO"]
    USHFRCO,
}
impl From<SDIOCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDIOCLKSEL_A) -> Self {
        match variant {
            SDIOCLKSEL_A::HFRCO => 0,
            SDIOCLKSEL_A::HFXO => 1,
            SDIOCLKSEL_A::AUXHFRCO => 2,
            SDIOCLKSEL_A::USHFRCO => 3,
        }
    }
}
#[doc = "Reader of field `SDIOCLKSEL`"]
pub type SDIOCLKSEL_R = crate::R<u8, SDIOCLKSEL_A>;
impl SDIOCLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIOCLKSEL_A {
        match self.bits {
            0 => SDIOCLKSEL_A::HFRCO,
            1 => SDIOCLKSEL_A::HFXO,
            2 => SDIOCLKSEL_A::AUXHFRCO,
            3 => SDIOCLKSEL_A::USHFRCO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == SDIOCLKSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == SDIOCLKSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == SDIOCLKSEL_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == SDIOCLKSEL_A::USHFRCO
    }
}
#[doc = "Write proxy for field `SDIOCLKSEL`"]
pub struct SDIOCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIOCLKSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "HFRCO clock is used to clock SDIO"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(SDIOCLKSEL_A::HFRCO)
    }
    #[doc = "HFXO clock is used to clock SDIO"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(SDIOCLKSEL_A::HFXO)
    }
    #[doc = "AUXHFRCO is used to clock SDIO"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(SDIOCLKSEL_A::AUXHFRCO)
    }
    #[doc = "USHFRCO is used to clock SDIO"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(SDIOCLKSEL_A::USHFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `SDIOCLKDIS`"]
pub type SDIOCLKDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIOCLKDIS`"]
pub struct SDIOCLKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOCLKDIS_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - SDIO Reference Clock Select"]
    #[inline(always)]
    pub fn sdioclksel(&self) -> SDIOCLKSEL_R {
        SDIOCLKSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 7 - SDIO Reference Clock Disable"]
    #[inline(always)]
    pub fn sdioclkdis(&self) -> SDIOCLKDIS_R {
        SDIOCLKDIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDIO Reference Clock Select"]
    #[inline(always)]
    pub fn sdioclksel(&mut self) -> SDIOCLKSEL_W {
        SDIOCLKSEL_W { w: self }
    }
    #[doc = "Bit 7 - SDIO Reference Clock Disable"]
    #[inline(always)]
    pub fn sdioclkdis(&mut self) -> SDIOCLKDIS_W {
        SDIOCLKDIS_W { w: self }
    }
}
