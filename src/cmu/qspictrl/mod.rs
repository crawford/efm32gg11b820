#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::QSPICTRL {
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
#[doc = "Possible values of the field `QSPI0CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QSPI0CLKSELR {
    #[doc = "HFRCO clock is used to clock QSPI0"]
    HFRCO,
    #[doc = "HFXO clock is used to clock QSPI0"]
    HFXO,
    #[doc = "AUXHFRCO is used to clock QSPI0"]
    AUXHFRCO,
    #[doc = "USHFRCO is used to clock QSPI0"]
    USHFRCO,
}
impl QSPI0CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            QSPI0CLKSELR::HFRCO => 0,
            QSPI0CLKSELR::HFXO => 1,
            QSPI0CLKSELR::AUXHFRCO => 2,
            QSPI0CLKSELR::USHFRCO => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> QSPI0CLKSELR {
        match value {
            0 => QSPI0CLKSELR::HFRCO,
            1 => QSPI0CLKSELR::HFXO,
            2 => QSPI0CLKSELR::AUXHFRCO,
            3 => QSPI0CLKSELR::USHFRCO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline]
    pub fn is_hfrco(&self) -> bool {
        *self == QSPI0CLKSELR::HFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline]
    pub fn is_hfxo(&self) -> bool {
        *self == QSPI0CLKSELR::HFXO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline]
    pub fn is_auxhfrco(&self) -> bool {
        *self == QSPI0CLKSELR::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline]
    pub fn is_ushfrco(&self) -> bool {
        *self == QSPI0CLKSELR::USHFRCO
    }
}
#[doc = r" Value of the field"]
pub struct QSPI0CLKDISR {
    bits: bool,
}
impl QSPI0CLKDISR {
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
#[doc = "Values that can be written to the field `QSPI0CLKSEL`"]
pub enum QSPI0CLKSELW {
    #[doc = "HFRCO clock is used to clock QSPI0"]
    HFRCO,
    #[doc = "HFXO clock is used to clock QSPI0"]
    HFXO,
    #[doc = "AUXHFRCO is used to clock QSPI0"]
    AUXHFRCO,
    #[doc = "USHFRCO is used to clock QSPI0"]
    USHFRCO,
}
impl QSPI0CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            QSPI0CLKSELW::HFRCO => 0,
            QSPI0CLKSELW::HFXO => 1,
            QSPI0CLKSELW::AUXHFRCO => 2,
            QSPI0CLKSELW::USHFRCO => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QSPI0CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _QSPI0CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QSPI0CLKSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "HFRCO clock is used to clock QSPI0"]
    #[inline]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(QSPI0CLKSELW::HFRCO)
    }
    #[doc = "HFXO clock is used to clock QSPI0"]
    #[inline]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(QSPI0CLKSELW::HFXO)
    }
    #[doc = "AUXHFRCO is used to clock QSPI0"]
    #[inline]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(QSPI0CLKSELW::AUXHFRCO)
    }
    #[doc = "USHFRCO is used to clock QSPI0"]
    #[inline]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(QSPI0CLKSELW::USHFRCO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _QSPI0CLKDISW<'a> {
    w: &'a mut W,
}
impl<'a> _QSPI0CLKDISW<'a> {
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bits 0:1 - QSPI0 Reference Clock Select"]
    #[inline]
    pub fn qspi0clksel(&self) -> QSPI0CLKSELR {
        QSPI0CLKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - QSPI0 Reference Clock Disable"]
    #[inline]
    pub fn qspi0clkdis(&self) -> QSPI0CLKDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        QSPI0CLKDISR { bits }
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
    #[doc = "Bits 0:1 - QSPI0 Reference Clock Select"]
    #[inline]
    pub fn qspi0clksel(&mut self) -> _QSPI0CLKSELW {
        _QSPI0CLKSELW { w: self }
    }
    #[doc = "Bit 7 - QSPI0 Reference Clock Disable"]
    #[inline]
    pub fn qspi0clkdis(&mut self) -> _QSPI0CLKDISW {
        _QSPI0CLKDISW { w: self }
    }
}
