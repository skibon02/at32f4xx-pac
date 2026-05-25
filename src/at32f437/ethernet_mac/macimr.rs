#[doc = "Register `MACIMR` reader"]
pub type R = crate::R<MACIMR_SPEC>;
#[doc = "Register `MACIMR` writer"]
pub type W = crate::W<MACIMR_SPEC>;
#[doc = "PMT interrupt mask\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIM_A {
    #[doc = "0: PMT interrupt is disabled"]
    InterruptDisabled = 0,
    #[doc = "1: PMT interrupt is enabled"]
    InterruptEnabled = 1,
}
impl From<PIM_A> for bool {
    #[inline(always)]
    fn from(variant: PIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIM` reader - PMT interrupt mask"]
pub type PIM_R = crate::BitReader<PIM_A>;
impl PIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIM_A {
        match self.bits {
            false => PIM_A::InterruptDisabled,
            true => PIM_A::InterruptEnabled,
        }
    }
    #[doc = "PMT interrupt is disabled"]
    #[inline(always)]
    pub fn is_interrupt_disabled(&self) -> bool {
        *self == PIM_A::InterruptDisabled
    }
    #[doc = "PMT interrupt is enabled"]
    #[inline(always)]
    pub fn is_interrupt_enabled(&self) -> bool {
        *self == PIM_A::InterruptEnabled
    }
}
#[doc = "Field `PIM` writer - PMT interrupt mask"]
pub type PIM_W<'a, REG> = crate::BitWriter<'a, REG, PIM_A>;
impl<'a, REG> PIM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PMT interrupt is disabled"]
    #[inline(always)]
    pub fn interrupt_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PIM_A::InterruptDisabled)
    }
    #[doc = "PMT interrupt is enabled"]
    #[inline(always)]
    pub fn interrupt_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PIM_A::InterruptEnabled)
    }
}
#[doc = "Timestamp interrupt mask\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM_A {
    #[doc = "0: Timestamp interrupt is disabled"]
    InterruptDisabled = 0,
    #[doc = "1: Timestamp interrupt is enabled"]
    InterruptEnabled = 1,
}
impl From<TIM_A> for bool {
    #[inline(always)]
    fn from(variant: TIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM` reader - Timestamp interrupt mask"]
pub type TIM_R = crate::BitReader<TIM_A>;
impl TIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM_A {
        match self.bits {
            false => TIM_A::InterruptDisabled,
            true => TIM_A::InterruptEnabled,
        }
    }
    #[doc = "Timestamp interrupt is disabled"]
    #[inline(always)]
    pub fn is_interrupt_disabled(&self) -> bool {
        *self == TIM_A::InterruptDisabled
    }
    #[doc = "Timestamp interrupt is enabled"]
    #[inline(always)]
    pub fn is_interrupt_enabled(&self) -> bool {
        *self == TIM_A::InterruptEnabled
    }
}
#[doc = "Field `TIM` writer - Timestamp interrupt mask"]
pub type TIM_W<'a, REG> = crate::BitWriter<'a, REG, TIM_A>;
impl<'a, REG> TIM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timestamp interrupt is disabled"]
    #[inline(always)]
    pub fn interrupt_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM_A::InterruptDisabled)
    }
    #[doc = "Timestamp interrupt is enabled"]
    #[inline(always)]
    pub fn interrupt_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM_A::InterruptEnabled)
    }
}
impl R {
    #[doc = "Bit 3 - PMT interrupt mask"]
    #[inline(always)]
    pub fn pim(&self) -> PIM_R {
        PIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp interrupt mask"]
    #[inline(always)]
    pub fn tim(&self) -> TIM_R {
        TIM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACIMR")
            .field("pim", &self.pim())
            .field("tim", &self.tim())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - PMT interrupt mask"]
    #[inline(always)]
    pub fn pim(&mut self) -> PIM_W<'_, MACIMR_SPEC> {
        PIM_W::new(self, 3)
    }
    #[doc = "Bit 9 - Timestamp interrupt mask"]
    #[inline(always)]
    pub fn tim(&mut self) -> TIM_W<'_, MACIMR_SPEC> {
        TIM_W::new(self, 9)
    }
}
#[doc = "Ethernet MAC interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`macimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACIMR_SPEC;
impl crate::RegisterSpec for MACIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macimr::R`](R) reader structure"]
impl crate::Readable for MACIMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macimr::W`](W) writer structure"]
impl crate::Writable for MACIMR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACIMR to value 0"]
impl crate::Resettable for MACIMR_SPEC {}
