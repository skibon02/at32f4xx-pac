#[doc = "Register `MUXCTRL` reader"]
pub type R = crate::R<MUXCTRL_SPEC>;
#[doc = "Register `MUXCTRL` writer"]
pub type W = crate::W<MUXCTRL_SPEC>;
#[doc = "Field `REQSEL` reader - DMA request select"]
pub type REQSEL_R = crate::FieldReader;
#[doc = "Field `REQSEL` writer - DMA request select"]
pub type REQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
#[doc = "Synchronization overrun interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syncovienr {
    #[doc = "0: Channel synchronization overrun interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Channel synchronization overrun interrupt is enabled"]
    Enabled = 1,
}
impl From<Syncovienr> for bool {
    #[inline(always)]
    fn from(variant: Syncovienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCOVIEN` reader - Synchronization overrun interrupt enable"]
pub type SYNCOVIEN_R = crate::BitReader<Syncovienr>;
impl SYNCOVIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syncovienr {
        match self.bits {
            false => Syncovienr::Disabled,
            true => Syncovienr::Enabled,
        }
    }
    #[doc = "Channel synchronization overrun interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Syncovienr::Disabled
    }
    #[doc = "Channel synchronization overrun interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Syncovienr::Enabled
    }
}
#[doc = "Synchronization overrun interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyncovienwWO {
    #[doc = "0: Channel synchronization overrun interrupt disable"]
    Disable = 0,
    #[doc = "1: Channel synchronization overrun interrupt enable"]
    Enable = 1,
}
impl From<SyncovienwWO> for bool {
    #[inline(always)]
    fn from(variant: SyncovienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCOVIEN` writer - Synchronization overrun interrupt enable"]
pub type SYNCOVIEN_W<'a, REG> = crate::BitWriter<'a, REG, SyncovienwWO>;
impl<'a, REG> SYNCOVIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel synchronization overrun interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SyncovienwWO::Disable)
    }
    #[doc = "Channel synchronization overrun interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SyncovienwWO::Enable)
    }
}
#[doc = "Event generation enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Evtgenr {
    #[doc = "0: Event generation is disabled"]
    Disabled = 0,
    #[doc = "1: Event generation is enabled"]
    Enabled = 1,
}
impl From<Evtgenr> for bool {
    #[inline(always)]
    fn from(variant: Evtgenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVTGEN` reader - Event generation enable"]
pub type EVTGEN_R = crate::BitReader<Evtgenr>;
impl EVTGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evtgenr {
        match self.bits {
            false => Evtgenr::Disabled,
            true => Evtgenr::Enabled,
        }
    }
    #[doc = "Event generation is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Evtgenr::Disabled
    }
    #[doc = "Event generation is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Evtgenr::Enabled
    }
}
#[doc = "Event generation enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvtgenwWO {
    #[doc = "0: Event generation disable"]
    Disable = 0,
    #[doc = "1: Event generation enable"]
    Enable = 1,
}
impl From<EvtgenwWO> for bool {
    #[inline(always)]
    fn from(variant: EvtgenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVTGEN` writer - Event generation enable"]
pub type EVTGEN_W<'a, REG> = crate::BitWriter<'a, REG, EvtgenwWO>;
impl<'a, REG> EVTGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event generation disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EvtgenwWO::Disable)
    }
    #[doc = "Event generation enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EvtgenwWO::Enable)
    }
}
#[doc = "Synchronization enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syncenr {
    #[doc = "0: Channel synchronization is disabled"]
    Disabled = 0,
    #[doc = "1: Channel synchronization is enabled"]
    Enabled = 1,
}
impl From<Syncenr> for bool {
    #[inline(always)]
    fn from(variant: Syncenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCEN` reader - Synchronization enable"]
pub type SYNCEN_R = crate::BitReader<Syncenr>;
impl SYNCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syncenr {
        match self.bits {
            false => Syncenr::Disabled,
            true => Syncenr::Enabled,
        }
    }
    #[doc = "Channel synchronization is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Syncenr::Disabled
    }
    #[doc = "Channel synchronization is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Syncenr::Enabled
    }
}
#[doc = "Synchronization enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyncenwWO {
    #[doc = "0: Channel synchronization disable"]
    Disable = 0,
    #[doc = "1: Channel synchronization enable"]
    Enable = 1,
}
impl From<SyncenwWO> for bool {
    #[inline(always)]
    fn from(variant: SyncenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCEN` writer - Synchronization enable"]
pub type SYNCEN_W<'a, REG> = crate::BitWriter<'a, REG, SyncenwWO>;
impl<'a, REG> SYNCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel synchronization disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SyncenwWO::Disable)
    }
    #[doc = "Channel synchronization enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SyncenwWO::Enable)
    }
}
#[doc = "Synchronization polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCPOL_A {
    #[doc = "0: No events"]
    NoEvents = 0,
    #[doc = "1: Rising edge"]
    RisingEdge = 1,
    #[doc = "2: Falling edge"]
    FallingEdge = 2,
    #[doc = "3: Rising and falling edges"]
    BothEdges = 3,
}
impl From<SYNCPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCPOL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNCPOL_A {
    type Ux = u8;
}
impl crate::IsEnum for SYNCPOL_A {}
#[doc = "Field `SYNCPOL` reader - Synchronization polarity"]
pub type SYNCPOL_R = crate::FieldReader<SYNCPOL_A>;
impl SYNCPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCPOL_A {
        match self.bits {
            0 => SYNCPOL_A::NoEvents,
            1 => SYNCPOL_A::RisingEdge,
            2 => SYNCPOL_A::FallingEdge,
            3 => SYNCPOL_A::BothEdges,
            _ => unreachable!(),
        }
    }
    #[doc = "No events"]
    #[inline(always)]
    pub fn is_no_events(&self) -> bool {
        *self == SYNCPOL_A::NoEvents
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SYNCPOL_A::RisingEdge
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SYNCPOL_A::FallingEdge
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == SYNCPOL_A::BothEdges
    }
}
#[doc = "Field `SYNCPOL` writer - Synchronization polarity"]
pub type SYNCPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYNCPOL_A, crate::Safe>;
impl<'a, REG> SYNCPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No events"]
    #[inline(always)]
    pub fn no_events(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCPOL_A::NoEvents)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCPOL_A::RisingEdge)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCPOL_A::FallingEdge)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCPOL_A::BothEdges)
    }
}
#[doc = "Field `REQCNT` reader - Number of DMA requests"]
pub type REQCNT_R = crate::FieldReader;
#[doc = "Field `REQCNT` writer - Number of DMA requests"]
pub type REQCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Field `SYNCSEL` reader - Synchronization Identification"]
pub type SYNCSEL_R = crate::FieldReader;
#[doc = "Field `SYNCSEL` writer - Synchronization Identification"]
pub type SYNCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:6 - DMA request select"]
    #[inline(always)]
    pub fn reqsel(&self) -> REQSEL_R {
        REQSEL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    pub fn syncovien(&self) -> SYNCOVIEN_R {
        SYNCOVIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    pub fn evtgen(&self) -> EVTGEN_R {
        EVTGEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchronization enable"]
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization polarity"]
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests"]
    #[inline(always)]
    pub fn reqcnt(&self) -> REQCNT_R {
        REQCNT_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Synchronization Identification"]
    #[inline(always)]
    pub fn syncsel(&self) -> SYNCSEL_R {
        SYNCSEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXCTRL")
            .field("reqsel", &self.reqsel())
            .field("syncovien", &self.syncovien())
            .field("evtgen", &self.evtgen())
            .field("syncen", &self.syncen())
            .field("syncpol", &self.syncpol())
            .field("reqcnt", &self.reqcnt())
            .field("syncsel", &self.syncsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - DMA request select"]
    #[inline(always)]
    pub fn reqsel(&mut self) -> REQSEL_W<'_, MUXCTRL_SPEC> {
        REQSEL_W::new(self, 0)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    pub fn syncovien(&mut self) -> SYNCOVIEN_W<'_, MUXCTRL_SPEC> {
        SYNCOVIEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    pub fn evtgen(&mut self) -> EVTGEN_W<'_, MUXCTRL_SPEC> {
        EVTGEN_W::new(self, 9)
    }
    #[doc = "Bit 16 - Synchronization enable"]
    #[inline(always)]
    pub fn syncen(&mut self) -> SYNCEN_W<'_, MUXCTRL_SPEC> {
        SYNCEN_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Synchronization polarity"]
    #[inline(always)]
    pub fn syncpol(&mut self) -> SYNCPOL_W<'_, MUXCTRL_SPEC> {
        SYNCPOL_W::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests"]
    #[inline(always)]
    pub fn reqcnt(&mut self) -> REQCNT_W<'_, MUXCTRL_SPEC> {
        REQCNT_W::new(self, 19)
    }
    #[doc = "Bits 24:28 - Synchronization Identification"]
    #[inline(always)]
    pub fn syncsel(&mut self) -> SYNCSEL_W<'_, MUXCTRL_SPEC> {
        SYNCSEL_W::new(self, 24)
    }
}
#[doc = "DMA channel mux control register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXCTRL_SPEC;
impl crate::RegisterSpec for MUXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxctrl::R`](R) reader structure"]
impl crate::Readable for MUXCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxctrl::W`](W) writer structure"]
impl crate::Writable for MUXCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MUXCTRL to value 0"]
impl crate::Resettable for MUXCTRL_SPEC {}
