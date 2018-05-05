#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LOCK {
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
#[doc = "Possible values of the field `TIMERLOCKKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMERLOCKKEYR {
    #[doc = "\"\""]
    UNLOCKED,
    #[doc = "\"\""]
    LOCKED,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl TIMERLOCKKEYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            TIMERLOCKKEYR::UNLOCKED => 0,
            TIMERLOCKKEYR::LOCKED => 1,
            TIMERLOCKKEYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> TIMERLOCKKEYR {
        match value {
            0 => TIMERLOCKKEYR::UNLOCKED,
            1 => TIMERLOCKKEYR::LOCKED,
            i => TIMERLOCKKEYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline]
    pub fn is_unlocked(&self) -> bool {
        *self == TIMERLOCKKEYR::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline]
    pub fn is_locked(&self) -> bool {
        *self == TIMERLOCKKEYR::LOCKED
    }
}
#[doc = "Values that can be written to the field `TIMERLOCKKEY`"]
pub enum TIMERLOCKKEYW {
    #[doc = "\"\""]
    UNLOCKED,
    #[doc = "\"\""]
    LOCKED,
}
impl TIMERLOCKKEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            TIMERLOCKKEYW::UNLOCKED => 0,
            TIMERLOCKKEYW::LOCKED => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMERLOCKKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMERLOCKKEYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMERLOCKKEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "\"\""]
    #[inline]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(TIMERLOCKKEYW::UNLOCKED)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn locked(self) -> &'a mut W {
        self.variant(TIMERLOCKKEYW::LOCKED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:15 - Timer Lock Key"]
    #[inline]
    pub fn timerlockkey(&self) -> TIMERLOCKKEYR {
        TIMERLOCKKEYR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
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
    #[doc = "Bits 0:15 - Timer Lock Key"]
    #[inline]
    pub fn timerlockkey(&mut self) -> _TIMERLOCKKEYW {
        _TIMERLOCKKEYW { w: self }
    }
}
