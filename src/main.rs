mod adapters;

use adapters::{CHashMapTable, ContrieTable, DashMapTable, FlurryTable, MutexStdTable};
use bustle::*;
use std::thread::sleep;
use std::time::Duration;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

fn gc_cycle() {
    sleep(Duration::from_millis(20000));
    let mut new_guard = crossbeam_epoch::pin();
    new_guard.flush();
    for _ in 0..32 {
        new_guard.repin();
    }
    let mut old_guard = crossbeam_epoch_old::pin();
    old_guard.flush();

    for _ in 0..32 {
        old_guard.repin();
    }
}

fn read_heavy(n: usize) -> Workload {
    *Workload::new(n, Mix::read_heavy())
        .initial_capacity_log2(24)
        .prefill_fraction(0.8)
        .operations(1.5)
}

fn rg_mix() -> Mix {
    Mix {
        read: 5,
        insert: 80,
        remove: 5,
        update: 10,
        upsert: 0,
    }
}

fn rapid_grow(n: usize) -> Workload {
    *Workload::new(n, rg_mix())
        .initial_capacity_log2(24)
        .prefill_fraction(0.0)
        .operations(1.5)
}

fn ex_mix() -> Mix {
    Mix {
        read: 10,
        insert: 40,
        remove: 40,
        update: 10,
        upsert: 0,
    }
}

fn exchange(n: usize) -> Workload {
    *Workload::new(n, ex_mix())
        .initial_capacity_log2(24)
        .prefill_fraction(0.8)
        .operations(1.5)
}

static CPUS: &[usize] = &[1, 4, 8, 16, 32];

fn exchange_task() {
    println!("== exchange");
    println!("-- MutexStd");
    for n in CPUS.iter().cloned() {
        exchange(n).run::<MutexStdTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- CHashMap");
    for n in CPUS.iter().cloned() {
        exchange(n).run::<CHashMapTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- Flurry");
    for n in CPUS.iter().cloned() {
        exchange(n).run::<FlurryTable>();
        gc_cycle();
    }
    println!("");
    println!("-- Contrie");
    for n in CPUS.iter().cloned() {
        exchange(n).run::<ContrieTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- DashMap");
    for n in CPUS.iter().cloned() {
        exchange(n).run::<DashMapTable<u64>>();
        gc_cycle();
    }
    println!("==");
}

fn cache_task() {
    println!("== cache");
    println!("-- MutexStd");
    for n in CPUS.iter().cloned() {
        read_heavy(n).run::<MutexStdTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- CHashMap");
    for n in CPUS.iter().cloned() {
        read_heavy(n).run::<CHashMapTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- Flurry");
    for n in CPUS.iter().cloned() {
        read_heavy(n).run::<FlurryTable>();
        gc_cycle();
    }
    println!("");
    println!("-- Contrie");
    for n in CPUS.iter().cloned() {
        read_heavy(n).run::<ContrieTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- DashMap");
    for n in CPUS.iter().cloned() {
        read_heavy(n).run::<DashMapTable<u64>>();
        gc_cycle();
    }
    println!("==");
}

fn rapid_grow_task() {
    println!("== rapid grow");
    println!("-- MutexStd");
    for n in CPUS.iter().cloned() {
        rapid_grow(n).run::<MutexStdTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- CHashMap");
    for n in CPUS.iter().cloned() {
        rapid_grow(n).run::<CHashMapTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- Flurry");
    for n in CPUS.iter().cloned() {
        rapid_grow(n).run::<FlurryTable>();
        gc_cycle();
    }
    println!("");
    println!("-- Contrie");
    for n in CPUS.iter().cloned() {
        rapid_grow(n).run::<ContrieTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- DashMap");
    for n in CPUS.iter().cloned() {
        rapid_grow(n).run::<DashMapTable<u64>>();
        gc_cycle();
    }
    println!("==");
}

fn main() {
    tracing_subscriber::fmt::init();

    cache_task();
    exchange_task();
    rapid_grow_task();
}
