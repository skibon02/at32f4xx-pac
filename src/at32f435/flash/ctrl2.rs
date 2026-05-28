#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Flash program\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fprgmr {
    #[doc = "0: Flash program not started"]
    NotStarted = 0,
    #[doc = "1: Flash program is in progress"]
    Started = 1,
}
impl From<Fprgmr> for bool {
    #[inline(always)]
    fn from(variant: Fprgmr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRGM` reader - Flash program"]
pub type FPRGM_R = crate::BitReader<Fprgmr>;
impl FPRGM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fprgmr {
        match self.bits {
            false => Fprgmr::NotStarted,
            true => Fprgmr::Started,
        }
    }
    #[doc = "Flash program not started"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == Fprgmr::NotStarted
    }
    #[doc = "Flash program is in progress"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == Fprgmr::Started
    }
}
#[doc = "Flash program\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FprgmwWO {
    #[doc = "0: Flash program not started"]
    NoStart = 0,
    #[doc = "1: Start of flash program operation"]
    Start = 1,
}
impl From<FprgmwWO> for bool {
    #[inline(always)]
    fn from(variant: FprgmwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPRGM` writer - Flash program"]
pub type FPRGM_W<'a, REG> = crate::BitWriter<'a, REG, FprgmwWO>;
impl<'a, REG> FPRGM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash program not started"]
    #[inline(always)]
    pub fn no_start(self) -> &'a mut crate::W<REG> {
        self.variant(FprgmwWO::NoStart)
    }
    #[doc = "Start of flash program operation"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(FprgmwWO::Start)
    }
}
#[doc = "Sector erase\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECERS_A {
    #[doc = "0: Not sector erase operation"]
    NotSectorErase = 0,
    #[doc = "1: Erase operation is sector erase operation"]
    SectorErase = 1,
}
impl From<SECERS_A> for bool {
    #[inline(always)]
    fn from(variant: SECERS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECERS` reader - Sector erase"]
pub type SECERS_R = crate::BitReader<SECERS_A>;
impl SECERS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SECERS_A {
        match self.bits {
            false => SECERS_A::NotSectorErase,
            true => SECERS_A::SectorErase,
        }
    }
    #[doc = "Not sector erase operation"]
    #[inline(always)]
    pub fn is_not_sector_erase(&self) -> bool {
        *self == SECERS_A::NotSectorErase
    }
    #[doc = "Erase operation is sector erase operation"]
    #[inline(always)]
    pub fn is_sector_erase(&self) -> bool {
        *self == SECERS_A::SectorErase
    }
}
#[doc = "Field `SECERS` writer - Sector erase"]
pub type SECERS_W<'a, REG> = crate::BitWriter<'a, REG, SECERS_A>;
impl<'a, REG> SECERS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not sector erase operation"]
    #[inline(always)]
    pub fn not_sector_erase(self) -> &'a mut crate::W<REG> {
        self.variant(SECERS_A::NotSectorErase)
    }
    #[doc = "Erase operation is sector erase operation"]
    #[inline(always)]
    pub fn sector_erase(self) -> &'a mut crate::W<REG> {
        self.variant(SECERS_A::SectorErase)
    }
}
#[doc = "Bank erase\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BANKERS_A {
    #[doc = "0: Not bank erase operation"]
    NotBankErase = 0,
    #[doc = "1: Erase operation is bank erase operation"]
    BankErase = 1,
}
impl From<BANKERS_A> for bool {
    #[inline(always)]
    fn from(variant: BANKERS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BANKERS` reader - Bank erase"]
pub type BANKERS_R = crate::BitReader<BANKERS_A>;
impl BANKERS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BANKERS_A {
        match self.bits {
            false => BANKERS_A::NotBankErase,
            true => BANKERS_A::BankErase,
        }
    }
    #[doc = "Not bank erase operation"]
    #[inline(always)]
    pub fn is_not_bank_erase(&self) -> bool {
        *self == BANKERS_A::NotBankErase
    }
    #[doc = "Erase operation is bank erase operation"]
    #[inline(always)]
    pub fn is_bank_erase(&self) -> bool {
        *self == BANKERS_A::BankErase
    }
}
#[doc = "Field `BANKERS` writer - Bank erase"]
pub type BANKERS_W<'a, REG> = crate::BitWriter<'a, REG, BANKERS_A>;
impl<'a, REG> BANKERS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not bank erase operation"]
    #[inline(always)]
    pub fn not_bank_erase(self) -> &'a mut crate::W<REG> {
        self.variant(BANKERS_A::NotBankErase)
    }
    #[doc = "Erase operation is bank erase operation"]
    #[inline(always)]
    pub fn bank_erase(self) -> &'a mut crate::W<REG> {
        self.variant(BANKERS_A::BankErase)
    }
}
#[doc = "Block erase\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLKERS_A {
    #[doc = "0: Not block erase operation"]
    NotBlockErase = 0,
    #[doc = "1: Erase operation is block erase operation"]
    BlockErase = 1,
}
impl From<BLKERS_A> for bool {
    #[inline(always)]
    fn from(variant: BLKERS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLKERS` reader - Block erase"]
pub type BLKERS_R = crate::BitReader<BLKERS_A>;
impl BLKERS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BLKERS_A {
        match self.bits {
            false => BLKERS_A::NotBlockErase,
            true => BLKERS_A::BlockErase,
        }
    }
    #[doc = "Not block erase operation"]
    #[inline(always)]
    pub fn is_not_block_erase(&self) -> bool {
        *self == BLKERS_A::NotBlockErase
    }
    #[doc = "Erase operation is block erase operation"]
    #[inline(always)]
    pub fn is_block_erase(&self) -> bool {
        *self == BLKERS_A::BlockErase
    }
}
#[doc = "Field `BLKERS` writer - Block erase"]
pub type BLKERS_W<'a, REG> = crate::BitWriter<'a, REG, BLKERS_A>;
impl<'a, REG> BLKERS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not block erase operation"]
    #[inline(always)]
    pub fn not_block_erase(self) -> &'a mut crate::W<REG> {
        self.variant(BLKERS_A::NotBlockErase)
    }
    #[doc = "Erase operation is block erase operation"]
    #[inline(always)]
    pub fn block_erase(self) -> &'a mut crate::W<REG> {
        self.variant(BLKERS_A::BlockErase)
    }
}
#[doc = "Erasing start\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erstrr {
    #[doc = "0: Erase operation not started"]
    NotStarted = 0,
    #[doc = "1: Erase operation is in progress"]
    Started = 1,
}
impl From<Erstrr> for bool {
    #[inline(always)]
    fn from(variant: Erstrr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERSTR` reader - Erasing start"]
pub type ERSTR_R = crate::BitReader<Erstrr>;
impl ERSTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erstrr {
        match self.bits {
            false => Erstrr::NotStarted,
            true => Erstrr::Started,
        }
    }
    #[doc = "Erase operation not started"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == Erstrr::NotStarted
    }
    #[doc = "Erase operation is in progress"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == Erstrr::Started
    }
}
#[doc = "Erasing start\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErstrwWO {
    #[doc = "0: Erase operation not started"]
    NoStart = 0,
    #[doc = "1: Start of erase operation"]
    Start = 1,
}
impl From<ErstrwWO> for bool {
    #[inline(always)]
    fn from(variant: ErstrwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERSTR` writer - Erasing start"]
pub type ERSTR_W<'a, REG> = crate::BitWriter<'a, REG, ErstrwWO>;
impl<'a, REG> ERSTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erase operation not started"]
    #[inline(always)]
    pub fn no_start(self) -> &'a mut crate::W<REG> {
        self.variant(ErstrwWO::NoStart)
    }
    #[doc = "Start of erase operation"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(ErstrwWO::Start)
    }
}
#[doc = "Operation lock\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPLK_A {
    #[doc = "0: Flash operations unlocked"]
    Unlocked = 0,
    #[doc = "1: Flash operations locked"]
    Locked = 1,
}
impl From<OPLK_A> for bool {
    #[inline(always)]
    fn from(variant: OPLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPLK` reader - Operation lock"]
pub type OPLK_R = crate::BitReader<OPLK_A>;
impl OPLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPLK_A {
        match self.bits {
            false => OPLK_A::Unlocked,
            true => OPLK_A::Locked,
        }
    }
    #[doc = "Flash operations unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == OPLK_A::Unlocked
    }
    #[doc = "Flash operations locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == OPLK_A::Locked
    }
}
#[doc = "Field `OPLK` writer - Operation lock"]
pub type OPLK_W<'a, REG> = crate::BitWriter<'a, REG, OPLK_A>;
impl<'a, REG> OPLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash operations unlocked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(OPLK_A::Unlocked)
    }
    #[doc = "Flash operations locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(OPLK_A::Locked)
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE_A {
    #[doc = "0: Error interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Error interrupt enabled"]
    Enabled = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::Disabled,
            true => ERRIE_A::Enabled,
        }
    }
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::Disabled
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::Enabled
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE_A>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::Disabled)
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::Enabled)
    }
}
#[doc = "Operation done flag interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODFIE_A {
    #[doc = "0: Operation done interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Operation done interrupt enabled"]
    Enabled = 1,
}
impl From<ODFIE_A> for bool {
    #[inline(always)]
    fn from(variant: ODFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODFIE` reader - Operation done flag interrupt enable"]
pub type ODFIE_R = crate::BitReader<ODFIE_A>;
impl ODFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ODFIE_A {
        match self.bits {
            false => ODFIE_A::Disabled,
            true => ODFIE_A::Enabled,
        }
    }
    #[doc = "Operation done interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ODFIE_A::Disabled
    }
    #[doc = "Operation done interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ODFIE_A::Enabled
    }
}
#[doc = "Field `ODFIE` writer - Operation done flag interrupt enable"]
pub type ODFIE_W<'a, REG> = crate::BitWriter<'a, REG, ODFIE_A>;
impl<'a, REG> ODFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operation done interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ODFIE_A::Disabled)
    }
    #[doc = "Operation done interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ODFIE_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Flash program"]
    #[inline(always)]
    pub fn fprgm(&self) -> FPRGM_R {
        FPRGM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sector erase"]
    #[inline(always)]
    pub fn secers(&self) -> SECERS_R {
        SECERS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank erase"]
    #[inline(always)]
    pub fn bankers(&self) -> BANKERS_R {
        BANKERS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Block erase"]
    #[inline(always)]
    pub fn blkers(&self) -> BLKERS_R {
        BLKERS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Erasing start"]
    #[inline(always)]
    pub fn erstr(&self) -> ERSTR_R {
        ERSTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Operation lock"]
    #[inline(always)]
    pub fn oplk(&self) -> OPLK_R {
        OPLK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Operation done flag interrupt enable"]
    #[inline(always)]
    pub fn odfie(&self) -> ODFIE_R {
        ODFIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("fprgm", &self.fprgm())
            .field("secers", &self.secers())
            .field("bankers", &self.bankers())
            .field("blkers", &self.blkers())
            .field("erstr", &self.erstr())
            .field("oplk", &self.oplk())
            .field("errie", &self.errie())
            .field("odfie", &self.odfie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Flash program"]
    #[inline(always)]
    pub fn fprgm(&mut self) -> FPRGM_W<'_, CTRL2_SPEC> {
        FPRGM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Sector erase"]
    #[inline(always)]
    pub fn secers(&mut self) -> SECERS_W<'_, CTRL2_SPEC> {
        SECERS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bank erase"]
    #[inline(always)]
    pub fn bankers(&mut self) -> BANKERS_W<'_, CTRL2_SPEC> {
        BANKERS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Block erase"]
    #[inline(always)]
    pub fn blkers(&mut self) -> BLKERS_W<'_, CTRL2_SPEC> {
        BLKERS_W::new(self, 3)
    }
    #[doc = "Bit 6 - Erasing start"]
    #[inline(always)]
    pub fn erstr(&mut self) -> ERSTR_W<'_, CTRL2_SPEC> {
        ERSTR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Operation lock"]
    #[inline(always)]
    pub fn oplk(&mut self) -> OPLK_W<'_, CTRL2_SPEC> {
        OPLK_W::new(self, 7)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, CTRL2_SPEC> {
        ERRIE_W::new(self, 10)
    }
    #[doc = "Bit 12 - Operation done flag interrupt enable"]
    #[inline(always)]
    pub fn odfie(&mut self) -> ODFIE_W<'_, CTRL2_SPEC> {
        ODFIE_W::new(self, 12)
    }
}
#[doc = "Control 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL2 to value 0x80"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
