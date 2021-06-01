use criterion::{
    black_box, criterion_group, criterion_main, measurement::Measurement, AxisScale,
    BenchmarkGroup, BenchmarkId, Criterion, PlotConfiguration,
};
use lattice_graph::SquareGraph;
use petgraph::{algo, graph::*, visit::EdgeRef};
use rand::{prelude::StdRng, Rng, SeedableRng};

fn graph_build(c: &mut Criterion) {
    let mut g = c.benchmark_group("build");
    g.bench_function("petgraph", |b| {
        b.iter_with_setup(
            || petgraph_gen(3, 2),
            |v| {
                black_box(UnGraph::<(), u32>::from_edges(v));
            },
        )
    });
    g.bench_function("lattice", |b| {
        b.iter_with_setup(
            || {},
            |_v| {
                black_box(SquareGraph::<_, _, u32>::new_edge_graph(3, 2, |i, j, d| {
                    i + j + if d.is_horizontal() { 0 } else { 1 }
                }));
            },
        )
    });
}

fn petgraph_gen(h: u32, v: u32) -> Vec<(u32, u32, u32)> {
    let iv = v;
    let h = h;
    let mut v = Vec::new();
    for i in 0..h {
        for j in 0..iv {
            v.push((i * iv + j, i * iv + j + 1, i + j));
            v.push((i * iv + j, (i + 1) * iv + j, i + j + 1));
        }
        let j = h;
        v.push((i * iv + j, (i + 1) * iv + j, i + j + 1));
    }
    let i = iv;
    for j in 0..h {
        v.push((i * iv + j, i * iv + j + 1, i + j));
    }
    v
}

fn graph_search_inner(c: &mut Criterion, h: u32, v: u32, seed: u64, name: &'static str) {
    let mut g = c.benchmark_group(name);
    g.bench_function("petgraph", |b| {
        let mut r = StdRng::seed_from_u64(seed);
        let g = UnGraph::<(), u32>::from_edges(petgraph_gen(h, v));
        b.iter_with_setup(
            || (&g, (r.gen_range(0..v), r.gen_range(0..h))),
            |(g, t)| {
                black_box(algo::astar(
                    g,
                    node_index(0),
                    |x| x.index() as u32 == t.0 + v * t.1,
                    |x| *x.weight(),
                    |_| 0,
                ));
            },
        )
    });
    g.bench_function("lattice", |b| {
        let mut r = StdRng::seed_from_u64(seed);
        let g = SquareGraph::<_, _, u32>::new_edge_graph(h as usize, v as usize, |i, j, d| {
            i + j + if d.is_horizontal() { 0 } else { 1 }
        });
        b.iter_with_setup(
            || (&g, (r.gen_range(0..h) as usize, r.gen_range(0..v) as usize)),
            |(g, t)| {
                black_box(algo::astar(
                    g,
                    (0, 0).into(),
                    |x| x == t,
                    |x| *x.weight(),
                    |_| 0,
                ));
            },
        )
    });
}

fn graph_search_small(c: &mut Criterion) {
    graph_search_inner(c, 4, 3, 12345, "astar_small")
}

criterion_group!(bench_graph, graph_build, graph_search_small);
criterion_main!(bench_graph);
