#[doc = "Reader of register SINGLECTRL"]
pub type R = crate::R<u32, super::SINGLECTRL>;
#[doc = "Writer for register SINGLECTRL"]
pub type W = crate::W<u32, super::SINGLECTRL>;
#[doc = "Register SINGLECTRL `reset()`'s with value 0x00ff_ff00"]
impl crate::ResetValue for super::SINGLECTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00ff_ff00
    }
}
#[doc = "Reader of field `REP`"]
pub type REP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REP`"]
pub struct REP_W<'a> {
    w: &'a mut W,
}
impl<'a> REP_W<'a> {
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
#[doc = "Reader of field `DIFF`"]
pub type DIFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIFF`"]
pub struct DIFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFF_W<'a> {
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
#[doc = "Reader of field `ADJ`"]
pub type ADJ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADJ`"]
pub struct ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADJ_W<'a> {
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
#[doc = "Single Channel Resolution Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RES_A {
    #[doc = "0: 12-bit resolution."]
    _12BIT,
    #[doc = "1: 8-bit resolution."]
    _8BIT,
    #[doc = "2: 6-bit resolution."]
    _6BIT,
    #[doc = "3: Oversampling enabled. Oversampling rate is set in OVSRSEL."]
    OVS,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        match variant {
            RES_A::_12BIT => 0,
            RES_A::_8BIT => 1,
            RES_A::_6BIT => 2,
            RES_A::OVS => 3,
        }
    }
}
#[doc = "Reader of field `RES`"]
pub type RES_R = crate::R<u8, RES_A>;
impl RES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RES_A {
        match self.bits {
            0 => RES_A::_12BIT,
            1 => RES_A::_8BIT,
            2 => RES_A::_6BIT,
            3 => RES_A::OVS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == RES_A::_12BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == RES_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_6BIT`"]
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == RES_A::_6BIT
    }
    #[doc = "Checks if the value of the field is `OVS`"]
    #[inline(always)]
    pub fn is_ovs(&self) -> bool {
        *self == RES_A::OVS
    }
}
#[doc = "Write proxy for field `RES`"]
pub struct RES_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "12-bit resolution."]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(RES_A::_12BIT)
    }
    #[doc = "8-bit resolution."]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(RES_A::_8BIT)
    }
    #[doc = "6-bit resolution."]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut W {
        self.variant(RES_A::_6BIT)
    }
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL."]
    #[inline(always)]
    pub fn ovs(self) -> &'a mut W {
        self.variant(RES_A::OVS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Single Channel Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REF_A {
    #[doc = "0: VFS = 1.25V with internal VBGR reference"]
    _1V25,
    #[doc = "1: VFS = 2.5V with internal VBGR reference"]
    _2V5,
    #[doc = "2: VFS = AVDD with AVDD as reference source"]
    VDD,
    #[doc = "3: VFS = 5V with internal VBGR reference"]
    _5V,
    #[doc = "4: Single ended external reference"]
    EXTSINGLE,
    #[doc = "5: Differential external reference, 2x"]
    _2XEXTDIFF,
    #[doc = "6: VFS = 2xAVDD with AVDD as the reference source"]
    _2XVDD,
    #[doc = "7: Use SINGLECTRLX to configure reference"]
    CONF,
}
impl From<REF_A> for u8 {
    #[inline(always)]
    fn from(variant: REF_A) -> Self {
        match variant {
            REF_A::_1V25 => 0,
            REF_A::_2V5 => 1,
            REF_A::VDD => 2,
            REF_A::_5V => 3,
            REF_A::EXTSINGLE => 4,
            REF_A::_2XEXTDIFF => 5,
            REF_A::_2XVDD => 6,
            REF_A::CONF => 7,
        }
    }
}
#[doc = "Reader of field `REF`"]
pub type REF_R = crate::R<u8, REF_A>;
impl REF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REF_A {
        match self.bits {
            0 => REF_A::_1V25,
            1 => REF_A::_2V5,
            2 => REF_A::VDD,
            3 => REF_A::_5V,
            4 => REF_A::EXTSINGLE,
            5 => REF_A::_2XEXTDIFF,
            6 => REF_A::_2XVDD,
            7 => REF_A::CONF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == REF_A::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == REF_A::_2V5
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == REF_A::VDD
    }
    #[doc = "Checks if the value of the field is `_5V`"]
    #[inline(always)]
    pub fn is_5v(&self) -> bool {
        *self == REF_A::_5V
    }
    #[doc = "Checks if the value of the field is `EXTSINGLE`"]
    #[inline(always)]
    pub fn is_extsingle(&self) -> bool {
        *self == REF_A::EXTSINGLE
    }
    #[doc = "Checks if the value of the field is `_2XEXTDIFF`"]
    #[inline(always)]
    pub fn is_2xextdiff(&self) -> bool {
        *self == REF_A::_2XEXTDIFF
    }
    #[doc = "Checks if the value of the field is `_2XVDD`"]
    #[inline(always)]
    pub fn is_2xvdd(&self) -> bool {
        *self == REF_A::_2XVDD
    }
    #[doc = "Checks if the value of the field is `CONF`"]
    #[inline(always)]
    pub fn is_conf(&self) -> bool {
        *self == REF_A::CONF
    }
}
#[doc = "Write proxy for field `REF`"]
pub struct REF_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "VFS = 1.25V with internal VBGR reference"]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(REF_A::_1V25)
    }
    #[doc = "VFS = 2.5V with internal VBGR reference"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(REF_A::_2V5)
    }
    #[doc = "VFS = AVDD with AVDD as reference source"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REF_A::VDD)
    }
    #[doc = "VFS = 5V with internal VBGR reference"]
    #[inline(always)]
    pub fn _5v(self) -> &'a mut W {
        self.variant(REF_A::_5V)
    }
    #[doc = "Single ended external reference"]
    #[inline(always)]
    pub fn extsingle(self) -> &'a mut W {
        self.variant(REF_A::EXTSINGLE)
    }
    #[doc = "Differential external reference, 2x"]
    #[inline(always)]
    pub fn _2xextdiff(self) -> &'a mut W {
        self.variant(REF_A::_2XEXTDIFF)
    }
    #[doc = "VFS = 2xAVDD with AVDD as the reference source"]
    #[inline(always)]
    pub fn _2xvdd(self) -> &'a mut W {
        self.variant(REF_A::_2XVDD)
    }
    #[doc = "Use SINGLECTRLX to configure reference"]
    #[inline(always)]
    pub fn conf(self) -> &'a mut W {
        self.variant(REF_A::CONF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `POSSEL`"]
pub type POSSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POSSEL`"]
pub struct POSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> POSSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `NEGSEL`"]
pub type NEGSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NEGSEL`"]
pub struct NEGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> NEGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Single Channel Acquisition Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AT_A {
    #[doc = "0: 1 conversion clock cycle acquisition time for single channel"]
    _1CYCLE,
    #[doc = "1: 2 conversion clock cycles acquisition time for single channel"]
    _2CYCLES,
    #[doc = "2: 3 conversion clock cycles acquisition time for single channel"]
    _3CYCLES,
    #[doc = "3: 4 conversion clock cycles acquisition time for single channel"]
    _4CYCLES,
    #[doc = "4: 8 conversion clock cycles acquisition time for single channel"]
    _8CYCLES,
    #[doc = "5: 16 conversion clock cycles acquisition time for single channel"]
    _16CYCLES,
    #[doc = "6: 32 conversion clock cycles acquisition time for single channel"]
    _32CYCLES,
    #[doc = "7: 64 conversion clock cycles acquisition time for single channel"]
    _64CYCLES,
    #[doc = "8: 128 conversion clock cycles acquisition time for single channel"]
    _128CYCLES,
    #[doc = "9: 256 conversion clock cycles acquisition time for single channel"]
    _256CYCLES,
}
impl From<AT_A> for u8 {
    #[inline(always)]
    fn from(variant: AT_A) -> Self {
        match variant {
            AT_A::_1CYCLE => 0,
            AT_A::_2CYCLES => 1,
            AT_A::_3CYCLES => 2,
            AT_A::_4CYCLES => 3,
            AT_A::_8CYCLES => 4,
            AT_A::_16CYCLES => 5,
            AT_A::_32CYCLES => 6,
            AT_A::_64CYCLES => 7,
            AT_A::_128CYCLES => 8,
            AT_A::_256CYCLES => 9,
        }
    }
}
#[doc = "Reader of field `AT`"]
pub type AT_R = crate::R<u8, AT_A>;
impl AT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AT_A::_1CYCLE),
            1 => Val(AT_A::_2CYCLES),
            2 => Val(AT_A::_3CYCLES),
            3 => Val(AT_A::_4CYCLES),
            4 => Val(AT_A::_8CYCLES),
            5 => Val(AT_A::_16CYCLES),
            6 => Val(AT_A::_32CYCLES),
            7 => Val(AT_A::_64CYCLES),
            8 => Val(AT_A::_128CYCLES),
            9 => Val(AT_A::_256CYCLES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1CYCLE`"]
    #[inline(always)]
    pub fn is_1cycle(&self) -> bool {
        *self == AT_A::_1CYCLE
    }
    #[doc = "Checks if the value of the field is `_2CYCLES`"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == AT_A::_2CYCLES
    }
    #[doc = "Checks if the value of the field is `_3CYCLES`"]
    #[inline(always)]
    pub fn is_3cycles(&self) -> bool {
        *self == AT_A::_3CYCLES
    }
    #[doc = "Checks if the value of the field is `_4CYCLES`"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == AT_A::_4CYCLES
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == AT_A::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_16CYCLES`"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == AT_A::_16CYCLES
    }
    #[doc = "Checks if the value of the field is `_32CYCLES`"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == AT_A::_32CYCLES
    }
    #[doc = "Checks if the value of the field is `_64CYCLES`"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == AT_A::_64CYCLES
    }
    #[doc = "Checks if the value of the field is `_128CYCLES`"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == AT_A::_128CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == AT_A::_256CYCLES
    }
}
#[doc = "Write proxy for field `AT`"]
pub struct AT_W<'a> {
    w: &'a mut W,
}
impl<'a> AT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 conversion clock cycle acquisition time for single channel"]
    #[inline(always)]
    pub fn _1cycle(self) -> &'a mut W {
        self.variant(AT_A::_1CYCLE)
    }
    #[doc = "2 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut W {
        self.variant(AT_A::_2CYCLES)
    }
    #[doc = "3 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _3cycles(self) -> &'a mut W {
        self.variant(AT_A::_3CYCLES)
    }
    #[doc = "4 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut W {
        self.variant(AT_A::_4CYCLES)
    }
    #[doc = "8 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(AT_A::_8CYCLES)
    }
    #[doc = "16 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut W {
        self.variant(AT_A::_16CYCLES)
    }
    #[doc = "32 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut W {
        self.variant(AT_A::_32CYCLES)
    }
    #[doc = "64 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut W {
        self.variant(AT_A::_64CYCLES)
    }
    #[doc = "128 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut W {
        self.variant(AT_A::_128CYCLES)
    }
    #[doc = "256 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(AT_A::_256CYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `PRSEN`"]
pub type PRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRSEN`"]
pub struct PRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Single Channel Repetitive Mode"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Single Channel Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Single Channel Result Adjustment"]
    #[inline(always)]
    pub fn adj(&self) -> ADJ_R {
        ADJ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Single Channel Resolution Select"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - Single Channel Reference Selection"]
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - Single Channel Positive Input Selection"]
    #[inline(always)]
    pub fn possel(&self) -> POSSEL_R {
        POSSEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Single Channel Negative Input Selection"]
    #[inline(always)]
    pub fn negsel(&self) -> NEGSEL_R {
        NEGSEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Single Channel Acquisition Time"]
    #[inline(always)]
    pub fn at(&self) -> AT_R {
        AT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Single Channel PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&self) -> PRSEN_R {
        PRSEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Compare Logic Enable for Single Channel"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Channel Repetitive Mode"]
    #[inline(always)]
    pub fn rep(&mut self) -> REP_W {
        REP_W { w: self }
    }
    #[doc = "Bit 1 - Single Channel Differential Mode"]
    #[inline(always)]
    pub fn diff(&mut self) -> DIFF_W {
        DIFF_W { w: self }
    }
    #[doc = "Bit 2 - Single Channel Result Adjustment"]
    #[inline(always)]
    pub fn adj(&mut self) -> ADJ_W {
        ADJ_W { w: self }
    }
    #[doc = "Bits 3:4 - Single Channel Resolution Select"]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W {
        RES_W { w: self }
    }
    #[doc = "Bits 5:7 - Single Channel Reference Selection"]
    #[inline(always)]
    pub fn ref_(&mut self) -> REF_W {
        REF_W { w: self }
    }
    #[doc = "Bits 8:15 - Single Channel Positive Input Selection"]
    #[inline(always)]
    pub fn possel(&mut self) -> POSSEL_W {
        POSSEL_W { w: self }
    }
    #[doc = "Bits 16:23 - Single Channel Negative Input Selection"]
    #[inline(always)]
    pub fn negsel(&mut self) -> NEGSEL_W {
        NEGSEL_W { w: self }
    }
    #[doc = "Bits 24:27 - Single Channel Acquisition Time"]
    #[inline(always)]
    pub fn at(&mut self) -> AT_W {
        AT_W { w: self }
    }
    #[doc = "Bit 29 - Single Channel PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&mut self) -> PRSEN_W {
        PRSEN_W { w: self }
    }
    #[doc = "Bit 31 - Compare Logic Enable for Single Channel"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CMPEN_W {
        CMPEN_W { w: self }
    }
}
