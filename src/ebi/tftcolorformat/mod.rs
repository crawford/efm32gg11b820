#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TFTCOLORFORMAT {
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
#[doc = "Possible values of the field `PIXEL0FORMAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIXEL0FORMATR {
    #[doc = "ARGB data is 0555"]
    ARGB0555,
    #[doc = "ARGB data is 0565"]
    ARGB0565,
    #[doc = "ARGB data is 0666"]
    ARGB0666,
    #[doc = "ARGB data is 0888"]
    ARGB0888,
    #[doc = "ARGB data is 5555"]
    ARGB5555,
    #[doc = "ARGB data is 6565"]
    ARGB6565,
    #[doc = "ARGB data is 6666"]
    ARGB6666,
    #[doc = "ARGB data is 8888"]
    ARGB8888,
}
impl PIXEL0FORMATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PIXEL0FORMATR::ARGB0555 => 0,
            PIXEL0FORMATR::ARGB0565 => 1,
            PIXEL0FORMATR::ARGB0666 => 2,
            PIXEL0FORMATR::ARGB0888 => 3,
            PIXEL0FORMATR::ARGB5555 => 4,
            PIXEL0FORMATR::ARGB6565 => 5,
            PIXEL0FORMATR::ARGB6666 => 6,
            PIXEL0FORMATR::ARGB8888 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PIXEL0FORMATR {
        match value {
            0 => PIXEL0FORMATR::ARGB0555,
            1 => PIXEL0FORMATR::ARGB0565,
            2 => PIXEL0FORMATR::ARGB0666,
            3 => PIXEL0FORMATR::ARGB0888,
            4 => PIXEL0FORMATR::ARGB5555,
            5 => PIXEL0FORMATR::ARGB6565,
            6 => PIXEL0FORMATR::ARGB6666,
            7 => PIXEL0FORMATR::ARGB8888,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ARGB0555`"]
    #[inline]
    pub fn is_argb0555(&self) -> bool {
        *self == PIXEL0FORMATR::ARGB0555
    }
    #[doc = "Checks if the value of the field is `ARGB0565`"]
    #[inline]
    pub fn is_argb0565(&self) -> bool {
        *self == PIXEL0FORMATR::ARGB0565
    }
    #[doc = "Checks if the value of the field is `ARGB0666`"]
    #[inline]
    pub fn is_argb0666(&self) -> bool {
        *self == PIXEL0FORMATR::ARGB0666
    }
    #[doc = "Checks if the value of the field is `ARGB0888`"]
    #[inline]
    pub fn is_argb0888(&self) -> bool {
        *self == PIXEL0FORMATR::ARGB0888
    }
    #[doc = "Checks if the value of the field is `ARGB5555`"]
    #[inline]
    pub fn is_argb5555(&self) -> bool {
        *self == PIXEL0FORMATR::ARGB5555
    }
    #[doc = "Checks if the value of the field is `ARGB6565`"]
    #[inline]
    pub fn is_argb6565(&self) -> bool {
        *self == PIXEL0FORMATR::ARGB6565
    }
    #[doc = "Checks if the value of the field is `ARGB6666`"]
    #[inline]
    pub fn is_argb6666(&self) -> bool {
        *self == PIXEL0FORMATR::ARGB6666
    }
    #[doc = "Checks if the value of the field is `ARGB8888`"]
    #[inline]
    pub fn is_argb8888(&self) -> bool {
        *self == PIXEL0FORMATR::ARGB8888
    }
}
#[doc = "Possible values of the field `PIXEL1FORMAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIXEL1FORMATR {
    #[doc = "RGB data is 555"]
    RGB555,
    #[doc = "RGB data is 565"]
    RGB565,
    #[doc = "RGB data is 666"]
    RGB666,
    #[doc = "RGB data is 888"]
    RGB888,
}
impl PIXEL1FORMATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PIXEL1FORMATR::RGB555 => 0,
            PIXEL1FORMATR::RGB565 => 1,
            PIXEL1FORMATR::RGB666 => 2,
            PIXEL1FORMATR::RGB888 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PIXEL1FORMATR {
        match value {
            0 => PIXEL1FORMATR::RGB555,
            1 => PIXEL1FORMATR::RGB565,
            2 => PIXEL1FORMATR::RGB666,
            3 => PIXEL1FORMATR::RGB888,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RGB555`"]
    #[inline]
    pub fn is_rgb555(&self) -> bool {
        *self == PIXEL1FORMATR::RGB555
    }
    #[doc = "Checks if the value of the field is `RGB565`"]
    #[inline]
    pub fn is_rgb565(&self) -> bool {
        *self == PIXEL1FORMATR::RGB565
    }
    #[doc = "Checks if the value of the field is `RGB666`"]
    #[inline]
    pub fn is_rgb666(&self) -> bool {
        *self == PIXEL1FORMATR::RGB666
    }
    #[doc = "Checks if the value of the field is `RGB888`"]
    #[inline]
    pub fn is_rgb888(&self) -> bool {
        *self == PIXEL1FORMATR::RGB888
    }
}
#[doc = "Values that can be written to the field `PIXEL0FORMAT`"]
pub enum PIXEL0FORMATW {
    #[doc = "ARGB data is 0555"]
    ARGB0555,
    #[doc = "ARGB data is 0565"]
    ARGB0565,
    #[doc = "ARGB data is 0666"]
    ARGB0666,
    #[doc = "ARGB data is 0888"]
    ARGB0888,
    #[doc = "ARGB data is 5555"]
    ARGB5555,
    #[doc = "ARGB data is 6565"]
    ARGB6565,
    #[doc = "ARGB data is 6666"]
    ARGB6666,
    #[doc = "ARGB data is 8888"]
    ARGB8888,
}
impl PIXEL0FORMATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PIXEL0FORMATW::ARGB0555 => 0,
            PIXEL0FORMATW::ARGB0565 => 1,
            PIXEL0FORMATW::ARGB0666 => 2,
            PIXEL0FORMATW::ARGB0888 => 3,
            PIXEL0FORMATW::ARGB5555 => 4,
            PIXEL0FORMATW::ARGB6565 => 5,
            PIXEL0FORMATW::ARGB6666 => 6,
            PIXEL0FORMATW::ARGB8888 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIXEL0FORMATW<'a> {
    w: &'a mut W,
}
impl<'a> _PIXEL0FORMATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIXEL0FORMATW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ARGB data is 0555"]
    #[inline]
    pub fn argb0555(self) -> &'a mut W {
        self.variant(PIXEL0FORMATW::ARGB0555)
    }
    #[doc = "ARGB data is 0565"]
    #[inline]
    pub fn argb0565(self) -> &'a mut W {
        self.variant(PIXEL0FORMATW::ARGB0565)
    }
    #[doc = "ARGB data is 0666"]
    #[inline]
    pub fn argb0666(self) -> &'a mut W {
        self.variant(PIXEL0FORMATW::ARGB0666)
    }
    #[doc = "ARGB data is 0888"]
    #[inline]
    pub fn argb0888(self) -> &'a mut W {
        self.variant(PIXEL0FORMATW::ARGB0888)
    }
    #[doc = "ARGB data is 5555"]
    #[inline]
    pub fn argb5555(self) -> &'a mut W {
        self.variant(PIXEL0FORMATW::ARGB5555)
    }
    #[doc = "ARGB data is 6565"]
    #[inline]
    pub fn argb6565(self) -> &'a mut W {
        self.variant(PIXEL0FORMATW::ARGB6565)
    }
    #[doc = "ARGB data is 6666"]
    #[inline]
    pub fn argb6666(self) -> &'a mut W {
        self.variant(PIXEL0FORMATW::ARGB6666)
    }
    #[doc = "ARGB data is 8888"]
    #[inline]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(PIXEL0FORMATW::ARGB8888)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIXEL1FORMAT`"]
pub enum PIXEL1FORMATW {
    #[doc = "RGB data is 555"]
    RGB555,
    #[doc = "RGB data is 565"]
    RGB565,
    #[doc = "RGB data is 666"]
    RGB666,
    #[doc = "RGB data is 888"]
    RGB888,
}
impl PIXEL1FORMATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PIXEL1FORMATW::RGB555 => 0,
            PIXEL1FORMATW::RGB565 => 1,
            PIXEL1FORMATW::RGB666 => 2,
            PIXEL1FORMATW::RGB888 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIXEL1FORMATW<'a> {
    w: &'a mut W,
}
impl<'a> _PIXEL1FORMATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIXEL1FORMATW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "RGB data is 555"]
    #[inline]
    pub fn rgb555(self) -> &'a mut W {
        self.variant(PIXEL1FORMATW::RGB555)
    }
    #[doc = "RGB data is 565"]
    #[inline]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(PIXEL1FORMATW::RGB565)
    }
    #[doc = "RGB data is 666"]
    #[inline]
    pub fn rgb666(self) -> &'a mut W {
        self.variant(PIXEL1FORMATW::RGB666)
    }
    #[doc = "RGB data is 888"]
    #[inline]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(PIXEL1FORMATW::RGB888)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Sprite Pixel Color Format"]
    #[inline]
    pub fn pixel0format(&self) -> PIXEL0FORMATR {
        PIXEL0FORMATR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Source and Destination Pixel Color Format"]
    #[inline]
    pub fn pixel1format(&self) -> PIXEL1FORMATR {
        PIXEL1FORMATR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:2 - Sprite Pixel Color Format"]
    #[inline]
    pub fn pixel0format(&mut self) -> _PIXEL0FORMATW {
        _PIXEL0FORMATW { w: self }
    }
    #[doc = "Bits 8:9 - Source and Destination Pixel Color Format"]
    #[inline]
    pub fn pixel1format(&mut self) -> _PIXEL1FORMATW {
        _PIXEL1FORMATW { w: self }
    }
}
