#[doc = "Register `ISTS` reader"]
pub type R = crate::R<ISTS_SPEC>;
#[doc = "Register `ISTS` writer"]
pub type W = crate::W<ISTS_SPEC>;
#[doc = "Overflow interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovfifr {
    #[doc = "0: No overflow event occurs"]
    NoOverflow = 0,
    #[doc = "1: An overflow event is generated"]
    Overflow = 1,
}
impl From<Ovfifr> for bool {
    #[inline(always)]
    fn from(variant: Ovfifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFIF` reader - Overflow interrupt flag"]
pub type OVFIF_R = crate::BitReader<Ovfifr>;
impl OVFIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovfifr {
        match self.bits {
            false => Ovfifr::NoOverflow,
            true => Ovfifr::Overflow,
        }
    }
    #[doc = "No overflow event occurs"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Ovfifr::NoOverflow
    }
    #[doc = "An overflow event is generated"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Ovfifr::Overflow
    }
}
#[doc = "Overflow interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OvfifwWO {
    #[doc = "0: Overflow interrupt flag clear"]
    Clear = 0,
}
impl From<OvfifwWO> for bool {
    #[inline(always)]
    fn from(variant: OvfifwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFIF` writer - Overflow interrupt flag"]
pub type OVFIF_W<'a, REG> = crate::BitWriter0C<'a, REG, OvfifwWO>;
impl<'a, REG> OVFIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overflow interrupt flag clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OvfifwWO::Clear)
    }
}
#[doc = "Field `C1IF` reader - Channel 1 interrupt flag"]
pub type C1IF_R = crate::BitReader;
#[doc = "Field `C1IF` writer - Channel 1 interrupt flag"]
pub type C1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2IF` reader - Channel 2 interrupt flag"]
pub type C2IF_R = crate::BitReader;
#[doc = "Field `C2IF` writer - Channel 2 interrupt flag"]
pub type C2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3IF` reader - Channel 3 interrupt flag"]
pub type C3IF_R = crate::BitReader;
#[doc = "Field `C3IF` writer - Channel 3 interrupt flag"]
pub type C3IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4IF` reader - Channel 4 interrupt flag"]
pub type C4IF_R = crate::BitReader;
#[doc = "Field `C4IF` writer - Channel 4 interrupt flag"]
pub type C4IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGIF` reader - Trigger interrupt flag"]
pub type TRGIF_R = crate::BitReader;
#[doc = "Field `TRGIF` writer - Trigger interrupt flag"]
pub type TRGIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1RF` reader - Channel 1 recapture flag"]
pub type C1RF_R = crate::BitReader;
#[doc = "Field `C1RF` writer - Channel 1 recapture flag"]
pub type C1RF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2RF` reader - Channel 2 recapture flag"]
pub type C2RF_R = crate::BitReader;
#[doc = "Field `C2RF` writer - Channel 2 recapture flag"]
pub type C2RF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3RF` reader - Channel 3 recapture flag"]
pub type C3RF_R = crate::BitReader;
#[doc = "Field `C3RF` writer - Channel 3 recapture flag"]
pub type C3RF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4RF` reader - Channel 4 recapture flag"]
pub type C4RF_R = crate::BitReader;
#[doc = "Field `C4RF` writer - Channel 4 recapture flag"]
pub type C4RF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow interrupt flag"]
    #[inline(always)]
    pub fn ovfif(&self) -> OVFIF_R {
        OVFIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 interrupt flag"]
    #[inline(always)]
    pub fn c1if(&self) -> C1IF_R {
        C1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 interrupt flag"]
    #[inline(always)]
    pub fn c2if(&self) -> C2IF_R {
        C2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 interrupt flag"]
    #[inline(always)]
    pub fn c3if(&self) -> C3IF_R {
        C3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 interrupt flag"]
    #[inline(always)]
    pub fn c4if(&self) -> C4IF_R {
        C4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgif(&self) -> TRGIF_R {
        TRGIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 recapture flag"]
    #[inline(always)]
    pub fn c1rf(&self) -> C1RF_R {
        C1RF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 recapture flag"]
    #[inline(always)]
    pub fn c2rf(&self) -> C2RF_R {
        C2RF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 recapture flag"]
    #[inline(always)]
    pub fn c3rf(&self) -> C3RF_R {
        C3RF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 recapture flag"]
    #[inline(always)]
    pub fn c4rf(&self) -> C4RF_R {
        C4RF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISTS")
            .field("c4rf", &self.c4rf())
            .field("c3rf", &self.c3rf())
            .field("c2rf", &self.c2rf())
            .field("c1rf", &self.c1rf())
            .field("trgif", &self.trgif())
            .field("c4if", &self.c4if())
            .field("c3if", &self.c3if())
            .field("c2if", &self.c2if())
            .field("c1if", &self.c1if())
            .field("ovfif", &self.ovfif())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt flag"]
    #[inline(always)]
    pub fn ovfif(&mut self) -> OVFIF_W<'_, ISTS_SPEC> {
        OVFIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 interrupt flag"]
    #[inline(always)]
    pub fn c1if(&mut self) -> C1IF_W<'_, ISTS_SPEC> {
        C1IF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 interrupt flag"]
    #[inline(always)]
    pub fn c2if(&mut self) -> C2IF_W<'_, ISTS_SPEC> {
        C2IF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 interrupt flag"]
    #[inline(always)]
    pub fn c3if(&mut self) -> C3IF_W<'_, ISTS_SPEC> {
        C3IF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 interrupt flag"]
    #[inline(always)]
    pub fn c4if(&mut self) -> C4IF_W<'_, ISTS_SPEC> {
        C4IF_W::new(self, 4)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgif(&mut self) -> TRGIF_W<'_, ISTS_SPEC> {
        TRGIF_W::new(self, 6)
    }
    #[doc = "Bit 9 - Channel 1 recapture flag"]
    #[inline(always)]
    pub fn c1rf(&mut self) -> C1RF_W<'_, ISTS_SPEC> {
        C1RF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 recapture flag"]
    #[inline(always)]
    pub fn c2rf(&mut self) -> C2RF_W<'_, ISTS_SPEC> {
        C2RF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 3 recapture flag"]
    #[inline(always)]
    pub fn c3rf(&mut self) -> C3RF_W<'_, ISTS_SPEC> {
        C3RF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 4 recapture flag"]
    #[inline(always)]
    pub fn c4rf(&mut self) -> C4RF_W<'_, ISTS_SPEC> {
        C4RF_W::new(self, 12)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ists::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ists::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISTS_SPEC;
impl crate::RegisterSpec for ISTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ists::R`](R) reader structure"]
impl crate::Readable for ISTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ists::W`](W) writer structure"]
impl crate::Writable for ISTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets ISTS to value 0"]
impl crate::Resettable for ISTS_SPEC {}
