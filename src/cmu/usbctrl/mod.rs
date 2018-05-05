#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBCTRL {
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
        R {
            bits: self.register.get(),
        }
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
#[doc = "Possible values of the field `USBCLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBCLKSELR {
    #[doc = "USHFRCO (clock recovery) is clocking USB"]
    USHFRCO,
    #[doc = "HFXO clock is used to clock USB"]
    HFXO,
    #[doc = "HFXO clock doubler is used to clock USB"]
    HFXOX2,
    #[doc = "HFRCO clock is used to clock USB"]
    HFRCO,
    #[doc = "LFXO clock is used to clock USB"]
    LFXO,
    #[doc = "LFRCO clock is used to clock USB"]
    LFRCO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl USBCLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USBCLKSELR::USHFRCO => 0,
            USBCLKSELR::HFXO => 1,
            USBCLKSELR::HFXOX2 => 2,
            USBCLKSELR::HFRCO => 3,
            USBCLKSELR::LFXO => 4,
            USBCLKSELR::LFRCO => 5,
            USBCLKSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USBCLKSELR {
        match value {
            0 => USBCLKSELR::USHFRCO,
            1 => USBCLKSELR::HFXO,
            2 => USBCLKSELR::HFXOX2,
            3 => USBCLKSELR::HFRCO,
            4 => USBCLKSELR::LFXO,
            5 => USBCLKSELR::LFRCO,
            i => USBCLKSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline]
    pub fn is_ushfrco(&self) -> bool {
        *self == USBCLKSELR::USHFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline]
    pub fn is_hfxo(&self) -> bool {
        *self == USBCLKSELR::HFXO
    }
    #[doc = "Checks if the value of the field is `HFXOX2`"]
    #[inline]
    pub fn is_hfxox2(&self) -> bool {
        *self == USBCLKSELR::HFXOX2
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline]
    pub fn is_hfrco(&self) -> bool {
        *self == USBCLKSELR::HFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == USBCLKSELR::LFXO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == USBCLKSELR::LFRCO
    }
}
#[doc = r" Value of the field"]
pub struct USBCLKENR {
    bits: bool,
}
impl USBCLKENR {
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
#[doc = "Values that can be written to the field `USBCLKSEL`"]
pub enum USBCLKSELW {
    #[doc = "USHFRCO (clock recovery) is clocking USB"]
    USHFRCO,
    #[doc = "HFXO clock is used to clock USB"]
    HFXO,
    #[doc = "HFXO clock doubler is used to clock USB"]
    HFXOX2,
    #[doc = "HFRCO clock is used to clock USB"]
    HFRCO,
    #[doc = "LFXO clock is used to clock USB"]
    LFXO,
    #[doc = "LFRCO clock is used to clock USB"]
    LFRCO,
}
impl USBCLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            USBCLKSELW::USHFRCO => 0,
            USBCLKSELW::HFXO => 1,
            USBCLKSELW::HFXOX2 => 2,
            USBCLKSELW::HFRCO => 3,
            USBCLKSELW::LFXO => 4,
            USBCLKSELW::LFRCO => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _USBCLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBCLKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "USHFRCO (clock recovery) is clocking USB"]
    #[inline]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(USBCLKSELW::USHFRCO)
    }
    #[doc = "HFXO clock is used to clock USB"]
    #[inline]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(USBCLKSELW::HFXO)
    }
    #[doc = "HFXO clock doubler is used to clock USB"]
    #[inline]
    pub fn hfxox2(self) -> &'a mut W {
        self.variant(USBCLKSELW::HFXOX2)
    }
    #[doc = "HFRCO clock is used to clock USB"]
    #[inline]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(USBCLKSELW::HFRCO)
    }
    #[doc = "LFXO clock is used to clock USB"]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(USBCLKSELW::LFXO)
    }
    #[doc = "LFRCO clock is used to clock USB"]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(USBCLKSELW::LFRCO)
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
pub struct _USBCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _USBCLKENW<'a> {
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
    #[doc = "Bits 0:2 - USB Rate Clock Select"]
    #[inline]
    pub fn usbclksel(&self) -> USBCLKSELR {
        USBCLKSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - USB Rate Clock Enable"]
    #[inline]
    pub fn usbclken(&self) -> USBCLKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USBCLKENR { bits }
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
    #[doc = "Bits 0:2 - USB Rate Clock Select"]
    #[inline]
    pub fn usbclksel(&mut self) -> _USBCLKSELW {
        _USBCLKSELW { w: self }
    }
    #[doc = "Bit 7 - USB Rate Clock Enable"]
    #[inline]
    pub fn usbclken(&mut self) -> _USBCLKENW {
        _USBCLKENW { w: self }
    }
}
