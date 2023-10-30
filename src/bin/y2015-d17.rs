use itertools::Itertools;

fn main() {
	let input_str = aoc::get_input!();

	let buckets = {
		let mut buckets = input_str.trim()
			.split('\n')
			.map(|b| b.parse::<usize>().unwrap())
			.collect::<Vec<_>>();
		buckets.sort_unstable();
		buckets
	};

	let target_amount = 150;

	let combos = calc_valid_bucket_combos(buckets, target_amount);
	println!("part 1: number of container combos: {}", combos.len());
}


fn calc_valid_bucket_combos(buckets: Vec<usize>, target_amount: usize) -> Vec<Vec<usize>> {
	(0..buckets.len())
		.flat_map(|i| buckets.iter().copied().combinations(i))
		.filter(|combo| combo.iter().sum::<usize>() == target_amount)
		.collect::<Vec<_>>()
}
