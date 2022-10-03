
use criterion::*;


macro_rules! bitloop {
    ( $bb:expr , $body:expr ) => {
        while $bb.has_bits() {
            let sq: usize = $bb.tzcnt();

            ($body)(sq);

            $bb = $bb.blsr();
        }
    }
}


#[derive(Clone, Copy)]
struct Bitboard(pub u64);

impl Bitboard {
    pub const fn has_bits(self) -> bool {
        self.0 != 0
    }
    
    pub const fn tzcnt(self) -> usize {
        self.0.trailing_zeros() as usize
    }
    
    pub const fn blsr(self) -> Bitboard {
        Bitboard(self.0 & self.0.wrapping_sub(1))
    }
}

fn impl1(data: Vec<u64>) {
    for bb in data {
        let mut bb = bb;
        let mut _cnt: usize = 0;
        while bb != 0 {
            
            let sq: usize = bb.trailing_zeros() as usize;
            
            _cnt += sq;
            
            bb = bb & bb.wrapping_sub(1);
        }
    }
}

fn impl2(data: Vec<Bitboard>) {
    for bb in data {
        let mut bb = bb;
        let mut _cnt: usize = 0;
        while bb.has_bits() {
            
            let sq: usize = bb.tzcnt();
            
            _cnt += sq;
            
            bb = bb.blsr();
        }
    }
}

fn impl3(data: Vec<Bitboard>) {
    for bb in data {
        let mut bb = bb;
        let mut _cnt: usize = 0;
        bitloop!( bb , ( |sq| {_cnt += sq;} ) );
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut data1: Vec<u64> = Vec::new();
    let mut data2: Vec<Bitboard> = Vec::new();
    let mut data3: Vec<Bitboard> = Vec::new();
    
    for _ in 0..1_000_000 {
        let n = rand::random::<u64>();
        data1.push(n);
        data2.push(Bitboard(n));
        data3.push(Bitboard(n));
    }

    c.bench_function(
        "<<bitloop, u64, manual>>",
        move |b| {
            b.iter_batched(|| data1.clone(), |d| impl1(d), BatchSize::SmallInput)
        }
    );
    c.bench_function(
        "<<bitloop, Bitboard, manual>>",
        move |b| {
            b.iter_batched(|| data2.clone(), |d| impl2(d), BatchSize::SmallInput)
        }
    );
    c.bench_function(
        "<<bitloop, Bitboard, macro>>",
        move |b| {
            b.iter_batched(|| data3.clone(), |d| impl3(d), BatchSize::SmallInput)
        }
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
