#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BLKSIZE {
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
#[doc = "Possible values of the field `TFRBLKSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFRBLKSIZER {
    #[doc = "\"\""]
    NOXFER,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl TFRBLKSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            TFRBLKSIZER::NOXFER => 0,
            TFRBLKSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> TFRBLKSIZER {
        match value {
            0 => TFRBLKSIZER::NOXFER,
            i => TFRBLKSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOXFER`"]
    #[inline]
    pub fn is_noxfer(&self) -> bool {
        *self == TFRBLKSIZER::NOXFER
    }
}
#[doc = "Possible values of the field `HSTSDMABUFSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSTSDMABUFSIZER {
    #[doc = "4KB(Detects A11 Carry out)"]
    SIZE4,
    #[doc = "8KB(Detects A12 Carry out)"]
    SIZE8,
    #[doc = "16KB(Detects A13 Carry out)"]
    SIZE16,
    #[doc = "32KB(Detects A14 Carry out)"]
    SIZE32,
    #[doc = "64KB(Detects A15 Carry out)"]
    SIZE64,
    #[doc = "128KB(Detects A16 Carry out)"]
    SIZE128,
    #[doc = "256KB(Detects A17 Carry out)"]
    SIZE256,
    #[doc = "512KB(Detects A18 Carry out)"]
    SIZE512,
}
impl HSTSDMABUFSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HSTSDMABUFSIZER::SIZE4 => 0,
            HSTSDMABUFSIZER::SIZE8 => 1,
            HSTSDMABUFSIZER::SIZE16 => 2,
            HSTSDMABUFSIZER::SIZE32 => 3,
            HSTSDMABUFSIZER::SIZE64 => 4,
            HSTSDMABUFSIZER::SIZE128 => 5,
            HSTSDMABUFSIZER::SIZE256 => 6,
            HSTSDMABUFSIZER::SIZE512 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HSTSDMABUFSIZER {
        match value {
            0 => HSTSDMABUFSIZER::SIZE4,
            1 => HSTSDMABUFSIZER::SIZE8,
            2 => HSTSDMABUFSIZER::SIZE16,
            3 => HSTSDMABUFSIZER::SIZE32,
            4 => HSTSDMABUFSIZER::SIZE64,
            5 => HSTSDMABUFSIZER::SIZE128,
            6 => HSTSDMABUFSIZER::SIZE256,
            7 => HSTSDMABUFSIZER::SIZE512,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SIZE4`"]
    #[inline]
    pub fn is_size4(&self) -> bool {
        *self == HSTSDMABUFSIZER::SIZE4
    }
    #[doc = "Checks if the value of the field is `SIZE8`"]
    #[inline]
    pub fn is_size8(&self) -> bool {
        *self == HSTSDMABUFSIZER::SIZE8
    }
    #[doc = "Checks if the value of the field is `SIZE16`"]
    #[inline]
    pub fn is_size16(&self) -> bool {
        *self == HSTSDMABUFSIZER::SIZE16
    }
    #[doc = "Checks if the value of the field is `SIZE32`"]
    #[inline]
    pub fn is_size32(&self) -> bool {
        *self == HSTSDMABUFSIZER::SIZE32
    }
    #[doc = "Checks if the value of the field is `SIZE64`"]
    #[inline]
    pub fn is_size64(&self) -> bool {
        *self == HSTSDMABUFSIZER::SIZE64
    }
    #[doc = "Checks if the value of the field is `SIZE128`"]
    #[inline]
    pub fn is_size128(&self) -> bool {
        *self == HSTSDMABUFSIZER::SIZE128
    }
    #[doc = "Checks if the value of the field is `SIZE256`"]
    #[inline]
    pub fn is_size256(&self) -> bool {
        *self == HSTSDMABUFSIZER::SIZE256
    }
    #[doc = "Checks if the value of the field is `SIZE512`"]
    #[inline]
    pub fn is_size512(&self) -> bool {
        *self == HSTSDMABUFSIZER::SIZE512
    }
}
#[doc = "Possible values of the field `BLKSCNTFORCURRTFR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLKSCNTFORCURRTFRR {
    #[doc = "\"\""]
    STOPCNT,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl BLKSCNTFORCURRTFRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            BLKSCNTFORCURRTFRR::STOPCNT => 0,
            BLKSCNTFORCURRTFRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> BLKSCNTFORCURRTFRR {
        match value {
            0 => BLKSCNTFORCURRTFRR::STOPCNT,
            i => BLKSCNTFORCURRTFRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOPCNT`"]
    #[inline]
    pub fn is_stopcnt(&self) -> bool {
        *self == BLKSCNTFORCURRTFRR::STOPCNT
    }
}
#[doc = "Values that can be written to the field `TFRBLKSIZE`"]
pub enum TFRBLKSIZEW {
    #[doc = "\"\""]
    NOXFER,
}
impl TFRBLKSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            TFRBLKSIZEW::NOXFER => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFRBLKSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _TFRBLKSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFRBLKSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "\"\""]
    #[inline]
    pub fn noxfer(self) -> &'a mut W {
        self.variant(TFRBLKSIZEW::NOXFER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HSTSDMABUFSIZE`"]
pub enum HSTSDMABUFSIZEW {
    #[doc = "4KB(Detects A11 Carry out)"]
    SIZE4,
    #[doc = "8KB(Detects A12 Carry out)"]
    SIZE8,
    #[doc = "16KB(Detects A13 Carry out)"]
    SIZE16,
    #[doc = "32KB(Detects A14 Carry out)"]
    SIZE32,
    #[doc = "64KB(Detects A15 Carry out)"]
    SIZE64,
    #[doc = "128KB(Detects A16 Carry out)"]
    SIZE128,
    #[doc = "256KB(Detects A17 Carry out)"]
    SIZE256,
    #[doc = "512KB(Detects A18 Carry out)"]
    SIZE512,
}
impl HSTSDMABUFSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HSTSDMABUFSIZEW::SIZE4 => 0,
            HSTSDMABUFSIZEW::SIZE8 => 1,
            HSTSDMABUFSIZEW::SIZE16 => 2,
            HSTSDMABUFSIZEW::SIZE32 => 3,
            HSTSDMABUFSIZEW::SIZE64 => 4,
            HSTSDMABUFSIZEW::SIZE128 => 5,
            HSTSDMABUFSIZEW::SIZE256 => 6,
            HSTSDMABUFSIZEW::SIZE512 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSTSDMABUFSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _HSTSDMABUFSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSTSDMABUFSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4KB(Detects A11 Carry out)"]
    #[inline]
    pub fn size4(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZEW::SIZE4)
    }
    #[doc = "8KB(Detects A12 Carry out)"]
    #[inline]
    pub fn size8(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZEW::SIZE8)
    }
    #[doc = "16KB(Detects A13 Carry out)"]
    #[inline]
    pub fn size16(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZEW::SIZE16)
    }
    #[doc = "32KB(Detects A14 Carry out)"]
    #[inline]
    pub fn size32(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZEW::SIZE32)
    }
    #[doc = "64KB(Detects A15 Carry out)"]
    #[inline]
    pub fn size64(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZEW::SIZE64)
    }
    #[doc = "128KB(Detects A16 Carry out)"]
    #[inline]
    pub fn size128(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZEW::SIZE128)
    }
    #[doc = "256KB(Detects A17 Carry out)"]
    #[inline]
    pub fn size256(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZEW::SIZE256)
    }
    #[doc = "512KB(Detects A18 Carry out)"]
    #[inline]
    pub fn size512(self) -> &'a mut W {
        self.variant(HSTSDMABUFSIZEW::SIZE512)
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
#[doc = "Values that can be written to the field `BLKSCNTFORCURRTFR`"]
pub enum BLKSCNTFORCURRTFRW {
    #[doc = "\"\""]
    STOPCNT,
}
impl BLKSCNTFORCURRTFRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            BLKSCNTFORCURRTFRW::STOPCNT => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLKSCNTFORCURRTFRW<'a> {
    w: &'a mut W,
}
impl<'a> _BLKSCNTFORCURRTFRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLKSCNTFORCURRTFRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "\"\""]
    #[inline]
    pub fn stopcnt(self) -> &'a mut W {
        self.variant(BLKSCNTFORCURRTFRW::STOPCNT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:11 - Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53"]
    #[inline]
    pub fn tfrblksize(&self) -> TFRBLKSIZER {
        TFRBLKSIZER::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 12:14 - Host SDMA Buffer Size"]
    #[inline]
    pub fn hstsdmabufsize(&self) -> HSTSDMABUFSIZER {
        HSTSDMABUFSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:31 - Blocks Count for Current Transfer"]
    #[inline]
    pub fn blkscntforcurrtfr(&self) -> BLKSCNTFORCURRTFRR {
        BLKSCNTFORCURRTFRR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:11 - Transfer Block Size, Specifies the Block Size for Block Data Transfers for CMD17, CMD18, CMD24, CMD25, and CMD53"]
    #[inline]
    pub fn tfrblksize(&mut self) -> _TFRBLKSIZEW {
        _TFRBLKSIZEW { w: self }
    }
    #[doc = "Bits 12:14 - Host SDMA Buffer Size"]
    #[inline]
    pub fn hstsdmabufsize(&mut self) -> _HSTSDMABUFSIZEW {
        _HSTSDMABUFSIZEW { w: self }
    }
    #[doc = "Bits 16:31 - Blocks Count for Current Transfer"]
    #[inline]
    pub fn blkscntforcurrtfr(&mut self) -> _BLKSCNTFORCURRTFRW {
        _BLKSCNTFORCURRTFRW { w: self }
    }
}
