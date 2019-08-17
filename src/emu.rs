#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x0c - Memory Control Register"]
    pub ram0ctrl: RAM0CTRL,
    #[doc = "0x10 - Command Register"]
    pub cmd: CMD,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - EM4 Control Register"]
    pub em4ctrl: EM4CTRL,
    #[doc = "0x1c - Temperature Limits for Interrupt Generation"]
    pub templimits: TEMPLIMITS,
    #[doc = "0x20 - Value of Last Temperature Measurement"]
    pub temp: TEMP,
    #[doc = "0x24 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x28 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x2c - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x30 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x34 - Regulator and Supply Lock Register"]
    pub pwrlock: PWRLOCK,
    _reserved13: [u8; 4usize],
    #[doc = "0x3c - Power Control Register"]
    pub pwrctrl: PWRCTRL,
    #[doc = "0x40 - DCDC Control"]
    pub dcdcctrl: DCDCCTRL,
    _reserved15: [u8; 8usize],
    #[doc = "0x4c - DCDC Miscellaneous Control Register"]
    pub dcdcmiscctrl: DCDCMISCCTRL,
    #[doc = "0x50 - DCDC Power Train NFET Zero Current Detector Control Register"]
    pub dcdczdetctrl: DCDCZDETCTRL,
    #[doc = "0x54 - DCDC Power Train PFET Current Limiter Control Register"]
    pub dcdcclimctrl: DCDCCLIMCTRL,
    #[doc = "0x58 - DCDC Low Noise Compensator Control Register"]
    pub dcdclncompctrl: DCDCLNCOMPCTRL,
    #[doc = "0x5c - DCDC Low Noise Voltage Register"]
    pub dcdclnvctrl: DCDCLNVCTRL,
    _reserved20: [u8; 4usize],
    #[doc = "0x64 - DCDC Low Power Voltage Register"]
    pub dcdclpvctrl: DCDCLPVCTRL,
    _reserved21: [u8; 4usize],
    #[doc = "0x6c - DCDC Low Power Control Register"]
    pub dcdclpctrl: DCDCLPCTRL,
    #[doc = "0x70 - DCDC Low Noise Controller Frequency Control"]
    pub dcdclnfreqctrl: DCDCLNFREQCTRL,
    _reserved23: [u8; 4usize],
    #[doc = "0x78 - DCDC Read Status Register"]
    pub dcdcsync: DCDCSYNC,
    _reserved24: [u8; 20usize],
    #[doc = "0x90 - VMON AVDD Channel Control"]
    pub vmonavddctrl: VMONAVDDCTRL,
    #[doc = "0x94 - Alternate VMON AVDD Channel Control"]
    pub vmonaltavddctrl: VMONALTAVDDCTRL,
    #[doc = "0x98 - VMON DVDD Channel Control"]
    pub vmondvddctrl: VMONDVDDCTRL,
    #[doc = "0x9c - VMON IOVDD0 Channel Control"]
    pub vmonio0ctrl: VMONIO0CTRL,
    #[doc = "0xa0 - VMON IOVDD1 Channel Control"]
    pub vmonio1ctrl: VMONIO1CTRL,
    #[doc = "0xa4 - VMON BUVDD Channel Control"]
    pub vmonbuvddctrl: VMONBUVDDCTRL,
    _reserved30: [u8; 12usize],
    #[doc = "0xb4 - Memory Control Register"]
    pub ram1ctrl: RAM1CTRL,
    #[doc = "0xb8 - Memory Control Register"]
    pub ram2ctrl: RAM2CTRL,
    #[doc = "0xbc - Backup Power Configuration Register"]
    pub buctrl: BUCTRL,
    _reserved33: [u8; 8usize],
    #[doc = "0xc8 - 5V Regulator Control"]
    pub r5vctrl: R5VCTRL,
    #[doc = "0xcc - 5V Regulator Control"]
    pub r5vadcctrl: R5VADCCTRL,
    #[doc = "0xd0 - 5V Regulator Voltage Select"]
    pub r5voutlevel: R5VOUTLEVEL,
    _reserved36: [u8; 8usize],
    #[doc = "0xdc - 5V Detector Enables"]
    pub r5vdetctrl: R5VDETCTRL,
    _reserved37: [u8; 12usize],
    #[doc = "0xec - Configuration Bits for Low Power Mode to Be Applied During EM01, This Field is Only Relevant If LP Mode is Used in EM01"]
    pub dcdclpem01cfg: DCDCLPEM01CFG,
    #[doc = "0xf0 - 5V Detector Status Register"]
    pub r5vstatus: R5VSTATUS,
    _reserved39: [u8; 4usize],
    #[doc = "0xf8 - 5V Read Status Register"]
    pub r5vsync: R5VSYNC,
    _reserved40: [u8; 4usize],
    #[doc = "0x100 - Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral"]
    pub em23pernoretaincmd: EM23PERNORETAINCMD,
    #[doc = "0x104 - Status Indicating If Peripherals Were Powered Down in EM23, Subsequently Locking Access to It"]
    pub em23pernoretainstatus: EM23PERNORETAINSTATUS,
    #[doc = "0x108 - When Set Corresponding Peripherals May Get Powered Down in EM23"]
    pub em23pernoretainctrl: EM23PERNORETAINCTRL,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "Configuration Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "Memory Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ram0ctrl](ram0ctrl) module"]
pub type RAM0CTRL = crate::Reg<u32, _RAM0CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM0CTRL;
#[doc = "`read()` method returns [ram0ctrl::R](ram0ctrl::R) reader structure"]
impl crate::Readable for RAM0CTRL {}
#[doc = "`write(|w| ..)` method takes [ram0ctrl::W](ram0ctrl::W) writer structure"]
impl crate::Writable for RAM0CTRL {}
#[doc = "Memory Control Register"]
pub mod ram0ctrl;
#[doc = "Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "EM4 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [em4ctrl](em4ctrl) module"]
pub type EM4CTRL = crate::Reg<u32, _EM4CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EM4CTRL;
#[doc = "`read()` method returns [em4ctrl::R](em4ctrl::R) reader structure"]
impl crate::Readable for EM4CTRL {}
#[doc = "`write(|w| ..)` method takes [em4ctrl::W](em4ctrl::W) writer structure"]
impl crate::Writable for EM4CTRL {}
#[doc = "EM4 Control Register"]
pub mod em4ctrl;
#[doc = "Temperature Limits for Interrupt Generation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [templimits](templimits) module"]
pub type TEMPLIMITS = crate::Reg<u32, _TEMPLIMITS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPLIMITS;
#[doc = "`read()` method returns [templimits::R](templimits::R) reader structure"]
impl crate::Readable for TEMPLIMITS {}
#[doc = "`write(|w| ..)` method takes [templimits::W](templimits::W) writer structure"]
impl crate::Writable for TEMPLIMITS {}
#[doc = "Temperature Limits for Interrupt Generation"]
pub mod templimits;
#[doc = "Value of Last Temperature Measurement\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [temp](temp) module"]
pub type TEMP = crate::Reg<u32, _TEMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMP;
#[doc = "`read()` method returns [temp::R](temp::R) reader structure"]
impl crate::Readable for TEMP {}
#[doc = "Value of Last Temperature Measurement"]
pub mod temp;
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [if_](if_) module"]
pub type IF = crate::Reg<u32, _IF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF;
#[doc = "`read()` method returns [if_::R](if_::R) reader structure"]
impl crate::Readable for IF {}
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ifs](ifs) module"]
pub type IFS = crate::Reg<u32, _IFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS;
#[doc = "`write(|w| ..)` method takes [ifs::W](ifs::W) writer structure"]
impl crate::Writable for IFS {}
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ifc](ifc) module"]
pub type IFC = crate::Reg<u32, _IFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFC;
#[doc = "`write(|w| ..)` method takes [ifc::W](ifc::W) writer structure"]
impl crate::Writable for IFC {}
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ien](ien) module"]
pub type IEN = crate::Reg<u32, _IEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEN;
#[doc = "`read()` method returns [ien::R](ien::R) reader structure"]
impl crate::Readable for IEN {}
#[doc = "`write(|w| ..)` method takes [ien::W](ien::W) writer structure"]
impl crate::Writable for IEN {}
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "Regulator and Supply Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwrlock](pwrlock) module"]
pub type PWRLOCK = crate::Reg<u32, _PWRLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRLOCK;
#[doc = "`read()` method returns [pwrlock::R](pwrlock::R) reader structure"]
impl crate::Readable for PWRLOCK {}
#[doc = "`write(|w| ..)` method takes [pwrlock::W](pwrlock::W) writer structure"]
impl crate::Writable for PWRLOCK {}
#[doc = "Regulator and Supply Lock Register"]
pub mod pwrlock;
#[doc = "Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwrctrl](pwrctrl) module"]
pub type PWRCTRL = crate::Reg<u32, _PWRCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCTRL;
#[doc = "`read()` method returns [pwrctrl::R](pwrctrl::R) reader structure"]
impl crate::Readable for PWRCTRL {}
#[doc = "`write(|w| ..)` method takes [pwrctrl::W](pwrctrl::W) writer structure"]
impl crate::Writable for PWRCTRL {}
#[doc = "Power Control Register"]
pub mod pwrctrl;
#[doc = "DCDC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdcctrl](dcdcctrl) module"]
pub type DCDCCTRL = crate::Reg<u32, _DCDCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCCTRL;
#[doc = "`read()` method returns [dcdcctrl::R](dcdcctrl::R) reader structure"]
impl crate::Readable for DCDCCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdcctrl::W](dcdcctrl::W) writer structure"]
impl crate::Writable for DCDCCTRL {}
#[doc = "DCDC Control"]
pub mod dcdcctrl;
#[doc = "DCDC Miscellaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdcmiscctrl](dcdcmiscctrl) module"]
pub type DCDCMISCCTRL = crate::Reg<u32, _DCDCMISCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCMISCCTRL;
#[doc = "`read()` method returns [dcdcmiscctrl::R](dcdcmiscctrl::R) reader structure"]
impl crate::Readable for DCDCMISCCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdcmiscctrl::W](dcdcmiscctrl::W) writer structure"]
impl crate::Writable for DCDCMISCCTRL {}
#[doc = "DCDC Miscellaneous Control Register"]
pub mod dcdcmiscctrl;
#[doc = "DCDC Power Train NFET Zero Current Detector Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdczdetctrl](dcdczdetctrl) module"]
pub type DCDCZDETCTRL = crate::Reg<u32, _DCDCZDETCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCZDETCTRL;
#[doc = "`read()` method returns [dcdczdetctrl::R](dcdczdetctrl::R) reader structure"]
impl crate::Readable for DCDCZDETCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdczdetctrl::W](dcdczdetctrl::W) writer structure"]
impl crate::Writable for DCDCZDETCTRL {}
#[doc = "DCDC Power Train NFET Zero Current Detector Control Register"]
pub mod dcdczdetctrl;
#[doc = "DCDC Power Train PFET Current Limiter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdcclimctrl](dcdcclimctrl) module"]
pub type DCDCCLIMCTRL = crate::Reg<u32, _DCDCCLIMCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCCLIMCTRL;
#[doc = "`read()` method returns [dcdcclimctrl::R](dcdcclimctrl::R) reader structure"]
impl crate::Readable for DCDCCLIMCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdcclimctrl::W](dcdcclimctrl::W) writer structure"]
impl crate::Writable for DCDCCLIMCTRL {}
#[doc = "DCDC Power Train PFET Current Limiter Control Register"]
pub mod dcdcclimctrl;
#[doc = "DCDC Low Noise Compensator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdclncompctrl](dcdclncompctrl) module"]
pub type DCDCLNCOMPCTRL = crate::Reg<u32, _DCDCLNCOMPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCLNCOMPCTRL;
#[doc = "`read()` method returns [dcdclncompctrl::R](dcdclncompctrl::R) reader structure"]
impl crate::Readable for DCDCLNCOMPCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdclncompctrl::W](dcdclncompctrl::W) writer structure"]
impl crate::Writable for DCDCLNCOMPCTRL {}
#[doc = "DCDC Low Noise Compensator Control Register"]
pub mod dcdclncompctrl;
#[doc = "DCDC Low Noise Voltage Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdclnvctrl](dcdclnvctrl) module"]
pub type DCDCLNVCTRL = crate::Reg<u32, _DCDCLNVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCLNVCTRL;
#[doc = "`read()` method returns [dcdclnvctrl::R](dcdclnvctrl::R) reader structure"]
impl crate::Readable for DCDCLNVCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdclnvctrl::W](dcdclnvctrl::W) writer structure"]
impl crate::Writable for DCDCLNVCTRL {}
#[doc = "DCDC Low Noise Voltage Register"]
pub mod dcdclnvctrl;
#[doc = "DCDC Low Power Voltage Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdclpvctrl](dcdclpvctrl) module"]
pub type DCDCLPVCTRL = crate::Reg<u32, _DCDCLPVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCLPVCTRL;
#[doc = "`read()` method returns [dcdclpvctrl::R](dcdclpvctrl::R) reader structure"]
impl crate::Readable for DCDCLPVCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdclpvctrl::W](dcdclpvctrl::W) writer structure"]
impl crate::Writable for DCDCLPVCTRL {}
#[doc = "DCDC Low Power Voltage Register"]
pub mod dcdclpvctrl;
#[doc = "DCDC Low Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdclpctrl](dcdclpctrl) module"]
pub type DCDCLPCTRL = crate::Reg<u32, _DCDCLPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCLPCTRL;
#[doc = "`read()` method returns [dcdclpctrl::R](dcdclpctrl::R) reader structure"]
impl crate::Readable for DCDCLPCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdclpctrl::W](dcdclpctrl::W) writer structure"]
impl crate::Writable for DCDCLPCTRL {}
#[doc = "DCDC Low Power Control Register"]
pub mod dcdclpctrl;
#[doc = "DCDC Low Noise Controller Frequency Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdclnfreqctrl](dcdclnfreqctrl) module"]
pub type DCDCLNFREQCTRL = crate::Reg<u32, _DCDCLNFREQCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCLNFREQCTRL;
#[doc = "`read()` method returns [dcdclnfreqctrl::R](dcdclnfreqctrl::R) reader structure"]
impl crate::Readable for DCDCLNFREQCTRL {}
#[doc = "`write(|w| ..)` method takes [dcdclnfreqctrl::W](dcdclnfreqctrl::W) writer structure"]
impl crate::Writable for DCDCLNFREQCTRL {}
#[doc = "DCDC Low Noise Controller Frequency Control"]
pub mod dcdclnfreqctrl;
#[doc = "DCDC Read Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdcsync](dcdcsync) module"]
pub type DCDCSYNC = crate::Reg<u32, _DCDCSYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCSYNC;
#[doc = "`read()` method returns [dcdcsync::R](dcdcsync::R) reader structure"]
impl crate::Readable for DCDCSYNC {}
#[doc = "DCDC Read Status Register"]
pub mod dcdcsync;
#[doc = "VMON AVDD Channel Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vmonavddctrl](vmonavddctrl) module"]
pub type VMONAVDDCTRL = crate::Reg<u32, _VMONAVDDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VMONAVDDCTRL;
#[doc = "`read()` method returns [vmonavddctrl::R](vmonavddctrl::R) reader structure"]
impl crate::Readable for VMONAVDDCTRL {}
#[doc = "`write(|w| ..)` method takes [vmonavddctrl::W](vmonavddctrl::W) writer structure"]
impl crate::Writable for VMONAVDDCTRL {}
#[doc = "VMON AVDD Channel Control"]
pub mod vmonavddctrl;
#[doc = "Alternate VMON AVDD Channel Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vmonaltavddctrl](vmonaltavddctrl) module"]
pub type VMONALTAVDDCTRL = crate::Reg<u32, _VMONALTAVDDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VMONALTAVDDCTRL;
#[doc = "`read()` method returns [vmonaltavddctrl::R](vmonaltavddctrl::R) reader structure"]
impl crate::Readable for VMONALTAVDDCTRL {}
#[doc = "`write(|w| ..)` method takes [vmonaltavddctrl::W](vmonaltavddctrl::W) writer structure"]
impl crate::Writable for VMONALTAVDDCTRL {}
#[doc = "Alternate VMON AVDD Channel Control"]
pub mod vmonaltavddctrl;
#[doc = "VMON DVDD Channel Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vmondvddctrl](vmondvddctrl) module"]
pub type VMONDVDDCTRL = crate::Reg<u32, _VMONDVDDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VMONDVDDCTRL;
#[doc = "`read()` method returns [vmondvddctrl::R](vmondvddctrl::R) reader structure"]
impl crate::Readable for VMONDVDDCTRL {}
#[doc = "`write(|w| ..)` method takes [vmondvddctrl::W](vmondvddctrl::W) writer structure"]
impl crate::Writable for VMONDVDDCTRL {}
#[doc = "VMON DVDD Channel Control"]
pub mod vmondvddctrl;
#[doc = "VMON IOVDD0 Channel Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vmonio0ctrl](vmonio0ctrl) module"]
pub type VMONIO0CTRL = crate::Reg<u32, _VMONIO0CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VMONIO0CTRL;
#[doc = "`read()` method returns [vmonio0ctrl::R](vmonio0ctrl::R) reader structure"]
impl crate::Readable for VMONIO0CTRL {}
#[doc = "`write(|w| ..)` method takes [vmonio0ctrl::W](vmonio0ctrl::W) writer structure"]
impl crate::Writable for VMONIO0CTRL {}
#[doc = "VMON IOVDD0 Channel Control"]
pub mod vmonio0ctrl;
#[doc = "VMON IOVDD1 Channel Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vmonio1ctrl](vmonio1ctrl) module"]
pub type VMONIO1CTRL = crate::Reg<u32, _VMONIO1CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VMONIO1CTRL;
#[doc = "`read()` method returns [vmonio1ctrl::R](vmonio1ctrl::R) reader structure"]
impl crate::Readable for VMONIO1CTRL {}
#[doc = "`write(|w| ..)` method takes [vmonio1ctrl::W](vmonio1ctrl::W) writer structure"]
impl crate::Writable for VMONIO1CTRL {}
#[doc = "VMON IOVDD1 Channel Control"]
pub mod vmonio1ctrl;
#[doc = "VMON BUVDD Channel Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vmonbuvddctrl](vmonbuvddctrl) module"]
pub type VMONBUVDDCTRL = crate::Reg<u32, _VMONBUVDDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VMONBUVDDCTRL;
#[doc = "`read()` method returns [vmonbuvddctrl::R](vmonbuvddctrl::R) reader structure"]
impl crate::Readable for VMONBUVDDCTRL {}
#[doc = "`write(|w| ..)` method takes [vmonbuvddctrl::W](vmonbuvddctrl::W) writer structure"]
impl crate::Writable for VMONBUVDDCTRL {}
#[doc = "VMON BUVDD Channel Control"]
pub mod vmonbuvddctrl;
#[doc = "Memory Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ram1ctrl](ram1ctrl) module"]
pub type RAM1CTRL = crate::Reg<u32, _RAM1CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM1CTRL;
#[doc = "`read()` method returns [ram1ctrl::R](ram1ctrl::R) reader structure"]
impl crate::Readable for RAM1CTRL {}
#[doc = "`write(|w| ..)` method takes [ram1ctrl::W](ram1ctrl::W) writer structure"]
impl crate::Writable for RAM1CTRL {}
#[doc = "Memory Control Register"]
pub mod ram1ctrl;
#[doc = "Memory Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ram2ctrl](ram2ctrl) module"]
pub type RAM2CTRL = crate::Reg<u32, _RAM2CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM2CTRL;
#[doc = "`read()` method returns [ram2ctrl::R](ram2ctrl::R) reader structure"]
impl crate::Readable for RAM2CTRL {}
#[doc = "`write(|w| ..)` method takes [ram2ctrl::W](ram2ctrl::W) writer structure"]
impl crate::Writable for RAM2CTRL {}
#[doc = "Memory Control Register"]
pub mod ram2ctrl;
#[doc = "Backup Power Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [buctrl](buctrl) module"]
pub type BUCTRL = crate::Reg<u32, _BUCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUCTRL;
#[doc = "`read()` method returns [buctrl::R](buctrl::R) reader structure"]
impl crate::Readable for BUCTRL {}
#[doc = "`write(|w| ..)` method takes [buctrl::W](buctrl::W) writer structure"]
impl crate::Writable for BUCTRL {}
#[doc = "Backup Power Configuration Register"]
pub mod buctrl;
#[doc = "5V Regulator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [r5vctrl](r5vctrl) module"]
pub type R5VCTRL = crate::Reg<u32, _R5VCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R5VCTRL;
#[doc = "`read()` method returns [r5vctrl::R](r5vctrl::R) reader structure"]
impl crate::Readable for R5VCTRL {}
#[doc = "`write(|w| ..)` method takes [r5vctrl::W](r5vctrl::W) writer structure"]
impl crate::Writable for R5VCTRL {}
#[doc = "5V Regulator Control"]
pub mod r5vctrl;
#[doc = "5V Regulator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [r5vadcctrl](r5vadcctrl) module"]
pub type R5VADCCTRL = crate::Reg<u32, _R5VADCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R5VADCCTRL;
#[doc = "`read()` method returns [r5vadcctrl::R](r5vadcctrl::R) reader structure"]
impl crate::Readable for R5VADCCTRL {}
#[doc = "`write(|w| ..)` method takes [r5vadcctrl::W](r5vadcctrl::W) writer structure"]
impl crate::Writable for R5VADCCTRL {}
#[doc = "5V Regulator Control"]
pub mod r5vadcctrl;
#[doc = "5V Regulator Voltage Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [r5voutlevel](r5voutlevel) module"]
pub type R5VOUTLEVEL = crate::Reg<u32, _R5VOUTLEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R5VOUTLEVEL;
#[doc = "`read()` method returns [r5voutlevel::R](r5voutlevel::R) reader structure"]
impl crate::Readable for R5VOUTLEVEL {}
#[doc = "`write(|w| ..)` method takes [r5voutlevel::W](r5voutlevel::W) writer structure"]
impl crate::Writable for R5VOUTLEVEL {}
#[doc = "5V Regulator Voltage Select"]
pub mod r5voutlevel;
#[doc = "5V Detector Enables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [r5vdetctrl](r5vdetctrl) module"]
pub type R5VDETCTRL = crate::Reg<u32, _R5VDETCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R5VDETCTRL;
#[doc = "`read()` method returns [r5vdetctrl::R](r5vdetctrl::R) reader structure"]
impl crate::Readable for R5VDETCTRL {}
#[doc = "`write(|w| ..)` method takes [r5vdetctrl::W](r5vdetctrl::W) writer structure"]
impl crate::Writable for R5VDETCTRL {}
#[doc = "5V Detector Enables"]
pub mod r5vdetctrl;
#[doc = "Configuration Bits for Low Power Mode to Be Applied During EM01, This Field is Only Relevant If LP Mode is Used in EM01\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdclpem01cfg](dcdclpem01cfg) module"]
pub type DCDCLPEM01CFG = crate::Reg<u32, _DCDCLPEM01CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCLPEM01CFG;
#[doc = "`read()` method returns [dcdclpem01cfg::R](dcdclpem01cfg::R) reader structure"]
impl crate::Readable for DCDCLPEM01CFG {}
#[doc = "`write(|w| ..)` method takes [dcdclpem01cfg::W](dcdclpem01cfg::W) writer structure"]
impl crate::Writable for DCDCLPEM01CFG {}
#[doc = "Configuration Bits for Low Power Mode to Be Applied During EM01, This Field is Only Relevant If LP Mode is Used in EM01"]
pub mod dcdclpem01cfg;
#[doc = "5V Detector Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [r5vstatus](r5vstatus) module"]
pub type R5VSTATUS = crate::Reg<u32, _R5VSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R5VSTATUS;
#[doc = "`read()` method returns [r5vstatus::R](r5vstatus::R) reader structure"]
impl crate::Readable for R5VSTATUS {}
#[doc = "5V Detector Status Register"]
pub mod r5vstatus;
#[doc = "5V Read Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [r5vsync](r5vsync) module"]
pub type R5VSYNC = crate::Reg<u32, _R5VSYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R5VSYNC;
#[doc = "`read()` method returns [r5vsync::R](r5vsync::R) reader structure"]
impl crate::Readable for R5VSYNC {}
#[doc = "5V Read Status Register"]
pub mod r5vsync;
#[doc = "Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [em23pernoretaincmd](em23pernoretaincmd) module"]
pub type EM23PERNORETAINCMD = crate::Reg<u32, _EM23PERNORETAINCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EM23PERNORETAINCMD;
#[doc = "`write(|w| ..)` method takes [em23pernoretaincmd::W](em23pernoretaincmd::W) writer structure"]
impl crate::Writable for EM23PERNORETAINCMD {}
#[doc = "Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral"]
pub mod em23pernoretaincmd;
#[doc = "Status Indicating If Peripherals Were Powered Down in EM23, Subsequently Locking Access to It\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [em23pernoretainstatus](em23pernoretainstatus) module"]
pub type EM23PERNORETAINSTATUS = crate::Reg<u32, _EM23PERNORETAINSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EM23PERNORETAINSTATUS;
#[doc = "`read()` method returns [em23pernoretainstatus::R](em23pernoretainstatus::R) reader structure"]
impl crate::Readable for EM23PERNORETAINSTATUS {}
#[doc = "Status Indicating If Peripherals Were Powered Down in EM23, Subsequently Locking Access to It"]
pub mod em23pernoretainstatus;
#[doc = "When Set Corresponding Peripherals May Get Powered Down in EM23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [em23pernoretainctrl](em23pernoretainctrl) module"]
pub type EM23PERNORETAINCTRL = crate::Reg<u32, _EM23PERNORETAINCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EM23PERNORETAINCTRL;
#[doc = "`read()` method returns [em23pernoretainctrl::R](em23pernoretainctrl::R) reader structure"]
impl crate::Readable for EM23PERNORETAINCTRL {}
#[doc = "`write(|w| ..)` method takes [em23pernoretainctrl::W](em23pernoretainctrl::W) writer structure"]
impl crate::Writable for EM23PERNORETAINCTRL {}
#[doc = "When Set Corresponding Peripherals May Get Powered Down in EM23"]
pub mod em23pernoretainctrl;
