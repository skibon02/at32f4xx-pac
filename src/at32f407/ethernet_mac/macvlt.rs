#[doc = "Register `MACVLT` reader"]
pub type R = crate::R<MACVLT_SPEC>;
#[doc = "Register `MACVLT` writer"]
pub type W = crate::W<MACVLT_SPEC>;
#[doc = "Field `VTI` reader - VLAN tag identifier (for receive frames)"]
pub type VTI_R = crate::FieldReader<u16>;
#[doc = "Field `VTI` writer - VLAN tag identifier (for receive frames)"]
pub type VTI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Enable 12-bit VLAN tag comparison\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETV_A {
    #[doc = "0: Full 16-bit VLAN tag is used for filtering"]
    Normal16bit = 0,
    #[doc = "1: 12-bit VLAN tag is used for comparison and filtering."]
    Compressed12bit = 1,
}
impl From<ETV_A> for bool {
    #[inline(always)]
    fn from(variant: ETV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETV` reader - Enable 12-bit VLAN tag comparison"]
pub type ETV_R = crate::BitReader<ETV_A>;
impl ETV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETV_A {
        match self.bits {
            false => ETV_A::Normal16bit,
            true => ETV_A::Compressed12bit,
        }
    }
    #[doc = "Full 16-bit VLAN tag is used for filtering"]
    #[inline(always)]
    pub fn is_normal16bit(&self) -> bool {
        *self == ETV_A::Normal16bit
    }
    #[doc = "12-bit VLAN tag is used for comparison and filtering."]
    #[inline(always)]
    pub fn is_compressed12bit(&self) -> bool {
        *self == ETV_A::Compressed12bit
    }
}
#[doc = "Field `ETV` writer - Enable 12-bit VLAN tag comparison"]
pub type ETV_W<'a, REG> = crate::BitWriter<'a, REG, ETV_A>;
impl<'a, REG> ETV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Full 16-bit VLAN tag is used for filtering"]
    #[inline(always)]
    pub fn normal16bit(self) -> &'a mut crate::W<REG> {
        self.variant(ETV_A::Normal16bit)
    }
    #[doc = "12-bit VLAN tag is used for comparison and filtering."]
    #[inline(always)]
    pub fn compressed12bit(self) -> &'a mut crate::W<REG> {
        self.variant(ETV_A::Compressed12bit)
    }
}
impl R {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vti(&self) -> VTI_R {
        VTI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACVLT")
            .field("vti", &self.vti())
            .field("etv", &self.etv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vti(&mut self) -> VTI_W<'_, MACVLT_SPEC> {
        VTI_W::new(self, 0)
    }
    #[doc = "Bit 16 - Enable 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn etv(&mut self) -> ETV_W<'_, MACVLT_SPEC> {
        ETV_W::new(self, 16)
    }
}
#[doc = "Ethernet MAC VLAN tag register\n\nYou can [`read`](crate::Reg::read) this register and get [`macvlt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvlt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACVLT_SPEC;
impl crate::RegisterSpec for MACVLT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macvlt::R`](R) reader structure"]
impl crate::Readable for MACVLT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macvlt::W`](W) writer structure"]
impl crate::Writable for MACVLT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACVLT to value 0"]
impl crate::Resettable for MACVLT_SPEC {}
