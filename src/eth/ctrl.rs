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
#[doc = "TSU Clock selection value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSUCLKSEL_A {
    #[doc = "0: No TSU clock source selected"]
    NOCLOCK,
    #[doc = "1: Select system clock as TSU Clock"]
    PLL,
    #[doc = "2: Select ethernet RX Clock as TSU Clock"]
    RXCLK,
    #[doc = "3: Select ref clock as TSU Clock"]
    REFCLK,
    #[doc = "4: Select tsu external pin as TSU Clock"]
    TSUEXTCLK,
}
impl From<TSUCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TSUCLKSEL_A) -> Self {
        match variant {
            TSUCLKSEL_A::NOCLOCK => 0,
            TSUCLKSEL_A::PLL => 1,
            TSUCLKSEL_A::RXCLK => 2,
            TSUCLKSEL_A::REFCLK => 3,
            TSUCLKSEL_A::TSUEXTCLK => 4,
        }
    }
}
#[doc = "Reader of field `TSUCLKSEL`"]
pub type TSUCLKSEL_R = crate::R<u8, TSUCLKSEL_A>;
impl TSUCLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSUCLKSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSUCLKSEL_A::NOCLOCK),
            1 => Val(TSUCLKSEL_A::PLL),
            2 => Val(TSUCLKSEL_A::RXCLK),
            3 => Val(TSUCLKSEL_A::REFCLK),
            4 => Val(TSUCLKSEL_A::TSUEXTCLK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOCLOCK`"]
    #[inline(always)]
    pub fn is_noclock(&self) -> bool {
        *self == TSUCLKSEL_A::NOCLOCK
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == TSUCLKSEL_A::PLL
    }
    #[doc = "Checks if the value of the field is `RXCLK`"]
    #[inline(always)]
    pub fn is_rxclk(&self) -> bool {
        *self == TSUCLKSEL_A::RXCLK
    }
    #[doc = "Checks if the value of the field is `REFCLK`"]
    #[inline(always)]
    pub fn is_refclk(&self) -> bool {
        *self == TSUCLKSEL_A::REFCLK
    }
    #[doc = "Checks if the value of the field is `TSUEXTCLK`"]
    #[inline(always)]
    pub fn is_tsuextclk(&self) -> bool {
        *self == TSUCLKSEL_A::TSUEXTCLK
    }
}
#[doc = "Write proxy for field `TSUCLKSEL`"]
pub struct TSUCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSUCLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No TSU clock source selected"]
    #[inline(always)]
    pub fn noclock(self) -> &'a mut W {
        self.variant(TSUCLKSEL_A::NOCLOCK)
    }
    #[doc = "Select system clock as TSU Clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(TSUCLKSEL_A::PLL)
    }
    #[doc = "Select ethernet RX Clock as TSU Clock"]
    #[inline(always)]
    pub fn rxclk(self) -> &'a mut W {
        self.variant(TSUCLKSEL_A::RXCLK)
    }
    #[doc = "Select ref clock as TSU Clock"]
    #[inline(always)]
    pub fn refclk(self) -> &'a mut W {
        self.variant(TSUCLKSEL_A::REFCLK)
    }
    #[doc = "Select tsu external pin as TSU Clock"]
    #[inline(always)]
    pub fn tsuextclk(self) -> &'a mut W {
        self.variant(TSUCLKSEL_A::TSUEXTCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `TSUPRESC`"]
pub type TSUPRESC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSUPRESC`"]
pub struct TSUPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUPRESC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MIISEL`"]
pub type MIISEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIISEL`"]
pub struct MIISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MIISEL_W<'a> {
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
#[doc = "Reader of field `GBLCLKEN`"]
pub type GBLCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GBLCLKEN`"]
pub struct GBLCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GBLCLKEN_W<'a> {
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
#[doc = "Reader of field `TXREFCLKSEL`"]
pub type TXREFCLKSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXREFCLKSEL`"]
pub struct TXREFCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXREFCLKSEL_W<'a> {
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
    #[doc = "Bits 0:2 - TSU Clock selection value"]
    #[inline(always)]
    pub fn tsuclksel(&self) -> TSUCLKSEL_R {
        TSUCLKSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - Clock division factor of TSUPRESC+1"]
    #[inline(always)]
    pub fn tsupresc(&self) -> TSUPRESC_R {
        TSUPRESC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - MII select signal"]
    #[inline(always)]
    pub fn miisel(&self) -> MIISEL_R {
        MIISEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk"]
    #[inline(always)]
    pub fn gblclken(&self) -> GBLCLKEN_R {
        GBLCLKEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - REFCLK source select for RMII_TXD and RMII_TX_EN"]
    #[inline(always)]
    pub fn txrefclksel(&self) -> TXREFCLKSEL_R {
        TXREFCLKSEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TSU Clock selection value"]
    #[inline(always)]
    pub fn tsuclksel(&mut self) -> TSUCLKSEL_W {
        TSUCLKSEL_W { w: self }
    }
    #[doc = "Bits 4:7 - Clock division factor of TSUPRESC+1"]
    #[inline(always)]
    pub fn tsupresc(&mut self) -> TSUPRESC_W {
        TSUPRESC_W { w: self }
    }
    #[doc = "Bit 8 - MII select signal"]
    #[inline(always)]
    pub fn miisel(&mut self) -> MIISEL_W {
        MIISEL_W { w: self }
    }
    #[doc = "Bit 9 - Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk"]
    #[inline(always)]
    pub fn gblclken(&mut self) -> GBLCLKEN_W {
        GBLCLKEN_W { w: self }
    }
    #[doc = "Bit 10 - REFCLK source select for RMII_TXD and RMII_TX_EN"]
    #[inline(always)]
    pub fn txrefclksel(&mut self) -> TXREFCLKSEL_W {
        TXREFCLKSEL_W { w: self }
    }
}
