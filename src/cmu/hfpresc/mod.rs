#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFPRESC {
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
#[doc = "Possible values of the field `PRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCR {
    #[doc = "\"\""]
    NODIVISION,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCR::NODIVISION => 0,
            PRESCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCR {
        match value {
            0 => PRESCR::NODIVISION,
            i => PRESCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESCR::NODIVISION
    }
}
#[doc = "Possible values of the field `HFCLKLEPRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLKLEPRESCR {
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 2."]
    DIV2,
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 4."]
    DIV4,
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 8."]
    DIV8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HFCLKLEPRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HFCLKLEPRESCR::DIV2 => 0,
            HFCLKLEPRESCR::DIV4 => 1,
            HFCLKLEPRESCR::DIV8 => 2,
            HFCLKLEPRESCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HFCLKLEPRESCR {
        match value {
            0 => HFCLKLEPRESCR::DIV2,
            1 => HFCLKLEPRESCR::DIV4,
            2 => HFCLKLEPRESCR::DIV8,
            i => HFCLKLEPRESCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == HFCLKLEPRESCR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == HFCLKLEPRESCR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == HFCLKLEPRESCR::DIV8
    }
}
#[doc = "Values that can be written to the field `PRESC`"]
pub enum PRESCW {
    #[doc = "\"\""]
    NODIVISION,
}
impl PRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCW::NODIVISION => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "\"\""]
    #[inline]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESCW::NODIVISION)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HFCLKLEPRESC`"]
pub enum HFCLKLEPRESCW {
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 2."]
    DIV2,
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 4."]
    DIV4,
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 8."]
    DIV8,
}
impl HFCLKLEPRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HFCLKLEPRESCW::DIV2 => 0,
            HFCLKLEPRESCW::DIV4 => 1,
            HFCLKLEPRESCW::DIV8 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFCLKLEPRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _HFCLKLEPRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFCLKLEPRESCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 2."]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(HFCLKLEPRESCW::DIV2)
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 4."]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(HFCLKLEPRESCW::DIV4)
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 8."]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(HFCLKLEPRESCW::DIV8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 8:12 - HFCLK Prescaler"]
    #[inline]
    pub fn presc(&self) -> PRESCR {
        PRESCR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - HFCLKLE Prescaler"]
    #[inline]
    pub fn hfclklepresc(&self) -> HFCLKLEPRESCR {
        HFCLKLEPRESCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
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
    #[doc = "Bits 8:12 - HFCLK Prescaler"]
    #[inline]
    pub fn presc(&mut self) -> _PRESCW {
        _PRESCW { w: self }
    }
    #[doc = "Bits 24:25 - HFCLKLE Prescaler"]
    #[inline]
    pub fn hfclklepresc(&mut self) -> _HFCLKLEPRESCW {
        _HFCLKLEPRESCW { w: self }
    }
}
