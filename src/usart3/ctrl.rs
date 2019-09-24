#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYNC`"]
pub type SYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNC`"]
pub struct SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_W<'a> {
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
#[doc = "Reader of field `LOOPBK`"]
pub type LOOPBK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOOPBK`"]
pub struct LOOPBK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBK_W<'a> {
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
#[doc = "Reader of field `CCEN`"]
pub type CCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCEN`"]
pub struct CCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCEN_W<'a> {
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
#[doc = "Reader of field `MPM`"]
pub type MPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPM`"]
pub struct MPM_W<'a> {
    w: &'a mut W,
}
impl<'a> MPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `MPAB`"]
pub type MPAB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPAB`"]
pub struct MPAB_W<'a> {
    w: &'a mut W,
}
impl<'a> MPAB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Oversampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVS_A {
    #[doc = "0: Regular UART mode with 16X oversampling in asynchronous mode"]
    X16,
    #[doc = "1: Double speed with 8X oversampling in asynchronous mode"]
    X8,
    #[doc = "2: 6X oversampling in asynchronous mode"]
    X6,
    #[doc = "3: Quadruple speed with 4X oversampling in asynchronous mode"]
    X4,
}
impl From<OVS_A> for u8 {
    #[inline(always)]
    fn from(variant: OVS_A) -> Self {
        match variant {
            OVS_A::X16 => 0,
            OVS_A::X8 => 1,
            OVS_A::X6 => 2,
            OVS_A::X4 => 3,
        }
    }
}
#[doc = "Reader of field `OVS`"]
pub type OVS_R = crate::R<u8, OVS_A>;
impl OVS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVS_A {
        match self.bits {
            0 => OVS_A::X16,
            1 => OVS_A::X8,
            2 => OVS_A::X6,
            3 => OVS_A::X4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `X16`"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == OVS_A::X16
    }
    #[doc = "Checks if the value of the field is `X8`"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == OVS_A::X8
    }
    #[doc = "Checks if the value of the field is `X6`"]
    #[inline(always)]
    pub fn is_x6(&self) -> bool {
        *self == OVS_A::X6
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == OVS_A::X4
    }
}
#[doc = "Write proxy for field `OVS`"]
pub struct OVS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Regular UART mode with 16X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut W {
        self.variant(OVS_A::X16)
    }
    #[doc = "Double speed with 8X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut W {
        self.variant(OVS_A::X8)
    }
    #[doc = "6X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x6(self) -> &'a mut W {
        self.variant(OVS_A::X6)
    }
    #[doc = "Quadruple speed with 4X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(OVS_A::X4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `CLKPOL`"]
pub type CLKPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKPOL`"]
pub struct CLKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPOL_W<'a> {
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
#[doc = "Reader of field `CLKPHA`"]
pub type CLKPHA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKPHA`"]
pub struct CLKPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPHA_W<'a> {
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
#[doc = "Reader of field `MSBF`"]
pub type MSBF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSBF`"]
pub struct MSBF_W<'a> {
    w: &'a mut W,
}
impl<'a> MSBF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CSMA`"]
pub type CSMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSMA`"]
pub struct CSMA_W<'a> {
    w: &'a mut W,
}
impl<'a> CSMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TXBIL`"]
pub type TXBIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBIL`"]
pub struct TXBIL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBIL_W<'a> {
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
#[doc = "Reader of field `RXINV`"]
pub type RXINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXINV`"]
pub struct RXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TXINV`"]
pub type TXINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXINV`"]
pub struct TXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CSINV`"]
pub type CSINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSINV`"]
pub struct CSINV_W<'a> {
    w: &'a mut W,
}
impl<'a> CSINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `AUTOCS`"]
pub type AUTOCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOCS`"]
pub struct AUTOCS_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCS_W<'a> {
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
#[doc = "Reader of field `AUTOTRI`"]
pub type AUTOTRI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOTRI`"]
pub struct AUTOTRI_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOTRI_W<'a> {
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
#[doc = "Reader of field `SCMODE`"]
pub type SCMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCMODE`"]
pub struct SCMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMODE_W<'a> {
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
#[doc = "Reader of field `SCRETRANS`"]
pub type SCRETRANS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCRETRANS`"]
pub struct SCRETRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRETRANS_W<'a> {
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
#[doc = "Reader of field `SKIPPERRF`"]
pub type SKIPPERRF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SKIPPERRF`"]
pub struct SKIPPERRF_W<'a> {
    w: &'a mut W,
}
impl<'a> SKIPPERRF_W<'a> {
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
#[doc = "Reader of field `BIT8DV`"]
pub type BIT8DV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIT8DV`"]
pub struct BIT8DV_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT8DV_W<'a> {
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
#[doc = "Reader of field `ERRSDMA`"]
pub type ERRSDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRSDMA`"]
pub struct ERRSDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRSDMA_W<'a> {
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
#[doc = "Reader of field `ERRSRX`"]
pub type ERRSRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRSRX`"]
pub struct ERRSRX_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRSRX_W<'a> {
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
#[doc = "Reader of field `ERRSTX`"]
pub type ERRSTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRSTX`"]
pub struct ERRSTX_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRSTX_W<'a> {
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
#[doc = "Reader of field `SSSEARLY`"]
pub type SSSEARLY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSSEARLY`"]
pub struct SSSEARLY_W<'a> {
    w: &'a mut W,
}
impl<'a> SSSEARLY_W<'a> {
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
#[doc = "Reader of field `BYTESWAP`"]
pub type BYTESWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYTESWAP`"]
pub struct BYTESWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTESWAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `AUTOTX`"]
pub type AUTOTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOTX`"]
pub struct AUTOTX_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOTX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `MVDIS`"]
pub type MVDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MVDIS`"]
pub struct MVDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MVDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SMSDELAY`"]
pub type SMSDELAY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMSDELAY`"]
pub struct SMSDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> SMSDELAY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USART Synchronous Mode"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&self) -> LOOPBK_R {
        LOOPBK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Collision Check Enable"]
    #[inline(always)]
    pub fn ccen(&self) -> CCEN_R {
        CCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&self) -> MPM_R {
        MPM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&self) -> MPAB_R {
        MPAB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Oversampling"]
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Clock Polarity"]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clock Edge for Setup/Sample"]
    #[inline(always)]
    pub fn clkpha(&self) -> CLKPHA_R {
        CLKPHA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Action on Slave-Select in Master Mode"]
    #[inline(always)]
    pub fn csma(&self) -> CSMA_R {
        CSMA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TX Buffer Interrupt Level"]
    #[inline(always)]
    pub fn txbil(&self) -> TXBIL_R {
        TXBIL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Receiver Input Invert"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmitter Output Invert"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Chip Select Invert"]
    #[inline(always)]
    pub fn csinv(&self) -> CSINV_R {
        CSINV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Automatic Chip Select"]
    #[inline(always)]
    pub fn autocs(&self) -> AUTOCS_R {
        AUTOCS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline(always)]
    pub fn autotri(&self) -> AUTOTRI_R {
        AUTOTRI_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SmartCard Mode"]
    #[inline(always)]
    pub fn scmode(&self) -> SCMODE_R {
        SCMODE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SmartCard Retransmit"]
    #[inline(always)]
    pub fn scretrans(&self) -> SCRETRANS_R {
        SCRETRANS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline(always)]
    pub fn skipperrf(&self) -> SKIPPERRF_R {
        SKIPPERRF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Bit 8 Default Value"]
    #[inline(always)]
    pub fn bit8dv(&self) -> BIT8DV_R {
        BIT8DV_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Halt DMA on Error"]
    #[inline(always)]
    pub fn errsdma(&self) -> ERRSDMA_R {
        ERRSDMA_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Disable RX on Error"]
    #[inline(always)]
    pub fn errsrx(&self) -> ERRSRX_R {
        ERRSRX_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Disable TX on Error"]
    #[inline(always)]
    pub fn errstx(&self) -> ERRSTX_R {
        ERRSTX_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Synchronous Slave Setup Early"]
    #[inline(always)]
    pub fn sssearly(&self) -> SSSEARLY_R {
        SSSEARLY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Byteswap in Double Accesses"]
    #[inline(always)]
    pub fn byteswap(&self) -> BYTESWAP_R {
        BYTESWAP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Always Transmit When RX Not Full"]
    #[inline(always)]
    pub fn autotx(&self) -> AUTOTX_R {
        AUTOTX_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline(always)]
    pub fn mvdis(&self) -> MVDIS_R {
        MVDIS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Synchronous Master Sample Delay"]
    #[inline(always)]
    pub fn smsdelay(&self) -> SMSDELAY_R {
        SMSDELAY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USART Synchronous Mode"]
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W {
        SYNC_W { w: self }
    }
    #[doc = "Bit 1 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&mut self) -> LOOPBK_W {
        LOOPBK_W { w: self }
    }
    #[doc = "Bit 2 - Collision Check Enable"]
    #[inline(always)]
    pub fn ccen(&mut self) -> CCEN_W {
        CCEN_W { w: self }
    }
    #[doc = "Bit 3 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&mut self) -> MPM_W {
        MPM_W { w: self }
    }
    #[doc = "Bit 4 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&mut self) -> MPAB_W {
        MPAB_W { w: self }
    }
    #[doc = "Bits 5:6 - Oversampling"]
    #[inline(always)]
    pub fn ovs(&mut self) -> OVS_W {
        OVS_W { w: self }
    }
    #[doc = "Bit 8 - Clock Polarity"]
    #[inline(always)]
    pub fn clkpol(&mut self) -> CLKPOL_W {
        CLKPOL_W { w: self }
    }
    #[doc = "Bit 9 - Clock Edge for Setup/Sample"]
    #[inline(always)]
    pub fn clkpha(&mut self) -> CLKPHA_W {
        CLKPHA_W { w: self }
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&mut self) -> MSBF_W {
        MSBF_W { w: self }
    }
    #[doc = "Bit 11 - Action on Slave-Select in Master Mode"]
    #[inline(always)]
    pub fn csma(&mut self) -> CSMA_W {
        CSMA_W { w: self }
    }
    #[doc = "Bit 12 - TX Buffer Interrupt Level"]
    #[inline(always)]
    pub fn txbil(&mut self) -> TXBIL_W {
        TXBIL_W { w: self }
    }
    #[doc = "Bit 13 - Receiver Input Invert"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W {
        RXINV_W { w: self }
    }
    #[doc = "Bit 14 - Transmitter Output Invert"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W {
        TXINV_W { w: self }
    }
    #[doc = "Bit 15 - Chip Select Invert"]
    #[inline(always)]
    pub fn csinv(&mut self) -> CSINV_W {
        CSINV_W { w: self }
    }
    #[doc = "Bit 16 - Automatic Chip Select"]
    #[inline(always)]
    pub fn autocs(&mut self) -> AUTOCS_W {
        AUTOCS_W { w: self }
    }
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline(always)]
    pub fn autotri(&mut self) -> AUTOTRI_W {
        AUTOTRI_W { w: self }
    }
    #[doc = "Bit 18 - SmartCard Mode"]
    #[inline(always)]
    pub fn scmode(&mut self) -> SCMODE_W {
        SCMODE_W { w: self }
    }
    #[doc = "Bit 19 - SmartCard Retransmit"]
    #[inline(always)]
    pub fn scretrans(&mut self) -> SCRETRANS_W {
        SCRETRANS_W { w: self }
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline(always)]
    pub fn skipperrf(&mut self) -> SKIPPERRF_W {
        SKIPPERRF_W { w: self }
    }
    #[doc = "Bit 21 - Bit 8 Default Value"]
    #[inline(always)]
    pub fn bit8dv(&mut self) -> BIT8DV_W {
        BIT8DV_W { w: self }
    }
    #[doc = "Bit 22 - Halt DMA on Error"]
    #[inline(always)]
    pub fn errsdma(&mut self) -> ERRSDMA_W {
        ERRSDMA_W { w: self }
    }
    #[doc = "Bit 23 - Disable RX on Error"]
    #[inline(always)]
    pub fn errsrx(&mut self) -> ERRSRX_W {
        ERRSRX_W { w: self }
    }
    #[doc = "Bit 24 - Disable TX on Error"]
    #[inline(always)]
    pub fn errstx(&mut self) -> ERRSTX_W {
        ERRSTX_W { w: self }
    }
    #[doc = "Bit 25 - Synchronous Slave Setup Early"]
    #[inline(always)]
    pub fn sssearly(&mut self) -> SSSEARLY_W {
        SSSEARLY_W { w: self }
    }
    #[doc = "Bit 28 - Byteswap in Double Accesses"]
    #[inline(always)]
    pub fn byteswap(&mut self) -> BYTESWAP_W {
        BYTESWAP_W { w: self }
    }
    #[doc = "Bit 29 - Always Transmit When RX Not Full"]
    #[inline(always)]
    pub fn autotx(&mut self) -> AUTOTX_W {
        AUTOTX_W { w: self }
    }
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline(always)]
    pub fn mvdis(&mut self) -> MVDIS_W {
        MVDIS_W { w: self }
    }
    #[doc = "Bit 31 - Synchronous Master Sample Delay"]
    #[inline(always)]
    pub fn smsdelay(&mut self) -> SMSDELAY_W {
        SMSDELAY_W { w: self }
    }
}
