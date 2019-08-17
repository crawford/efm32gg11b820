#[doc = "Reader of register CAPAB0"]
pub type R = crate::R<u32, super::CAPAB0>;
#[doc = "Reader of field `TMOUTCLKFREQ`"]
pub type TMOUTCLKFREQ_R = crate::R<u8, u8>;
#[doc = "Reader of field `TMOUTCLKUNIT`"]
pub type TMOUTCLKUNIT_R = crate::R<bool, bool>;
#[doc = "Reader of field `BASECLKFREQSD`"]
pub type BASECLKFREQSD_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAXBLOCKLEN`"]
pub type MAXBLOCKLEN_R = crate::R<u8, u8>;
#[doc = "Reader of field `EXTMEDIABUSSUP`"]
pub type EXTMEDIABUSSUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADMA2SUP`"]
pub type ADMA2SUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSSUP`"]
pub type HSSUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDMASUP`"]
pub type SDMASUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUSRESSUP`"]
pub type SUSRESSUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `VOLTSUP3P3V`"]
pub type VOLTSUP3P3V_R = crate::R<bool, bool>;
#[doc = "Reader of field `VOLTSUP3P0V`"]
pub type VOLTSUP3P0V_R = crate::R<bool, bool>;
#[doc = "Reader of field `VOLTSUP1P8V`"]
pub type VOLTSUP1P8V_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYSBUS64BSUP`"]
pub type SYSBUS64BSUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASYNCINTSUP`"]
pub type ASYNCINTSUP_R = crate::R<bool, bool>;
#[doc = "Possible values of the field `IFSLOTTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFSLOTTYPE_A {
    #[doc = "Removable Card Slot"]
    REMOVABLE,
    #[doc = "Only one non-removable device is conected to a SD bus slot"]
    EMBEDDED,
    #[doc = "Can be set if Host controller supports Shared Bus CTRL register"]
    SHARED,
}
impl crate::ToBits<u8> for IFSLOTTYPE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            IFSLOTTYPE_A::REMOVABLE => 0,
            IFSLOTTYPE_A::EMBEDDED => 1,
            IFSLOTTYPE_A::SHARED => 2,
        }
    }
}
#[doc = "Reader of field `IFSLOTTYPE`"]
pub type IFSLOTTYPE_R = crate::R<u8, IFSLOTTYPE_A>;
impl IFSLOTTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IFSLOTTYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IFSLOTTYPE_A::REMOVABLE),
            1 => Val(IFSLOTTYPE_A::EMBEDDED),
            2 => Val(IFSLOTTYPE_A::SHARED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REMOVABLE`"]
    #[inline(always)]
    pub fn is_removable(&self) -> bool {
        *self == IFSLOTTYPE_A::REMOVABLE
    }
    #[doc = "Checks if the value of the field is `EMBEDDED`"]
    #[inline(always)]
    pub fn is_embedded(&self) -> bool {
        *self == IFSLOTTYPE_A::EMBEDDED
    }
    #[doc = "Checks if the value of the field is `SHARED`"]
    #[inline(always)]
    pub fn is_shared(&self) -> bool {
        *self == IFSLOTTYPE_A::SHARED
    }
}
impl R {
    #[doc = "Bits 0:5 - Timeout Clock Frequency"]
    #[inline(always)]
    pub fn tmoutclkfreq(&self) -> TMOUTCLKFREQ_R {
        TMOUTCLKFREQ_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Timeout Clock Unit"]
    #[inline(always)]
    pub fn tmoutclkunit(&self) -> TMOUTCLKUNIT_R {
        TMOUTCLKUNIT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Base Clock Frequency for SD_CLK"]
    #[inline(always)]
    pub fn baseclkfreqsd(&self) -> BASECLKFREQSD_R {
        BASECLKFREQSD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Maximum Block Length"]
    #[inline(always)]
    pub fn maxblocklen(&self) -> MAXBLOCKLEN_R {
        MAXBLOCKLEN_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Extended Media Bus Support"]
    #[inline(always)]
    pub fn extmediabussup(&self) -> EXTMEDIABUSSUP_R {
        EXTMEDIABUSSUP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ADMA2 Support"]
    #[inline(always)]
    pub fn adma2sup(&self) -> ADMA2SUP_R {
        ADMA2SUP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - High Speed Support"]
    #[inline(always)]
    pub fn hssup(&self) -> HSSUP_R {
        HSSUP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SDMA Support"]
    #[inline(always)]
    pub fn sdmasup(&self) -> SDMASUP_R {
        SDMASUP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Suspend / Resume Support"]
    #[inline(always)]
    pub fn susressup(&self) -> SUSRESSUP_R {
        SUSRESSUP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Voltage Support 3.3V"]
    #[inline(always)]
    pub fn voltsup3p3v(&self) -> VOLTSUP3P3V_R {
        VOLTSUP3P3V_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Voltage Support 3.0V"]
    #[inline(always)]
    pub fn voltsup3p0v(&self) -> VOLTSUP3P0V_R {
        VOLTSUP3P0V_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Voltage Support 1.8V"]
    #[inline(always)]
    pub fn voltsup1p8v(&self) -> VOLTSUP1P8V_R {
        VOLTSUP1P8V_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - System Bus 64-bit Support"]
    #[inline(always)]
    pub fn sysbus64bsup(&self) -> SYSBUS64BSUP_R {
        SYSBUS64BSUP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Asynchronous Interrupt Support"]
    #[inline(always)]
    pub fn asyncintsup(&self) -> ASYNCINTSUP_R {
        ASYNCINTSUP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Interface Card Slot Type"]
    #[inline(always)]
    pub fn ifslottype(&self) -> IFSLOTTYPE_R {
        IFSLOTTYPE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
