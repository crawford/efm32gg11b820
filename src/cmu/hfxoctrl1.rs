#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFXOCTRL1 {
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
#[doc = "Possible values of the field `PEAKDETTHR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEAKDETTHRR {
    #[doc = "50mV amplitude detection level"]
    THR0,
    #[doc = "75mV amplitude detection level"]
    THR1,
    #[doc = "115mV amplitude detection level"]
    THR2,
    #[doc = "160mV amplitude detection level"]
    THR3,
    #[doc = "220mV amplitude detection level"]
    THR4,
    #[doc = "260mV amplitude detection level"]
    THR5,
    #[doc = "320mV amplitude detection level"]
    THR6,
    #[doc = "Same as THR6"]
    THR7,
}
impl PEAKDETTHRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PEAKDETTHRR::THR0 => 0,
            PEAKDETTHRR::THR1 => 1,
            PEAKDETTHRR::THR2 => 2,
            PEAKDETTHRR::THR3 => 3,
            PEAKDETTHRR::THR4 => 4,
            PEAKDETTHRR::THR5 => 5,
            PEAKDETTHRR::THR6 => 6,
            PEAKDETTHRR::THR7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PEAKDETTHRR {
        match value {
            0 => PEAKDETTHRR::THR0,
            1 => PEAKDETTHRR::THR1,
            2 => PEAKDETTHRR::THR2,
            3 => PEAKDETTHRR::THR3,
            4 => PEAKDETTHRR::THR4,
            5 => PEAKDETTHRR::THR5,
            6 => PEAKDETTHRR::THR6,
            7 => PEAKDETTHRR::THR7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `THR0`"]
    #[inline]
    pub fn is_thr0(&self) -> bool {
        *self == PEAKDETTHRR::THR0
    }
    #[doc = "Checks if the value of the field is `THR1`"]
    #[inline]
    pub fn is_thr1(&self) -> bool {
        *self == PEAKDETTHRR::THR1
    }
    #[doc = "Checks if the value of the field is `THR2`"]
    #[inline]
    pub fn is_thr2(&self) -> bool {
        *self == PEAKDETTHRR::THR2
    }
    #[doc = "Checks if the value of the field is `THR3`"]
    #[inline]
    pub fn is_thr3(&self) -> bool {
        *self == PEAKDETTHRR::THR3
    }
    #[doc = "Checks if the value of the field is `THR4`"]
    #[inline]
    pub fn is_thr4(&self) -> bool {
        *self == PEAKDETTHRR::THR4
    }
    #[doc = "Checks if the value of the field is `THR5`"]
    #[inline]
    pub fn is_thr5(&self) -> bool {
        *self == PEAKDETTHRR::THR5
    }
    #[doc = "Checks if the value of the field is `THR6`"]
    #[inline]
    pub fn is_thr6(&self) -> bool {
        *self == PEAKDETTHRR::THR6
    }
    #[doc = "Checks if the value of the field is `THR7`"]
    #[inline]
    pub fn is_thr7(&self) -> bool {
        *self == PEAKDETTHRR::THR7
    }
}
#[doc = "Values that can be written to the field `PEAKDETTHR`"]
pub enum PEAKDETTHRW {
    #[doc = "50mV amplitude detection level"]
    THR0,
    #[doc = "75mV amplitude detection level"]
    THR1,
    #[doc = "115mV amplitude detection level"]
    THR2,
    #[doc = "160mV amplitude detection level"]
    THR3,
    #[doc = "220mV amplitude detection level"]
    THR4,
    #[doc = "260mV amplitude detection level"]
    THR5,
    #[doc = "320mV amplitude detection level"]
    THR6,
    #[doc = "Same as THR6"]
    THR7,
}
impl PEAKDETTHRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PEAKDETTHRW::THR0 => 0,
            PEAKDETTHRW::THR1 => 1,
            PEAKDETTHRW::THR2 => 2,
            PEAKDETTHRW::THR3 => 3,
            PEAKDETTHRW::THR4 => 4,
            PEAKDETTHRW::THR5 => 5,
            PEAKDETTHRW::THR6 => 6,
            PEAKDETTHRW::THR7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEAKDETTHRW<'a> {
    w: &'a mut W,
}
impl<'a> _PEAKDETTHRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEAKDETTHRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "50mV amplitude detection level"]
    #[inline]
    pub fn thr0(self) -> &'a mut W {
        self.variant(PEAKDETTHRW::THR0)
    }
    #[doc = "75mV amplitude detection level"]
    #[inline]
    pub fn thr1(self) -> &'a mut W {
        self.variant(PEAKDETTHRW::THR1)
    }
    #[doc = "115mV amplitude detection level"]
    #[inline]
    pub fn thr2(self) -> &'a mut W {
        self.variant(PEAKDETTHRW::THR2)
    }
    #[doc = "160mV amplitude detection level"]
    #[inline]
    pub fn thr3(self) -> &'a mut W {
        self.variant(PEAKDETTHRW::THR3)
    }
    #[doc = "220mV amplitude detection level"]
    #[inline]
    pub fn thr4(self) -> &'a mut W {
        self.variant(PEAKDETTHRW::THR4)
    }
    #[doc = "260mV amplitude detection level"]
    #[inline]
    pub fn thr5(self) -> &'a mut W {
        self.variant(PEAKDETTHRW::THR5)
    }
    #[doc = "320mV amplitude detection level"]
    #[inline]
    pub fn thr6(self) -> &'a mut W {
        self.variant(PEAKDETTHRW::THR6)
    }
    #[doc = "Same as THR6"]
    #[inline]
    pub fn thr7(self) -> &'a mut W {
        self.variant(PEAKDETTHRW::THR7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 12:14 - Sets the Amplitude Detection Level (mV)"]
    #[inline]
    pub fn peakdetthr(&self) -> PEAKDETTHRR {
        PEAKDETTHRR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8192 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 12:14 - Sets the Amplitude Detection Level (mV)"]
    #[inline]
    pub fn peakdetthr(&mut self) -> _PEAKDETTHRW {
        _PEAKDETTHRW { w: self }
    }
}
