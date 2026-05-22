#[doc = "Register `LDOOV` reader"]
pub type R = crate::R<LDOOV_SPEC>;
#[doc = "Register `LDOOV` writer"]
pub type W = crate::W<LDOOV_SPEC>;
#[doc = "LDO output voltage select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LDOOVSELECT_A {
    #[doc = "0: LDO output voltage is 1.1 V"]
    V1_1 = 0,
    #[doc = "1: LDO output voltage is 1.2 V"]
    V1_2 = 1,
    #[doc = "2: LDO output voltage is 1.3 V"]
    V1_3 = 2,
}
impl From<LDOOVSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: LDOOVSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LDOOVSELECT_A {
    type Ux = u8;
}
impl crate::IsEnum for LDOOVSELECT_A {}
#[doc = "Field `LDOOVSEL` reader - LDO output voltage select"]
pub type LDOOVSEL_R = crate::FieldReader<LDOOVSELECT_A>;
impl LDOOVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LDOOVSELECT_A> {
        match self.bits {
            0 => Some(LDOOVSELECT_A::V1_1),
            1 => Some(LDOOVSELECT_A::V1_2),
            2 => Some(LDOOVSELECT_A::V1_3),
            _ => None,
        }
    }
    #[doc = "LDO output voltage is 1.1 V"]
    #[inline(always)]
    pub fn is_v1_1(&self) -> bool {
        *self == LDOOVSELECT_A::V1_1
    }
    #[doc = "LDO output voltage is 1.2 V"]
    #[inline(always)]
    pub fn is_v1_2(&self) -> bool {
        *self == LDOOVSELECT_A::V1_2
    }
    #[doc = "LDO output voltage is 1.3 V"]
    #[inline(always)]
    pub fn is_v1_3(&self) -> bool {
        *self == LDOOVSELECT_A::V1_3
    }
}
#[doc = "Field `LDOOVSEL` writer - LDO output voltage select"]
pub type LDOOVSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LDOOVSELECT_A>;
impl<'a, REG> LDOOVSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LDO output voltage is 1.1 V"]
    #[inline(always)]
    pub fn v1_1(self) -> &'a mut crate::W<REG> {
        self.variant(LDOOVSELECT_A::V1_1)
    }
    #[doc = "LDO output voltage is 1.2 V"]
    #[inline(always)]
    pub fn v1_2(self) -> &'a mut crate::W<REG> {
        self.variant(LDOOVSELECT_A::V1_2)
    }
    #[doc = "LDO output voltage is 1.3 V"]
    #[inline(always)]
    pub fn v1_3(self) -> &'a mut crate::W<REG> {
        self.variant(LDOOVSELECT_A::V1_3)
    }
}
impl R {
    #[doc = "Bits 0:2 - LDO output voltage select"]
    #[inline(always)]
    pub fn ldoovsel(&self) -> LDOOVSEL_R {
        LDOOVSEL_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LDOOV")
            .field("ldoovsel", &self.ldoovsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - LDO output voltage select"]
    #[inline(always)]
    pub fn ldoovsel(&mut self) -> LDOOVSEL_W<'_, LDOOV_SPEC> {
        LDOOVSEL_W::new(self, 0)
    }
}
#[doc = "LDO output voltage register\n\nYou can [`read`](crate::Reg::read) this register and get [`ldoov::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldoov::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LDOOV_SPEC;
impl crate::RegisterSpec for LDOOV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldoov::R`](R) reader structure"]
impl crate::Readable for LDOOV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ldoov::W`](W) writer structure"]
impl crate::Writable for LDOOV_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LDOOV to value 0"]
impl crate::Resettable for LDOOV_SPEC {}
