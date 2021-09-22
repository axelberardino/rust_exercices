use std::collections::HashMap;
use std::sync::Mutex;

/// Handle the pops world ranking. Updates users pops, and get the ranking given
/// a list of pops numbers.
/// This object is thread safe.
#[derive(Default, Debug)]
pub struct UserRank {
    // mutex
    // Mutex<HashMap<
    user_pops: Mutex<HashMap<String, u32>>,
    ranker: Mutex<crate::ranker::Ranker>,
}

// if someone got more descendant than this. it will be put in that bucket
const MAX_POPS: u32 = 9999;

impl UserRank {
    pub fn new() -> Self {
        UserRank {
            ..Default::default()
        }
    }

    /// updates the world ranking by update a user pops number.
    pub fn update_user_pops(&self, user_uuid: String, pops_number: u32) {
        let new_pops_number = std::cmp::min(pops_number, MAX_POPS);
        let mut user_guard = self.user_pops.lock().unwrap();
        let mut ranker_guard = self.ranker.lock().unwrap();
        match user_guard.get(&user_uuid) {
            Some(old_pops_number) => ranker_guard.transfer(*old_pops_number, new_pops_number),
            None => ranker_guard.add(new_pops_number),
        }
        user_guard.insert(user_uuid, new_pops_number);
    }

    /// return the world rankings of the given pops.
    pub fn world_rankings(&self, pops_numbers: &[u32]) -> Vec<u32> {
        let mut ranker_guard = self.ranker.lock().unwrap();
        ranker_guard.get_ranks(pops_numbers)
    }
}

////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // checks empty state is fine.
    fn empty_state() {
        let user_rank = UserRank::new();
        user_rank.update_user_pops("u-1".into(), 10);
        user_rank.update_user_pops("u-2".into(), 20);
        user_rank.update_user_pops("u-3".into(), 30);
        user_rank.update_user_pops("u-1".into(), 40);
        assert_eq!(
            vec![4, 4, 3, 2, 1],
            user_rank.world_rankings(&[0, 10, 20, 30, 40])
        );
    }
}
