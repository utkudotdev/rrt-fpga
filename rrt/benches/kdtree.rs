use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use na::SVector;
use nalgebra as na;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use rrt::cpu::ds::kdtree::KdTree;
use rrt::shared::ds::point_list::PointList;

fn generate_points<const DIMS: usize>(
    count: usize,
    dist: &Uniform<f32>,
    rng: &mut StdRng,
) -> Vec<SVector<f32, DIMS>> {
    (0..count)
        .map(|_| {
            let mut p = SVector::<f32, DIMS>::zeros();
            for i in 0..DIMS {
                p[i] = dist.sample(rng);
            }
            p
        })
        .collect()
}

fn run_insertion_bench<const DIMS: usize, const LEAF_CAP: usize>(
    group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
    rng: &mut StdRng,
    dist: &Uniform<f32>,
) {
    for point_count in [256, 1024, 4096, 16384, 65536].iter() {
        group.throughput(Throughput::Elements(*point_count as u64));
        let points = generate_points::<DIMS>(*point_count, dist, rng);

        group.bench_with_input(
            BenchmarkId::new(format!("Leaf{}", LEAF_CAP), point_count),
            &points,
            |b, p| {
                b.iter(|| {
                    let mut tree = KdTree::<DIMS, LEAF_CAP>::empty();
                    for point in p {
                        tree.add_point(*point);
                    }
                });
            },
        );
    }
}

fn bench_insertion(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(10);
    let dist = Uniform::new(-1.0, 1.0);

    {
        let mut group_2d = c.benchmark_group("KDTree_Insert_2D");
        run_insertion_bench::<2, 1>(&mut group_2d, &mut rng, &dist);
        run_insertion_bench::<2, 4>(&mut group_2d, &mut rng, &dist);
        run_insertion_bench::<2, 8>(&mut group_2d, &mut rng, &dist);
        run_insertion_bench::<2, 16>(&mut group_2d, &mut rng, &dist);
    }

    {
        let mut group_5d = c.benchmark_group("KDTree_Insert_5D");
        run_insertion_bench::<5, 1>(&mut group_5d, &mut rng, &dist);
        run_insertion_bench::<5, 4>(&mut group_5d, &mut rng, &dist);
        run_insertion_bench::<5, 8>(&mut group_5d, &mut rng, &dist);
        run_insertion_bench::<5, 16>(&mut group_5d, &mut rng, &dist);
    }
}

fn run_lookup_bench<const DIMS: usize, const LEAF_CAP: usize>(
    group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
    rng: &mut StdRng,
    dist: &Uniform<f32>,
) {
    const QUERY_COUNT: usize = 256;
    for point_count in [256, 1024, 4096, 16384, 65536].iter() {
        group.throughput(Throughput::Elements(QUERY_COUNT as u64));

        let points = generate_points::<DIMS>(*point_count, dist, rng);
        let mut tree = KdTree::<DIMS, LEAF_CAP>::empty();
        for p in &points {
            tree.add_point(*p);
        }

        let queries = generate_points::<DIMS>(QUERY_COUNT, dist, rng);

        group.bench_with_input(
            BenchmarkId::new(format!("Leaf{}", LEAF_CAP), point_count),
            &queries,
            |b, q| {
                b.iter(|| {
                    for query in q {
                        black_box(tree.closest_point(*query));
                    }
                });
            },
        );
    }
}

fn bench_lookup(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(10);
    let dist = Uniform::new(-1.0, 1.0);

    {
        let mut group_2d = c.benchmark_group("KDTree_Lookup_2D");
        run_lookup_bench::<2, 1>(&mut group_2d, &mut rng, &dist);
        run_lookup_bench::<2, 4>(&mut group_2d, &mut rng, &dist);
        run_lookup_bench::<2, 8>(&mut group_2d, &mut rng, &dist);
        run_lookup_bench::<2, 16>(&mut group_2d, &mut rng, &dist);
    }

    {
        let mut group_5d = c.benchmark_group("KDTree_Lookup_5D");
        run_lookup_bench::<5, 1>(&mut group_5d, &mut rng, &dist);
        run_lookup_bench::<5, 4>(&mut group_5d, &mut rng, &dist);
        run_lookup_bench::<5, 8>(&mut group_5d, &mut rng, &dist);
        run_lookup_bench::<5, 16>(&mut group_5d, &mut rng, &dist);
    }
}

criterion_group!(benches, bench_insertion, bench_lookup);
criterion_main!(benches);
