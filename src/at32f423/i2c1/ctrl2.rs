#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `SADDR` reader - Slave address"]
pub type SADDR_R = crate::FieldReader<u16>;
#[doc = "Field `SADDR` writer - Slave address"]
pub type SADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
#[doc = "Master data transmission direction\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: Master transmits data to slave"]
    Transmit = 0,
    #[doc = "1: Master receives data from slave"]
    Receive = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Master data transmission direction"]
pub type DIR_R = crate::BitReader<DIR_A>;
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::Transmit,
            true => DIR_A::Receive,
        }
    }
    #[doc = "Master transmits data to slave"]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == DIR_A::Transmit
    }
    #[doc = "Master receives data from slave"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == DIR_A::Receive
    }
}
#[doc = "Field `DIR` writer - Master data transmission direction"]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG, DIR_A>;
impl<'a, REG> DIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master transmits data to slave"]
    #[inline(always)]
    pub fn transmit(self) -> &'a mut crate::W<REG> {
        self.variant(DIR_A::Transmit)
    }
    #[doc = "Master receives data from slave"]
    #[inline(always)]
    pub fn receive(self) -> &'a mut crate::W<REG> {
        self.variant(DIR_A::Receive)
    }
}
#[doc = "Host send 10-bit address mode enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR10_A {
    #[doc = "0: 7-bit address mode"]
    Mode7bit = 0,
    #[doc = "1: 10-bit address mode"]
    Mode10bit = 1,
}
impl From<ADDR10_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDR10` reader - Host send 10-bit address mode enable"]
pub type ADDR10_R = crate::BitReader<ADDR10_A>;
impl ADDR10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDR10_A {
        match self.bits {
            false => ADDR10_A::Mode7bit,
            true => ADDR10_A::Mode10bit,
        }
    }
    #[doc = "7-bit address mode"]
    #[inline(always)]
    pub fn is_mode7bit(&self) -> bool {
        *self == ADDR10_A::Mode7bit
    }
    #[doc = "10-bit address mode"]
    #[inline(always)]
    pub fn is_mode10bit(&self) -> bool {
        *self == ADDR10_A::Mode10bit
    }
}
#[doc = "Field `ADDR10` writer - Host send 10-bit address mode enable"]
pub type ADDR10_W<'a, REG> = crate::BitWriter<'a, REG, ADDR10_A>;
impl<'a, REG> ADDR10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "7-bit address mode"]
    #[inline(always)]
    pub fn mode7bit(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR10_A::Mode7bit)
    }
    #[doc = "10-bit address mode"]
    #[inline(always)]
    pub fn mode10bit(self) -> &'a mut crate::W<REG> {
        self.variant(ADDR10_A::Mode10bit)
    }
}
#[doc = "10-bit address header read enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READH10_A {
    #[doc = "0: 10-bit address header read disabled"]
    Disabled = 0,
    #[doc = "1: 10-bit address header read enabled"]
    Enabled = 1,
}
impl From<READH10_A> for bool {
    #[inline(always)]
    fn from(variant: READH10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READH10` reader - 10-bit address header read enable"]
pub type READH10_R = crate::BitReader<READH10_A>;
impl READH10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> READH10_A {
        match self.bits {
            false => READH10_A::Disabled,
            true => READH10_A::Enabled,
        }
    }
    #[doc = "10-bit address header read disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READH10_A::Disabled
    }
    #[doc = "10-bit address header read enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READH10_A::Enabled
    }
}
#[doc = "Field `READH10` writer - 10-bit address header read enable"]
pub type READH10_W<'a, REG> = crate::BitWriter<'a, REG, READH10_A>;
impl<'a, REG> READH10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "10-bit address header read disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(READH10_A::Disabled)
    }
    #[doc = "10-bit address header read enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(READH10_A::Enabled)
    }
}
#[doc = "Generate start condition\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GENSTARTW_A {
    #[doc = "1: Generate start condition in master mode"]
    Start = 1,
}
impl From<GENSTARTW_A> for bool {
    #[inline(always)]
    fn from(variant: GENSTARTW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GENSTART` reader - Generate start condition"]
pub type GENSTART_R = crate::BitReader<GENSTARTW_A>;
impl GENSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GENSTARTW_A> {
        match self.bits {
            true => Some(GENSTARTW_A::Start),
            _ => None,
        }
    }
    #[doc = "Generate start condition in master mode"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == GENSTARTW_A::Start
    }
}
#[doc = "Field `GENSTART` writer - Generate start condition"]
pub type GENSTART_W<'a, REG> = crate::BitWriter1S<'a, REG, GENSTARTW_A>;
impl<'a, REG> GENSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate start condition in master mode"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(GENSTARTW_A::Start)
    }
}
#[doc = "Generate stop condition\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GENSTOPW_A {
    #[doc = "1: Generate stop condition in master mode"]
    Stop = 1,
}
impl From<GENSTOPW_A> for bool {
    #[inline(always)]
    fn from(variant: GENSTOPW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GENSTOP` reader - Generate stop condition"]
pub type GENSTOP_R = crate::BitReader<GENSTOPW_A>;
impl GENSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GENSTOPW_A> {
        match self.bits {
            true => Some(GENSTOPW_A::Stop),
            _ => None,
        }
    }
    #[doc = "Generate stop condition in master mode"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == GENSTOPW_A::Stop
    }
}
#[doc = "Field `GENSTOP` writer - Generate stop condition"]
pub type GENSTOP_W<'a, REG> = crate::BitWriter1S<'a, REG, GENSTOPW_A>;
impl<'a, REG> GENSTOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate stop condition in master mode"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(GENSTOPW_A::Stop)
    }
}
#[doc = "Not acknowledge enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKEN_A {
    #[doc = "0: NACK generation disabled"]
    Disabled = 0,
    #[doc = "1: NACK generation enabled"]
    Enabled = 1,
}
impl From<NACKEN_A> for bool {
    #[inline(always)]
    fn from(variant: NACKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKEN` reader - Not acknowledge enable"]
pub type NACKEN_R = crate::BitReader<NACKEN_A>;
impl NACKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NACKEN_A {
        match self.bits {
            false => NACKEN_A::Disabled,
            true => NACKEN_A::Enabled,
        }
    }
    #[doc = "NACK generation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NACKEN_A::Disabled
    }
    #[doc = "NACK generation enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NACKEN_A::Enabled
    }
}
#[doc = "Field `NACKEN` writer - Not acknowledge enable"]
pub type NACKEN_W<'a, REG> = crate::BitWriter<'a, REG, NACKEN_A>;
impl<'a, REG> NACKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NACK generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(NACKEN_A::Disabled)
    }
    #[doc = "NACK generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(NACKEN_A::Enabled)
    }
}
#[doc = "Field `CNT` reader - Transmit data counter"]
pub type CNT_R = crate::FieldReader;
#[doc = "Field `CNT` writer - Transmit data counter"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
#[doc = "Send data reload mode enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RLDEN_A {
    #[doc = "0: Send data reload mode disabled"]
    Disabled = 0,
    #[doc = "1: Send data reload mode enabled"]
    Enabled = 1,
}
impl From<RLDEN_A> for bool {
    #[inline(always)]
    fn from(variant: RLDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLDEN` reader - Send data reload mode enable"]
pub type RLDEN_R = crate::BitReader<RLDEN_A>;
impl RLDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RLDEN_A {
        match self.bits {
            false => RLDEN_A::Disabled,
            true => RLDEN_A::Enabled,
        }
    }
    #[doc = "Send data reload mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RLDEN_A::Disabled
    }
    #[doc = "Send data reload mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RLDEN_A::Enabled
    }
}
#[doc = "Field `RLDEN` writer - Send data reload mode enable"]
pub type RLDEN_W<'a, REG> = crate::BitWriter<'a, REG, RLDEN_A>;
impl<'a, REG> RLDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Send data reload mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RLDEN_A::Disabled)
    }
    #[doc = "Send data reload mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RLDEN_A::Enabled)
    }
}
#[doc = "Automatically send stop condition enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASTOPEN_A {
    #[doc = "0: Automatic end mode disabled, software sends STOP condition"]
    Disabled = 0,
    #[doc = "1: Automatic end mode enabled. automatically send STOP condition"]
    Enabled = 1,
}
impl From<ASTOPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ASTOPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASTOPEN` reader - Automatically send stop condition enable"]
pub type ASTOPEN_R = crate::BitReader<ASTOPEN_A>;
impl ASTOPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASTOPEN_A {
        match self.bits {
            false => ASTOPEN_A::Disabled,
            true => ASTOPEN_A::Enabled,
        }
    }
    #[doc = "Automatic end mode disabled, software sends STOP condition"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ASTOPEN_A::Disabled
    }
    #[doc = "Automatic end mode enabled. automatically send STOP condition"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ASTOPEN_A::Enabled
    }
}
#[doc = "Field `ASTOPEN` writer - Automatically send stop condition enable"]
pub type ASTOPEN_W<'a, REG> = crate::BitWriter<'a, REG, ASTOPEN_A>;
impl<'a, REG> ASTOPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic end mode disabled, software sends STOP condition"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ASTOPEN_A::Disabled)
    }
    #[doc = "Automatic end mode enabled. automatically send STOP condition"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ASTOPEN_A::Enabled)
    }
}
#[doc = "Request PEC transmission enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pectenr {
    #[doc = "0: PEC transmission is disabled"]
    Disabled = 0,
    #[doc = "1: PEC transmission is enabled"]
    Enabled = 1,
}
impl From<Pectenr> for bool {
    #[inline(always)]
    fn from(variant: Pectenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECTEN` reader - Request PEC transmission enable"]
pub type PECTEN_R = crate::BitReader<Pectenr>;
impl PECTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pectenr {
        match self.bits {
            false => Pectenr::Disabled,
            true => Pectenr::Enabled,
        }
    }
    #[doc = "PEC transmission is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pectenr::Disabled
    }
    #[doc = "PEC transmission is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pectenr::Enabled
    }
}
#[doc = "Request PEC transmission enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PectenwWO {
    #[doc = "0: Disable PEC transmission"]
    Disable = 0,
    #[doc = "1: Enable PEC transmission"]
    Enable = 1,
}
impl From<PectenwWO> for bool {
    #[inline(always)]
    fn from(variant: PectenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECTEN` writer - Request PEC transmission enable"]
pub type PECTEN_W<'a, REG> = crate::BitWriter<'a, REG, PectenwWO>;
impl<'a, REG> PECTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable PEC transmission"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PectenwWO::Disable)
    }
    #[doc = "Enable PEC transmission"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PectenwWO::Enable)
    }
}
impl R {
    #[doc = "Bits 0:9 - Slave address"]
    #[inline(always)]
    pub fn saddr(&self) -> SADDR_R {
        SADDR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Master data transmission direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Host send 10-bit address mode enable"]
    #[inline(always)]
    pub fn addr10(&self) -> ADDR10_R {
        ADDR10_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 10-bit address header read enable"]
    #[inline(always)]
    pub fn readh10(&self) -> READH10_R {
        READH10_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Generate start condition"]
    #[inline(always)]
    pub fn genstart(&self) -> GENSTART_R {
        GENSTART_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Generate stop condition"]
    #[inline(always)]
    pub fn genstop(&self) -> GENSTOP_R {
        GENSTOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Not acknowledge enable"]
    #[inline(always)]
    pub fn nacken(&self) -> NACKEN_R {
        NACKEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Transmit data counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Send data reload mode enable"]
    #[inline(always)]
    pub fn rlden(&self) -> RLDEN_R {
        RLDEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Automatically send stop condition enable"]
    #[inline(always)]
    pub fn astopen(&self) -> ASTOPEN_R {
        ASTOPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Request PEC transmission enable"]
    #[inline(always)]
    pub fn pecten(&self) -> PECTEN_R {
        PECTEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("pecten", &self.pecten())
            .field("astopen", &self.astopen())
            .field("rlden", &self.rlden())
            .field("cnt", &self.cnt())
            .field("nacken", &self.nacken())
            .field("genstop", &self.genstop())
            .field("genstart", &self.genstart())
            .field("readh10", &self.readh10())
            .field("addr10", &self.addr10())
            .field("dir", &self.dir())
            .field("saddr", &self.saddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave address"]
    #[inline(always)]
    pub fn saddr(&mut self) -> SADDR_W<'_, CTRL2_SPEC> {
        SADDR_W::new(self, 0)
    }
    #[doc = "Bit 10 - Master data transmission direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<'_, CTRL2_SPEC> {
        DIR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Host send 10-bit address mode enable"]
    #[inline(always)]
    pub fn addr10(&mut self) -> ADDR10_W<'_, CTRL2_SPEC> {
        ADDR10_W::new(self, 11)
    }
    #[doc = "Bit 12 - 10-bit address header read enable"]
    #[inline(always)]
    pub fn readh10(&mut self) -> READH10_W<'_, CTRL2_SPEC> {
        READH10_W::new(self, 12)
    }
    #[doc = "Bit 13 - Generate start condition"]
    #[inline(always)]
    pub fn genstart(&mut self) -> GENSTART_W<'_, CTRL2_SPEC> {
        GENSTART_W::new(self, 13)
    }
    #[doc = "Bit 14 - Generate stop condition"]
    #[inline(always)]
    pub fn genstop(&mut self) -> GENSTOP_W<'_, CTRL2_SPEC> {
        GENSTOP_W::new(self, 14)
    }
    #[doc = "Bit 15 - Not acknowledge enable"]
    #[inline(always)]
    pub fn nacken(&mut self) -> NACKEN_W<'_, CTRL2_SPEC> {
        NACKEN_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - Transmit data counter"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<'_, CTRL2_SPEC> {
        CNT_W::new(self, 16)
    }
    #[doc = "Bit 24 - Send data reload mode enable"]
    #[inline(always)]
    pub fn rlden(&mut self) -> RLDEN_W<'_, CTRL2_SPEC> {
        RLDEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Automatically send stop condition enable"]
    #[inline(always)]
    pub fn astopen(&mut self) -> ASTOPEN_W<'_, CTRL2_SPEC> {
        ASTOPEN_W::new(self, 25)
    }
    #[doc = "Bit 26 - Request PEC transmission enable"]
    #[inline(always)]
    pub fn pecten(&mut self) -> PECTEN_W<'_, CTRL2_SPEC> {
        PECTEN_W::new(self, 26)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x6000;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {}
