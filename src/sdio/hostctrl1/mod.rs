#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HOSTCTRL1 {
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
#[doc = r" Value of the field"]
pub struct LEDCTRLR {
    bits: bool,
}
impl LEDCTRLR {
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
#[doc = r" Value of the field"]
pub struct DATTRANWDR {
    bits: bool,
}
impl DATTRANWDR {
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
#[doc = r" Value of the field"]
pub struct HSENR {
    bits: bool,
}
impl HSENR {
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
#[doc = "Possible values of the field `DMASEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMASELR {
    #[doc = "SDMA selected"]
    SDMA,
    #[doc = "32-bit ADMA1 selected"]
    ADMA1,
    #[doc = "32-bit ADMA2 selected"]
    ADMA2,
    #[doc = "64-bit ADMA2 selected"]
    _64BITADMA2,
}
impl DMASELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMASELR::SDMA => 0,
            DMASELR::ADMA1 => 1,
            DMASELR::ADMA2 => 2,
            DMASELR::_64BITADMA2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMASELR {
        match value {
            0 => DMASELR::SDMA,
            1 => DMASELR::ADMA1,
            2 => DMASELR::ADMA2,
            3 => DMASELR::_64BITADMA2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SDMA`"]
    #[inline]
    pub fn is_sdma(&self) -> bool {
        *self == DMASELR::SDMA
    }
    #[doc = "Checks if the value of the field is `ADMA1`"]
    #[inline]
    pub fn is_adma1(&self) -> bool {
        *self == DMASELR::ADMA1
    }
    #[doc = "Checks if the value of the field is `ADMA2`"]
    #[inline]
    pub fn is_adma2(&self) -> bool {
        *self == DMASELR::ADMA2
    }
    #[doc = "Checks if the value of the field is `_64BITADMA2`"]
    #[inline]
    pub fn is_64bitadma2(&self) -> bool {
        *self == DMASELR::_64BITADMA2
    }
}
#[doc = r" Value of the field"]
pub struct EXTDATTRANWDR {
    bits: bool,
}
impl EXTDATTRANWDR {
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
#[doc = r" Value of the field"]
pub struct CDTSTLVLR {
    bits: bool,
}
impl CDTSTLVLR {
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
#[doc = r" Value of the field"]
pub struct CDSIGDETR {
    bits: bool,
}
impl CDSIGDETR {
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
#[doc = r" Value of the field"]
pub struct SDBUSPOWERR {
    bits: bool,
}
impl SDBUSPOWERR {
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
#[doc = "Possible values of the field `SDBUSVOLTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDBUSVOLTSELR {
    #[doc = "Select 1.8V"]
    _1P8V,
    #[doc = "Select 3.0V"]
    _3P0V,
    #[doc = "Select 3.3V"]
    _3P3V,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SDBUSVOLTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SDBUSVOLTSELR::_1P8V => 5,
            SDBUSVOLTSELR::_3P0V => 6,
            SDBUSVOLTSELR::_3P3V => 7,
            SDBUSVOLTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SDBUSVOLTSELR {
        match value {
            5 => SDBUSVOLTSELR::_1P8V,
            6 => SDBUSVOLTSELR::_3P0V,
            7 => SDBUSVOLTSELR::_3P3V,
            i => SDBUSVOLTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1P8V`"]
    #[inline]
    pub fn is_1p8v(&self) -> bool {
        *self == SDBUSVOLTSELR::_1P8V
    }
    #[doc = "Checks if the value of the field is `_3P0V`"]
    #[inline]
    pub fn is_3p0v(&self) -> bool {
        *self == SDBUSVOLTSELR::_3P0V
    }
    #[doc = "Checks if the value of the field is `_3P3V`"]
    #[inline]
    pub fn is_3p3v(&self) -> bool {
        *self == SDBUSVOLTSELR::_3P3V
    }
}
#[doc = r" Value of the field"]
pub struct HRDRSTR {
    bits: bool,
}
impl HRDRSTR {
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
#[doc = r" Value of the field"]
pub struct STOPATBLKGAPREQR {
    bits: bool,
}
impl STOPATBLKGAPREQR {
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
#[doc = r" Value of the field"]
pub struct CONTINUEREQR {
    bits: bool,
}
impl CONTINUEREQR {
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
#[doc = r" Value of the field"]
pub struct RDWAITCTRLR {
    bits: bool,
}
impl RDWAITCTRLR {
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
#[doc = r" Value of the field"]
pub struct INTATBLKGAPR {
    bits: bool,
}
impl INTATBLKGAPR {
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
#[doc = r" Value of the field"]
pub struct SPIMODER {
    bits: bool,
}
impl SPIMODER {
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
#[doc = r" Value of the field"]
pub struct BOOTENR {
    bits: bool,
}
impl BOOTENR {
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
#[doc = r" Value of the field"]
pub struct ALTBOOTENR {
    bits: bool,
}
impl ALTBOOTENR {
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
#[doc = r" Value of the field"]
pub struct BOOTACKCHKR {
    bits: bool,
}
impl BOOTACKCHKR {
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
#[doc = r" Value of the field"]
pub struct WKUPEVNTENONCARDINTR {
    bits: bool,
}
impl WKUPEVNTENONCARDINTR {
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
#[doc = r" Value of the field"]
pub struct WKUPEVNTENONCINSR {
    bits: bool,
}
impl WKUPEVNTENONCINSR {
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
#[doc = r" Value of the field"]
pub struct WKUPEVNTENONCRMR {
    bits: bool,
}
impl WKUPEVNTENONCRMR {
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
#[doc = r" Proxy"]
pub struct _LEDCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _LEDCTRLW<'a> {
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
#[doc = r" Proxy"]
pub struct _DATTRANWDW<'a> {
    w: &'a mut W,
}
impl<'a> _DATTRANWDW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HSENW<'a> {
    w: &'a mut W,
}
impl<'a> _HSENW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMASEL`"]
pub enum DMASELW {
    #[doc = "SDMA selected"]
    SDMA,
    #[doc = "32-bit ADMA1 selected"]
    ADMA1,
    #[doc = "32-bit ADMA2 selected"]
    ADMA2,
    #[doc = "64-bit ADMA2 selected"]
    _64BITADMA2,
}
impl DMASELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMASELW::SDMA => 0,
            DMASELW::ADMA1 => 1,
            DMASELW::ADMA2 => 2,
            DMASELW::_64BITADMA2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMASELW<'a> {
    w: &'a mut W,
}
impl<'a> _DMASELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMASELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SDMA selected"]
    #[inline]
    pub fn sdma(self) -> &'a mut W {
        self.variant(DMASELW::SDMA)
    }
    #[doc = "32-bit ADMA1 selected"]
    #[inline]
    pub fn adma1(self) -> &'a mut W {
        self.variant(DMASELW::ADMA1)
    }
    #[doc = "32-bit ADMA2 selected"]
    #[inline]
    pub fn adma2(self) -> &'a mut W {
        self.variant(DMASELW::ADMA2)
    }
    #[doc = "64-bit ADMA2 selected"]
    #[inline]
    pub fn _64bitadma2(self) -> &'a mut W {
        self.variant(DMASELW::_64BITADMA2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXTDATTRANWDW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTDATTRANWDW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CDTSTLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _CDTSTLVLW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CDSIGDETW<'a> {
    w: &'a mut W,
}
impl<'a> _CDSIGDETW<'a> {
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
#[doc = r" Proxy"]
pub struct _SDBUSPOWERW<'a> {
    w: &'a mut W,
}
impl<'a> _SDBUSPOWERW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDBUSVOLTSEL`"]
pub enum SDBUSVOLTSELW {
    #[doc = "Select 1.8V"]
    _1P8V,
    #[doc = "Select 3.0V"]
    _3P0V,
    #[doc = "Select 3.3V"]
    _3P3V,
}
impl SDBUSVOLTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SDBUSVOLTSELW::_1P8V => 5,
            SDBUSVOLTSELW::_3P0V => 6,
            SDBUSVOLTSELW::_3P3V => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDBUSVOLTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SDBUSVOLTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDBUSVOLTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select 1.8V"]
    #[inline]
    pub fn _1p8v(self) -> &'a mut W {
        self.variant(SDBUSVOLTSELW::_1P8V)
    }
    #[doc = "Select 3.0V"]
    #[inline]
    pub fn _3p0v(self) -> &'a mut W {
        self.variant(SDBUSVOLTSELW::_3P0V)
    }
    #[doc = "Select 3.3V"]
    #[inline]
    pub fn _3p3v(self) -> &'a mut W {
        self.variant(SDBUSVOLTSELW::_3P3V)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HRDRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HRDRSTW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STOPATBLKGAPREQW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPATBLKGAPREQW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CONTINUEREQW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTINUEREQW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RDWAITCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _RDWAITCTRLW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTATBLKGAPW<'a> {
    w: &'a mut W,
}
impl<'a> _INTATBLKGAPW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPIMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIMODEW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BOOTENW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOTENW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ALTBOOTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ALTBOOTENW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BOOTACKCHKW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOTACKCHKW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEVNTENONCARDINTW<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEVNTENONCARDINTW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEVNTENONCINSW<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEVNTENONCINSW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEVNTENONCRMW<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEVNTENONCRMW<'a> {
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
        const OFFSET: u8 = 26;
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
    #[doc = "Bit 0 - LED Control"]
    #[inline]
    pub fn ledctrl(&self) -> LEDCTRLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LEDCTRLR { bits }
    }
    #[doc = "Bit 1 - Data Transfer Width 1-bit or 4-bit Mode"]
    #[inline]
    pub fn dattranwd(&self) -> DATTRANWDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATTRANWDR { bits }
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline]
    pub fn hsen(&self) -> HSENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSENR { bits }
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline]
    pub fn dmasel(&self) -> DMASELR {
        DMASELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Extended Data Transfer Width"]
    #[inline]
    pub fn extdattranwd(&self) -> EXTDATTRANWDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EXTDATTRANWDR { bits }
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline]
    pub fn cdtstlvl(&self) -> CDTSTLVLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CDTSTLVLR { bits }
    }
    #[doc = "Bit 7 - Card Detetct Signal Detection"]
    #[inline]
    pub fn cdsigdet(&self) -> CDSIGDETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CDSIGDETR { bits }
    }
    #[doc = "Bit 8 - SD Bus Power"]
    #[inline]
    pub fn sdbuspower(&self) -> SDBUSPOWERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDBUSPOWERR { bits }
    }
    #[doc = "Bits 9:11 - SD Bus Voltage Select"]
    #[inline]
    pub fn sdbusvoltsel(&self) -> SDBUSVOLTSELR {
        SDBUSVOLTSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Hardware Reset Signal"]
    #[inline]
    pub fn hrdrst(&self) -> HRDRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HRDRSTR { bits }
    }
    #[doc = "Bit 16 - Stop at Block Gap Request"]
    #[inline]
    pub fn stopatblkgapreq(&self) -> STOPATBLKGAPREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STOPATBLKGAPREQR { bits }
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline]
    pub fn continuereq(&self) -> CONTINUEREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CONTINUEREQR { bits }
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline]
    pub fn rdwaitctrl(&self) -> RDWAITCTRLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RDWAITCTRLR { bits }
    }
    #[doc = "Bit 19 - Interrupt at Block Gap"]
    #[inline]
    pub fn intatblkgap(&self) -> INTATBLKGAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTATBLKGAPR { bits }
    }
    #[doc = "Bit 20 - SPI Mode Enable"]
    #[inline]
    pub fn spimode(&self) -> SPIMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPIMODER { bits }
    }
    #[doc = "Bit 21 - Boot Enable"]
    #[inline]
    pub fn booten(&self) -> BOOTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BOOTENR { bits }
    }
    #[doc = "Bit 22 - Alternate Boot Enable"]
    #[inline]
    pub fn altbooten(&self) -> ALTBOOTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALTBOOTENR { bits }
    }
    #[doc = "Bit 23 - Boot Ack Check"]
    #[inline]
    pub fn bootackchk(&self) -> BOOTACKCHKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BOOTACKCHKR { bits }
    }
    #[doc = "Bit 24 - Wakeup Event Enable on Card Interrupt"]
    #[inline]
    pub fn wkupevntenoncardint(&self) -> WKUPEVNTENONCARDINTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WKUPEVNTENONCARDINTR { bits }
    }
    #[doc = "Bit 25 - Wakeup Event Enable on SD Card Insertion"]
    #[inline]
    pub fn wkupevntenoncins(&self) -> WKUPEVNTENONCINSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WKUPEVNTENONCINSR { bits }
    }
    #[doc = "Bit 26 - Wakeup Event Enable on SD Card Removal"]
    #[inline]
    pub fn wkupevntenoncrm(&self) -> WKUPEVNTENONCRMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WKUPEVNTENONCRMR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8388608 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - LED Control"]
    #[inline]
    pub fn ledctrl(&mut self) -> _LEDCTRLW {
        _LEDCTRLW { w: self }
    }
    #[doc = "Bit 1 - Data Transfer Width 1-bit or 4-bit Mode"]
    #[inline]
    pub fn dattranwd(&mut self) -> _DATTRANWDW {
        _DATTRANWDW { w: self }
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline]
    pub fn hsen(&mut self) -> _HSENW {
        _HSENW { w: self }
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline]
    pub fn dmasel(&mut self) -> _DMASELW {
        _DMASELW { w: self }
    }
    #[doc = "Bit 5 - Extended Data Transfer Width"]
    #[inline]
    pub fn extdattranwd(&mut self) -> _EXTDATTRANWDW {
        _EXTDATTRANWDW { w: self }
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline]
    pub fn cdtstlvl(&mut self) -> _CDTSTLVLW {
        _CDTSTLVLW { w: self }
    }
    #[doc = "Bit 7 - Card Detetct Signal Detection"]
    #[inline]
    pub fn cdsigdet(&mut self) -> _CDSIGDETW {
        _CDSIGDETW { w: self }
    }
    #[doc = "Bit 8 - SD Bus Power"]
    #[inline]
    pub fn sdbuspower(&mut self) -> _SDBUSPOWERW {
        _SDBUSPOWERW { w: self }
    }
    #[doc = "Bits 9:11 - SD Bus Voltage Select"]
    #[inline]
    pub fn sdbusvoltsel(&mut self) -> _SDBUSVOLTSELW {
        _SDBUSVOLTSELW { w: self }
    }
    #[doc = "Bit 12 - Hardware Reset Signal"]
    #[inline]
    pub fn hrdrst(&mut self) -> _HRDRSTW {
        _HRDRSTW { w: self }
    }
    #[doc = "Bit 16 - Stop at Block Gap Request"]
    #[inline]
    pub fn stopatblkgapreq(&mut self) -> _STOPATBLKGAPREQW {
        _STOPATBLKGAPREQW { w: self }
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline]
    pub fn continuereq(&mut self) -> _CONTINUEREQW {
        _CONTINUEREQW { w: self }
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline]
    pub fn rdwaitctrl(&mut self) -> _RDWAITCTRLW {
        _RDWAITCTRLW { w: self }
    }
    #[doc = "Bit 19 - Interrupt at Block Gap"]
    #[inline]
    pub fn intatblkgap(&mut self) -> _INTATBLKGAPW {
        _INTATBLKGAPW { w: self }
    }
    #[doc = "Bit 20 - SPI Mode Enable"]
    #[inline]
    pub fn spimode(&mut self) -> _SPIMODEW {
        _SPIMODEW { w: self }
    }
    #[doc = "Bit 21 - Boot Enable"]
    #[inline]
    pub fn booten(&mut self) -> _BOOTENW {
        _BOOTENW { w: self }
    }
    #[doc = "Bit 22 - Alternate Boot Enable"]
    #[inline]
    pub fn altbooten(&mut self) -> _ALTBOOTENW {
        _ALTBOOTENW { w: self }
    }
    #[doc = "Bit 23 - Boot Ack Check"]
    #[inline]
    pub fn bootackchk(&mut self) -> _BOOTACKCHKW {
        _BOOTACKCHKW { w: self }
    }
    #[doc = "Bit 24 - Wakeup Event Enable on Card Interrupt"]
    #[inline]
    pub fn wkupevntenoncardint(&mut self) -> _WKUPEVNTENONCARDINTW {
        _WKUPEVNTENONCARDINTW { w: self }
    }
    #[doc = "Bit 25 - Wakeup Event Enable on SD Card Insertion"]
    #[inline]
    pub fn wkupevntenoncins(&mut self) -> _WKUPEVNTENONCINSW {
        _WKUPEVNTENONCINSW { w: self }
    }
    #[doc = "Bit 26 - Wakeup Event Enable on SD Card Removal"]
    #[inline]
    pub fn wkupevntenoncrm(&mut self) -> _WKUPEVNTENONCRMW {
        _WKUPEVNTENONCRMW { w: self }
    }
}
