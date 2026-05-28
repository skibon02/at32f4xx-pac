#[doc = "Register `PWRCTRL` reader"]
pub type R = crate::R<PWRCTRL_SPEC>;
#[doc = "Register `PWRCTRL` writer"]
pub type W = crate::W<PWRCTRL_SPEC>;
#[doc = "Power switch\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PS_A {
    #[doc = "0: Power off, card clock stopped"]
    PoweredOff = 0,
    #[doc = "1: Power on, card clock started"]
    PoweredOn = 1,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PS_A {
    type Ux = u8;
}
impl crate::IsEnum for PS_A {}
#[doc = "Field `PS` reader - Power switch"]
pub type PS_R = crate::FieldReader<PS_A>;
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PS_A> {
        match self.bits {
            0 => Some(PS_A::PoweredOff),
            1 => Some(PS_A::PoweredOn),
            _ => None,
        }
    }
    #[doc = "Power off, card clock stopped"]
    #[inline(always)]
    pub fn is_powered_off(&self) -> bool {
        *self == PS_A::PoweredOff
    }
    #[doc = "Power on, card clock started"]
    #[inline(always)]
    pub fn is_powered_on(&self) -> bool {
        *self == PS_A::PoweredOn
    }
}
#[doc = "Field `PS` writer - Power switch"]
pub type PS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PS_A>;
impl<'a, REG> PS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Power off, card clock stopped"]
    #[inline(always)]
    pub fn powered_off(self) -> &'a mut crate::W<REG> {
        self.variant(PS_A::PoweredOff)
    }
    #[doc = "Power on, card clock started"]
    #[inline(always)]
    pub fn powered_on(self) -> &'a mut crate::W<REG> {
        self.variant(PS_A::PoweredOn)
    }
}
impl R {
    #[doc = "Bits 0:1 - Power switch"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRCTRL").field("ps", &self.ps()).finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Power switch"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<'_, PWRCTRL_SPEC> {
        PS_W::new(self, 0)
    }
}
#[doc = "Bits 1:0 = PWRCTRL: Power supply control bits\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRCTRL_SPEC;
impl crate::RegisterSpec for PWRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrctrl::R`](R) reader structure"]
impl crate::Readable for PWRCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwrctrl::W`](W) writer structure"]
impl crate::Writable for PWRCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRCTRL to value 0"]
impl crate::Resettable for PWRCTRL_SPEC {}
