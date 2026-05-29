#[doc = "Register `MACFRMF` reader"]
pub type R = crate::R<MACFRMF_SPEC>;
#[doc = "Register `MACFRMF` writer"]
pub type W = crate::W<MACFRMF_SPEC>;
#[doc = "Promiscuous mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR_A {
    #[doc = "0: Promiscuous mode disabled."]
    Disable = 0,
    #[doc = "1: The MAC passes all frames to the application"]
    PassAll = 1,
}
impl From<PR_A> for bool {
    #[inline(always)]
    fn from(variant: PR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR` reader - Promiscuous mode"]
pub type PR_R = crate::BitReader<PR_A>;
impl PR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PR_A {
        match self.bits {
            false => PR_A::Disable,
            true => PR_A::PassAll,
        }
    }
    #[doc = "Promiscuous mode disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PR_A::Disable
    }
    #[doc = "The MAC passes all frames to the application"]
    #[inline(always)]
    pub fn is_pass_all(&self) -> bool {
        *self == PR_A::PassAll
    }
}
#[doc = "Field `PR` writer - Promiscuous mode"]
pub type PR_W<'a, REG> = crate::BitWriter<'a, REG, PR_A>;
impl<'a, REG> PR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Promiscuous mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::Disable)
    }
    #[doc = "The MAC passes all frames to the application"]
    #[inline(always)]
    pub fn pass_all(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::PassAll)
    }
}
#[doc = "Hash unicast\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HUC_A {
    #[doc = "0: Direct destination address comparison for unicast"]
    Disable = 0,
    #[doc = "1: The MAC performs destination address filtering for unicast frames according to the hash table"]
    EnableHash = 1,
}
impl From<HUC_A> for bool {
    #[inline(always)]
    fn from(variant: HUC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HUC` reader - Hash unicast"]
pub type HUC_R = crate::BitReader<HUC_A>;
impl HUC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HUC_A {
        match self.bits {
            false => HUC_A::Disable,
            true => HUC_A::EnableHash,
        }
    }
    #[doc = "Direct destination address comparison for unicast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HUC_A::Disable
    }
    #[doc = "The MAC performs destination address filtering for unicast frames according to the hash table"]
    #[inline(always)]
    pub fn is_enable_hash(&self) -> bool {
        *self == HUC_A::EnableHash
    }
}
#[doc = "Field `HUC` writer - Hash unicast"]
pub type HUC_W<'a, REG> = crate::BitWriter<'a, REG, HUC_A>;
impl<'a, REG> HUC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Direct destination address comparison for unicast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HUC_A::Disable)
    }
    #[doc = "The MAC performs destination address filtering for unicast frames according to the hash table"]
    #[inline(always)]
    pub fn enable_hash(self) -> &'a mut crate::W<REG> {
        self.variant(HUC_A::EnableHash)
    }
}
#[doc = "Hash multicast\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HMC_A {
    #[doc = "0: Direct destination address comparison for multicast"]
    Disable = 0,
    #[doc = "1: The MAC performs destination address filtering for multicast frames according to the hash table"]
    EnableHash = 1,
}
impl From<HMC_A> for bool {
    #[inline(always)]
    fn from(variant: HMC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HMC` reader - Hash multicast"]
pub type HMC_R = crate::BitReader<HMC_A>;
impl HMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HMC_A {
        match self.bits {
            false => HMC_A::Disable,
            true => HMC_A::EnableHash,
        }
    }
    #[doc = "Direct destination address comparison for multicast"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HMC_A::Disable
    }
    #[doc = "The MAC performs destination address filtering for multicast frames according to the hash table"]
    #[inline(always)]
    pub fn is_enable_hash(&self) -> bool {
        *self == HMC_A::EnableHash
    }
}
#[doc = "Field `HMC` writer - Hash multicast"]
pub type HMC_W<'a, REG> = crate::BitWriter<'a, REG, HMC_A>;
impl<'a, REG> HMC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Direct destination address comparison for multicast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HMC_A::Disable)
    }
    #[doc = "The MAC performs destination address filtering for multicast frames according to the hash table"]
    #[inline(always)]
    pub fn enable_hash(self) -> &'a mut crate::W<REG> {
        self.variant(HMC_A::EnableHash)
    }
}
#[doc = "Destination address inverse filtering\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAIF_A {
    #[doc = "0: Work normally"]
    Normal = 0,
    #[doc = "1: The address check block operates in inverse filtering mode for the destination address comparison for both unicast and multicast frames"]
    Inverse = 1,
}
impl From<DAIF_A> for bool {
    #[inline(always)]
    fn from(variant: DAIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAIF` reader - Destination address inverse filtering"]
pub type DAIF_R = crate::BitReader<DAIF_A>;
impl DAIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAIF_A {
        match self.bits {
            false => DAIF_A::Normal,
            true => DAIF_A::Inverse,
        }
    }
    #[doc = "Work normally"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DAIF_A::Normal
    }
    #[doc = "The address check block operates in inverse filtering mode for the destination address comparison for both unicast and multicast frames"]
    #[inline(always)]
    pub fn is_inverse(&self) -> bool {
        *self == DAIF_A::Inverse
    }
}
#[doc = "Field `DAIF` writer - Destination address inverse filtering"]
pub type DAIF_W<'a, REG> = crate::BitWriter<'a, REG, DAIF_A>;
impl<'a, REG> DAIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Work normally"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(DAIF_A::Normal)
    }
    #[doc = "The address check block operates in inverse filtering mode for the destination address comparison for both unicast and multicast frames"]
    #[inline(always)]
    pub fn inverse(self) -> &'a mut crate::W<REG> {
        self.variant(DAIF_A::Inverse)
    }
}
#[doc = "Pass multicast\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMC_A {
    #[doc = "0: Normal operation"]
    Normal = 0,
    #[doc = "1: All frames with a multicast destination address (first bit in the destination address is set) are passed"]
    PassAll = 1,
}
impl From<PMC_A> for bool {
    #[inline(always)]
    fn from(variant: PMC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMC` reader - Pass multicast"]
pub type PMC_R = crate::BitReader<PMC_A>;
impl PMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PMC_A {
        match self.bits {
            false => PMC_A::Normal,
            true => PMC_A::PassAll,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PMC_A::Normal
    }
    #[doc = "All frames with a multicast destination address (first bit in the destination address is set) are passed"]
    #[inline(always)]
    pub fn is_pass_all(&self) -> bool {
        *self == PMC_A::PassAll
    }
}
#[doc = "Field `PMC` writer - Pass multicast"]
pub type PMC_W<'a, REG> = crate::BitWriter<'a, REG, PMC_A>;
impl<'a, REG> PMC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PMC_A::Normal)
    }
    #[doc = "All frames with a multicast destination address (first bit in the destination address is set) are passed"]
    #[inline(always)]
    pub fn pass_all(self) -> &'a mut crate::W<REG> {
        self.variant(PMC_A::PassAll)
    }
}
#[doc = "Disable broadcast frames\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBF_A {
    #[doc = "0: The MAC passes all received broadcast frames to the application"]
    PassBroadcast = 0,
    #[doc = "1: The MAC filters all received broadcast frames and does not pass them to the application"]
    FilterBroadcast = 1,
}
impl From<DBF_A> for bool {
    #[inline(always)]
    fn from(variant: DBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBF` reader - Disable broadcast frames"]
pub type DBF_R = crate::BitReader<DBF_A>;
impl DBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBF_A {
        match self.bits {
            false => DBF_A::PassBroadcast,
            true => DBF_A::FilterBroadcast,
        }
    }
    #[doc = "The MAC passes all received broadcast frames to the application"]
    #[inline(always)]
    pub fn is_pass_broadcast(&self) -> bool {
        *self == DBF_A::PassBroadcast
    }
    #[doc = "The MAC filters all received broadcast frames and does not pass them to the application"]
    #[inline(always)]
    pub fn is_filter_broadcast(&self) -> bool {
        *self == DBF_A::FilterBroadcast
    }
}
#[doc = "Field `DBF` writer - Disable broadcast frames"]
pub type DBF_W<'a, REG> = crate::BitWriter<'a, REG, DBF_A>;
impl<'a, REG> DBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The MAC passes all received broadcast frames to the application"]
    #[inline(always)]
    pub fn pass_broadcast(self) -> &'a mut crate::W<REG> {
        self.variant(DBF_A::PassBroadcast)
    }
    #[doc = "The MAC filters all received broadcast frames and does not pass them to the application"]
    #[inline(always)]
    pub fn filter_broadcast(self) -> &'a mut crate::W<REG> {
        self.variant(DBF_A::FilterBroadcast)
    }
}
#[doc = "Pass control frames\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCF_A {
    #[doc = "0: MAC filters all control frames and prevents them from reaching the application"]
    FilterAll = 0,
    #[doc = "1: MAC forwards all control frames, except Pause frame, to the application even if they fail the address filter"]
    ForwardAllExceptPause = 1,
    #[doc = "2: MAC forwards all control frames to the application even if they fail the address filter"]
    ForwardAll = 2,
    #[doc = "3: MAC forwards control frames that pass the address filter to the application"]
    ForwardFilter = 3,
}
impl From<PCF_A> for u8 {
    #[inline(always)]
    fn from(variant: PCF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCF_A {
    type Ux = u8;
}
impl crate::IsEnum for PCF_A {}
#[doc = "Field `PCF` reader - Pass control frames"]
pub type PCF_R = crate::FieldReader<PCF_A>;
impl PCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCF_A {
        match self.bits {
            0 => PCF_A::FilterAll,
            1 => PCF_A::ForwardAllExceptPause,
            2 => PCF_A::ForwardAll,
            3 => PCF_A::ForwardFilter,
            _ => unreachable!(),
        }
    }
    #[doc = "MAC filters all control frames and prevents them from reaching the application"]
    #[inline(always)]
    pub fn is_filter_all(&self) -> bool {
        *self == PCF_A::FilterAll
    }
    #[doc = "MAC forwards all control frames, except Pause frame, to the application even if they fail the address filter"]
    #[inline(always)]
    pub fn is_forward_all_except_pause(&self) -> bool {
        *self == PCF_A::ForwardAllExceptPause
    }
    #[doc = "MAC forwards all control frames to the application even if they fail the address filter"]
    #[inline(always)]
    pub fn is_forward_all(&self) -> bool {
        *self == PCF_A::ForwardAll
    }
    #[doc = "MAC forwards control frames that pass the address filter to the application"]
    #[inline(always)]
    pub fn is_forward_filter(&self) -> bool {
        *self == PCF_A::ForwardFilter
    }
}
#[doc = "Field `PCF` writer - Pass control frames"]
pub type PCF_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PCF_A, crate::Safe>;
impl<'a, REG> PCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MAC filters all control frames and prevents them from reaching the application"]
    #[inline(always)]
    pub fn filter_all(self) -> &'a mut crate::W<REG> {
        self.variant(PCF_A::FilterAll)
    }
    #[doc = "MAC forwards all control frames, except Pause frame, to the application even if they fail the address filter"]
    #[inline(always)]
    pub fn forward_all_except_pause(self) -> &'a mut crate::W<REG> {
        self.variant(PCF_A::ForwardAllExceptPause)
    }
    #[doc = "MAC forwards all control frames to the application even if they fail the address filter"]
    #[inline(always)]
    pub fn forward_all(self) -> &'a mut crate::W<REG> {
        self.variant(PCF_A::ForwardAll)
    }
    #[doc = "MAC forwards control frames that pass the address filter to the application"]
    #[inline(always)]
    pub fn forward_filter(self) -> &'a mut crate::W<REG> {
        self.variant(PCF_A::ForwardFilter)
    }
}
#[doc = "Source address inverse filtering\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIF_A {
    #[doc = "0: Work normally"]
    Normal = 0,
    #[doc = "1: The address check block operates in inverse filtering mode for the source address comparison"]
    Inverse = 1,
}
impl From<SAIF_A> for bool {
    #[inline(always)]
    fn from(variant: SAIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAIF` reader - Source address inverse filtering"]
pub type SAIF_R = crate::BitReader<SAIF_A>;
impl SAIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAIF_A {
        match self.bits {
            false => SAIF_A::Normal,
            true => SAIF_A::Inverse,
        }
    }
    #[doc = "Work normally"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SAIF_A::Normal
    }
    #[doc = "The address check block operates in inverse filtering mode for the source address comparison"]
    #[inline(always)]
    pub fn is_inverse(&self) -> bool {
        *self == SAIF_A::Inverse
    }
}
#[doc = "Field `SAIF` writer - Source address inverse filtering"]
pub type SAIF_W<'a, REG> = crate::BitWriter<'a, REG, SAIF_A>;
impl<'a, REG> SAIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Work normally"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SAIF_A::Normal)
    }
    #[doc = "The address check block operates in inverse filtering mode for the source address comparison"]
    #[inline(always)]
    pub fn inverse(self) -> &'a mut crate::W<REG> {
        self.variant(SAIF_A::Inverse)
    }
}
#[doc = "Source address filter\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAF_A {
    #[doc = "0: If source address filter mismatches, updates the source address filter bit (SAF) in the receive status"]
    UpdateBit = 0,
    #[doc = "1: If source address filter mismatches, frame is dropped"]
    Drop = 1,
}
impl From<SAF_A> for bool {
    #[inline(always)]
    fn from(variant: SAF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAF` reader - Source address filter"]
pub type SAF_R = crate::BitReader<SAF_A>;
impl SAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAF_A {
        match self.bits {
            false => SAF_A::UpdateBit,
            true => SAF_A::Drop,
        }
    }
    #[doc = "If source address filter mismatches, updates the source address filter bit (SAF) in the receive status"]
    #[inline(always)]
    pub fn is_update_bit(&self) -> bool {
        *self == SAF_A::UpdateBit
    }
    #[doc = "If source address filter mismatches, frame is dropped"]
    #[inline(always)]
    pub fn is_drop(&self) -> bool {
        *self == SAF_A::Drop
    }
}
#[doc = "Field `SAF` writer - Source address filter"]
pub type SAF_W<'a, REG> = crate::BitWriter<'a, REG, SAF_A>;
impl<'a, REG> SAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If source address filter mismatches, updates the source address filter bit (SAF) in the receive status"]
    #[inline(always)]
    pub fn update_bit(self) -> &'a mut crate::W<REG> {
        self.variant(SAF_A::UpdateBit)
    }
    #[doc = "If source address filter mismatches, frame is dropped"]
    #[inline(always)]
    pub fn drop(self) -> &'a mut crate::W<REG> {
        self.variant(SAF_A::Drop)
    }
}
#[doc = "Hash or perfect filter\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPF_A {
    #[doc = "0: The MAC performs destination address filtering according to the hash table"]
    HashFilter = 0,
    #[doc = "1: The MAC performs destination address filtering according to the hash table and perfect address filter. A frame is passed if it matches either the hash filter or the perfect address filter"]
    HashOrPerfectFilter = 1,
}
impl From<HPF_A> for bool {
    #[inline(always)]
    fn from(variant: HPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPF` reader - Hash or perfect filter"]
pub type HPF_R = crate::BitReader<HPF_A>;
impl HPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HPF_A {
        match self.bits {
            false => HPF_A::HashFilter,
            true => HPF_A::HashOrPerfectFilter,
        }
    }
    #[doc = "The MAC performs destination address filtering according to the hash table"]
    #[inline(always)]
    pub fn is_hash_filter(&self) -> bool {
        *self == HPF_A::HashFilter
    }
    #[doc = "The MAC performs destination address filtering according to the hash table and perfect address filter. A frame is passed if it matches either the hash filter or the perfect address filter"]
    #[inline(always)]
    pub fn is_hash_or_perfect_filter(&self) -> bool {
        *self == HPF_A::HashOrPerfectFilter
    }
}
#[doc = "Field `HPF` writer - Hash or perfect filter"]
pub type HPF_W<'a, REG> = crate::BitWriter<'a, REG, HPF_A>;
impl<'a, REG> HPF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The MAC performs destination address filtering according to the hash table"]
    #[inline(always)]
    pub fn hash_filter(self) -> &'a mut crate::W<REG> {
        self.variant(HPF_A::HashFilter)
    }
    #[doc = "The MAC performs destination address filtering according to the hash table and perfect address filter. A frame is passed if it matches either the hash filter or the perfect address filter"]
    #[inline(always)]
    pub fn hash_or_perfect_filter(self) -> &'a mut crate::W<REG> {
        self.variant(HPF_A::HashOrPerfectFilter)
    }
}
#[doc = "Receive all\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RA_A {
    #[doc = "0: Normal operation"]
    Normal = 0,
    #[doc = "1: Disable filters and pass all incoming frames"]
    PassAll = 1,
}
impl From<RA_A> for bool {
    #[inline(always)]
    fn from(variant: RA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RA` reader - Receive all"]
pub type RA_R = crate::BitReader<RA_A>;
impl RA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RA_A {
        match self.bits {
            false => RA_A::Normal,
            true => RA_A::PassAll,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RA_A::Normal
    }
    #[doc = "Disable filters and pass all incoming frames"]
    #[inline(always)]
    pub fn is_pass_all(&self) -> bool {
        *self == RA_A::PassAll
    }
}
#[doc = "Field `RA` writer - Receive all"]
pub type RA_W<'a, REG> = crate::BitWriter<'a, REG, RA_A>;
impl<'a, REG> RA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(RA_A::Normal)
    }
    #[doc = "Disable filters and pass all incoming frames"]
    #[inline(always)]
    pub fn pass_all(self) -> &'a mut crate::W<REG> {
        self.variant(RA_A::PassAll)
    }
}
impl R {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline(always)]
    pub fn huc(&self) -> HUC_R {
        HUC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline(always)]
    pub fn hmc(&self) -> HMC_R {
        HMC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination address inverse filtering"]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass multicast"]
    #[inline(always)]
    pub fn pmc(&self) -> PMC_R {
        PMC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable broadcast frames"]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Source address inverse filtering"]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source address filter"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or perfect filter"]
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive all"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACFRMF")
            .field("pr", &self.pr())
            .field("huc", &self.huc())
            .field("hmc", &self.hmc())
            .field("daif", &self.daif())
            .field("pmc", &self.pmc())
            .field("dbf", &self.dbf())
            .field("pcf", &self.pcf())
            .field("saif", &self.saif())
            .field("saf", &self.saf())
            .field("hpf", &self.hpf())
            .field("ra", &self.ra())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<'_, MACFRMF_SPEC> {
        PR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline(always)]
    pub fn huc(&mut self) -> HUC_W<'_, MACFRMF_SPEC> {
        HUC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline(always)]
    pub fn hmc(&mut self) -> HMC_W<'_, MACFRMF_SPEC> {
        HMC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Destination address inverse filtering"]
    #[inline(always)]
    pub fn daif(&mut self) -> DAIF_W<'_, MACFRMF_SPEC> {
        DAIF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pass multicast"]
    #[inline(always)]
    pub fn pmc(&mut self) -> PMC_W<'_, MACFRMF_SPEC> {
        PMC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Disable broadcast frames"]
    #[inline(always)]
    pub fn dbf(&mut self) -> DBF_W<'_, MACFRMF_SPEC> {
        DBF_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W<'_, MACFRMF_SPEC> {
        PCF_W::new(self, 6)
    }
    #[doc = "Bit 8 - Source address inverse filtering"]
    #[inline(always)]
    pub fn saif(&mut self) -> SAIF_W<'_, MACFRMF_SPEC> {
        SAIF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Source address filter"]
    #[inline(always)]
    pub fn saf(&mut self) -> SAF_W<'_, MACFRMF_SPEC> {
        SAF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Hash or perfect filter"]
    #[inline(always)]
    pub fn hpf(&mut self) -> HPF_W<'_, MACFRMF_SPEC> {
        HPF_W::new(self, 10)
    }
    #[doc = "Bit 31 - Receive all"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W<'_, MACFRMF_SPEC> {
        RA_W::new(self, 31)
    }
}
#[doc = "Ethernet MAC frame filter register\n\nYou can [`read`](crate::Reg::read) this register and get [`macfrmf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macfrmf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACFRMF_SPEC;
impl crate::RegisterSpec for MACFRMF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macfrmf::R`](R) reader structure"]
impl crate::Readable for MACFRMF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macfrmf::W`](W) writer structure"]
impl crate::Writable for MACFRMF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACFRMF to value 0"]
impl crate::Resettable for MACFRMF_SPEC {}
