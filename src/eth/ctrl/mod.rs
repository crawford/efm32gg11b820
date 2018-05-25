#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `TSUCLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSUCLKSELR {
    #[doc = "No TSU clock source selected"]
    NOCLOCK,
    #[doc = "Select system clock as TSU Clock"]
    PLL,
    #[doc = "Select ethernet RX Clock as TSU Clock"]
    RXCLK,
    #[doc = "Select ref clock as TSU Clock"]
    REFCLK,
    #[doc = "Select tsu external pin as TSU Clock"]
    TSUEXTCLK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSUCLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSUCLKSELR::NOCLOCK => 0,
            TSUCLKSELR::PLL => 1,
            TSUCLKSELR::RXCLK => 2,
            TSUCLKSELR::REFCLK => 3,
            TSUCLKSELR::TSUEXTCLK => 4,
            TSUCLKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSUCLKSELR {
        match value {
            0 => TSUCLKSELR::NOCLOCK,
            1 => TSUCLKSELR::PLL,
            2 => TSUCLKSELR::RXCLK,
            3 => TSUCLKSELR::REFCLK,
            4 => TSUCLKSELR::TSUEXTCLK,
            i => TSUCLKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOCLOCK`"]
    #[inline]
    pub fn is_noclock(&self) -> bool {
        *self == TSUCLKSELR::NOCLOCK
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline]
    pub fn is_pll(&self) -> bool {
        *self == TSUCLKSELR::PLL
    }
    #[doc = "Checks if the value of the field is `RXCLK`"]
    #[inline]
    pub fn is_rxclk(&self) -> bool {
        *self == TSUCLKSELR::RXCLK
    }
    #[doc = "Checks if the value of the field is `REFCLK`"]
    #[inline]
    pub fn is_refclk(&self) -> bool {
        *self == TSUCLKSELR::REFCLK
    }
    #[doc = "Checks if the value of the field is `TSUEXTCLK`"]
    #[inline]
    pub fn is_tsuextclk(&self) -> bool {
        *self == TSUCLKSELR::TSUEXTCLK
    }
}
#[doc = r" Value of the field"]
pub struct TSUPRESCR {
    bits: u8,
}
impl TSUPRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MIISELR {
    bits: bool,
}
impl MIISELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct GBLCLKENR {
    bits: bool,
}
impl GBLCLKENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TXREFCLKSELR {
    bits: bool,
}
impl TXREFCLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `TSUCLKSEL`"]
pub enum TSUCLKSELW {
    #[doc = "No TSU clock source selected"]
    NOCLOCK,
    #[doc = "Select system clock as TSU Clock"]
    PLL,
    #[doc = "Select ethernet RX Clock as TSU Clock"]
    RXCLK,
    #[doc = "Select ref clock as TSU Clock"]
    REFCLK,
    #[doc = "Select tsu external pin as TSU Clock"]
    TSUEXTCLK,
}
impl TSUCLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSUCLKSELW::NOCLOCK => 0,
            TSUCLKSELW::PLL => 1,
            TSUCLKSELW::RXCLK => 2,
            TSUCLKSELW::REFCLK => 3,
            TSUCLKSELW::TSUEXTCLK => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSUCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TSUCLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSUCLKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No TSU clock source selected"]
    #[inline]
    pub fn noclock(self) -> &'a mut W {
        self.variant(TSUCLKSELW::NOCLOCK)
    }
    #[doc = "Select system clock as TSU Clock"]
    #[inline]
    pub fn pll(self) -> &'a mut W {
        self.variant(TSUCLKSELW::PLL)
    }
    #[doc = "Select ethernet RX Clock as TSU Clock"]
    #[inline]
    pub fn rxclk(self) -> &'a mut W {
        self.variant(TSUCLKSELW::RXCLK)
    }
    #[doc = "Select ref clock as TSU Clock"]
    #[inline]
    pub fn refclk(self) -> &'a mut W {
        self.variant(TSUCLKSELW::REFCLK)
    }
    #[doc = "Select tsu external pin as TSU Clock"]
    #[inline]
    pub fn tsuextclk(self) -> &'a mut W {
        self.variant(TSUCLKSELW::TSUEXTCLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSUPRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _TSUPRESCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MIISELW<'a> {
    w: &'a mut W,
}
impl<'a> _MIISELW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GBLCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _GBLCLKENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXREFCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TXREFCLKSELW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - TSU Clock selection value"]
    #[inline]
    pub fn tsuclksel(&self) -> TSUCLKSELR {
        TSUCLKSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Clock division factor of TSUPRESC+1"]
    #[inline]
    pub fn tsupresc(&self) -> TSUPRESCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TSUPRESCR { bits }
    }
    #[doc = "Bit 8 - MII select signal"]
    #[inline]
    pub fn miisel(&self) -> MIISELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MIISELR { bits }
    }
    #[doc = "Bit 9 - Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk"]
    #[inline]
    pub fn gblclken(&self) -> GBLCLKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GBLCLKENR { bits }
    }
    #[doc = "Bit 10 - REFCLK source select for RMII_TXD and RMII_TX_EN"]
    #[inline]
    pub fn txrefclksel(&self) -> TXREFCLKSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXREFCLKSELR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - TSU Clock selection value"]
    #[inline]
    pub fn tsuclksel(&mut self) -> _TSUCLKSELW {
        _TSUCLKSELW { w: self }
    }
    #[doc = "Bits 4:7 - Clock division factor of TSUPRESC+1"]
    #[inline]
    pub fn tsupresc(&mut self) -> _TSUPRESCW {
        _TSUPRESCW { w: self }
    }
    #[doc = "Bit 8 - MII select signal"]
    #[inline]
    pub fn miisel(&mut self) -> _MIISELW {
        _MIISELW { w: self }
    }
    #[doc = "Bit 9 - Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk"]
    #[inline]
    pub fn gblclken(&mut self) -> _GBLCLKENW {
        _GBLCLKENW { w: self }
    }
    #[doc = "Bit 10 - REFCLK source select for RMII_TXD and RMII_TX_EN"]
    #[inline]
    pub fn txrefclksel(&mut self) -> _TXREFCLKSELW {
        _TXREFCLKSELW { w: self }
    }
}
