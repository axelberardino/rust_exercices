mod ranker;
mod user_rank;

fn main() {
    let mut ranker = ranker::Ranker::new();
    println!("{}: {:?}", line!(), ranker);
    ranker.add(99);
    ranker.add(100);
    ranker.add(100);
    ranker.add(100);
    ranker.add(100);
    ranker.add(0);
    println!("{}: {:?}", line!(), ranker);
    ranker.transfer(1, 2);
    ranker.transfer(100, 99);
    println!("{}: {:?}", line!(), ranker);

    println!("{}: {}", line!(), ranker.get_rank(101));
    println!("{}: {}", line!(), ranker.get_rank(100));
    println!("{}: {}", line!(), ranker.get_rank(99));
    println!("{}: {}", line!(), ranker.get_rank(98));
    println!("{}: {}", line!(), ranker.get_rank(1));
    println!("{}: {}", line!(), ranker.get_rank(0));
    println!("{}: {:?}", line!(), ranker.get_ranks(&[0, 1, 2, 3, 4]));

    println!("{}: {:?}", line!(), ranker);

    let user_rank = user_rank::UserRank::new();
    user_rank.update_user_pops("u-1".into(), 10);
    user_rank.update_user_pops("u-2".into(), 20);
    user_rank.update_user_pops("u-3".into(), 30);
    user_rank.update_user_pops("u-1".into(), 40);
    println!("{}: {:?}", line!(), user_rank);
    println!(
        "{}: {:?}",
        line!(),
        user_rank.world_rankings(&[0, 10, 20, 30, 40])
    );
}
