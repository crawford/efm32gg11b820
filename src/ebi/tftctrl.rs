#[doc = "Reader of register TFTCTRL"]
pub type R = crate::R<u32, super::TFTCTRL>;
#[doc = "Writer for register TFTCTRL"]
pub type W = crate::W<u32, super::TFTCTRL>;
#[doc = "Register TFTCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TFTCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `DD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DD_A {
    #[doc = "Direct Drive is disabled."]
    DISABLED,
    #[doc = "Direct Drive from internal memory enabled and started."]
    INTERNAL,
    #[doc = "Direct Drive from external memory enabled and started."]
    EXTERNAL,
}
impl crate::ToBits<u8> for DD_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DD_A::DISABLED => 0,
            DD_A::INTERNAL => 1,
            DD_A::EXTERNAL => 2,
        }
    }
}
#[doc = "Reader of field `DD`"]
pub type DD_R = crate::R<u8, DD_A>;
impl DD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DD_A::DISABLED),
            1 => Val(DD_A::INTERNAL),
            2 => Val(DD_A::EXTERNAL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == DD_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == DD_A::EXTERNAL
    }
}
#[doc = "Write proxy for field `DD`"]
pub struct DD_W<'a> {
    w: &'a mut W,
}
impl<'a> DD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DD_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Direct Drive is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DD_A::DISABLED)
    }
    #[doc = "Direct Drive from internal memory enabled and started."]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(DD_A::INTERNAL)
    }
    #[doc = "Direct Drive from external memory enabled and started."]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(DD_A::EXTERNAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `MASKBLEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASKBLEND_A {
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
impl crate::ToBits<u8> for MASKBLEND_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MASKBLEND_A::DISABLED => 0,
            MASKBLEND_A::IMASK => 1,
            MASKBLEND_A::IALPHA => 2,
            MASKBLEND_A::IMASKALPHA => 3,
            MASKBLEND_A::EFBMASK => 4,
            MASKBLEND_A::EFBALPHA => 5,
            MASKBLEND_A::EFBMASKALPHA => 6,
            MASKBLEND_A::IFBMASK => 7,
            MASKBLEND_A::IFBALPHA => 8,
            MASKBLEND_A::IFBMASKALPHA => 9,
        }
    }
}
#[doc = "Reader of field `MASKBLEND`"]
pub type MASKBLEND_R = crate::R<u8, MASKBLEND_A>;
impl MASKBLEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MASKBLEND_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MASKBLEND_A::DISABLED),
            1 => Val(MASKBLEND_A::IMASK),
            2 => Val(MASKBLEND_A::IALPHA),
            3 => Val(MASKBLEND_A::IMASKALPHA),
            4 => Val(MASKBLEND_A::EFBMASK),
            5 => Val(MASKBLEND_A::EFBALPHA),
            6 => Val(MASKBLEND_A::EFBMASKALPHA),
            7 => Val(MASKBLEND_A::IFBMASK),
            8 => Val(MASKBLEND_A::IFBALPHA),
            9 => Val(MASKBLEND_A::IFBMASKALPHA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MASKBLEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `IMASK`"]
    #[inline(always)]
    pub fn is_imask(&self) -> bool {
        *self == MASKBLEND_A::IMASK
    }
    #[doc = "Checks if the value of the field is `IALPHA`"]
    #[inline(always)]
    pub fn is_ialpha(&self) -> bool {
        *self == MASKBLEND_A::IALPHA
    }
    #[doc = "Checks if the value of the field is `IMASKALPHA`"]
    #[inline(always)]
    pub fn is_imaskalpha(&self) -> bool {
        *self == MASKBLEND_A::IMASKALPHA
    }
    #[doc = "Checks if the value of the field is `EFBMASK`"]
    #[inline(always)]
    pub fn is_efbmask(&self) -> bool {
        *self == MASKBLEND_A::EFBMASK
    }
    #[doc = "Checks if the value of the field is `EFBALPHA`"]
    #[inline(always)]
    pub fn is_efbalpha(&self) -> bool {
        *self == MASKBLEND_A::EFBALPHA
    }
    #[doc = "Checks if the value of the field is `EFBMASKALPHA`"]
    #[inline(always)]
    pub fn is_efbmaskalpha(&self) -> bool {
        *self == MASKBLEND_A::EFBMASKALPHA
    }
    #[doc = "Checks if the value of the field is `IFBMASK`"]
    #[inline(always)]
    pub fn is_ifbmask(&self) -> bool {
        *self == MASKBLEND_A::IFBMASK
    }
    #[doc = "Checks if the value of the field is `IFBALPHA`"]
    #[inline(always)]
    pub fn is_ifbalpha(&self) -> bool {
        *self == MASKBLEND_A::IFBALPHA
    }
    #[doc = "Checks if the value of the field is `IFBMASKALPHA`"]
    #[inline(always)]
    pub fn is_ifbmaskalpha(&self) -> bool {
        *self == MASKBLEND_A::IFBMASKALPHA
    }
}
#[doc = "Write proxy for field `MASKBLEND`"]
pub struct MASKBLEND_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKBLEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASKBLEND_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Masking and Blending are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MASKBLEND_A::DISABLED)
    }
    #[doc = "Internal Masking is enabled."]
    #[inline(always)]
    pub fn imask(self) -> &'a mut W {
        self.variant(MASKBLEND_A::IMASK)
    }
    #[doc = "Internal Alpha Blending is enabled."]
    #[inline(always)]
    pub fn ialpha(self) -> &'a mut W {
        self.variant(MASKBLEND_A::IALPHA)
    }
    #[doc = "Internal Masking and Alpha Blending are enabled."]
    #[inline(always)]
    pub fn imaskalpha(self) -> &'a mut W {
        self.variant(MASKBLEND_A::IMASKALPHA)
    }
    #[doc = "External Frame Buffer Masking is enabled."]
    #[inline(always)]
    pub fn efbmask(self) -> &'a mut W {
        self.variant(MASKBLEND_A::EFBMASK)
    }
    #[doc = "External Frame Buffer Alpha Blending is enabled."]
    #[inline(always)]
    pub fn efbalpha(self) -> &'a mut W {
        self.variant(MASKBLEND_A::EFBALPHA)
    }
    #[doc = "External Frame Buffer Masking and Alpha Blending are enabled."]
    #[inline(always)]
    pub fn efbmaskalpha(self) -> &'a mut W {
        self.variant(MASKBLEND_A::EFBMASKALPHA)
    }
    #[doc = "Internal Frame Buffer Masking is enabled."]
    #[inline(always)]
    pub fn ifbmask(self) -> &'a mut W {
        self.variant(MASKBLEND_A::IFBMASK)
    }
    #[doc = "Internal Frame Buffer Alpha Blending is enabled."]
    #[inline(always)]
    pub fn ifbalpha(self) -> &'a mut W {
        self.variant(MASKBLEND_A::IFBALPHA)
    }
    #[doc = "Internal Frame Buffer Masking and Alpha Blending are enabled."]
    #[inline(always)]
    pub fn ifbmaskalpha(self) -> &'a mut W {
        self.variant(MASKBLEND_A::IFBMASKALPHA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `SHIFTDCLKEN`"]
pub type SHIFTDCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHIFTDCLKEN`"]
pub struct SHIFTDCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFTDCLKEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `FBCTRIG`"]
pub type FBCTRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBCTRIG`"]
pub struct FBCTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> FBCTRIG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Possible values of the field `INTERLEAVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERLEAVE_A {
    #[doc = "Allow unlimited interleaved EBI accesses per EBI_DCLK period. This can cause jitter on the EBI_DCLK"]
    UNLIMITED,
    #[doc = "Allow 1 interleaved EBI access per EBI_DCLK period."]
    ONEPERDCLK,
    #[doc = "Only allow EBI accesses during TFT porches."]
    PORCH,
}
impl crate::ToBits<u8> for INTERLEAVE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            INTERLEAVE_A::UNLIMITED => 0,
            INTERLEAVE_A::ONEPERDCLK => 1,
            INTERLEAVE_A::PORCH => 2,
        }
    }
}
#[doc = "Reader of field `INTERLEAVE`"]
pub type INTERLEAVE_R = crate::R<u8, INTERLEAVE_A>;
impl INTERLEAVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INTERLEAVE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INTERLEAVE_A::UNLIMITED),
            1 => Val(INTERLEAVE_A::ONEPERDCLK),
            2 => Val(INTERLEAVE_A::PORCH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNLIMITED`"]
    #[inline(always)]
    pub fn is_unlimited(&self) -> bool {
        *self == INTERLEAVE_A::UNLIMITED
    }
    #[doc = "Checks if the value of the field is `ONEPERDCLK`"]
    #[inline(always)]
    pub fn is_oneperdclk(&self) -> bool {
        *self == INTERLEAVE_A::ONEPERDCLK
    }
    #[doc = "Checks if the value of the field is `PORCH`"]
    #[inline(always)]
    pub fn is_porch(&self) -> bool {
        *self == INTERLEAVE_A::PORCH
    }
}
#[doc = "Write proxy for field `INTERLEAVE`"]
pub struct INTERLEAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERLEAVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTERLEAVE_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Allow unlimited interleaved EBI accesses per EBI_DCLK period. This can cause jitter on the EBI_DCLK"]
    #[inline(always)]
    pub fn unlimited(self) -> &'a mut W {
        self.variant(INTERLEAVE_A::UNLIMITED)
    }
    #[doc = "Allow 1 interleaved EBI access per EBI_DCLK period."]
    #[inline(always)]
    pub fn oneperdclk(self) -> &'a mut W {
        self.variant(INTERLEAVE_A::ONEPERDCLK)
    }
    #[doc = "Only allow EBI accesses during TFT porches."]
    #[inline(always)]
    pub fn porch(self) -> &'a mut W {
        self.variant(INTERLEAVE_A::PORCH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `COLOR1SRC`"]
pub type COLOR1SRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COLOR1SRC`"]
pub struct COLOR1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> COLOR1SRC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `WIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIDTH_A {
    #[doc = "TFT Data is 8 bit wide."]
    BYTE,
    #[doc = "TFT Data is 16 bit wide."]
    HALFWORD,
}
impl crate::ToBits<u8> for WIDTH_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            WIDTH_A::BYTE => 0,
            WIDTH_A::HALFWORD => 1,
        }
    }
}
#[doc = "Reader of field `WIDTH`"]
pub type WIDTH_R = crate::R<u8, WIDTH_A>;
impl WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WIDTH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WIDTH_A::BYTE),
            1 => Val(WIDTH_A::HALFWORD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == WIDTH_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == WIDTH_A::HALFWORD
    }
}
#[doc = "Write proxy for field `WIDTH`"]
pub struct WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIDTH_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "TFT Data is 8 bit wide."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(WIDTH_A::BYTE)
    }
    #[doc = "TFT Data is 16 bit wide."]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut W {
        self.variant(WIDTH_A::HALFWORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `ALIASBANKEN`"]
pub type ALIASBANKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALIASBANKEN`"]
pub struct ALIASBANKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIASBANKEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Possible values of the field `BANKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANKSEL_A {
    #[doc = "Memory bank 0 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK0,
    #[doc = "Memory bank 1 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK1,
    #[doc = "Memory bank 2 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK2,
    #[doc = "Memory bank 3 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK3,
}
impl crate::ToBits<u8> for BANKSEL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            BANKSEL_A::BANK0 => 0,
            BANKSEL_A::BANK1 => 1,
            BANKSEL_A::BANK2 => 2,
            BANKSEL_A::BANK3 => 3,
        }
    }
}
#[doc = "Reader of field `BANKSEL`"]
pub type BANKSEL_R = crate::R<u8, BANKSEL_A>;
impl BANKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BANKSEL_A {
        match self.bits {
            0 => BANKSEL_A::BANK0,
            1 => BANKSEL_A::BANK1,
            2 => BANKSEL_A::BANK2,
            3 => BANKSEL_A::BANK3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BANK0`"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == BANKSEL_A::BANK0
    }
    #[doc = "Checks if the value of the field is `BANK1`"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == BANKSEL_A::BANK1
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == BANKSEL_A::BANK2
    }
    #[doc = "Checks if the value of the field is `BANK3`"]
    #[inline(always)]
    pub fn is_bank3(&self) -> bool {
        *self == BANKSEL_A::BANK3
    }
}
#[doc = "Write proxy for field `BANKSEL`"]
pub struct BANKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANKSEL_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Memory bank 0 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn bank0(self) -> &'a mut W {
        self.variant(BANKSEL_A::BANK0)
    }
    #[doc = "Memory bank 1 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn bank1(self) -> &'a mut W {
        self.variant(BANKSEL_A::BANK1)
    }
    #[doc = "Memory bank 2 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut W {
        self.variant(BANKSEL_A::BANK2)
    }
    #[doc = "Memory bank 3 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn bank3(self) -> &'a mut W {
        self.variant(BANKSEL_A::BANK3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `ALIASBANK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIASBANK_A {
    #[doc = "Graphic Bank Select is alias to Bank Select 0"]
    ALIASBANK0,
    #[doc = "Graphic Bank Select is alias to Bank Select 1"]
    ALIASBANK1,
    #[doc = "Graphic Bank Select is alias to Bank Select 2"]
    ALIASBANK2,
    #[doc = "Graphic Bank Select is alias to Bank Select 3"]
    ALIASBANK3,
}
impl crate::ToBits<u8> for ALIASBANK_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ALIASBANK_A::ALIASBANK0 => 0,
            ALIASBANK_A::ALIASBANK1 => 1,
            ALIASBANK_A::ALIASBANK2 => 2,
            ALIASBANK_A::ALIASBANK3 => 3,
        }
    }
}
#[doc = "Reader of field `ALIASBANK`"]
pub type ALIASBANK_R = crate::R<u8, ALIASBANK_A>;
impl ALIASBANK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIASBANK_A {
        match self.bits {
            0 => ALIASBANK_A::ALIASBANK0,
            1 => ALIASBANK_A::ALIASBANK1,
            2 => ALIASBANK_A::ALIASBANK2,
            3 => ALIASBANK_A::ALIASBANK3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALIASBANK0`"]
    #[inline(always)]
    pub fn is_aliasbank0(&self) -> bool {
        *self == ALIASBANK_A::ALIASBANK0
    }
    #[doc = "Checks if the value of the field is `ALIASBANK1`"]
    #[inline(always)]
    pub fn is_aliasbank1(&self) -> bool {
        *self == ALIASBANK_A::ALIASBANK1
    }
    #[doc = "Checks if the value of the field is `ALIASBANK2`"]
    #[inline(always)]
    pub fn is_aliasbank2(&self) -> bool {
        *self == ALIASBANK_A::ALIASBANK2
    }
    #[doc = "Checks if the value of the field is `ALIASBANK3`"]
    #[inline(always)]
    pub fn is_aliasbank3(&self) -> bool {
        *self == ALIASBANK_A::ALIASBANK3
    }
}
#[doc = "Write proxy for field `ALIASBANK`"]
pub struct ALIASBANK_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIASBANK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALIASBANK_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 0"]
    #[inline(always)]
    pub fn aliasbank0(self) -> &'a mut W {
        self.variant(ALIASBANK_A::ALIASBANK0)
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 1"]
    #[inline(always)]
    pub fn aliasbank1(self) -> &'a mut W {
        self.variant(ALIASBANK_A::ALIASBANK1)
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 2"]
    #[inline(always)]
    pub fn aliasbank2(self) -> &'a mut W {
        self.variant(ALIASBANK_A::ALIASBANK2)
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 3"]
    #[inline(always)]
    pub fn aliasbank3(self) -> &'a mut W {
        self.variant(ALIASBANK_A::ALIASBANK3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - TFT Direct Drive Mode"]
    #[inline(always)]
    pub fn dd(&self) -> DD_R {
        DD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:5 - TFT Mask and Blend Mode"]
    #[inline(always)]
    pub fn maskblend(&self) -> MASKBLEND_R {
        MASKBLEND_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - TFT EBI_DCLK Shift Enable"]
    #[inline(always)]
    pub fn shiftdclken(&self) -> SHIFTDCLKEN_R {
        SHIFTDCLKEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TFT Frame Base Copy Trigger"]
    #[inline(always)]
    pub fn fbctrig(&self) -> FBCTRIG_R {
        FBCTRIG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Interleave Mode"]
    #[inline(always)]
    pub fn interleave(&self) -> INTERLEAVE_R {
        INTERLEAVE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Masking/Alpha Blending Color1 Source"]
    #[inline(always)]
    pub fn color1src(&self) -> COLOR1SRC_R {
        COLOR1SRC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - TFT Transaction Width"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Alias to Graphics Bank Enable"]
    #[inline(always)]
    pub fn aliasbanken(&self) -> ALIASBANKEN_R {
        ALIASBANKEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Graphics Bank"]
    #[inline(always)]
    pub fn banksel(&self) -> BANKSEL_R {
        BANKSEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Graphic Bank Select Aliasing"]
    #[inline(always)]
    pub fn aliasbank(&self) -> ALIASBANK_R {
        ALIASBANK_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TFT Direct Drive Mode"]
    #[inline(always)]
    pub fn dd(&mut self) -> DD_W {
        DD_W { w: self }
    }
    #[doc = "Bits 2:5 - TFT Mask and Blend Mode"]
    #[inline(always)]
    pub fn maskblend(&mut self) -> MASKBLEND_W {
        MASKBLEND_W { w: self }
    }
    #[doc = "Bit 8 - TFT EBI_DCLK Shift Enable"]
    #[inline(always)]
    pub fn shiftdclken(&mut self) -> SHIFTDCLKEN_W {
        SHIFTDCLKEN_W { w: self }
    }
    #[doc = "Bit 9 - TFT Frame Base Copy Trigger"]
    #[inline(always)]
    pub fn fbctrig(&mut self) -> FBCTRIG_W {
        FBCTRIG_W { w: self }
    }
    #[doc = "Bits 10:11 - Interleave Mode"]
    #[inline(always)]
    pub fn interleave(&mut self) -> INTERLEAVE_W {
        INTERLEAVE_W { w: self }
    }
    #[doc = "Bit 12 - Masking/Alpha Blending Color1 Source"]
    #[inline(always)]
    pub fn color1src(&mut self) -> COLOR1SRC_W {
        COLOR1SRC_W { w: self }
    }
    #[doc = "Bits 16:17 - TFT Transaction Width"]
    #[inline(always)]
    pub fn width(&mut self) -> WIDTH_W {
        WIDTH_W { w: self }
    }
    #[doc = "Bit 19 - Alias to Graphics Bank Enable"]
    #[inline(always)]
    pub fn aliasbanken(&mut self) -> ALIASBANKEN_W {
        ALIASBANKEN_W { w: self }
    }
    #[doc = "Bits 20:21 - Graphics Bank"]
    #[inline(always)]
    pub fn banksel(&mut self) -> BANKSEL_W {
        BANKSEL_W { w: self }
    }
    #[doc = "Bits 22:23 - Graphic Bank Select Aliasing"]
    #[inline(always)]
    pub fn aliasbank(&mut self) -> ALIASBANK_W {
        ALIASBANK_W { w: self }
    }
}
