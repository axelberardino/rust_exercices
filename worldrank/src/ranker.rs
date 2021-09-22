/// This structure is optimized for recomputing ranks lazily. It will update
/// pops, but it will only recompute ranks when trying to get one in range.
/// Example:
///   pops(100) -> 1 person     ranks(100) => #1
///   pops(95)  -> 0 person     ranks(95)  => #2
///   pops(90)  -> 5 people     ranks(90)  => #2
///   pops(50)  -> 10 people    ranks(50)  => #7
///                             ranks(51)  => #17
#[derive(Default, Debug)]
pub struct Ranker {
    // Number of people with a given pops number.
    // pops[3] = 5678 => there is 5678 users with a pops number of 3.
    pops: Vec<u32>,
    // Current ranking associated to a number of pops.
    // ranks[3] = 147000 => having 3 pops, put you at rank 147000.
    ranks: Vec<u32>,
    // Every ranks below this index, must be recomputed.
    // Number of pops is used as an index, a big one increase the chance of
    // recomputing many ranks (because it will be top ranked).
    // Example:
    //   dirty_index = 90
    //
    //   ranks(100) => #1  Fine
    //   ranks(95)  => #2  Fine
    //   ranks(90)  => #2  Fine
    //   ranks(50)  => #7  Must be recomputed
    //   ranks(51)  => #17 Must be recomputed
    dirty_index: u32,
}

/// get the highest values between an arbitrary number of values.
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::max($x, max!($($z),*)));
}

impl Ranker {
    /// instanciate a default ranker.
    pub fn new() -> Self {
        Ranker {
            ..Default::default()
        }
    }

    /// change the size of the internal buckets.
    fn resize(&mut self, size: u32) {
        self.pops.resize((size + 1) as usize, 0);
        self.ranks.resize((size + 1) as usize, 0);
    }

    /// add a new user with a given number of pops.
    pub fn add(&mut self, pops_number: u32) {
        if pops_number as usize >= self.pops.len() {
            self.resize(pops_number);
        }
        self.pops[pops_number as usize] += 1;
        self.dirty_index = max!(self.dirty_index, pops_number);
    }

    /// change the pops number of a given user.
    pub fn transfer(&mut self, old_pops_number: u32, new_pops_number: u32) {
        if old_pops_number == new_pops_number {
            return;
        }
        let max_pops_number = max!(old_pops_number, new_pops_number);
        if max_pops_number as usize >= self.pops.len() {
            self.resize(max_pops_number);
        }

        if self.pops[old_pops_number as usize] > 0 {
            self.pops[old_pops_number as usize] -= 1;
        }
        self.pops[new_pops_number as usize] += 1;
        self.dirty_index = max!(self.dirty_index, old_pops_number, new_pops_number);
    }

    /// get the ranks associated to a pops number.
    pub fn get_rank(&mut self, pops_number: u32) -> u32 {
        // pops number is so high, you're number #1.
        if pops_number as usize >= self.ranks.len() {
            return 1;
        }

        // lazily computes ranking.
        while pops_number < self.dirty_index {
            let new_rank =
                self.ranks[self.dirty_index as usize] + self.pops[self.dirty_index as usize];
            self.ranks[(self.dirty_index - 1) as usize] = new_rank;
            self.dirty_index -= 1;
        }

        self.ranks[pops_number as usize] + 1
    }

    pub fn get_ranks(&mut self, pops_numbers: &[u32]) -> Vec<u32> {
        pops_numbers
            .iter()
            .map(|pops_number| self.get_rank(*pops_number))
            .collect::<Vec<u32>>()
    }
}

////////////////

#[cfg(test)]
mod tests {
    use super::*;

    const NB_USER: u32 = 1234;

    #[test]
    // checks empty state is fine.
    fn empty_state() {
        let mut ranker = Ranker::new();
        assert_eq!(ranker.get_rank(0), 1);
        assert_eq!(ranker.get_rank(100000), 1);
    }

    #[test]
    // add pops and check we get at the correct ranks.
    fn add_get_one() {
        let mut ranker = Ranker::new();
        for i in 0..NB_USER {
            ranker.add(i);
            for j in 0..i {
                assert_eq!(i + 1 - j, ranker.get_rank(j));
            }
        }
    }

    #[test]
    // test adding many pops works.
    fn add_many_get_many() {
        let mut ranker = Ranker::new();
        let expected_idx = [0, NB_USER / 2, NB_USER / 4, NB_USER * 3 / 4];
        let mut expected_values = [0, 0, 0, 0];
        for (idx, val) in expected_idx.iter().enumerate() {
            expected_values[idx] = ranker.get_rank(*val)
        }
        let mut rng = rand::thread_rng();
        for order in 0..10000 {
            let nb_pops = rand::Rng::gen_range(&mut rng, 0..NB_USER);
            ranker.add(nb_pops);
            let iter = if order % 2 == 0 {
                expected_idx.iter()
            } else {
                expected_idx.iter() // .rev()
            };
            for (idx, val) in iter.enumerate() {
                if nb_pops > *val {
                    expected_values[idx] += 1;
                }
                assert_eq!(expected_values[idx], ranker.get_rank(*val));
            }
        }
        assert_eq!(expected_values.to_vec(), ranker.get_ranks(&expected_idx));
    }

    fn random_get_valid(rng: &mut rand::prelude::ThreadRng, ranker: &Ranker) -> u32 {
        loop {
            let idx = rand::Rng::gen_range(rng, 0..NB_USER);
            if ranker.pops[idx as usize] > 0 {
                return idx;
            }
        }
    }

    #[test]
    // test transfering pops works.
    fn transfer_many() {
        let mut ranker = Ranker::new();
        // 10 pops in each cells.
        for i in 0..NB_USER {
            for _ in 0..10 {
                ranker.add(i);
            }
        }

        let expected_idx = [0, NB_USER / 2, NB_USER / 4, NB_USER * 3 / 4];
        let mut expected_values = [0, 0, 0, 0];
        for (idx, val) in expected_idx.iter().enumerate() {
            expected_values[idx] = ranker.get_rank(*val)
        }
        let mut rng = rand::thread_rng();
        for order in 0..10000 {
            let nb_pops_from = random_get_valid(&mut rng, &ranker);
            let nb_pops_to = rand::Rng::gen_range(&mut rng, 0..NB_USER);
            ranker.transfer(nb_pops_from, nb_pops_to);
            let iter = if order % 2 == 0 {
                expected_idx.iter()
            } else {
                expected_idx.iter() // .rev()
            };
            for (idx, val) in iter.enumerate() {
                if nb_pops_from > *val {
                    expected_values[idx] -= 1;
                }
                if nb_pops_to > *val {
                    expected_values[idx] += 1;
                }
                assert_eq!(expected_values[idx], ranker.get_rank(*val));
            }
        }
        assert_eq!(expected_values.to_vec(), ranker.get_ranks(&expected_idx));
    }

    #[test]
    // test get_ranks is working.
    fn multiple_get_ranks() {
        let mut ranker = Ranker::new();
        assert_eq!(Vec::<u32>::new(), ranker.get_ranks(&[]));
        assert_eq!(vec![1], ranker.get_ranks(&[1000000]));

        ranker.add(3);
        ranker.add(5);
        ranker.add(5);
        ranker.add(8);
        ranker.add(8);
        ranker.add(8);

        let expected_idx = [0, 6, 4, 1, 8, 5, 3, 10000];
        let expected_val = vec![7, 4, 6, 7, 1, 4, 6, 1];
        assert_eq!(expected_val, ranker.get_ranks(&expected_idx));
    }
}
