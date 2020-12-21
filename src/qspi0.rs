#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Octal-SPI Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x04 - Device Read Instruction Configuration Register"]
    pub devinstrrdconfig: DEVINSTRRDCONFIG,
    #[doc = "0x08 - Device Write Instruction Configuration Register"]
    pub devinstrwrconfig: DEVINSTRWRCONFIG,
    #[doc = "0x0c - Device Delay Register"]
    pub devdelay: DEVDELAY,
    #[doc = "0x10 - Read Data Capture Register"]
    pub rddatacapture: RDDATACAPTURE,
    #[doc = "0x14 - Device Size Configuration Register"]
    pub devsizeconfig: DEVSIZECONFIG,
    #[doc = "0x18 - SRAM Partition Configuration Register"]
    pub srampartitioncfg: SRAMPARTITIONCFG,
    #[doc = "0x1c - Indirect Address Trigger Register"]
    pub indahbaddrtrigger: INDAHBADDRTRIGGER,
    _reserved8: [u8; 4usize],
    #[doc = "0x24 - Remap Address Register"]
    pub remapaddr: REMAPADDR,
    #[doc = "0x28 - Mode Bit Configuration Register"]
    pub modebitconfig: MODEBITCONFIG,
    #[doc = "0x2c - SRAM Fill Register"]
    pub sramfill: SRAMFILL,
    #[doc = "0x30 - TX Threshold Register"]
    pub txthresh: TXTHRESH,
    #[doc = "0x34 - RX Threshold Register"]
    pub rxthresh: RXTHRESH,
    #[doc = "0x38 - Write Completion Control Register"]
    pub writecompletionctrl: WRITECOMPLETIONCTRL,
    #[doc = "0x3c - Polling Expiration Register"]
    pub noofpollsbefexp: NOOFPOLLSBEFEXP,
    #[doc = "0x40 - Interrupt Status Register"]
    pub irqstatus: IRQSTATUS,
    #[doc = "0x44 - Interrupt Mask"]
    pub irqmask: IRQMASK,
    _reserved17: [u8; 8usize],
    #[doc = "0x50 - Lower Write Protection Register"]
    pub lowerwrprot: LOWERWRPROT,
    #[doc = "0x54 - Upper Write Protection Register"]
    pub upperwrprot: UPPERWRPROT,
    #[doc = "0x58 - Write Protection Control Register"]
    pub wrprotctrl: WRPROTCTRL,
    _reserved20: [u8; 4usize],
    #[doc = "0x60 - Indirect Read Transfer Control Register"]
    pub indirectreadxferctrl: INDIRECTREADXFERCTRL,
    #[doc = "0x64 - Indirect Read Transfer Watermark Register"]
    pub indirectreadxferwatermark: INDIRECTREADXFERWATERMARK,
    #[doc = "0x68 - Indirect Read Transfer Start Address Register"]
    pub indirectreadxferstart: INDIRECTREADXFERSTART,
    #[doc = "0x6c - Indirect Read Transfer Number Bytes Register"]
    pub indirectreadxfernumbytes: INDIRECTREADXFERNUMBYTES,
    #[doc = "0x70 - Indirect Write Transfer Control Register"]
    pub indirectwritexferctrl: INDIRECTWRITEXFERCTRL,
    #[doc = "0x74 - Indirect Write Transfer Watermark Register"]
    pub indirectwritexferwatermark: INDIRECTWRITEXFERWATERMARK,
    #[doc = "0x78 - Indirect Write Transfer Start Address Register"]
    pub indirectwritexferstart: INDIRECTWRITEXFERSTART,
    #[doc = "0x7c - Indirect Write Transfer Number Bytes Register"]
    pub indirectwritexfernumbytes: INDIRECTWRITEXFERNUMBYTES,
    #[doc = "0x80 - Indirect Trigger Address Range Register"]
    pub indirecttriggeraddrrange: INDIRECTTRIGGERADDRRANGE,
    _reserved29: [u8; 8usize],
    #[doc = "0x8c - Flash Command Control Memory Register (STIG)"]
    pub flashcommandctrlmem: FLASHCOMMANDCTRLMEM,
    #[doc = "0x90 - Flash Command Control Register (STIG)"]
    pub flashcmdctrl: FLASHCMDCTRL,
    #[doc = "0x94 - Flash Command Address Register (STIG)"]
    pub flashcmdaddr: FLASHCMDADDR,
    _reserved32: [u8; 8usize],
    #[doc = "0xa0 - Flash Command Read Data Register (Lower) (STIG)"]
    pub flashrddatalower: FLASHRDDATALOWER,
    #[doc = "0xa4 - Flash Command Read Data Register (Upper) (STIG)"]
    pub flashrddataupper: FLASHRDDATAUPPER,
    #[doc = "0xa8 - Flash Command Write Data Register (Lower) (STIG)"]
    pub flashwrdatalower: FLASHWRDATALOWER,
    #[doc = "0xac - Flash Command Write Data Register (Upper) (STIG)"]
    pub flashwrdataupper: FLASHWRDATAUPPER,
    #[doc = "0xb0 - Polling Flash Status Register"]
    pub pollingflashstatus: POLLINGFLASHSTATUS,
    #[doc = "0xb4 - PHY Configuration Register"]
    pub phyconfiguration: PHYCONFIGURATION,
    _reserved38: [u8; 40usize],
    #[doc = "0xe0 - Opcode Extension Register (Lower)"]
    pub opcodeextlower: OPCODEEXTLOWER,
    #[doc = "0xe4 - Opcode Extension Register (Upper)"]
    pub opcodeextupper: OPCODEEXTUPPER,
    _reserved40: [u8; 20usize],
    #[doc = "0xfc - Module ID Register"]
    pub moduleid: MODULEID,
    _reserved41: [u8; 4usize],
    #[doc = "0x104 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x108 - I/O Route Location Register 0"]
    pub routeloc0: ROUTELOC0,
}
#[doc = "Octal-SPI Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Octal-SPI Configuration Register"]
pub mod config;
#[doc = "Device Read Instruction Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devinstrrdconfig](devinstrrdconfig) module"]
pub type DEVINSTRRDCONFIG = crate::Reg<u32, _DEVINSTRRDCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVINSTRRDCONFIG;
#[doc = "`read()` method returns [devinstrrdconfig::R](devinstrrdconfig::R) reader structure"]
impl crate::Readable for DEVINSTRRDCONFIG {}
#[doc = "`write(|w| ..)` method takes [devinstrrdconfig::W](devinstrrdconfig::W) writer structure"]
impl crate::Writable for DEVINSTRRDCONFIG {}
#[doc = "Device Read Instruction Configuration Register"]
pub mod devinstrrdconfig;
#[doc = "Device Write Instruction Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devinstrwrconfig](devinstrwrconfig) module"]
pub type DEVINSTRWRCONFIG = crate::Reg<u32, _DEVINSTRWRCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVINSTRWRCONFIG;
#[doc = "`read()` method returns [devinstrwrconfig::R](devinstrwrconfig::R) reader structure"]
impl crate::Readable for DEVINSTRWRCONFIG {}
#[doc = "`write(|w| ..)` method takes [devinstrwrconfig::W](devinstrwrconfig::W) writer structure"]
impl crate::Writable for DEVINSTRWRCONFIG {}
#[doc = "Device Write Instruction Configuration Register"]
pub mod devinstrwrconfig;
#[doc = "Device Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devdelay](devdelay) module"]
pub type DEVDELAY = crate::Reg<u32, _DEVDELAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDELAY;
#[doc = "`read()` method returns [devdelay::R](devdelay::R) reader structure"]
impl crate::Readable for DEVDELAY {}
#[doc = "`write(|w| ..)` method takes [devdelay::W](devdelay::W) writer structure"]
impl crate::Writable for DEVDELAY {}
#[doc = "Device Delay Register"]
pub mod devdelay;
#[doc = "Read Data Capture Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rddatacapture](rddatacapture) module"]
pub type RDDATACAPTURE = crate::Reg<u32, _RDDATACAPTURE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDDATACAPTURE;
#[doc = "`read()` method returns [rddatacapture::R](rddatacapture::R) reader structure"]
impl crate::Readable for RDDATACAPTURE {}
#[doc = "`write(|w| ..)` method takes [rddatacapture::W](rddatacapture::W) writer structure"]
impl crate::Writable for RDDATACAPTURE {}
#[doc = "Read Data Capture Register"]
pub mod rddatacapture;
#[doc = "Device Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devsizeconfig](devsizeconfig) module"]
pub type DEVSIZECONFIG = crate::Reg<u32, _DEVSIZECONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVSIZECONFIG;
#[doc = "`read()` method returns [devsizeconfig::R](devsizeconfig::R) reader structure"]
impl crate::Readable for DEVSIZECONFIG {}
#[doc = "`write(|w| ..)` method takes [devsizeconfig::W](devsizeconfig::W) writer structure"]
impl crate::Writable for DEVSIZECONFIG {}
#[doc = "Device Size Configuration Register"]
pub mod devsizeconfig;
#[doc = "SRAM Partition Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srampartitioncfg](srampartitioncfg) module"]
pub type SRAMPARTITIONCFG = crate::Reg<u32, _SRAMPARTITIONCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAMPARTITIONCFG;
#[doc = "`read()` method returns [srampartitioncfg::R](srampartitioncfg::R) reader structure"]
impl crate::Readable for SRAMPARTITIONCFG {}
#[doc = "`write(|w| ..)` method takes [srampartitioncfg::W](srampartitioncfg::W) writer structure"]
impl crate::Writable for SRAMPARTITIONCFG {}
#[doc = "SRAM Partition Configuration Register"]
pub mod srampartitioncfg;
#[doc = "Indirect Address Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indahbaddrtrigger](indahbaddrtrigger) module"]
pub type INDAHBADDRTRIGGER = crate::Reg<u32, _INDAHBADDRTRIGGER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INDAHBADDRTRIGGER;
#[doc = "`read()` method returns [indahbaddrtrigger::R](indahbaddrtrigger::R) reader structure"]
impl crate::Readable for INDAHBADDRTRIGGER {}
#[doc = "`write(|w| ..)` method takes [indahbaddrtrigger::W](indahbaddrtrigger::W) writer structure"]
impl crate::Writable for INDAHBADDRTRIGGER {}
#[doc = "Indirect Address Trigger Register"]
pub mod indahbaddrtrigger;
#[doc = "Remap Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [remapaddr](remapaddr) module"]
pub type REMAPADDR = crate::Reg<u32, _REMAPADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REMAPADDR;
#[doc = "`read()` method returns [remapaddr::R](remapaddr::R) reader structure"]
impl crate::Readable for REMAPADDR {}
#[doc = "`write(|w| ..)` method takes [remapaddr::W](remapaddr::W) writer structure"]
impl crate::Writable for REMAPADDR {}
#[doc = "Remap Address Register"]
pub mod remapaddr;
#[doc = "Mode Bit Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modebitconfig](modebitconfig) module"]
pub type MODEBITCONFIG = crate::Reg<u32, _MODEBITCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODEBITCONFIG;
#[doc = "`read()` method returns [modebitconfig::R](modebitconfig::R) reader structure"]
impl crate::Readable for MODEBITCONFIG {}
#[doc = "`write(|w| ..)` method takes [modebitconfig::W](modebitconfig::W) writer structure"]
impl crate::Writable for MODEBITCONFIG {}
#[doc = "Mode Bit Configuration Register"]
pub mod modebitconfig;
#[doc = "SRAM Fill Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sramfill](sramfill) module"]
pub type SRAMFILL = crate::Reg<u32, _SRAMFILL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAMFILL;
#[doc = "`read()` method returns [sramfill::R](sramfill::R) reader structure"]
impl crate::Readable for SRAMFILL {}
#[doc = "SRAM Fill Register"]
pub mod sramfill;
#[doc = "TX Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txthresh](txthresh) module"]
pub type TXTHRESH = crate::Reg<u32, _TXTHRESH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXTHRESH;
#[doc = "`read()` method returns [txthresh::R](txthresh::R) reader structure"]
impl crate::Readable for TXTHRESH {}
#[doc = "`write(|w| ..)` method takes [txthresh::W](txthresh::W) writer structure"]
impl crate::Writable for TXTHRESH {}
#[doc = "TX Threshold Register"]
pub mod txthresh;
#[doc = "RX Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxthresh](rxthresh) module"]
pub type RXTHRESH = crate::Reg<u32, _RXTHRESH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXTHRESH;
#[doc = "`read()` method returns [rxthresh::R](rxthresh::R) reader structure"]
impl crate::Readable for RXTHRESH {}
#[doc = "`write(|w| ..)` method takes [rxthresh::W](rxthresh::W) writer structure"]
impl crate::Writable for RXTHRESH {}
#[doc = "RX Threshold Register"]
pub mod rxthresh;
#[doc = "Write Completion Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writecompletionctrl](writecompletionctrl) module"]
pub type WRITECOMPLETIONCTRL = crate::Reg<u32, _WRITECOMPLETIONCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRITECOMPLETIONCTRL;
#[doc = "`read()` method returns [writecompletionctrl::R](writecompletionctrl::R) reader structure"]
impl crate::Readable for WRITECOMPLETIONCTRL {}
#[doc = "`write(|w| ..)` method takes [writecompletionctrl::W](writecompletionctrl::W) writer structure"]
impl crate::Writable for WRITECOMPLETIONCTRL {}
#[doc = "Write Completion Control Register"]
pub mod writecompletionctrl;
#[doc = "Polling Expiration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [noofpollsbefexp](noofpollsbefexp) module"]
pub type NOOFPOLLSBEFEXP = crate::Reg<u32, _NOOFPOLLSBEFEXP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NOOFPOLLSBEFEXP;
#[doc = "`read()` method returns [noofpollsbefexp::R](noofpollsbefexp::R) reader structure"]
impl crate::Readable for NOOFPOLLSBEFEXP {}
#[doc = "`write(|w| ..)` method takes [noofpollsbefexp::W](noofpollsbefexp::W) writer structure"]
impl crate::Writable for NOOFPOLLSBEFEXP {}
#[doc = "Polling Expiration Register"]
pub mod noofpollsbefexp;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqstatus](irqstatus) module"]
pub type IRQSTATUS = crate::Reg<u32, _IRQSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQSTATUS;
#[doc = "`read()` method returns [irqstatus::R](irqstatus::R) reader structure"]
impl crate::Readable for IRQSTATUS {}
#[doc = "`write(|w| ..)` method takes [irqstatus::W](irqstatus::W) writer structure"]
impl crate::Writable for IRQSTATUS {}
#[doc = "Interrupt Status Register"]
pub mod irqstatus;
#[doc = "Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqmask](irqmask) module"]
pub type IRQMASK = crate::Reg<u32, _IRQMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQMASK;
#[doc = "`read()` method returns [irqmask::R](irqmask::R) reader structure"]
impl crate::Readable for IRQMASK {}
#[doc = "`write(|w| ..)` method takes [irqmask::W](irqmask::W) writer structure"]
impl crate::Writable for IRQMASK {}
#[doc = "Interrupt Mask"]
pub mod irqmask;
#[doc = "Lower Write Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lowerwrprot](lowerwrprot) module"]
pub type LOWERWRPROT = crate::Reg<u32, _LOWERWRPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOWERWRPROT;
#[doc = "`read()` method returns [lowerwrprot::R](lowerwrprot::R) reader structure"]
impl crate::Readable for LOWERWRPROT {}
#[doc = "`write(|w| ..)` method takes [lowerwrprot::W](lowerwrprot::W) writer structure"]
impl crate::Writable for LOWERWRPROT {}
#[doc = "Lower Write Protection Register"]
pub mod lowerwrprot;
#[doc = "Upper Write Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upperwrprot](upperwrprot) module"]
pub type UPPERWRPROT = crate::Reg<u32, _UPPERWRPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPPERWRPROT;
#[doc = "`read()` method returns [upperwrprot::R](upperwrprot::R) reader structure"]
impl crate::Readable for UPPERWRPROT {}
#[doc = "`write(|w| ..)` method takes [upperwrprot::W](upperwrprot::W) writer structure"]
impl crate::Writable for UPPERWRPROT {}
#[doc = "Upper Write Protection Register"]
pub mod upperwrprot;
#[doc = "Write Protection Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrprotctrl](wrprotctrl) module"]
pub type WRPROTCTRL = crate::Reg<u32, _WRPROTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRPROTCTRL;
#[doc = "`read()` method returns [wrprotctrl::R](wrprotctrl::R) reader structure"]
impl crate::Readable for WRPROTCTRL {}
#[doc = "`write(|w| ..)` method takes [wrprotctrl::W](wrprotctrl::W) writer structure"]
impl crate::Writable for WRPROTCTRL {}
#[doc = "Write Protection Control Register"]
pub mod wrprotctrl;
#[doc = "Indirect Read Transfer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indirectreadxferctrl](indirectreadxferctrl) module"]
pub type INDIRECTREADXFERCTRL = crate::Reg<u32, _INDIRECTREADXFERCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INDIRECTREADXFERCTRL;
#[doc = "`read()` method returns [indirectreadxferctrl::R](indirectreadxferctrl::R) reader structure"]
impl crate::Readable for INDIRECTREADXFERCTRL {}
#[doc = "`write(|w| ..)` method takes [indirectreadxferctrl::W](indirectreadxferctrl::W) writer structure"]
impl crate::Writable for INDIRECTREADXFERCTRL {}
#[doc = "Indirect Read Transfer Control Register"]
pub mod indirectreadxferctrl;
#[doc = "Indirect Read Transfer Watermark Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indirectreadxferwatermark](indirectreadxferwatermark) module"]
pub type INDIRECTREADXFERWATERMARK = crate::Reg<u32, _INDIRECTREADXFERWATERMARK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INDIRECTREADXFERWATERMARK;
#[doc = "`read()` method returns [indirectreadxferwatermark::R](indirectreadxferwatermark::R) reader structure"]
impl crate::Readable for INDIRECTREADXFERWATERMARK {}
#[doc = "`write(|w| ..)` method takes [indirectreadxferwatermark::W](indirectreadxferwatermark::W) writer structure"]
impl crate::Writable for INDIRECTREADXFERWATERMARK {}
#[doc = "Indirect Read Transfer Watermark Register"]
pub mod indirectreadxferwatermark;
#[doc = "Indirect Read Transfer Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indirectreadxferstart](indirectreadxferstart) module"]
pub type INDIRECTREADXFERSTART = crate::Reg<u32, _INDIRECTREADXFERSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INDIRECTREADXFERSTART;
#[doc = "`read()` method returns [indirectreadxferstart::R](indirectreadxferstart::R) reader structure"]
impl crate::Readable for INDIRECTREADXFERSTART {}
#[doc = "`write(|w| ..)` method takes [indirectreadxferstart::W](indirectreadxferstart::W) writer structure"]
impl crate::Writable for INDIRECTREADXFERSTART {}
#[doc = "Indirect Read Transfer Start Address Register"]
pub mod indirectreadxferstart;
#[doc = "Indirect Read Transfer Number Bytes Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indirectreadxfernumbytes](indirectreadxfernumbytes) module"]
pub type INDIRECTREADXFERNUMBYTES = crate::Reg<u32, _INDIRECTREADXFERNUMBYTES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INDIRECTREADXFERNUMBYTES;
#[doc = "`read()` method returns [indirectreadxfernumbytes::R](indirectreadxfernumbytes::R) reader structure"]
impl crate::Readable for INDIRECTREADXFERNUMBYTES {}
#[doc = "`write(|w| ..)` method takes [indirectreadxfernumbytes::W](indirectreadxfernumbytes::W) writer structure"]
impl crate::Writable for INDIRECTREADXFERNUMBYTES {}
#[doc = "Indirect Read Transfer Number Bytes Register"]
pub mod indirectreadxfernumbytes;
#[doc = "Indirect Write Transfer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indirectwritexferctrl](indirectwritexferctrl) module"]
pub type INDIRECTWRITEXFERCTRL = crate::Reg<u32, _INDIRECTWRITEXFERCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INDIRECTWRITEXFERCTRL;
#[doc = "`read()` method returns [indirectwritexferctrl::R](indirectwritexferctrl::R) reader structure"]
impl crate::Readable for INDIRECTWRITEXFERCTRL {}
#[doc = "`write(|w| ..)` method takes [indirectwritexferctrl::W](indirectwritexferctrl::W) writer structure"]
impl crate::Writable for INDIRECTWRITEXFERCTRL {}
#[doc = "Indirect Write Transfer Control Register"]
pub mod indirectwritexferctrl;
#[doc = "Indirect Write Transfer Watermark Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indirectwritexferwatermark](indirectwritexferwatermark) module"]
pub type INDIRECTWRITEXFERWATERMARK = crate::Reg<u32, _INDIRECTWRITEXFERWATERMARK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INDIRECTWRITEXFERWATERMARK;
#[doc = "`read()` method returns [indirectwritexferwatermark::R](indirectwritexferwatermark::R) reader structure"]
impl crate::Readable for INDIRECTWRITEXFERWATERMARK {}
#[doc = "`write(|w| ..)` method takes [indirectwritexferwatermark::W](indirectwritexferwatermark::W) writer structure"]
impl crate::Writable for INDIRECTWRITEXFERWATERMARK {}
#[doc = "Indirect Write Transfer Watermark Register"]
pub mod indirectwritexferwatermark;
#[doc = "Indirect Write Transfer Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indirectwritexferstart](indirectwritexferstart) module"]
pub type INDIRECTWRITEXFERSTART = crate::Reg<u32, _INDIRECTWRITEXFERSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INDIRECTWRITEXFERSTART;
#[doc = "`read()` method returns [indirectwritexferstart::R](indirectwritexferstart::R) reader structure"]
impl crate::Readable for INDIRECTWRITEXFERSTART {}
#[doc = "`write(|w| ..)` method takes [indirectwritexferstart::W](indirectwritexferstart::W) writer structure"]
impl crate::Writable for INDIRECTWRITEXFERSTART {}
#[doc = "Indirect Write Transfer Start Address Register"]
pub mod indirectwritexferstart;
#[doc = "Indirect Write Transfer Number Bytes Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indirectwritexfernumbytes](indirectwritexfernumbytes) module"]
pub type INDIRECTWRITEXFERNUMBYTES = crate::Reg<u32, _INDIRECTWRITEXFERNUMBYTES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INDIRECTWRITEXFERNUMBYTES;
#[doc = "`read()` method returns [indirectwritexfernumbytes::R](indirectwritexfernumbytes::R) reader structure"]
impl crate::Readable for INDIRECTWRITEXFERNUMBYTES {}
#[doc = "`write(|w| ..)` method takes [indirectwritexfernumbytes::W](indirectwritexfernumbytes::W) writer structure"]
impl crate::Writable for INDIRECTWRITEXFERNUMBYTES {}
#[doc = "Indirect Write Transfer Number Bytes Register"]
pub mod indirectwritexfernumbytes;
#[doc = "Indirect Trigger Address Range Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indirecttriggeraddrrange](indirecttriggeraddrrange) module"]
pub type INDIRECTTRIGGERADDRRANGE = crate::Reg<u32, _INDIRECTTRIGGERADDRRANGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INDIRECTTRIGGERADDRRANGE;
#[doc = "`read()` method returns [indirecttriggeraddrrange::R](indirecttriggeraddrrange::R) reader structure"]
impl crate::Readable for INDIRECTTRIGGERADDRRANGE {}
#[doc = "`write(|w| ..)` method takes [indirecttriggeraddrrange::W](indirecttriggeraddrrange::W) writer structure"]
impl crate::Writable for INDIRECTTRIGGERADDRRANGE {}
#[doc = "Indirect Trigger Address Range Register"]
pub mod indirecttriggeraddrrange;
#[doc = "Flash Command Control Memory Register (STIG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcommandctrlmem](flashcommandctrlmem) module"]
pub type FLASHCOMMANDCTRLMEM = crate::Reg<u32, _FLASHCOMMANDCTRLMEM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHCOMMANDCTRLMEM;
#[doc = "`read()` method returns [flashcommandctrlmem::R](flashcommandctrlmem::R) reader structure"]
impl crate::Readable for FLASHCOMMANDCTRLMEM {}
#[doc = "`write(|w| ..)` method takes [flashcommandctrlmem::W](flashcommandctrlmem::W) writer structure"]
impl crate::Writable for FLASHCOMMANDCTRLMEM {}
#[doc = "Flash Command Control Memory Register (STIG)"]
pub mod flashcommandctrlmem;
#[doc = "Flash Command Control Register (STIG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcmdctrl](flashcmdctrl) module"]
pub type FLASHCMDCTRL = crate::Reg<u32, _FLASHCMDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHCMDCTRL;
#[doc = "`read()` method returns [flashcmdctrl::R](flashcmdctrl::R) reader structure"]
impl crate::Readable for FLASHCMDCTRL {}
#[doc = "`write(|w| ..)` method takes [flashcmdctrl::W](flashcmdctrl::W) writer structure"]
impl crate::Writable for FLASHCMDCTRL {}
#[doc = "Flash Command Control Register (STIG)"]
pub mod flashcmdctrl;
#[doc = "Flash Command Address Register (STIG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcmdaddr](flashcmdaddr) module"]
pub type FLASHCMDADDR = crate::Reg<u32, _FLASHCMDADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHCMDADDR;
#[doc = "`read()` method returns [flashcmdaddr::R](flashcmdaddr::R) reader structure"]
impl crate::Readable for FLASHCMDADDR {}
#[doc = "`write(|w| ..)` method takes [flashcmdaddr::W](flashcmdaddr::W) writer structure"]
impl crate::Writable for FLASHCMDADDR {}
#[doc = "Flash Command Address Register (STIG)"]
pub mod flashcmdaddr;
#[doc = "Flash Command Read Data Register (Lower) (STIG)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashrddatalower](flashrddatalower) module"]
pub type FLASHRDDATALOWER = crate::Reg<u32, _FLASHRDDATALOWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHRDDATALOWER;
#[doc = "`read()` method returns [flashrddatalower::R](flashrddatalower::R) reader structure"]
impl crate::Readable for FLASHRDDATALOWER {}
#[doc = "Flash Command Read Data Register (Lower) (STIG)"]
pub mod flashrddatalower;
#[doc = "Flash Command Read Data Register (Upper) (STIG)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashrddataupper](flashrddataupper) module"]
pub type FLASHRDDATAUPPER = crate::Reg<u32, _FLASHRDDATAUPPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHRDDATAUPPER;
#[doc = "`read()` method returns [flashrddataupper::R](flashrddataupper::R) reader structure"]
impl crate::Readable for FLASHRDDATAUPPER {}
#[doc = "Flash Command Read Data Register (Upper) (STIG)"]
pub mod flashrddataupper;
#[doc = "Flash Command Write Data Register (Lower) (STIG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashwrdatalower](flashwrdatalower) module"]
pub type FLASHWRDATALOWER = crate::Reg<u32, _FLASHWRDATALOWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHWRDATALOWER;
#[doc = "`read()` method returns [flashwrdatalower::R](flashwrdatalower::R) reader structure"]
impl crate::Readable for FLASHWRDATALOWER {}
#[doc = "`write(|w| ..)` method takes [flashwrdatalower::W](flashwrdatalower::W) writer structure"]
impl crate::Writable for FLASHWRDATALOWER {}
#[doc = "Flash Command Write Data Register (Lower) (STIG)"]
pub mod flashwrdatalower;
#[doc = "Flash Command Write Data Register (Upper) (STIG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashwrdataupper](flashwrdataupper) module"]
pub type FLASHWRDATAUPPER = crate::Reg<u32, _FLASHWRDATAUPPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHWRDATAUPPER;
#[doc = "`read()` method returns [flashwrdataupper::R](flashwrdataupper::R) reader structure"]
impl crate::Readable for FLASHWRDATAUPPER {}
#[doc = "`write(|w| ..)` method takes [flashwrdataupper::W](flashwrdataupper::W) writer structure"]
impl crate::Writable for FLASHWRDATAUPPER {}
#[doc = "Flash Command Write Data Register (Upper) (STIG)"]
pub mod flashwrdataupper;
#[doc = "Polling Flash Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pollingflashstatus](pollingflashstatus) module"]
pub type POLLINGFLASHSTATUS = crate::Reg<u32, _POLLINGFLASHSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POLLINGFLASHSTATUS;
#[doc = "`read()` method returns [pollingflashstatus::R](pollingflashstatus::R) reader structure"]
impl crate::Readable for POLLINGFLASHSTATUS {}
#[doc = "`write(|w| ..)` method takes [pollingflashstatus::W](pollingflashstatus::W) writer structure"]
impl crate::Writable for POLLINGFLASHSTATUS {}
#[doc = "Polling Flash Status Register"]
pub mod pollingflashstatus;
#[doc = "PHY Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phyconfiguration](phyconfiguration) module"]
pub type PHYCONFIGURATION = crate::Reg<u32, _PHYCONFIGURATION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PHYCONFIGURATION;
#[doc = "`read()` method returns [phyconfiguration::R](phyconfiguration::R) reader structure"]
impl crate::Readable for PHYCONFIGURATION {}
#[doc = "`write(|w| ..)` method takes [phyconfiguration::W](phyconfiguration::W) writer structure"]
impl crate::Writable for PHYCONFIGURATION {}
#[doc = "PHY Configuration Register"]
pub mod phyconfiguration;
#[doc = "Opcode Extension Register (Lower)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opcodeextlower](opcodeextlower) module"]
pub type OPCODEEXTLOWER = crate::Reg<u32, _OPCODEEXTLOWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPCODEEXTLOWER;
#[doc = "`read()` method returns [opcodeextlower::R](opcodeextlower::R) reader structure"]
impl crate::Readable for OPCODEEXTLOWER {}
#[doc = "`write(|w| ..)` method takes [opcodeextlower::W](opcodeextlower::W) writer structure"]
impl crate::Writable for OPCODEEXTLOWER {}
#[doc = "Opcode Extension Register (Lower)"]
pub mod opcodeextlower;
#[doc = "Opcode Extension Register (Upper)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opcodeextupper](opcodeextupper) module"]
pub type OPCODEEXTUPPER = crate::Reg<u32, _OPCODEEXTUPPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPCODEEXTUPPER;
#[doc = "`read()` method returns [opcodeextupper::R](opcodeextupper::R) reader structure"]
impl crate::Readable for OPCODEEXTUPPER {}
#[doc = "`write(|w| ..)` method takes [opcodeextupper::W](opcodeextupper::W) writer structure"]
impl crate::Writable for OPCODEEXTUPPER {}
#[doc = "Opcode Extension Register (Upper)"]
pub mod opcodeextupper;
#[doc = "Module ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moduleid](moduleid) module"]
pub type MODULEID = crate::Reg<u32, _MODULEID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODULEID;
#[doc = "`read()` method returns [moduleid::R](moduleid::R) reader structure"]
impl crate::Readable for MODULEID {}
#[doc = "Module ID Register"]
pub mod moduleid;
#[doc = "I/O Routing Pin Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routepen](routepen) module"]
pub type ROUTEPEN = crate::Reg<u32, _ROUTEPEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTEPEN;
#[doc = "`read()` method returns [routepen::R](routepen::R) reader structure"]
impl crate::Readable for ROUTEPEN {}
#[doc = "`write(|w| ..)` method takes [routepen::W](routepen::W) writer structure"]
impl crate::Writable for ROUTEPEN {}
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "I/O Route Location Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc0](routeloc0) module"]
pub type ROUTELOC0 = crate::Reg<u32, _ROUTELOC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTELOC0;
#[doc = "`read()` method returns [routeloc0::R](routeloc0::R) reader structure"]
impl crate::Readable for ROUTELOC0 {}
#[doc = "`write(|w| ..)` method takes [routeloc0::W](routeloc0::W) writer structure"]
impl crate::Writable for ROUTELOC0 {}
#[doc = "I/O Route Location Register 0"]
pub mod routeloc0;
