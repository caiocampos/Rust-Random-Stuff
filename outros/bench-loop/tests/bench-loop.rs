#![feature(test)]

extern crate test;

#[cfg(test)]

mod tests {
    use bench_loop::hard_work;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(12.015541800917314, hard_work(2));
    }

    #[bench]
    fn bench_hard_work_while(b: &mut Bencher) {
        b.iter(|| hard_work_while(100000));
    }

    #[bench]
    fn bench_hard_work_iter(b: &mut Bencher) {
        b.iter(|| hard_work_iter(100000));
    }

    #[bench]
    fn bench_hard_work_while_sum(b: &mut Bencher) {
        b.iter(|| hard_work_while_sum(100000));
    }

    #[bench]
    fn bench_hard_work_iter_sum(b: &mut Bencher) {
        b.iter(|| hard_work_iter_sum(100000));
    }

    fn hard_work_while(limit: u64) {
        let mut i = 0;
        while i < limit {
            hard_work(i);
            i += 1;
        }
    }

    fn hard_work_iter(limit: u64) {
        (0..limit).for_each(|i| {
            hard_work(i);
        });
    }

    fn hard_work_while_sum(limit: u64) {
        let (mut i, mut sum) = (0, 0.0);
        while i < limit {
            sum = sum + hard_work(i);
            i += 1;
        }
    }

    fn hard_work_iter_sum(limit: u64) {
        (0..limit).map(hard_work).sum::<f64>();
    }
}
