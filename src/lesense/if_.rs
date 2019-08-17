#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `CH0`"]
pub type CH0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1`"]
pub type CH1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH2`"]
pub type CH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH3`"]
pub type CH3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH4`"]
pub type CH4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH5`"]
pub type CH5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH6`"]
pub type CH6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH7`"]
pub type CH7_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH8`"]
pub type CH8_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH9`"]
pub type CH9_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH10`"]
pub type CH10_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH11`"]
pub type CH11_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH12`"]
pub type CH12_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH13`"]
pub type CH13_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH14`"]
pub type CH14_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH15`"]
pub type CH15_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCANCOMPLETE`"]
pub type SCANCOMPLETE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DEC`"]
pub type DEC_R = crate::R<bool, bool>;
#[doc = "Reader of field `DECERR`"]
pub type DECERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUFDATAV`"]
pub type BUFDATAV_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUFLEVEL`"]
pub type BUFLEVEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUFOF`"]
pub type BUFOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CNTOF`"]
pub type CNTOF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - CH0 Interrupt Flag"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CH1 Interrupt Flag"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CH2 Interrupt Flag"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CH3 Interrupt Flag"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CH4 Interrupt Flag"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CH5 Interrupt Flag"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CH6 Interrupt Flag"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CH7 Interrupt Flag"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CH8 Interrupt Flag"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CH9 Interrupt Flag"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CH10 Interrupt Flag"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CH11 Interrupt Flag"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CH12 Interrupt Flag"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CH13 Interrupt Flag"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CH14 Interrupt Flag"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CH15 Interrupt Flag"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SCANCOMPLETE Interrupt Flag"]
    #[inline(always)]
    pub fn scancomplete(&self) -> SCANCOMPLETE_R {
        SCANCOMPLETE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DEC Interrupt Flag"]
    #[inline(always)]
    pub fn dec(&self) -> DEC_R {
        DEC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DECERR Interrupt Flag"]
    #[inline(always)]
    pub fn decerr(&self) -> DECERR_R {
        DECERR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - BUFDATAV Interrupt Flag"]
    #[inline(always)]
    pub fn bufdatav(&self) -> BUFDATAV_R {
        BUFDATAV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - BUFLEVEL Interrupt Flag"]
    #[inline(always)]
    pub fn buflevel(&self) -> BUFLEVEL_R {
        BUFLEVEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - BUFOF Interrupt Flag"]
    #[inline(always)]
    pub fn bufof(&self) -> BUFOF_R {
        BUFOF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CNTOF Interrupt Flag"]
    #[inline(always)]
    pub fn cntof(&self) -> CNTOF_R {
        CNTOF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
