#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BANKSWITCHLOCK {
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
#[doc = "Possible values of the field `BANKSWITCHLOCKKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANKSWITCHLOCKKEYR {
    #[doc = "undocumented"]
    UNLOCKED,
    #[doc = "undocumented"]
    LOCKED,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl BANKSWITCHLOCKKEYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            BANKSWITCHLOCKKEYR::UNLOCKED => 0,
            BANKSWITCHLOCKKEYR::LOCKED => 1,
            BANKSWITCHLOCKKEYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> BANKSWITCHLOCKKEYR {
        match value {
            0 => BANKSWITCHLOCKKEYR::UNLOCKED,
            1 => BANKSWITCHLOCKKEYR::LOCKED,
            i => BANKSWITCHLOCKKEYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline]
    pub fn is_unlocked(&self) -> bool {
        *self == BANKSWITCHLOCKKEYR::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline]
    pub fn is_locked(&self) -> bool {
        *self == BANKSWITCHLOCKKEYR::LOCKED
    }
}
#[doc = "Values that can be written to the field `BANKSWITCHLOCKKEY`"]
pub enum BANKSWITCHLOCKKEYW {
    #[doc = "`0`"]
    UNLOCKED,
    #[doc = "`1`"]
    LOCKED,
}
impl BANKSWITCHLOCKKEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            BANKSWITCHLOCKKEYW::UNLOCKED => 0,
            BANKSWITCHLOCKKEYW::LOCKED => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BANKSWITCHLOCKKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _BANKSWITCHLOCKKEYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BANKSWITCHLOCKKEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(BANKSWITCHLOCKKEYW::UNLOCKED)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn locked(self) -> &'a mut W {
        self.variant(BANKSWITCHLOCKKEYW::LOCKED)
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
    #[doc = "Bits 0:15 - Bank Switching Lock"]
    #[inline]
    pub fn bankswitchlockkey(&self) -> BANKSWITCHLOCKKEYR {
        BANKSWITCHLOCKKEYR::_from({
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
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - Bank Switching Lock"]
    #[inline]
    pub fn bankswitchlockkey(&mut self) -> _BANKSWITCHLOCKKEYW {
        _BANKSWITCHLOCKKEYW { w: self }
    }
}
