#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCDCLPEM01CFG {
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
#[doc = "Possible values of the field `LPCMPBIASEM01`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCMPBIASEM01R {
    #[doc = "Maximum load current less than 75uA."]
    BIAS0,
    #[doc = "Maximum load current less than 500uA."]
    BIAS1,
    #[doc = "Maximum load current less than 2.5mA."]
    BIAS2,
    #[doc = "Maximum load current less than 10mA."]
    BIAS3,
}
impl LPCMPBIASEM01R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPCMPBIASEM01R::BIAS0 => 0,
            LPCMPBIASEM01R::BIAS1 => 1,
            LPCMPBIASEM01R::BIAS2 => 2,
            LPCMPBIASEM01R::BIAS3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPCMPBIASEM01R {
        match value {
            0 => LPCMPBIASEM01R::BIAS0,
            1 => LPCMPBIASEM01R::BIAS1,
            2 => LPCMPBIASEM01R::BIAS2,
            3 => LPCMPBIASEM01R::BIAS3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS0`"]
    #[inline]
    pub fn is_bias0(&self) -> bool {
        *self == LPCMPBIASEM01R::BIAS0
    }
    #[doc = "Checks if the value of the field is `BIAS1`"]
    #[inline]
    pub fn is_bias1(&self) -> bool {
        *self == LPCMPBIASEM01R::BIAS1
    }
    #[doc = "Checks if the value of the field is `BIAS2`"]
    #[inline]
    pub fn is_bias2(&self) -> bool {
        *self == LPCMPBIASEM01R::BIAS2
    }
    #[doc = "Checks if the value of the field is `BIAS3`"]
    #[inline]
    pub fn is_bias3(&self) -> bool {
        *self == LPCMPBIASEM01R::BIAS3
    }
}
#[doc = r" Value of the field"]
pub struct LPCMPHYSSELEM01R {
    bits: u8,
}
impl LPCMPHYSSELEM01R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `LPCMPBIASEM01`"]
pub enum LPCMPBIASEM01W {
    #[doc = "Maximum load current less than 75uA."]
    BIAS0,
    #[doc = "Maximum load current less than 500uA."]
    BIAS1,
    #[doc = "Maximum load current less than 2.5mA."]
    BIAS2,
    #[doc = "Maximum load current less than 10mA."]
    BIAS3,
}
impl LPCMPBIASEM01W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPCMPBIASEM01W::BIAS0 => 0,
            LPCMPBIASEM01W::BIAS1 => 1,
            LPCMPBIASEM01W::BIAS2 => 2,
            LPCMPBIASEM01W::BIAS3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPCMPBIASEM01W<'a> {
    w: &'a mut W,
}
impl<'a> _LPCMPBIASEM01W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPCMPBIASEM01W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Maximum load current less than 75uA."]
    #[inline]
    pub fn bias0(self) -> &'a mut W {
        self.variant(LPCMPBIASEM01W::BIAS0)
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline]
    pub fn bias1(self) -> &'a mut W {
        self.variant(LPCMPBIASEM01W::BIAS1)
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline]
    pub fn bias2(self) -> &'a mut W {
        self.variant(LPCMPBIASEM01W::BIAS2)
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline]
    pub fn bias3(self) -> &'a mut W {
        self.variant(LPCMPBIASEM01W::BIAS3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPCMPHYSSELEM01W<'a> {
    w: &'a mut W,
}
impl<'a> _LPCMPHYSSELEM01W<'a> {
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
    #[doc = "Bits 8:9 - LP Mode Comparator Bias Selection for EM01"]
    #[inline]
    pub fn lpcmpbiasem01(&self) -> LPCMPBIASEM01R {
        LPCMPBIASEM01R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection for EM01"]
    #[inline]
    pub fn lpcmphysselem01(&self) -> LPCMPHYSSELEM01R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPCMPHYSSELEM01R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 768 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:9 - LP Mode Comparator Bias Selection for EM01"]
    #[inline]
    pub fn lpcmpbiasem01(&mut self) -> _LPCMPBIASEM01W {
        _LPCMPBIASEM01W { w: self }
    }
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection for EM01"]
    #[inline]
    pub fn lpcmphysselem01(&mut self) -> _LPCMPHYSSELEM01W {
        _LPCMPHYSSELEM01W { w: self }
    }
}
