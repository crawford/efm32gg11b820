#[doc = "Reader of register HOSTCTRL1"]
pub type R = crate::R<u32, super::HOSTCTRL1>;
#[doc = "Writer for register HOSTCTRL1"]
pub type W = crate::W<u32, super::HOSTCTRL1>;
#[doc = "Register HOSTCTRL1 `reset()`'s with value 0x0080_0000"]
impl crate::ResetValue for super::HOSTCTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0080_0000
    }
}
#[doc = "Reader of field `LEDCTRL`"]
pub type LEDCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDCTRL`"]
pub struct LEDCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDCTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `DATTRANWD`"]
pub type DATTRANWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATTRANWD`"]
pub struct DATTRANWD_W<'a> {
    w: &'a mut W,
}
impl<'a> DATTRANWD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `HSEN`"]
pub type HSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSEN`"]
pub struct HSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMASEL_A {
    #[doc = "0: SDMA selected"]
    SDMA,
    #[doc = "1: 32-bit ADMA1 selected"]
    ADMA1,
    #[doc = "2: 32-bit ADMA2 selected"]
    ADMA2,
    #[doc = "3: 64-bit ADMA2 selected"]
    _64BITADMA2,
}
impl From<DMASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMASEL_A) -> Self {
        match variant {
            DMASEL_A::SDMA => 0,
            DMASEL_A::ADMA1 => 1,
            DMASEL_A::ADMA2 => 2,
            DMASEL_A::_64BITADMA2 => 3,
        }
    }
}
#[doc = "Reader of field `DMASEL`"]
pub type DMASEL_R = crate::R<u8, DMASEL_A>;
impl DMASEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMASEL_A {
        match self.bits {
            0 => DMASEL_A::SDMA,
            1 => DMASEL_A::ADMA1,
            2 => DMASEL_A::ADMA2,
            3 => DMASEL_A::_64BITADMA2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SDMA`"]
    #[inline(always)]
    pub fn is_sdma(&self) -> bool {
        *self == DMASEL_A::SDMA
    }
    #[doc = "Checks if the value of the field is `ADMA1`"]
    #[inline(always)]
    pub fn is_adma1(&self) -> bool {
        *self == DMASEL_A::ADMA1
    }
    #[doc = "Checks if the value of the field is `ADMA2`"]
    #[inline(always)]
    pub fn is_adma2(&self) -> bool {
        *self == DMASEL_A::ADMA2
    }
    #[doc = "Checks if the value of the field is `_64BITADMA2`"]
    #[inline(always)]
    pub fn is_64bitadma2(&self) -> bool {
        *self == DMASEL_A::_64BITADMA2
    }
}
#[doc = "Write proxy for field `DMASEL`"]
pub struct DMASEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMASEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SDMA selected"]
    #[inline(always)]
    pub fn sdma(self) -> &'a mut W {
        self.variant(DMASEL_A::SDMA)
    }
    #[doc = "32-bit ADMA1 selected"]
    #[inline(always)]
    pub fn adma1(self) -> &'a mut W {
        self.variant(DMASEL_A::ADMA1)
    }
    #[doc = "32-bit ADMA2 selected"]
    #[inline(always)]
    pub fn adma2(self) -> &'a mut W {
        self.variant(DMASEL_A::ADMA2)
    }
    #[doc = "64-bit ADMA2 selected"]
    #[inline(always)]
    pub fn _64bitadma2(self) -> &'a mut W {
        self.variant(DMASEL_A::_64BITADMA2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `EXTDATTRANWD`"]
pub type EXTDATTRANWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTDATTRANWD`"]
pub struct EXTDATTRANWD_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTDATTRANWD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CDTSTLVL`"]
pub type CDTSTLVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDTSTLVL`"]
pub struct CDTSTLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> CDTSTLVL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CDSIGDET`"]
pub type CDSIGDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDSIGDET`"]
pub struct CDSIGDET_W<'a> {
    w: &'a mut W,
}
impl<'a> CDSIGDET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SDBUSPOWER`"]
pub type SDBUSPOWER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDBUSPOWER`"]
pub struct SDBUSPOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> SDBUSPOWER_W<'a> {
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
#[doc = "SD Bus Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDBUSVOLTSEL_A {
    #[doc = "5: Select 1.8V"]
    _1P8V,
    #[doc = "6: Select 3.0V"]
    _3P0V,
    #[doc = "7: Select 3.3V"]
    _3P3V,
}
impl From<SDBUSVOLTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDBUSVOLTSEL_A) -> Self {
        match variant {
            SDBUSVOLTSEL_A::_1P8V => 5,
            SDBUSVOLTSEL_A::_3P0V => 6,
            SDBUSVOLTSEL_A::_3P3V => 7,
        }
    }
}
#[doc = "Reader of field `SDBUSVOLTSEL`"]
pub type SDBUSVOLTSEL_R = crate::R<u8, SDBUSVOLTSEL_A>;
impl SDBUSVOLTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SDBUSVOLTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            5 => Val(SDBUSVOLTSEL_A::_1P8V),
            6 => Val(SDBUSVOLTSEL_A::_3P0V),
            7 => Val(SDBUSVOLTSEL_A::_3P3V),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1P8V`"]
    #[inline(always)]
    pub fn is_1p8v(&self) -> bool {
        *self == SDBUSVOLTSEL_A::_1P8V
    }
    #[doc = "Checks if the value of the field is `_3P0V`"]
    #[inline(always)]
    pub fn is_3p0v(&self) -> bool {
        *self == SDBUSVOLTSEL_A::_3P0V
    }
    #[doc = "Checks if the value of the field is `_3P3V`"]
    #[inline(always)]
    pub fn is_3p3v(&self) -> bool {
        *self == SDBUSVOLTSEL_A::_3P3V
    }
}
#[doc = "Write proxy for field `SDBUSVOLTSEL`"]
pub struct SDBUSVOLTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDBUSVOLTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDBUSVOLTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select 1.8V"]
    #[inline(always)]
    pub fn _1p8v(self) -> &'a mut W {
        self.variant(SDBUSVOLTSEL_A::_1P8V)
    }
    #[doc = "Select 3.0V"]
    #[inline(always)]
    pub fn _3p0v(self) -> &'a mut W {
        self.variant(SDBUSVOLTSEL_A::_3P0V)
    }
    #[doc = "Select 3.3V"]
    #[inline(always)]
    pub fn _3p3v(self) -> &'a mut W {
        self.variant(SDBUSVOLTSEL_A::_3P3V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `HRDRST`"]
pub type HRDRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HRDRST`"]
pub struct HRDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> HRDRST_W<'a> {
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
#[doc = "Reader of field `STOPATBLKGAPREQ`"]
pub type STOPATBLKGAPREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOPATBLKGAPREQ`"]
pub struct STOPATBLKGAPREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPATBLKGAPREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `CONTINUEREQ`"]
pub type CONTINUEREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONTINUEREQ`"]
pub struct CONTINUEREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTINUEREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `RDWAITCTRL`"]
pub type RDWAITCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDWAITCTRL`"]
pub struct RDWAITCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWAITCTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `INTATBLKGAP`"]
pub type INTATBLKGAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTATBLKGAP`"]
pub struct INTATBLKGAP_W<'a> {
    w: &'a mut W,
}
impl<'a> INTATBLKGAP_W<'a> {
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
#[doc = "Reader of field `SPIMODE`"]
pub type SPIMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIMODE`"]
pub struct SPIMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `BOOTEN`"]
pub type BOOTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOTEN`"]
pub struct BOOTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ALTBOOTEN`"]
pub type ALTBOOTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALTBOOTEN`"]
pub struct ALTBOOTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTBOOTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `BOOTACKCHK`"]
pub type BOOTACKCHK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOTACKCHK`"]
pub struct BOOTACKCHK_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTACKCHK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `WKUPEVNTENONCARDINT`"]
pub type WKUPEVNTENONCARDINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPEVNTENONCARDINT`"]
pub struct WKUPEVNTENONCARDINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEVNTENONCARDINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `WKUPEVNTENONCINS`"]
pub type WKUPEVNTENONCINS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPEVNTENONCINS`"]
pub struct WKUPEVNTENONCINS_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEVNTENONCINS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `WKUPEVNTENONCRM`"]
pub type WKUPEVNTENONCRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPEVNTENONCRM`"]
pub struct WKUPEVNTENONCRM_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPEVNTENONCRM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn ledctrl(&self) -> LEDCTRL_R {
        LEDCTRL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Transfer Width 1-bit or 4-bit Mode"]
    #[inline(always)]
    pub fn dattranwd(&self) -> DATTRANWD_R {
        DATTRANWD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    pub fn hsen(&self) -> HSEN_R {
        HSEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline(always)]
    pub fn dmasel(&self) -> DMASEL_R {
        DMASEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width"]
    #[inline(always)]
    pub fn extdattranwd(&self) -> EXTDATTRANWD_R {
        EXTDATTRANWD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn cdtstlvl(&self) -> CDTSTLVL_R {
        CDTSTLVL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Detetct Signal Detection"]
    #[inline(always)]
    pub fn cdsigdet(&self) -> CDSIGDET_R {
        CDSIGDET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SD Bus Power"]
    #[inline(always)]
    pub fn sdbuspower(&self) -> SDBUSPOWER_R {
        SDBUSPOWER_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - SD Bus Voltage Select"]
    #[inline(always)]
    pub fn sdbusvoltsel(&self) -> SDBUSVOLTSEL_R {
        SDBUSVOLTSEL_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Hardware Reset Signal"]
    #[inline(always)]
    pub fn hrdrst(&self) -> HRDRST_R {
        HRDRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Stop at Block Gap Request"]
    #[inline(always)]
    pub fn stopatblkgapreq(&self) -> STOPATBLKGAPREQ_R {
        STOPATBLKGAPREQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline(always)]
    pub fn continuereq(&self) -> CONTINUEREQ_R {
        CONTINUEREQ_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline(always)]
    pub fn rdwaitctrl(&self) -> RDWAITCTRL_R {
        RDWAITCTRL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Interrupt at Block Gap"]
    #[inline(always)]
    pub fn intatblkgap(&self) -> INTATBLKGAP_R {
        INTATBLKGAP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SPI Mode Enable"]
    #[inline(always)]
    pub fn spimode(&self) -> SPIMODE_R {
        SPIMODE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Boot Enable"]
    #[inline(always)]
    pub fn booten(&self) -> BOOTEN_R {
        BOOTEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Alternate Boot Enable"]
    #[inline(always)]
    pub fn altbooten(&self) -> ALTBOOTEN_R {
        ALTBOOTEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Boot Ack Check"]
    #[inline(always)]
    pub fn bootackchk(&self) -> BOOTACKCHK_R {
        BOOTACKCHK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Wakeup Event Enable on Card Interrupt"]
    #[inline(always)]
    pub fn wkupevntenoncardint(&self) -> WKUPEVNTENONCARDINT_R {
        WKUPEVNTENONCARDINT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Wakeup Event Enable on SD Card Insertion"]
    #[inline(always)]
    pub fn wkupevntenoncins(&self) -> WKUPEVNTENONCINS_R {
        WKUPEVNTENONCINS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Wakeup Event Enable on SD Card Removal"]
    #[inline(always)]
    pub fn wkupevntenoncrm(&self) -> WKUPEVNTENONCRM_R {
        WKUPEVNTENONCRM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED Control"]
    #[inline(always)]
    pub fn ledctrl(&mut self) -> LEDCTRL_W {
        LEDCTRL_W { w: self }
    }
    #[doc = "Bit 1 - Data Transfer Width 1-bit or 4-bit Mode"]
    #[inline(always)]
    pub fn dattranwd(&mut self) -> DATTRANWD_W {
        DATTRANWD_W { w: self }
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline(always)]
    pub fn hsen(&mut self) -> HSEN_W {
        HSEN_W { w: self }
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline(always)]
    pub fn dmasel(&mut self) -> DMASEL_W {
        DMASEL_W { w: self }
    }
    #[doc = "Bit 5 - Extended Data Transfer Width"]
    #[inline(always)]
    pub fn extdattranwd(&mut self) -> EXTDATTRANWD_W {
        EXTDATTRANWD_W { w: self }
    }
    #[doc = "Bit 6 - Card Detect Test Level"]
    #[inline(always)]
    pub fn cdtstlvl(&mut self) -> CDTSTLVL_W {
        CDTSTLVL_W { w: self }
    }
    #[doc = "Bit 7 - Card Detetct Signal Detection"]
    #[inline(always)]
    pub fn cdsigdet(&mut self) -> CDSIGDET_W {
        CDSIGDET_W { w: self }
    }
    #[doc = "Bit 8 - SD Bus Power"]
    #[inline(always)]
    pub fn sdbuspower(&mut self) -> SDBUSPOWER_W {
        SDBUSPOWER_W { w: self }
    }
    #[doc = "Bits 9:11 - SD Bus Voltage Select"]
    #[inline(always)]
    pub fn sdbusvoltsel(&mut self) -> SDBUSVOLTSEL_W {
        SDBUSVOLTSEL_W { w: self }
    }
    #[doc = "Bit 12 - Hardware Reset Signal"]
    #[inline(always)]
    pub fn hrdrst(&mut self) -> HRDRST_W {
        HRDRST_W { w: self }
    }
    #[doc = "Bit 16 - Stop at Block Gap Request"]
    #[inline(always)]
    pub fn stopatblkgapreq(&mut self) -> STOPATBLKGAPREQ_W {
        STOPATBLKGAPREQ_W { w: self }
    }
    #[doc = "Bit 17 - Continue Request"]
    #[inline(always)]
    pub fn continuereq(&mut self) -> CONTINUEREQ_W {
        CONTINUEREQ_W { w: self }
    }
    #[doc = "Bit 18 - Read Wait Control"]
    #[inline(always)]
    pub fn rdwaitctrl(&mut self) -> RDWAITCTRL_W {
        RDWAITCTRL_W { w: self }
    }
    #[doc = "Bit 19 - Interrupt at Block Gap"]
    #[inline(always)]
    pub fn intatblkgap(&mut self) -> INTATBLKGAP_W {
        INTATBLKGAP_W { w: self }
    }
    #[doc = "Bit 20 - SPI Mode Enable"]
    #[inline(always)]
    pub fn spimode(&mut self) -> SPIMODE_W {
        SPIMODE_W { w: self }
    }
    #[doc = "Bit 21 - Boot Enable"]
    #[inline(always)]
    pub fn booten(&mut self) -> BOOTEN_W {
        BOOTEN_W { w: self }
    }
    #[doc = "Bit 22 - Alternate Boot Enable"]
    #[inline(always)]
    pub fn altbooten(&mut self) -> ALTBOOTEN_W {
        ALTBOOTEN_W { w: self }
    }
    #[doc = "Bit 23 - Boot Ack Check"]
    #[inline(always)]
    pub fn bootackchk(&mut self) -> BOOTACKCHK_W {
        BOOTACKCHK_W { w: self }
    }
    #[doc = "Bit 24 - Wakeup Event Enable on Card Interrupt"]
    #[inline(always)]
    pub fn wkupevntenoncardint(&mut self) -> WKUPEVNTENONCARDINT_W {
        WKUPEVNTENONCARDINT_W { w: self }
    }
    #[doc = "Bit 25 - Wakeup Event Enable on SD Card Insertion"]
    #[inline(always)]
    pub fn wkupevntenoncins(&mut self) -> WKUPEVNTENONCINS_W {
        WKUPEVNTENONCINS_W { w: self }
    }
    #[doc = "Bit 26 - Wakeup Event Enable on SD Card Removal"]
    #[inline(always)]
    pub fn wkupevntenoncrm(&mut self) -> WKUPEVNTENONCRM_W {
        WKUPEVNTENONCRM_W { w: self }
    }
}
