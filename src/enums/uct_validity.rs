#[derive(Debug, Clone, PartialEq, Copy)]
pub enum UCTValidity {
    LikelyReal,
    PossibleCrossTag,
    PossibleManeuver,
    Inconclusive,
}
