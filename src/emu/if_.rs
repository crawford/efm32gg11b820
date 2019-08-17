#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `VMONAVDDFALL`"]
pub type VMONAVDDFALL_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONAVDDRISE`"]
pub type VMONAVDDRISE_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONALTAVDDFALL`"]
pub type VMONALTAVDDFALL_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONALTAVDDRISE`"]
pub type VMONALTAVDDRISE_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONDVDDFALL`"]
pub type VMONDVDDFALL_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONDVDDRISE`"]
pub type VMONDVDDRISE_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONIO0FALL`"]
pub type VMONIO0FALL_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONIO0RISE`"]
pub type VMONIO0RISE_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONIO1FALL`"]
pub type VMONIO1FALL_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONIO1RISE`"]
pub type VMONIO1RISE_R = crate::R<bool, bool>;
#[doc = "Reader of field `R5VREADY`"]
pub type R5VREADY_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONBUVDDFALL`"]
pub type VMONBUVDDFALL_R = crate::R<bool, bool>;
#[doc = "Reader of field `VMONBUVDDRISE`"]
pub type VMONBUVDDRISE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PFETOVERCURRENTLIMIT`"]
pub type PFETOVERCURRENTLIMIT_R = crate::R<bool, bool>;
#[doc = "Reader of field `NFETOVERCURRENTLIMIT`"]
pub type NFETOVERCURRENTLIMIT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCDCLPRUNNING`"]
pub type DCDCLPRUNNING_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCDCLNRUNNING`"]
pub type DCDCLNRUNNING_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCDCINBYPASS`"]
pub type DCDCINBYPASS_R = crate::R<bool, bool>;
#[doc = "Reader of field `BURDY`"]
pub type BURDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `R5VVSINT`"]
pub type R5VVSINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EM23WAKEUP`"]
pub type EM23WAKEUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `VSCALEDONE`"]
pub type VSCALEDONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEMP`"]
pub type TEMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEMPLOW`"]
pub type TEMPLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEMPHIGH`"]
pub type TEMPHIGH_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - VMON AVDD Channel Fall"]
    #[inline(always)]
    pub fn vmonavddfall(&self) -> VMONAVDDFALL_R {
        VMONAVDDFALL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VMON AVDD Channel Rise"]
    #[inline(always)]
    pub fn vmonavddrise(&self) -> VMONAVDDRISE_R {
        VMONAVDDRISE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Alternate VMON AVDD Channel Fall"]
    #[inline(always)]
    pub fn vmonaltavddfall(&self) -> VMONALTAVDDFALL_R {
        VMONALTAVDDFALL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Alternate VMON AVDD Channel Rise"]
    #[inline(always)]
    pub fn vmonaltavddrise(&self) -> VMONALTAVDDRISE_R {
        VMONALTAVDDRISE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - VMON DVDD Channel Fall"]
    #[inline(always)]
    pub fn vmondvddfall(&self) -> VMONDVDDFALL_R {
        VMONDVDDFALL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - VMON DVDD Channel Rise"]
    #[inline(always)]
    pub fn vmondvddrise(&self) -> VMONDVDDRISE_R {
        VMONDVDDRISE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - VMON IOVDD0 Channel Fall"]
    #[inline(always)]
    pub fn vmonio0fall(&self) -> VMONIO0FALL_R {
        VMONIO0FALL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - VMON IOVDD0 Channel Rise"]
    #[inline(always)]
    pub fn vmonio0rise(&self) -> VMONIO0RISE_R {
        VMONIO0RISE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - VMON IOVDD1 Channel Fall"]
    #[inline(always)]
    pub fn vmonio1fall(&self) -> VMONIO1FALL_R {
        VMONIO1FALL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VMON IOVDD1 Channel Rise"]
    #[inline(always)]
    pub fn vmonio1rise(&self) -> VMONIO1RISE_R {
        VMONIO1RISE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 5V Regulator is Ready to Use"]
    #[inline(always)]
    pub fn r5vready(&self) -> R5VREADY_R {
        R5VREADY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - VMON BACKUP Channel Fall"]
    #[inline(always)]
    pub fn vmonbuvddfall(&self) -> VMONBUVDDFALL_R {
        VMONBUVDDFALL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VMON BUVDD Channel Rise"]
    #[inline(always)]
    pub fn vmonbuvddrise(&self) -> VMONBUVDDRISE_R {
        VMONBUVDDRISE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PFET Current Limit Hit"]
    #[inline(always)]
    pub fn pfetovercurrentlimit(&self) -> PFETOVERCURRENTLIMIT_R {
        PFETOVERCURRENTLIMIT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - NFET Current Limit Hit"]
    #[inline(always)]
    pub fn nfetovercurrentlimit(&self) -> NFETOVERCURRENTLIMIT_R {
        NFETOVERCURRENTLIMIT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - LP Mode is Running"]
    #[inline(always)]
    pub fn dcdclprunning(&self) -> DCDCLPRUNNING_R {
        DCDCLPRUNNING_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - LN Mode is Running"]
    #[inline(always)]
    pub fn dcdclnrunning(&self) -> DCDCLNRUNNING_R {
        DCDCLNRUNNING_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DCDC is in Bypass"]
    #[inline(always)]
    pub fn dcdcinbypass(&self) -> DCDCINBYPASS_R {
        DCDCINBYPASS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Backup Functionality Ready Interrupt Flag"]
    #[inline(always)]
    pub fn burdy(&self) -> BURDY_R {
        BURDY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 5V Regulator Voltage Update Done"]
    #[inline(always)]
    pub fn r5vvsint(&self) -> R5VVSINT_R {
        R5VVSINT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Wakeup IRQ From EM2 and EM3"]
    #[inline(always)]
    pub fn em23wakeup(&self) -> EM23WAKEUP_R {
        EM23WAKEUP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Voltage Scale Steps Done IRQ"]
    #[inline(always)]
    pub fn vscaledone(&self) -> VSCALEDONE_R {
        VSCALEDONE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 29 - New Temperature Measurement Valid"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Temperature Low Limit Reached"]
    #[inline(always)]
    pub fn templow(&self) -> TEMPLOW_R {
        TEMPLOW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Temperature High Limit Reached"]
    #[inline(always)]
    pub fn temphigh(&self) -> TEMPHIGH_R {
        TEMPHIGH_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
