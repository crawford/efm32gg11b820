#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TFTCTRL {
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
#[doc = "Possible values of the field `DD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDR {
    #[doc = "Direct Drive is disabled."]
    DISABLED,
    #[doc = "Direct Drive from internal memory enabled and started."]
    INTERNAL,
    #[doc = "Direct Drive from external memory enabled and started."]
    EXTERNAL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DDR::DISABLED => 0,
            DDR::INTERNAL => 1,
            DDR::EXTERNAL => 2,
            DDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DDR {
        match value {
            0 => DDR::DISABLED,
            1 => DDR::INTERNAL,
            2 => DDR::EXTERNAL,
            i => DDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline]
    pub fn is_internal(&self) -> bool {
        *self == DDR::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline]
    pub fn is_external(&self) -> bool {
        *self == DDR::EXTERNAL
    }
}
#[doc = "Possible values of the field `MASKBLEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASKBLENDR {
    #[doc = "Masking and Blending are disabled."]
    DISABLED,
    #[doc = "Internal Masking is enabled."]
    IMASK,
    #[doc = "Internal Alpha Blending is enabled."]
    IALPHA,
    #[doc = "Internal Masking and Alpha Blending are enabled."]
    IMASKALPHA,
    #[doc = "External Frame Buffer Masking is enabled."]
    EFBMASK,
    #[doc = "External Frame Buffer Alpha Blending is enabled."]
    EFBALPHA,
    #[doc = "External Frame Buffer Masking and Alpha Blending are enabled."]
    EFBMASKALPHA,
    #[doc = "Internal Frame Buffer Masking is enabled."]
    IFBMASK,
    #[doc = "Internal Frame Buffer Alpha Blending is enabled."]
    IFBALPHA,
    #[doc = "Internal Frame Buffer Masking and Alpha Blending are enabled."]
    IFBMASKALPHA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MASKBLENDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MASKBLENDR::DISABLED => 0,
            MASKBLENDR::IMASK => 1,
            MASKBLENDR::IALPHA => 2,
            MASKBLENDR::IMASKALPHA => 3,
            MASKBLENDR::EFBMASK => 4,
            MASKBLENDR::EFBALPHA => 5,
            MASKBLENDR::EFBMASKALPHA => 6,
            MASKBLENDR::IFBMASK => 7,
            MASKBLENDR::IFBALPHA => 8,
            MASKBLENDR::IFBMASKALPHA => 9,
            MASKBLENDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MASKBLENDR {
        match value {
            0 => MASKBLENDR::DISABLED,
            1 => MASKBLENDR::IMASK,
            2 => MASKBLENDR::IALPHA,
            3 => MASKBLENDR::IMASKALPHA,
            4 => MASKBLENDR::EFBMASK,
            5 => MASKBLENDR::EFBALPHA,
            6 => MASKBLENDR::EFBMASKALPHA,
            7 => MASKBLENDR::IFBMASK,
            8 => MASKBLENDR::IFBALPHA,
            9 => MASKBLENDR::IFBMASKALPHA,
            i => MASKBLENDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MASKBLENDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `IMASK`"]
    #[inline]
    pub fn is_imask(&self) -> bool {
        *self == MASKBLENDR::IMASK
    }
    #[doc = "Checks if the value of the field is `IALPHA`"]
    #[inline]
    pub fn is_ialpha(&self) -> bool {
        *self == MASKBLENDR::IALPHA
    }
    #[doc = "Checks if the value of the field is `IMASKALPHA`"]
    #[inline]
    pub fn is_imaskalpha(&self) -> bool {
        *self == MASKBLENDR::IMASKALPHA
    }
    #[doc = "Checks if the value of the field is `EFBMASK`"]
    #[inline]
    pub fn is_efbmask(&self) -> bool {
        *self == MASKBLENDR::EFBMASK
    }
    #[doc = "Checks if the value of the field is `EFBALPHA`"]
    #[inline]
    pub fn is_efbalpha(&self) -> bool {
        *self == MASKBLENDR::EFBALPHA
    }
    #[doc = "Checks if the value of the field is `EFBMASKALPHA`"]
    #[inline]
    pub fn is_efbmaskalpha(&self) -> bool {
        *self == MASKBLENDR::EFBMASKALPHA
    }
    #[doc = "Checks if the value of the field is `IFBMASK`"]
    #[inline]
    pub fn is_ifbmask(&self) -> bool {
        *self == MASKBLENDR::IFBMASK
    }
    #[doc = "Checks if the value of the field is `IFBALPHA`"]
    #[inline]
    pub fn is_ifbalpha(&self) -> bool {
        *self == MASKBLENDR::IFBALPHA
    }
    #[doc = "Checks if the value of the field is `IFBMASKALPHA`"]
    #[inline]
    pub fn is_ifbmaskalpha(&self) -> bool {
        *self == MASKBLENDR::IFBMASKALPHA
    }
}
#[doc = r" Value of the field"]
pub struct SHIFTDCLKENR {
    bits: bool,
}
impl SHIFTDCLKENR {
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
pub struct FBCTRIGR {
    bits: bool,
}
impl FBCTRIGR {
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
#[doc = "Possible values of the field `INTERLEAVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERLEAVER {
    #[doc = "Allow unlimited interleaved EBI accesses per EBI_DCLK period. This can cause jitter on the EBI_DCLK"]
    UNLIMITED,
    #[doc = "Allow 1 interleaved EBI access per EBI_DCLK period."]
    ONEPERDCLK,
    #[doc = "Only allow EBI accesses during TFT porches."]
    PORCH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INTERLEAVER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INTERLEAVER::UNLIMITED => 0,
            INTERLEAVER::ONEPERDCLK => 1,
            INTERLEAVER::PORCH => 2,
            INTERLEAVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INTERLEAVER {
        match value {
            0 => INTERLEAVER::UNLIMITED,
            1 => INTERLEAVER::ONEPERDCLK,
            2 => INTERLEAVER::PORCH,
            i => INTERLEAVER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNLIMITED`"]
    #[inline]
    pub fn is_unlimited(&self) -> bool {
        *self == INTERLEAVER::UNLIMITED
    }
    #[doc = "Checks if the value of the field is `ONEPERDCLK`"]
    #[inline]
    pub fn is_oneperdclk(&self) -> bool {
        *self == INTERLEAVER::ONEPERDCLK
    }
    #[doc = "Checks if the value of the field is `PORCH`"]
    #[inline]
    pub fn is_porch(&self) -> bool {
        *self == INTERLEAVER::PORCH
    }
}
#[doc = r" Value of the field"]
pub struct COLOR1SRCR {
    bits: bool,
}
impl COLOR1SRCR {
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
#[doc = "Possible values of the field `WIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIDTHR {
    #[doc = "TFT Data is 8 bit wide."]
    BYTE,
    #[doc = "TFT Data is 16 bit wide."]
    HALFWORD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WIDTHR::BYTE => 0,
            WIDTHR::HALFWORD => 1,
            WIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WIDTHR {
        match value {
            0 => WIDTHR::BYTE,
            1 => WIDTHR::HALFWORD,
            i => WIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline]
    pub fn is_byte(&self) -> bool {
        *self == WIDTHR::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline]
    pub fn is_halfword(&self) -> bool {
        *self == WIDTHR::HALFWORD
    }
}
#[doc = r" Value of the field"]
pub struct ALIASBANKENR {
    bits: bool,
}
impl ALIASBANKENR {
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
#[doc = "Possible values of the field `BANKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANKSELR {
    #[doc = "Memory bank 0 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK0,
    #[doc = "Memory bank 1 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK1,
    #[doc = "Memory bank 2 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK2,
    #[doc = "Memory bank 3 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK3,
}
impl BANKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BANKSELR::BANK0 => 0,
            BANKSELR::BANK1 => 1,
            BANKSELR::BANK2 => 2,
            BANKSELR::BANK3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BANKSELR {
        match value {
            0 => BANKSELR::BANK0,
            1 => BANKSELR::BANK1,
            2 => BANKSELR::BANK2,
            3 => BANKSELR::BANK3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BANK0`"]
    #[inline]
    pub fn is_bank0(&self) -> bool {
        *self == BANKSELR::BANK0
    }
    #[doc = "Checks if the value of the field is `BANK1`"]
    #[inline]
    pub fn is_bank1(&self) -> bool {
        *self == BANKSELR::BANK1
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline]
    pub fn is_bank2(&self) -> bool {
        *self == BANKSELR::BANK2
    }
    #[doc = "Checks if the value of the field is `BANK3`"]
    #[inline]
    pub fn is_bank3(&self) -> bool {
        *self == BANKSELR::BANK3
    }
}
#[doc = "Possible values of the field `ALIASBANK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIASBANKR {
    #[doc = "Graphic Bank Select is alias to Bank Select 0"]
    ALIASBANK0,
    #[doc = "Graphic Bank Select is alias to Bank Select 1"]
    ALIASBANK1,
    #[doc = "Graphic Bank Select is alias to Bank Select 2"]
    ALIASBANK2,
    #[doc = "Graphic Bank Select is alias to Bank Select 3"]
    ALIASBANK3,
}
impl ALIASBANKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALIASBANKR::ALIASBANK0 => 0,
            ALIASBANKR::ALIASBANK1 => 1,
            ALIASBANKR::ALIASBANK2 => 2,
            ALIASBANKR::ALIASBANK3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALIASBANKR {
        match value {
            0 => ALIASBANKR::ALIASBANK0,
            1 => ALIASBANKR::ALIASBANK1,
            2 => ALIASBANKR::ALIASBANK2,
            3 => ALIASBANKR::ALIASBANK3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALIASBANK0`"]
    #[inline]
    pub fn is_aliasbank0(&self) -> bool {
        *self == ALIASBANKR::ALIASBANK0
    }
    #[doc = "Checks if the value of the field is `ALIASBANK1`"]
    #[inline]
    pub fn is_aliasbank1(&self) -> bool {
        *self == ALIASBANKR::ALIASBANK1
    }
    #[doc = "Checks if the value of the field is `ALIASBANK2`"]
    #[inline]
    pub fn is_aliasbank2(&self) -> bool {
        *self == ALIASBANKR::ALIASBANK2
    }
    #[doc = "Checks if the value of the field is `ALIASBANK3`"]
    #[inline]
    pub fn is_aliasbank3(&self) -> bool {
        *self == ALIASBANKR::ALIASBANK3
    }
}
#[doc = "Values that can be written to the field `DD`"]
pub enum DDW {
    #[doc = "Direct Drive is disabled."]
    DISABLED,
    #[doc = "Direct Drive from internal memory enabled and started."]
    INTERNAL,
    #[doc = "Direct Drive from external memory enabled and started."]
    EXTERNAL,
}
impl DDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DDW::DISABLED => 0,
            DDW::INTERNAL => 1,
            DDW::EXTERNAL => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DDW<'a> {
    w: &'a mut W,
}
impl<'a> _DDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Direct Drive is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DDW::DISABLED)
    }
    #[doc = "Direct Drive from internal memory enabled and started."]
    #[inline]
    pub fn internal(self) -> &'a mut W {
        self.variant(DDW::INTERNAL)
    }
    #[doc = "Direct Drive from external memory enabled and started."]
    #[inline]
    pub fn external(self) -> &'a mut W {
        self.variant(DDW::EXTERNAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MASKBLEND`"]
pub enum MASKBLENDW {
    #[doc = "Masking and Blending are disabled."]
    DISABLED,
    #[doc = "Internal Masking is enabled."]
    IMASK,
    #[doc = "Internal Alpha Blending is enabled."]
    IALPHA,
    #[doc = "Internal Masking and Alpha Blending are enabled."]
    IMASKALPHA,
    #[doc = "External Frame Buffer Masking is enabled."]
    EFBMASK,
    #[doc = "External Frame Buffer Alpha Blending is enabled."]
    EFBALPHA,
    #[doc = "External Frame Buffer Masking and Alpha Blending are enabled."]
    EFBMASKALPHA,
    #[doc = "Internal Frame Buffer Masking is enabled."]
    IFBMASK,
    #[doc = "Internal Frame Buffer Alpha Blending is enabled."]
    IFBALPHA,
    #[doc = "Internal Frame Buffer Masking and Alpha Blending are enabled."]
    IFBMASKALPHA,
}
impl MASKBLENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MASKBLENDW::DISABLED => 0,
            MASKBLENDW::IMASK => 1,
            MASKBLENDW::IALPHA => 2,
            MASKBLENDW::IMASKALPHA => 3,
            MASKBLENDW::EFBMASK => 4,
            MASKBLENDW::EFBALPHA => 5,
            MASKBLENDW::EFBMASKALPHA => 6,
            MASKBLENDW::IFBMASK => 7,
            MASKBLENDW::IFBALPHA => 8,
            MASKBLENDW::IFBMASKALPHA => 9,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASKBLENDW<'a> {
    w: &'a mut W,
}
impl<'a> _MASKBLENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASKBLENDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Masking and Blending are disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MASKBLENDW::DISABLED)
    }
    #[doc = "Internal Masking is enabled."]
    #[inline]
    pub fn imask(self) -> &'a mut W {
        self.variant(MASKBLENDW::IMASK)
    }
    #[doc = "Internal Alpha Blending is enabled."]
    #[inline]
    pub fn ialpha(self) -> &'a mut W {
        self.variant(MASKBLENDW::IALPHA)
    }
    #[doc = "Internal Masking and Alpha Blending are enabled."]
    #[inline]
    pub fn imaskalpha(self) -> &'a mut W {
        self.variant(MASKBLENDW::IMASKALPHA)
    }
    #[doc = "External Frame Buffer Masking is enabled."]
    #[inline]
    pub fn efbmask(self) -> &'a mut W {
        self.variant(MASKBLENDW::EFBMASK)
    }
    #[doc = "External Frame Buffer Alpha Blending is enabled."]
    #[inline]
    pub fn efbalpha(self) -> &'a mut W {
        self.variant(MASKBLENDW::EFBALPHA)
    }
    #[doc = "External Frame Buffer Masking and Alpha Blending are enabled."]
    #[inline]
    pub fn efbmaskalpha(self) -> &'a mut W {
        self.variant(MASKBLENDW::EFBMASKALPHA)
    }
    #[doc = "Internal Frame Buffer Masking is enabled."]
    #[inline]
    pub fn ifbmask(self) -> &'a mut W {
        self.variant(MASKBLENDW::IFBMASK)
    }
    #[doc = "Internal Frame Buffer Alpha Blending is enabled."]
    #[inline]
    pub fn ifbalpha(self) -> &'a mut W {
        self.variant(MASKBLENDW::IFBALPHA)
    }
    #[doc = "Internal Frame Buffer Masking and Alpha Blending are enabled."]
    #[inline]
    pub fn ifbmaskalpha(self) -> &'a mut W {
        self.variant(MASKBLENDW::IFBMASKALPHA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SHIFTDCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _SHIFTDCLKENW<'a> {
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
#[doc = r" Proxy"]
pub struct _FBCTRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _FBCTRIGW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INTERLEAVE`"]
pub enum INTERLEAVEW {
    #[doc = "Allow unlimited interleaved EBI accesses per EBI_DCLK period. This can cause jitter on the EBI_DCLK"]
    UNLIMITED,
    #[doc = "Allow 1 interleaved EBI access per EBI_DCLK period."]
    ONEPERDCLK,
    #[doc = "Only allow EBI accesses during TFT porches."]
    PORCH,
}
impl INTERLEAVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INTERLEAVEW::UNLIMITED => 0,
            INTERLEAVEW::ONEPERDCLK => 1,
            INTERLEAVEW::PORCH => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTERLEAVEW<'a> {
    w: &'a mut W,
}
impl<'a> _INTERLEAVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTERLEAVEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Allow unlimited interleaved EBI accesses per EBI_DCLK period. This can cause jitter on the EBI_DCLK"]
    #[inline]
    pub fn unlimited(self) -> &'a mut W {
        self.variant(INTERLEAVEW::UNLIMITED)
    }
    #[doc = "Allow 1 interleaved EBI access per EBI_DCLK period."]
    #[inline]
    pub fn oneperdclk(self) -> &'a mut W {
        self.variant(INTERLEAVEW::ONEPERDCLK)
    }
    #[doc = "Only allow EBI accesses during TFT porches."]
    #[inline]
    pub fn porch(self) -> &'a mut W {
        self.variant(INTERLEAVEW::PORCH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COLOR1SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _COLOR1SRCW<'a> {
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
#[doc = "Values that can be written to the field `WIDTH`"]
pub enum WIDTHW {
    #[doc = "TFT Data is 8 bit wide."]
    BYTE,
    #[doc = "TFT Data is 16 bit wide."]
    HALFWORD,
}
impl WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WIDTHW::BYTE => 0,
            WIDTHW::HALFWORD => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "TFT Data is 8 bit wide."]
    #[inline]
    pub fn byte(self) -> &'a mut W {
        self.variant(WIDTHW::BYTE)
    }
    #[doc = "TFT Data is 16 bit wide."]
    #[inline]
    pub fn halfword(self) -> &'a mut W {
        self.variant(WIDTHW::HALFWORD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ALIASBANKENW<'a> {
    w: &'a mut W,
}
impl<'a> _ALIASBANKENW<'a> {
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
#[doc = "Values that can be written to the field `BANKSEL`"]
pub enum BANKSELW {
    #[doc = "Memory bank 0 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK0,
    #[doc = "Memory bank 1 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK1,
    #[doc = "Memory bank 2 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK2,
    #[doc = "Memory bank 3 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK3,
}
impl BANKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BANKSELW::BANK0 => 0,
            BANKSELW::BANK1 => 1,
            BANKSELW::BANK2 => 2,
            BANKSELW::BANK3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BANKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _BANKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BANKSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Memory bank 0 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline]
    pub fn bank0(self) -> &'a mut W {
        self.variant(BANKSELW::BANK0)
    }
    #[doc = "Memory bank 1 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline]
    pub fn bank1(self) -> &'a mut W {
        self.variant(BANKSELW::BANK1)
    }
    #[doc = "Memory bank 2 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline]
    pub fn bank2(self) -> &'a mut W {
        self.variant(BANKSELW::BANK2)
    }
    #[doc = "Memory bank 3 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline]
    pub fn bank3(self) -> &'a mut W {
        self.variant(BANKSELW::BANK3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALIASBANK`"]
pub enum ALIASBANKW {
    #[doc = "Graphic Bank Select is alias to Bank Select 0"]
    ALIASBANK0,
    #[doc = "Graphic Bank Select is alias to Bank Select 1"]
    ALIASBANK1,
    #[doc = "Graphic Bank Select is alias to Bank Select 2"]
    ALIASBANK2,
    #[doc = "Graphic Bank Select is alias to Bank Select 3"]
    ALIASBANK3,
}
impl ALIASBANKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ALIASBANKW::ALIASBANK0 => 0,
            ALIASBANKW::ALIASBANK1 => 1,
            ALIASBANKW::ALIASBANK2 => 2,
            ALIASBANKW::ALIASBANK3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALIASBANKW<'a> {
    w: &'a mut W,
}
impl<'a> _ALIASBANKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALIASBANKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 0"]
    #[inline]
    pub fn aliasbank0(self) -> &'a mut W {
        self.variant(ALIASBANKW::ALIASBANK0)
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 1"]
    #[inline]
    pub fn aliasbank1(self) -> &'a mut W {
        self.variant(ALIASBANKW::ALIASBANK1)
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 2"]
    #[inline]
    pub fn aliasbank2(self) -> &'a mut W {
        self.variant(ALIASBANKW::ALIASBANK2)
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 3"]
    #[inline]
    pub fn aliasbank3(self) -> &'a mut W {
        self.variant(ALIASBANKW::ALIASBANK3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
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
    #[doc = "Bits 0:1 - TFT Direct Drive Mode"]
    #[inline]
    pub fn dd(&self) -> DDR {
        DDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:5 - TFT Mask and Blend Mode"]
    #[inline]
    pub fn maskblend(&self) -> MASKBLENDR {
        MASKBLENDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - TFT EBI_DCLK Shift Enable"]
    #[inline]
    pub fn shiftdclken(&self) -> SHIFTDCLKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SHIFTDCLKENR { bits }
    }
    #[doc = "Bit 9 - TFT Frame Base Copy Trigger"]
    #[inline]
    pub fn fbctrig(&self) -> FBCTRIGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FBCTRIGR { bits }
    }
    #[doc = "Bits 10:11 - Interleave Mode"]
    #[inline]
    pub fn interleave(&self) -> INTERLEAVER {
        INTERLEAVER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Masking/Alpha Blending Color1 Source"]
    #[inline]
    pub fn color1src(&self) -> COLOR1SRCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COLOR1SRCR { bits }
    }
    #[doc = "Bits 16:17 - TFT Transaction Width"]
    #[inline]
    pub fn width(&self) -> WIDTHR {
        WIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - Alias to Graphics Bank Enable"]
    #[inline]
    pub fn aliasbanken(&self) -> ALIASBANKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALIASBANKENR { bits }
    }
    #[doc = "Bits 20:21 - Graphics Bank"]
    #[inline]
    pub fn banksel(&self) -> BANKSELR {
        BANKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Graphic Bank Select Aliasing"]
    #[inline]
    pub fn aliasbank(&self) -> ALIASBANKR {
        ALIASBANKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
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
    #[doc = "Bits 0:1 - TFT Direct Drive Mode"]
    #[inline]
    pub fn dd(&mut self) -> _DDW {
        _DDW { w: self }
    }
    #[doc = "Bits 2:5 - TFT Mask and Blend Mode"]
    #[inline]
    pub fn maskblend(&mut self) -> _MASKBLENDW {
        _MASKBLENDW { w: self }
    }
    #[doc = "Bit 8 - TFT EBI_DCLK Shift Enable"]
    #[inline]
    pub fn shiftdclken(&mut self) -> _SHIFTDCLKENW {
        _SHIFTDCLKENW { w: self }
    }
    #[doc = "Bit 9 - TFT Frame Base Copy Trigger"]
    #[inline]
    pub fn fbctrig(&mut self) -> _FBCTRIGW {
        _FBCTRIGW { w: self }
    }
    #[doc = "Bits 10:11 - Interleave Mode"]
    #[inline]
    pub fn interleave(&mut self) -> _INTERLEAVEW {
        _INTERLEAVEW { w: self }
    }
    #[doc = "Bit 12 - Masking/Alpha Blending Color1 Source"]
    #[inline]
    pub fn color1src(&mut self) -> _COLOR1SRCW {
        _COLOR1SRCW { w: self }
    }
    #[doc = "Bits 16:17 - TFT Transaction Width"]
    #[inline]
    pub fn width(&mut self) -> _WIDTHW {
        _WIDTHW { w: self }
    }
    #[doc = "Bit 19 - Alias to Graphics Bank Enable"]
    #[inline]
    pub fn aliasbanken(&mut self) -> _ALIASBANKENW {
        _ALIASBANKENW { w: self }
    }
    #[doc = "Bits 20:21 - Graphics Bank"]
    #[inline]
    pub fn banksel(&mut self) -> _BANKSELW {
        _BANKSELW { w: self }
    }
    #[doc = "Bits 22:23 - Graphic Bank Select Aliasing"]
    #[inline]
    pub fn aliasbank(&mut self) -> _ALIASBANKW {
        _ALIASBANKW { w: self }
    }
}
