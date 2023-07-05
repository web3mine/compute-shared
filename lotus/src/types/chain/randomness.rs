pub type Randomness = Vec<u8>;

pub type Entropy = Vec<u8>;

#[derive(PartialEq, Debug, Eq, Copy, Clone)]
pub enum DomainSeparationTag {
    TicketProduction = 1,
    ElectionProofProduction,
    WinningPoStChallengeSeed,
    WindowedPoStChallengeSeed,
    SealRandomness,
    InteractiveSealChallengeSeed,
    WindowedPoStDeadlineAssignment,
    MarketDealCronSeed,
    PoStChainCommit,
}
impl DomainSeparationTag {
    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::DomainSeparationTag;

    #[test]
    fn test_conversion() {
        assert_eq!(DomainSeparationTag::SealRandomness.to_u8(), 5)
    }
}
