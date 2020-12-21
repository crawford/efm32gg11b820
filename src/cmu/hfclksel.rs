#[doc = "Writer for register HFCLKSEL"]
pub type W = crate::W<u32, super::HFCLKSEL>;
#[doc = "Register HFCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::HFCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "HFCLK Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HF_AW {
    #[doc = "1: Select HFRCO as HFCLK"]
    HFRCO = 1,
    #[doc = "2: Select HFXO as HFCLK"]
    HFXO = 2,
    #[doc = "3: Select LFRCO as HFCLK"]
    LFRCO = 3,
    #[doc = "4: Select LFXO as HFCLK"]
    LFXO = 4,
    #[doc = "5: Select HFRCO divided by 2 as HFCLK"]
    HFRCODIV2 = 5,
    #[doc = "6: Select USHFRCO as HFCLK"]
    USHFRCO = 6,
    #[doc = "7: Select CLKIN0 as HFCLK"]
    CLKIN0 = 7,
}
impl From<HF_AW> for u8 {
    #[inline(always)]
    fn from(variant: HF_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `HF`"]
pub struct HF_W<'a> {
    w: &'a mut W,
}
impl<'a> HF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HF_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select HFRCO as HFCLK"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(HF_AW::HFRCO)
    }
    #[doc = "Select HFXO as HFCLK"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(HF_AW::HFXO)
    }
    #[doc = "Select LFRCO as HFCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(HF_AW::LFRCO)
    }
    #[doc = "Select LFXO as HFCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(HF_AW::LFXO)
    }
    #[doc = "Select HFRCO divided by 2 as HFCLK"]
    #[inline(always)]
    pub fn hfrcodiv2(self) -> &'a mut W {
        self.variant(HF_AW::HFRCODIV2)
    }
    #[doc = "Select USHFRCO as HFCLK"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(HF_AW::USHFRCO)
    }
    #[doc = "Select CLKIN0 as HFCLK"]
    #[inline(always)]
    pub fn clkin0(self) -> &'a mut W {
        self.variant(HF_AW::CLKIN0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:2 - HFCLK Select"]
    #[inline(always)]
    pub fn hf(&mut self) -> HF_W {
        HF_W { w: self }
    }
}
