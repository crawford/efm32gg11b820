#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `VMONRDY`"]
pub type VMONRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONAVDD`"]
pub type VMONAVDD_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONALTAVDD`"]
pub type VMONALTAVDD_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONDVDD`"]
pub type VMONDVDD_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONIO0`"]
pub type VMONIO0_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONIO1`"]
pub type VMONIO1_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONBUVDD`"]
pub type VMONBUVDD_R = crate::R<bool, bool>;
#[doc = "Reader of field `BURDY`"]
pub type BURDY_R = crate::R<bool, bool>;
#[doc = "Current Voltage Scale Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSCALE_A {
    #[doc = "0: Voltage Scale Level 2"]
    VSCALE2,
    #[doc = "2: Voltage Scale Level 0"]
    VSCALE0,
    #[doc = "3: RESV"]
    RESV,
}
impl From<VSCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: VSCALE_A) -> Self {
        match variant {
            VSCALE_A::VSCALE2 => 0,
            VSCALE_A::VSCALE0 => 2,
            VSCALE_A::RESV => 3,
        }
    }
}
#[doc = "Reader of field `VSCALE`"]
pub type VSCALE_R = crate::R<u8, VSCALE_A>;
impl VSCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VSCALE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VSCALE_A::VSCALE2),
            2 => Val(VSCALE_A::VSCALE0),
            3 => Val(VSCALE_A::RESV),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VSCALE2`"]
    #[inline(always)]
    pub fn is_vscale2(&self) -> bool {
        *self == VSCALE_A::VSCALE2
    }
    #[doc = "Checks if the value of the field is `VSCALE0`"]
    #[inline(always)]
    pub fn is_vscale0(&self) -> bool {
        *self == VSCALE_A::VSCALE0
    }
    #[doc = "Checks if the value of the field is `RESV`"]
    #[inline(always)]
    pub fn is_resv(&self) -> bool {
        *self == VSCALE_A::RESV
    }
}
#[doc = "Reader of field `VSCALEBUSY`"]
pub type VSCALEBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `EM4IORET`"]
pub type EM4IORET_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEMPACTIVE`"]
pub type TEMPACTIVE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - VMON Ready"]
    #[inline(always)]
    pub fn vmonrdy(&self) -> VMONRDY_R {
        VMONRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VMON AVDD Channel"]
    #[inline(always)]
    pub fn vmonavdd(&self) -> VMONAVDD_R {
        VMONAVDD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Alternate VMON AVDD Channel"]
    #[inline(always)]
    pub fn vmonaltavdd(&self) -> VMONALTAVDD_R {
        VMONALTAVDD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VMON DVDD Channel"]
    #[inline(always)]
    pub fn vmondvdd(&self) -> VMONDVDD_R {
        VMONDVDD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - VMON IOVDD0 Channel"]
    #[inline(always)]
    pub fn vmonio0(&self) -> VMONIO0_R {
        VMONIO0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - VMON IOVDD1 Channel"]
    #[inline(always)]
    pub fn vmonio1(&self) -> VMONIO1_R {
        VMONIO1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - VMON BUVDD Channel"]
    #[inline(always)]
    pub fn vmonbuvdd(&self) -> VMONBUVDD_R {
        VMONBUVDD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Backup Mode Ready"]
    #[inline(always)]
    pub fn burdy(&self) -> BURDY_R {
        BURDY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Current Voltage Scale Value"]
    #[inline(always)]
    pub fn vscale(&self) -> VSCALE_R {
        VSCALE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - System is Busy Scaling Voltage"]
    #[inline(always)]
    pub fn vscalebusy(&self) -> VSCALEBUSY_R {
        VSCALEBUSY_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IO Retention Status"]
    #[inline(always)]
    pub fn em4ioret(&self) -> EM4IORET_R {
        EM4IORET_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Temperature Measurement Active"]
    #[inline(always)]
    pub fn tempactive(&self) -> TEMPACTIVE_R {
        TEMPACTIVE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
