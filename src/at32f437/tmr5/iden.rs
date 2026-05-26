#[doc = "Register `IDEN` reader"]
pub type R = crate::R<IDEN_SPEC>;
#[doc = "Register `IDEN` writer"]
pub type W = crate::W<IDEN_SPEC>;
#[doc = "Overflow interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovfienr {
    #[doc = "0: Overflow interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Overflow interrupt is enabled"]
    Enabled = 1,
}
impl From<Ovfienr> for bool {
    #[inline(always)]
    fn from(variant: Ovfienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFIEN` reader - Overflow interrupt enable"]
pub type OVFIEN_R = crate::BitReader<Ovfienr>;
impl OVFIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovfienr {
        match self.bits {
            false => Ovfienr::Disabled,
            true => Ovfienr::Enabled,
        }
    }
    #[doc = "Overflow interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ovfienr::Disabled
    }
    #[doc = "Overflow interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ovfienr::Enabled
    }
}
#[doc = "Overflow interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OvfienwWO {
    #[doc = "0: Overflow interrupt disable"]
    Disable = 0,
    #[doc = "1: Overflow interrupt enable"]
    Enable = 1,
}
impl From<OvfienwWO> for bool {
    #[inline(always)]
    fn from(variant: OvfienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFIEN` writer - Overflow interrupt enable"]
pub type OVFIEN_W<'a, REG> = crate::BitWriter<'a, REG, OvfienwWO>;
impl<'a, REG> OVFIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overflow interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(OvfienwWO::Disable)
    }
    #[doc = "Overflow interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(OvfienwWO::Enable)
    }
}
#[doc = "Field `C1IEN` reader - Channel 1 interrupt enable"]
pub type C1IEN_R = crate::BitReader;
#[doc = "Field `C1IEN` writer - Channel 1 interrupt enable"]
pub type C1IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2IEN` reader - Channel 2 interrupt enable"]
pub type C2IEN_R = crate::BitReader;
#[doc = "Field `C2IEN` writer - Channel 2 interrupt enable"]
pub type C2IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3IEN` reader - Channel 3 interrupt enable"]
pub type C3IEN_R = crate::BitReader;
#[doc = "Field `C3IEN` writer - Channel 3 interrupt enable"]
pub type C3IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4IEN` reader - Channel 4 interrupt enable"]
pub type C4IEN_R = crate::BitReader;
#[doc = "Field `C4IEN` writer - Channel 4 interrupt enable"]
pub type C4IEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIEN` reader - Trigger interrupt enable"]
pub type TIEN_R = crate::BitReader;
#[doc = "Field `TIEN` writer - Trigger interrupt enable"]
pub type TIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFDEN` reader - Overflow DMA request enable"]
pub type OVFDEN_R = crate::BitReader;
#[doc = "Field `OVFDEN` writer - Overflow DMA request enable"]
pub type OVFDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1DEN` reader - Channel 1 DMA request enable"]
pub type C1DEN_R = crate::BitReader;
#[doc = "Field `C1DEN` writer - Channel 1 DMA request enable"]
pub type C1DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2DEN` reader - Channel 2 DMA request enable"]
pub type C2DEN_R = crate::BitReader;
#[doc = "Field `C2DEN` writer - Channel 2 DMA request enable"]
pub type C2DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3DEN` reader - Channel 3 DMA request enable"]
pub type C3DEN_R = crate::BitReader;
#[doc = "Field `C3DEN` writer - Channel 3 DMA request enable"]
pub type C3DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4DEN` reader - Channel 4 DMA request enable"]
pub type C4DEN_R = crate::BitReader;
#[doc = "Field `C4DEN` writer - Channel 4 DMA request enable"]
pub type C4DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDEN` reader - Trigger DMA request enable"]
pub type TDEN_R = crate::BitReader;
#[doc = "Field `TDEN` writer - Trigger DMA request enable"]
pub type TDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfien(&self) -> OVFIEN_R {
        OVFIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 interrupt enable"]
    #[inline(always)]
    pub fn c1ien(&self) -> C1IEN_R {
        C1IEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 interrupt enable"]
    #[inline(always)]
    pub fn c2ien(&self) -> C2IEN_R {
        C2IEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 interrupt enable"]
    #[inline(always)]
    pub fn c3ien(&self) -> C3IEN_R {
        C3IEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 interrupt enable"]
    #[inline(always)]
    pub fn c4ien(&self) -> C4IEN_R {
        C4IEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn tien(&self) -> TIEN_R {
        TIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Overflow DMA request enable"]
    #[inline(always)]
    pub fn ovfden(&self) -> OVFDEN_R {
        OVFDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 DMA request enable"]
    #[inline(always)]
    pub fn c1den(&self) -> C1DEN_R {
        C1DEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 DMA request enable"]
    #[inline(always)]
    pub fn c2den(&self) -> C2DEN_R {
        C2DEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 DMA request enable"]
    #[inline(always)]
    pub fn c3den(&self) -> C3DEN_R {
        C3DEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 DMA request enable"]
    #[inline(always)]
    pub fn c4den(&self) -> C4DEN_R {
        C4DEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn tden(&self) -> TDEN_R {
        TDEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDEN")
            .field("tden", &self.tden())
            .field("c4den", &self.c4den())
            .field("c3den", &self.c3den())
            .field("c2den", &self.c2den())
            .field("c1den", &self.c1den())
            .field("ovfden", &self.ovfden())
            .field("tien", &self.tien())
            .field("c4ien", &self.c4ien())
            .field("c3ien", &self.c3ien())
            .field("c2ien", &self.c2ien())
            .field("c1ien", &self.c1ien())
            .field("ovfien", &self.ovfien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfien(&mut self) -> OVFIEN_W<'_, IDEN_SPEC> {
        OVFIEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 interrupt enable"]
    #[inline(always)]
    pub fn c1ien(&mut self) -> C1IEN_W<'_, IDEN_SPEC> {
        C1IEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 interrupt enable"]
    #[inline(always)]
    pub fn c2ien(&mut self) -> C2IEN_W<'_, IDEN_SPEC> {
        C2IEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 interrupt enable"]
    #[inline(always)]
    pub fn c3ien(&mut self) -> C3IEN_W<'_, IDEN_SPEC> {
        C3IEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 interrupt enable"]
    #[inline(always)]
    pub fn c4ien(&mut self) -> C4IEN_W<'_, IDEN_SPEC> {
        C4IEN_W::new(self, 4)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn tien(&mut self) -> TIEN_W<'_, IDEN_SPEC> {
        TIEN_W::new(self, 6)
    }
    #[doc = "Bit 8 - Overflow DMA request enable"]
    #[inline(always)]
    pub fn ovfden(&mut self) -> OVFDEN_W<'_, IDEN_SPEC> {
        OVFDEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 1 DMA request enable"]
    #[inline(always)]
    pub fn c1den(&mut self) -> C1DEN_W<'_, IDEN_SPEC> {
        C1DEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 DMA request enable"]
    #[inline(always)]
    pub fn c2den(&mut self) -> C2DEN_W<'_, IDEN_SPEC> {
        C2DEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 3 DMA request enable"]
    #[inline(always)]
    pub fn c3den(&mut self) -> C3DEN_W<'_, IDEN_SPEC> {
        C3DEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 4 DMA request enable"]
    #[inline(always)]
    pub fn c4den(&mut self) -> C4DEN_W<'_, IDEN_SPEC> {
        C4DEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn tden(&mut self) -> TDEN_W<'_, IDEN_SPEC> {
        TDEN_W::new(self, 14)
    }
}
#[doc = "Interrupt/DMA enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`iden::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iden::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDEN_SPEC;
impl crate::RegisterSpec for IDEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iden::R`](R) reader structure"]
impl crate::Readable for IDEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iden::W`](W) writer structure"]
impl crate::Writable for IDEN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDEN to value 0"]
impl crate::Resettable for IDEN_SPEC {}
