#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `BUFDATAV`"]
pub type BUFDATAV_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUFHALFFULL`"]
pub type BUFHALFFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUFFULL`"]
pub type BUFFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RUNNING`"]
pub type RUNNING_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCANACTIVE`"]
pub type SCANACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DACACTIVE`"]
pub type DACACTIVE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Result Data Valid"]
    #[inline(always)]
    pub fn bufdatav(&self) -> BUFDATAV_R {
        BUFDATAV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Result Buffer Half Full"]
    #[inline(always)]
    pub fn bufhalffull(&self) -> BUFHALFFULL_R {
        BUFHALFFULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Result Buffer Full"]
    #[inline(always)]
    pub fn buffull(&self) -> BUFFULL_R {
        BUFFULL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LESENSE Periodic Counter Running"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LESENSE Scan Active"]
    #[inline(always)]
    pub fn scanactive(&self) -> SCANACTIVE_R {
        SCANACTIVE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LESENSE VDAC Interface is Active"]
    #[inline(always)]
    pub fn dacactive(&self) -> DACACTIVE_R {
        DACACTIVE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
