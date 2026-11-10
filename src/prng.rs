const GAMMA: u64 = 0x9e37_79b9_7f4a_7c15;
const MIX_MULTIPLIER_FIRST: u64 = 0xbf58_476d_1ce4_e5b9;
const MIX_MULTIPLIER_SECOND: u64 = 0x94d0_49bb_1331_11eb;
const PERCENT_MAX: u64 = 100;
const _: () = assert!(GAMMA != 0, "a zero increment would never move the state");
const _: () = assert!(PERCENT_MAX == 100, "a chance is a percentage");

pub(crate) type Seed = u64;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Prng {
    state: u64,
}

impl Prng {
    pub(crate) const fn boolean(&mut self) -> bool {
        self.int() & 1 == 1
    }

    pub(crate) fn chance(&mut self, percent: u64) -> bool {
        assert!(percent <= PERCENT_MAX, "a chance is at most certain");

        let drawn = self.int_inclusive(PERCENT_MAX.strict_sub(1));

        assert!(drawn < PERCENT_MAX, "a percentile draw stays under a hundred");

        drawn < percent
    }

    pub(crate) const fn from_seed(seed: Seed) -> Self {
        Self { state: seed }
    }

    pub(crate) fn index(&mut self, count: u64) -> u64 {
        assert!(count >= 1, "an index is drawn from at least one slot");

        let drawn = self.int().rem_euclid(count);

        assert!(drawn < count, "an index stays inside its count");

        drawn
    }

    pub(crate) const fn int(&mut self) -> u64 {
        self.state = self.state.wrapping_add(GAMMA);

        let mut word = self.state;

        word = (word ^ word.wrapping_shr(30)).wrapping_mul(MIX_MULTIPLIER_FIRST);
        word = (word ^ word.wrapping_shr(27)).wrapping_mul(MIX_MULTIPLIER_SECOND);

        word ^ word.wrapping_shr(31)
    }

    pub(crate) fn int_inclusive(&mut self, limit: u64) -> u64 {
        assert!(limit < u64::MAX, "an inclusive limit leaves room for one more value");

        let drawn = self.int().rem_euclid(limit.strict_add(1));

        assert!(drawn <= limit, "an inclusive draw stays at or under its limit");

        drawn
    }
}

#[cfg(test)]
mod tests {
    use super::{PERCENT_MAX, Prng};

    const SAMPLE_COUNT: u32 = 4_096;

    #[test]
    fn a_boolean_lands_on_both_faces() {
        let mut prng = Prng::from_seed(19);
        let mut true_count: u32 = 0;

        for _sample_index in 0..SAMPLE_COUNT {
            true_count = true_count.strict_add(u32::from(prng.boolean()));
        }

        assert!(true_count > SAMPLE_COUNT.div_euclid(4), "true lands a fair share");

        assert!(true_count < SAMPLE_COUNT.strict_mul(3).div_euclid(4), "false lands a fair share");
    }

    #[test]
    fn a_boolean_reads_the_low_bit_of_the_published_sequence() {
        let mut prng = Prng::from_seed(0);

        assert!(prng.boolean(), "the first published draw is odd");
        assert!(!prng.boolean(), "the second published draw is even");
    }

    #[test]
    fn a_bound_of_zero_is_the_only_value_it_can_draw() {
        let mut prng = Prng::from_seed(11);

        for _sample_index in 0..64_u32 {
            assert_eq!(prng.int_inclusive(0), 0, "a zero limit draws zero");
        }
    }

    #[test]
    fn a_bounded_draw_stays_inside_its_bound_and_reaches_both_ends() {
        let mut prng = Prng::from_seed(7);
        let mut is_low_seen = false;
        let mut is_high_seen = false;

        for _sample_index in 0..SAMPLE_COUNT {
            let drawn = prng.int_inclusive(9);

            assert!(drawn <= 9, "a draw stays at or under its limit");

            is_low_seen |= drawn == 0;
            is_high_seen |= drawn == 9;
        }

        assert!(is_low_seen, "the low end is reached");
        assert!(is_high_seen, "the high end is reached");
    }

    #[test]
    fn a_certainty_and_an_impossibility_are_honoured_exactly() {
        let mut prng = Prng::from_seed(17);

        for _sample_index in 0..SAMPLE_COUNT {
            assert!(prng.chance(PERCENT_MAX), "a certain chance always lands");
            assert!(!prng.chance(0), "an impossible chance never lands");
        }
    }

    #[test]
    fn a_zero_seed_draws_the_published_split_mix_sequence() {
        let mut prng = Prng::from_seed(0);

        assert_eq!(prng.int(), 0xe220_a839_7b1d_cdaf, "the first published draw");
        assert_eq!(prng.int(), 0x6e78_9e6a_a1b9_65f4, "the second published draw");
        assert_eq!(prng.int(), 0x06c4_5d18_8009_454f, "the third published draw");
        assert_eq!(prng.int(), 0xf88b_b8a8_724c_81ec, "the fourth published draw");
        assert_eq!(prng.int(), 0x1b39_896a_51a8_749b, "the fifth published draw");
        assert_ne!(prng.int(), 0x1b39_896a_51a8_749b, "the sixth draw moves on");
    }

    #[test]
    fn a_zero_seed_still_moves() {
        let mut prng = Prng::from_seed(0);
        let first = prng.int();
        let second = prng.int();

        assert_ne!(first, 0, "a zero seed does not draw zero");
        assert_ne!(first, second, "a zero seed moves between draws");
    }

    #[test]
    #[should_panic(expected = "an index is drawn from at least one slot")]
    fn an_index_into_nothing_is_a_programmer_error() {
        let _index = Prng::from_seed(1).index(0);
    }

    #[test]
    fn an_index_stays_inside_the_collection() {
        let mut prng = Prng::from_seed(13);

        for _sample_index in 0..SAMPLE_COUNT {
            assert_eq!(prng.index(1), 0, "one slot has one index");
            assert!(prng.index(5) < 5, "five slots have five indices");
        }
    }

    #[test]
    fn the_same_seed_replays_the_same_sequence() {
        let mut first = Prng::from_seed(123);
        let mut second = Prng::from_seed(123);
        let mut third = Prng::from_seed(124);
        let mut is_identical = true;

        for _sample_index in 0..SAMPLE_COUNT {
            let drawn = first.int();

            assert_eq!(drawn, second.int(), "one seed draws one sequence");

            is_identical &= third.int() == drawn;
        }

        assert!(!is_identical, "a neighbouring seed draws a different sequence");
    }
}
