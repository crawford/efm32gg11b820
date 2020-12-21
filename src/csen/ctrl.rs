#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x0003_0000"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0003_0000
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Reader of field `CMPPOL`"]
pub type CMPPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPPOL`"]
pub struct CMPPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPPOL_W<'a> {
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
#[doc = "CSEN Conversion Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: Single Channel Mode: One conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1) per conversion trigger."]
    SGL = 0,
    #[doc = "1: Scan Mode: Scans multiple selected channels once per conversion trigger."]
    SCAN = 1,
    #[doc = "2: Continuous Single Channel: Continuous conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1)."]
    CONTSGL = 2,
    #[doc = "3: Continuous Scan Mode: Continuously scans multiple selected channels."]
    CONTSCAN = 3,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CM`"]
pub type CM_R = crate::R<u8, CM_A>;
impl CM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM_A {
        match self.bits {
            0 => CM_A::SGL,
            1 => CM_A::SCAN,
            2 => CM_A::CONTSGL,
            3 => CM_A::CONTSCAN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SGL`"]
    #[inline(always)]
    pub fn is_sgl(&self) -> bool {
        *self == CM_A::SGL
    }
    #[doc = "Checks if the value of the field is `SCAN`"]
    #[inline(always)]
    pub fn is_scan(&self) -> bool {
        *self == CM_A::SCAN
    }
    #[doc = "Checks if the value of the field is `CONTSGL`"]
    #[inline(always)]
    pub fn is_contsgl(&self) -> bool {
        *self == CM_A::CONTSGL
    }
    #[doc = "Checks if the value of the field is `CONTSCAN`"]
    #[inline(always)]
    pub fn is_contscan(&self) -> bool {
        *self == CM_A::CONTSCAN
    }
}
#[doc = "Write proxy for field `CM`"]
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single Channel Mode: One conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1) per conversion trigger."]
    #[inline(always)]
    pub fn sgl(self) -> &'a mut W {
        self.variant(CM_A::SGL)
    }
    #[doc = "Scan Mode: Scans multiple selected channels once per conversion trigger."]
    #[inline(always)]
    pub fn scan(self) -> &'a mut W {
        self.variant(CM_A::SCAN)
    }
    #[doc = "Continuous Single Channel: Continuous conversion of a single channel (when MCE = 0) or set of bonded channels (when MCE = 1)."]
    #[inline(always)]
    pub fn contsgl(self) -> &'a mut W {
        self.variant(CM_A::CONTSGL)
    }
    #[doc = "Continuous Scan Mode: Continuously scans multiple selected channels."]
    #[inline(always)]
    pub fn contscan(self) -> &'a mut W {
        self.variant(CM_A::CONTSCAN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "SAR Conversion Resolution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SARCR_A {
    #[doc = "0: Conversions last 10 internal CSEN clocks and are 10-bits in length."]
    CLK10 = 0,
    #[doc = "1: Conversions last 12 internal CSEN clocks and are 12-bits in length."]
    CLK12 = 1,
    #[doc = "2: Conversions last 14 internal CSEN clocks and are 14-bits in length."]
    CLK14 = 2,
    #[doc = "3: Conversions last 16 internal CSEN clocks and are 16-bits in length."]
    CLK16 = 3,
}
impl From<SARCR_A> for u8 {
    #[inline(always)]
    fn from(variant: SARCR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SARCR`"]
pub type SARCR_R = crate::R<u8, SARCR_A>;
impl SARCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SARCR_A {
        match self.bits {
            0 => SARCR_A::CLK10,
            1 => SARCR_A::CLK12,
            2 => SARCR_A::CLK14,
            3 => SARCR_A::CLK16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLK10`"]
    #[inline(always)]
    pub fn is_clk10(&self) -> bool {
        *self == SARCR_A::CLK10
    }
    #[doc = "Checks if the value of the field is `CLK12`"]
    #[inline(always)]
    pub fn is_clk12(&self) -> bool {
        *self == SARCR_A::CLK12
    }
    #[doc = "Checks if the value of the field is `CLK14`"]
    #[inline(always)]
    pub fn is_clk14(&self) -> bool {
        *self == SARCR_A::CLK14
    }
    #[doc = "Checks if the value of the field is `CLK16`"]
    #[inline(always)]
    pub fn is_clk16(&self) -> bool {
        *self == SARCR_A::CLK16
    }
}
#[doc = "Write proxy for field `SARCR`"]
pub struct SARCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SARCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SARCR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Conversions last 10 internal CSEN clocks and are 10-bits in length."]
    #[inline(always)]
    pub fn clk10(self) -> &'a mut W {
        self.variant(SARCR_A::CLK10)
    }
    #[doc = "Conversions last 12 internal CSEN clocks and are 12-bits in length."]
    #[inline(always)]
    pub fn clk12(self) -> &'a mut W {
        self.variant(SARCR_A::CLK12)
    }
    #[doc = "Conversions last 14 internal CSEN clocks and are 14-bits in length."]
    #[inline(always)]
    pub fn clk14(self) -> &'a mut W {
        self.variant(SARCR_A::CLK14)
    }
    #[doc = "Conversions last 16 internal CSEN clocks and are 16-bits in length."]
    #[inline(always)]
    pub fn clk16(self) -> &'a mut W {
        self.variant(SARCR_A::CLK16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "CSEN Accumulator Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACU_A {
    #[doc = "0: Accumulate 1 sample."]
    ACC1 = 0,
    #[doc = "1: Accumulate 2 sample."]
    ACC2 = 1,
    #[doc = "2: Accumulate 4 sample."]
    ACC4 = 2,
    #[doc = "3: Accumulate 8 sample."]
    ACC8 = 3,
    #[doc = "4: Accumulate 16 sample."]
    ACC16 = 4,
    #[doc = "5: Accumulate 32 sample."]
    ACC32 = 5,
    #[doc = "6: Accumulate 64 sample."]
    ACC64 = 6,
}
impl From<ACU_A> for u8 {
    #[inline(always)]
    fn from(variant: ACU_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ACU`"]
pub type ACU_R = crate::R<u8, ACU_A>;
impl ACU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ACU_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ACU_A::ACC1),
            1 => Val(ACU_A::ACC2),
            2 => Val(ACU_A::ACC4),
            3 => Val(ACU_A::ACC8),
            4 => Val(ACU_A::ACC16),
            5 => Val(ACU_A::ACC32),
            6 => Val(ACU_A::ACC64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ACC1`"]
    #[inline(always)]
    pub fn is_acc1(&self) -> bool {
        *self == ACU_A::ACC1
    }
    #[doc = "Checks if the value of the field is `ACC2`"]
    #[inline(always)]
    pub fn is_acc2(&self) -> bool {
        *self == ACU_A::ACC2
    }
    #[doc = "Checks if the value of the field is `ACC4`"]
    #[inline(always)]
    pub fn is_acc4(&self) -> bool {
        *self == ACU_A::ACC4
    }
    #[doc = "Checks if the value of the field is `ACC8`"]
    #[inline(always)]
    pub fn is_acc8(&self) -> bool {
        *self == ACU_A::ACC8
    }
    #[doc = "Checks if the value of the field is `ACC16`"]
    #[inline(always)]
    pub fn is_acc16(&self) -> bool {
        *self == ACU_A::ACC16
    }
    #[doc = "Checks if the value of the field is `ACC32`"]
    #[inline(always)]
    pub fn is_acc32(&self) -> bool {
        *self == ACU_A::ACC32
    }
    #[doc = "Checks if the value of the field is `ACC64`"]
    #[inline(always)]
    pub fn is_acc64(&self) -> bool {
        *self == ACU_A::ACC64
    }
}
#[doc = "Write proxy for field `ACU`"]
pub struct ACU_W<'a> {
    w: &'a mut W,
}
impl<'a> ACU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACU_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accumulate 1 sample."]
    #[inline(always)]
    pub fn acc1(self) -> &'a mut W {
        self.variant(ACU_A::ACC1)
    }
    #[doc = "Accumulate 2 sample."]
    #[inline(always)]
    pub fn acc2(self) -> &'a mut W {
        self.variant(ACU_A::ACC2)
    }
    #[doc = "Accumulate 4 sample."]
    #[inline(always)]
    pub fn acc4(self) -> &'a mut W {
        self.variant(ACU_A::ACC4)
    }
    #[doc = "Accumulate 8 sample."]
    #[inline(always)]
    pub fn acc8(self) -> &'a mut W {
        self.variant(ACU_A::ACC8)
    }
    #[doc = "Accumulate 16 sample."]
    #[inline(always)]
    pub fn acc16(self) -> &'a mut W {
        self.variant(ACU_A::ACC16)
    }
    #[doc = "Accumulate 32 sample."]
    #[inline(always)]
    pub fn acc32(self) -> &'a mut W {
        self.variant(ACU_A::ACC32)
    }
    #[doc = "Accumulate 64 sample."]
    #[inline(always)]
    pub fn acc64(self) -> &'a mut W {
        self.variant(ACU_A::ACC64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `MCEN`"]
pub type MCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCEN`"]
pub struct MCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEN_W<'a> {
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
#[doc = "Start Trigger Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STM_A {
    #[doc = "0: PRS Triggering. Conversions are triggered by the PRS channel selected in PRSSEL."]
    PRS = 0,
    #[doc = "1: Timer Triggering. Conversions are triggered by a local CSEN timer reload."]
    TIMER = 1,
    #[doc = "2: Software Triggering. Conversions are triggered by writing a 1 to the START field of the CMD register."]
    START = 2,
}
impl From<STM_A> for u8 {
    #[inline(always)]
    fn from(variant: STM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STM`"]
pub type STM_R = crate::R<u8, STM_A>;
impl STM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STM_A::PRS),
            1 => Val(STM_A::TIMER),
            2 => Val(STM_A::START),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == STM_A::PRS
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == STM_A::TIMER
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STM_A::START
    }
}
#[doc = "Write proxy for field `STM`"]
pub struct STM_W<'a> {
    w: &'a mut W,
}
impl<'a> STM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Triggering. Conversions are triggered by the PRS channel selected in PRSSEL."]
    #[inline(always)]
    pub fn prs(self) -> &'a mut W {
        self.variant(STM_A::PRS)
    }
    #[doc = "Timer Triggering. Conversions are triggered by a local CSEN timer reload."]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(STM_A::TIMER)
    }
    #[doc = "Software Triggering. Conversions are triggered by writing a 1 to the START field of the CMD register."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(STM_A::START)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CMPEN`"]
pub type CMPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPEN`"]
pub struct CMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPEN_W<'a> {
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
#[doc = "Reader of field `DRSF`"]
pub type DRSF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRSF`"]
pub struct DRSF_W<'a> {
    w: &'a mut W,
}
impl<'a> DRSF_W<'a> {
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
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
#[doc = "Reader of field `CONVSEL`"]
pub type CONVSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONVSEL`"]
pub struct CONVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CONVSEL_W<'a> {
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
#[doc = "Reader of field `CHOPEN`"]
pub type CHOPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHOPEN`"]
pub struct CHOPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOPEN_W<'a> {
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
#[doc = "Reader of field `AUTOGND`"]
pub type AUTOGND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOGND`"]
pub struct AUTOGND_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOGND_W<'a> {
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
#[doc = "Reader of field `MXUC`"]
pub type MXUC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MXUC`"]
pub struct MXUC_W<'a> {
    w: &'a mut W,
}
impl<'a> MXUC_W<'a> {
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
#[doc = "Reader of field `EMACMPEN`"]
pub type EMACMPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMACMPEN`"]
pub struct EMACMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EMACMPEN_W<'a> {
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
#[doc = "Reader of field `WARMUPMODE`"]
pub type WARMUPMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WARMUPMODE`"]
pub struct WARMUPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WARMUPMODE_W<'a> {
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
#[doc = "Reader of field `LOCALSENS`"]
pub type LOCALSENS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCALSENS`"]
pub struct LOCALSENS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCALSENS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `CPACCURACY`"]
pub type CPACCURACY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPACCURACY`"]
pub struct CPACCURACY_W<'a> {
    w: &'a mut W,
}
impl<'a> CPACCURACY_W<'a> {
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
impl R {
    #[doc = "Bit 1 - CSEN Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CSEN Digital Comparator Polarity Select"]
    #[inline(always)]
    pub fn cmppol(&self) -> CMPPOL_R {
        CMPPOL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - CSEN Conversion Mode Select"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - SAR Conversion Resolution."]
    #[inline(always)]
    pub fn sarcr(&self) -> SARCR_R {
        SARCR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:14 - CSEN Accumulator Mode Select"]
    #[inline(always)]
    pub fn acu(&self) -> ACU_R {
        ACU_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - CSEN Multiple Channel Enable"]
    #[inline(always)]
    pub fn mcen(&self) -> MCEN_R {
        MCEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Start Trigger Select"]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - CSEN Digital Comparator Enable"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CSEN Disable Right-Shift"]
    #[inline(always)]
    pub fn drsf(&self) -> DRSF_R {
        DRSF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CSEN DMA Enable Bit"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CSEN Converter Select"]
    #[inline(always)]
    pub fn convsel(&self) -> CONVSEL_R {
        CONVSEL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CSEN Chop Enable"]
    #[inline(always)]
    pub fn chopen(&self) -> CHOPEN_R {
        CHOPEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CSEN Automatic Ground Enable"]
    #[inline(always)]
    pub fn autognd(&self) -> AUTOGND_R {
        AUTOGND_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CSEN Mux Disconnect"]
    #[inline(always)]
    pub fn mxuc(&self) -> MXUC_R {
        MXUC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Greater and Less Than Comparison Using the Exponential Moving Average (EMA) is Enabled"]
    #[inline(always)]
    pub fn emacmpen(&self) -> EMACMPEN_R {
        EMACMPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Select Warmup Mode for CSEN"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WARMUPMODE_R {
        WARMUPMODE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Local Sensing Enable"]
    #[inline(always)]
    pub fn localsens(&self) -> LOCALSENS_R {
        LOCALSENS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Charge Pump Accuracy"]
    #[inline(always)]
    pub fn cpaccuracy(&self) -> CPACCURACY_R {
        CPACCURACY_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CSEN Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 2 - CSEN Digital Comparator Polarity Select"]
    #[inline(always)]
    pub fn cmppol(&mut self) -> CMPPOL_W {
        CMPPOL_W { w: self }
    }
    #[doc = "Bits 4:5 - CSEN Conversion Mode Select"]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
    #[doc = "Bits 8:9 - SAR Conversion Resolution."]
    #[inline(always)]
    pub fn sarcr(&mut self) -> SARCR_W {
        SARCR_W { w: self }
    }
    #[doc = "Bits 12:14 - CSEN Accumulator Mode Select"]
    #[inline(always)]
    pub fn acu(&mut self) -> ACU_W {
        ACU_W { w: self }
    }
    #[doc = "Bit 15 - CSEN Multiple Channel Enable"]
    #[inline(always)]
    pub fn mcen(&mut self) -> MCEN_W {
        MCEN_W { w: self }
    }
    #[doc = "Bits 16:17 - Start Trigger Select"]
    #[inline(always)]
    pub fn stm(&mut self) -> STM_W {
        STM_W { w: self }
    }
    #[doc = "Bit 18 - CSEN Digital Comparator Enable"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CMPEN_W {
        CMPEN_W { w: self }
    }
    #[doc = "Bit 19 - CSEN Disable Right-Shift"]
    #[inline(always)]
    pub fn drsf(&mut self) -> DRSF_W {
        DRSF_W { w: self }
    }
    #[doc = "Bit 20 - CSEN DMA Enable Bit"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 21 - CSEN Converter Select"]
    #[inline(always)]
    pub fn convsel(&mut self) -> CONVSEL_W {
        CONVSEL_W { w: self }
    }
    #[doc = "Bit 22 - CSEN Chop Enable"]
    #[inline(always)]
    pub fn chopen(&mut self) -> CHOPEN_W {
        CHOPEN_W { w: self }
    }
    #[doc = "Bit 23 - CSEN Automatic Ground Enable"]
    #[inline(always)]
    pub fn autognd(&mut self) -> AUTOGND_W {
        AUTOGND_W { w: self }
    }
    #[doc = "Bit 24 - CSEN Mux Disconnect"]
    #[inline(always)]
    pub fn mxuc(&mut self) -> MXUC_W {
        MXUC_W { w: self }
    }
    #[doc = "Bit 25 - Greater and Less Than Comparison Using the Exponential Moving Average (EMA) is Enabled"]
    #[inline(always)]
    pub fn emacmpen(&mut self) -> EMACMPEN_W {
        EMACMPEN_W { w: self }
    }
    #[doc = "Bit 26 - Select Warmup Mode for CSEN"]
    #[inline(always)]
    pub fn warmupmode(&mut self) -> WARMUPMODE_W {
        WARMUPMODE_W { w: self }
    }
    #[doc = "Bit 27 - Local Sensing Enable"]
    #[inline(always)]
    pub fn localsens(&mut self) -> LOCALSENS_W {
        LOCALSENS_W { w: self }
    }
    #[doc = "Bit 28 - Charge Pump Accuracy"]
    #[inline(always)]
    pub fn cpaccuracy(&mut self) -> CPACCURACY_W {
        CPACCURACY_W { w: self }
    }
}
