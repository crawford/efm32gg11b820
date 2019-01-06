#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::R5VADCCTRL {
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
#[doc = r" Value of the field"]
pub struct ENAMUXR {
    bits: bool,
}
impl ENAMUXR {
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
#[doc = "Possible values of the field `AMUXSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMUXSELR {
    #[doc = "VBUS divided by 10"]
    VBUSDIV10,
    #[doc = "VREGI divided by 10"]
    VREGIDIV10,
    #[doc = "VREGO divided by 6"]
    VREGODIV6,
    #[doc = "VREGI current monitor"]
    VREGIIMON,
    #[doc = "VBUS current monitor"]
    VBUSIMON,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AMUXSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AMUXSELR::VBUSDIV10 => 0,
            AMUXSELR::VREGIDIV10 => 1,
            AMUXSELR::VREGODIV6 => 2,
            AMUXSELR::VREGIIMON => 3,
            AMUXSELR::VBUSIMON => 4,
            AMUXSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AMUXSELR {
        match value {
            0 => AMUXSELR::VBUSDIV10,
            1 => AMUXSELR::VREGIDIV10,
            2 => AMUXSELR::VREGODIV6,
            3 => AMUXSELR::VREGIIMON,
            4 => AMUXSELR::VBUSIMON,
            i => AMUXSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VBUSDIV10`"]
    #[inline]
    pub fn is_vbusdiv10(&self) -> bool {
        *self == AMUXSELR::VBUSDIV10
    }
    #[doc = "Checks if the value of the field is `VREGIDIV10`"]
    #[inline]
    pub fn is_vregidiv10(&self) -> bool {
        *self == AMUXSELR::VREGIDIV10
    }
    #[doc = "Checks if the value of the field is `VREGODIV6`"]
    #[inline]
    pub fn is_vregodiv6(&self) -> bool {
        *self == AMUXSELR::VREGODIV6
    }
    #[doc = "Checks if the value of the field is `VREGIIMON`"]
    #[inline]
    pub fn is_vregiimon(&self) -> bool {
        *self == AMUXSELR::VREGIIMON
    }
    #[doc = "Checks if the value of the field is `VBUSIMON`"]
    #[inline]
    pub fn is_vbusimon(&self) -> bool {
        *self == AMUXSELR::VBUSIMON
    }
}
#[doc = r" Proxy"]
pub struct _ENAMUXW<'a> {
    w: &'a mut W,
}
impl<'a> _ENAMUXW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AMUXSEL`"]
pub enum AMUXSELW {
    #[doc = "VBUS divided by 10"]
    VBUSDIV10,
    #[doc = "VREGI divided by 10"]
    VREGIDIV10,
    #[doc = "VREGO divided by 6"]
    VREGODIV6,
    #[doc = "VREGI current monitor"]
    VREGIIMON,
    #[doc = "VBUS current monitor"]
    VBUSIMON,
}
impl AMUXSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AMUXSELW::VBUSDIV10 => 0,
            AMUXSELW::VREGIDIV10 => 1,
            AMUXSELW::VREGODIV6 => 2,
            AMUXSELW::VREGIIMON => 3,
            AMUXSELW::VBUSIMON => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AMUXSELW<'a> {
    w: &'a mut W,
}
impl<'a> _AMUXSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AMUXSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "VBUS divided by 10"]
    #[inline]
    pub fn vbusdiv10(self) -> &'a mut W {
        self.variant(AMUXSELW::VBUSDIV10)
    }
    #[doc = "VREGI divided by 10"]
    #[inline]
    pub fn vregidiv10(self) -> &'a mut W {
        self.variant(AMUXSELW::VREGIDIV10)
    }
    #[doc = "VREGO divided by 6"]
    #[inline]
    pub fn vregodiv6(self) -> &'a mut W {
        self.variant(AMUXSELW::VREGODIV6)
    }
    #[doc = "VREGI current monitor"]
    #[inline]
    pub fn vregiimon(self) -> &'a mut W {
        self.variant(AMUXSELW::VREGIIMON)
    }
    #[doc = "VBUS current monitor"]
    #[inline]
    pub fn vbusimon(self) -> &'a mut W {
        self.variant(AMUXSELW::VBUSIMON)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Enable the 5V Subsystem ADC MUX"]
    #[inline]
    pub fn enamux(&self) -> ENAMUXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENAMUXR { bits }
    }
    #[doc = "Bits 12:15 - ADC Mux Selection"]
    #[inline]
    pub fn amuxsel(&self) -> AMUXSELR {
        AMUXSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bit 0 - Enable the 5V Subsystem ADC MUX"]
    #[inline]
    pub fn enamux(&mut self) -> _ENAMUXW {
        _ENAMUXW { w: self }
    }
    #[doc = "Bits 12:15 - ADC Mux Selection"]
    #[inline]
    pub fn amuxsel(&mut self) -> _AMUXSELW {
        _AMUXSELW { w: self }
    }
}
