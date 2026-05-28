#[doc = "Register `MUXGCTRL` reader"]
pub type R = crate::R<MUXGCTRL_SPEC>;
#[doc = "Register `MUXGCTRL` writer"]
pub type W = crate::W<MUXGCTRL_SPEC>;
#[doc = "Field `SIGSEL` reader - Signal select"]
pub type SIGSEL_R = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal select"]
pub type SIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Trigger overrun interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgovienr {
    #[doc = "0: Trigger overrun interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Trigger overrun interrupt is enabled"]
    Enabled = 1,
}
impl From<Trgovienr> for bool {
    #[inline(always)]
    fn from(variant: Trgovienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGOVIEN` reader - Trigger overrun interrupt enable"]
pub type TRGOVIEN_R = crate::BitReader<Trgovienr>;
impl TRGOVIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgovienr {
        match self.bits {
            false => Trgovienr::Disabled,
            true => Trgovienr::Enabled,
        }
    }
    #[doc = "Trigger overrun interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Trgovienr::Disabled
    }
    #[doc = "Trigger overrun interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Trgovienr::Enabled
    }
}
#[doc = "Trigger overrun interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrgovienwWO {
    #[doc = "0: Trigger overrun interrupt disable"]
    Disable = 0,
    #[doc = "1: Trigger overrun interrupt enable"]
    Enable = 1,
}
impl From<TrgovienwWO> for bool {
    #[inline(always)]
    fn from(variant: TrgovienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGOVIEN` writer - Trigger overrun interrupt enable"]
pub type TRGOVIEN_W<'a, REG> = crate::BitWriter<'a, REG, TrgovienwWO>;
impl<'a, REG> TRGOVIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger overrun interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TrgovienwWO::Disable)
    }
    #[doc = "Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TrgovienwWO::Enable)
    }
}
#[doc = "DMA request generator enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Genr {
    #[doc = "0: DMA request generation is disabled"]
    Disabled = 0,
    #[doc = "1: DMA request generation is enabled"]
    Enabled = 1,
}
impl From<Genr> for bool {
    #[inline(always)]
    fn from(variant: Genr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GEN` reader - DMA request generator enable"]
pub type GEN_R = crate::BitReader<Genr>;
impl GEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Genr {
        match self.bits {
            false => Genr::Disabled,
            true => Genr::Enabled,
        }
    }
    #[doc = "DMA request generation is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Genr::Disabled
    }
    #[doc = "DMA request generation is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Genr::Enabled
    }
}
#[doc = "DMA request generator enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenwWO {
    #[doc = "0: DMA request generation disable"]
    Disable = 0,
    #[doc = "1: DMA request generation enable"]
    Enable = 1,
}
impl From<GenwWO> for bool {
    #[inline(always)]
    fn from(variant: GenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GEN` writer - DMA request generator enable"]
pub type GEN_W<'a, REG> = crate::BitWriter<'a, REG, GenwWO>;
impl<'a, REG> GEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA request generation disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(GenwWO::Disable)
    }
    #[doc = "DMA request generation enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(GenwWO::Enable)
    }
}
#[doc = "DMA request generator trigger polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPOL_A {
    #[doc = "0: No events"]
    NoEvents = 0,
    #[doc = "1: Rising edge"]
    RisingEdge = 1,
    #[doc = "2: Falling edge"]
    FallingEdge = 2,
    #[doc = "3: Rising and falling edges"]
    BothEdges = 3,
}
impl From<GPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: GPOL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPOL_A {
    type Ux = u8;
}
impl crate::IsEnum for GPOL_A {}
#[doc = "Field `GPOL` reader - DMA request generator trigger polarity"]
pub type GPOL_R = crate::FieldReader<GPOL_A>;
impl GPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPOL_A {
        match self.bits {
            0 => GPOL_A::NoEvents,
            1 => GPOL_A::RisingEdge,
            2 => GPOL_A::FallingEdge,
            3 => GPOL_A::BothEdges,
            _ => unreachable!(),
        }
    }
    #[doc = "No events"]
    #[inline(always)]
    pub fn is_no_events(&self) -> bool {
        *self == GPOL_A::NoEvents
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == GPOL_A::RisingEdge
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == GPOL_A::FallingEdge
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == GPOL_A::BothEdges
    }
}
#[doc = "Field `GPOL` writer - DMA request generator trigger polarity"]
pub type GPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, GPOL_A, crate::Safe>;
impl<'a, REG> GPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No events"]
    #[inline(always)]
    pub fn no_events(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL_A::NoEvents)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL_A::RisingEdge)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL_A::FallingEdge)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL_A::BothEdges)
    }
}
#[doc = "Field `GREQCNT` reader - Number of DMA requests to be generated"]
pub type GREQCNT_R = crate::FieldReader;
#[doc = "Field `GREQCNT` writer - Number of DMA requests to be generated"]
pub type GREQCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:4 - Signal select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SIGSEL_R {
        SIGSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn trgovien(&self) -> TRGOVIEN_R {
        TRGOVIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA request generator enable"]
    #[inline(always)]
    pub fn gen_(&self) -> GEN_R {
        GEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated"]
    #[inline(always)]
    pub fn greqcnt(&self) -> GREQCNT_R {
        GREQCNT_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXGCTRL")
            .field("sigsel", &self.sigsel())
            .field("trgovien", &self.trgovien())
            .field("gen_", &self.gen_())
            .field("gpol", &self.gpol())
            .field("greqcnt", &self.greqcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Signal select"]
    #[inline(always)]
    pub fn sigsel(&mut self) -> SIGSEL_W<'_, MUXGCTRL_SPEC> {
        SIGSEL_W::new(self, 0)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn trgovien(&mut self) -> TRGOVIEN_W<'_, MUXGCTRL_SPEC> {
        TRGOVIEN_W::new(self, 8)
    }
    #[doc = "Bit 16 - DMA request generator enable"]
    #[inline(always)]
    pub fn gen_(&mut self) -> GEN_W<'_, MUXGCTRL_SPEC> {
        GEN_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity"]
    #[inline(always)]
    pub fn gpol(&mut self) -> GPOL_W<'_, MUXGCTRL_SPEC> {
        GPOL_W::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated"]
    #[inline(always)]
    pub fn greqcnt(&mut self) -> GREQCNT_W<'_, MUXGCTRL_SPEC> {
        GREQCNT_W::new(self, 19)
    }
}
#[doc = "DMA generator channel mux control register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxgctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxgctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXGCTRL_SPEC;
impl crate::RegisterSpec for MUXGCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxgctrl::R`](R) reader structure"]
impl crate::Readable for MUXGCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxgctrl::W`](W) writer structure"]
impl crate::Writable for MUXGCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MUXGCTRL to value 0"]
impl crate::Resettable for MUXGCTRL_SPEC {}
