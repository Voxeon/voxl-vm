use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use vxl_iset::syscall_handler::SyscallHandler;
use vxlvm::validator::{BulkValidator, Validator};
use vxlvm::vm::VM;

struct System;

impl System {
    pub fn new() -> Self {
        return Self;
    }
}

impl SyscallHandler<VM> for System {
    fn execute_target_specific_call(&mut self, _call: u64, _machine: &mut VM) -> Option<u64> {
        return None;
    }

    fn exit(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn write_byte_terminal(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn write_terminal(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn read_byte_terminal(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn read_terminal(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn open_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn close_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn read_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn write_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn execute_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn execute_vxl_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn delete_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn move_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn copy_file(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }

    fn time_of_day(&mut self, _machine: &mut VM) -> Option<u64> {
        panic!("Failed. Unexpectedly reached an unimplemented handler function.");
    }
}

fn benchmark_load_byte(c: &mut Criterion) {
    // ldb 63, $r0
    let bytes: [u8; 10] = [
        0b0000_0010, // ldb
        0b0011_1111, // 63
        0b0000_0100,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0110_0000, // r0
    ];

    c.bench_function("load-byte", |b| {
        b.iter_batched(
            || {
                (
                    VM::new(
                        BulkValidator::with_bytes(bytes.to_vec())
                            .process_all_instructions()
                            .unwrap(),
                    ),
                    System::new(),
                )
            },
            |(mut vm, mut handler)| {
                vm.run_next(&mut handler).unwrap();
            },
            BatchSize::SmallInput,
        )
    });
}

fn benchmark_load_integer(c: &mut Criterion) {
    // ldi 663, $r0
    let bytes: [u8; 10] = [
        0b0000_0011, // ldi
        0b0110_0011, // 663
        0b0000_0110,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0110_0000, // r0
    ];

    c.bench_function("load-integer", |b| {
        b.iter_batched(
            || {
                (
                    VM::new(
                        BulkValidator::with_bytes(bytes.to_vec())
                            .process_all_instructions()
                            .unwrap(),
                    ),
                    System::new(),
                )
            },
            |(mut vm, mut handler)| {
                vm.run_next(&mut handler).unwrap();
            },
            BatchSize::SmallInput,
        )
    });
}

fn benchmark_malloc(c: &mut Criterion) {
    // ldi 32, $r1
    // malloc $r0, $r1
    let bytes: Vec<u8> = vec![
        0b0000_0011, // ldi
        0b0010_0000, // 32
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0111_0000, // r1
        0b0000_1001, // malloc
        0b0110_0111, // r0, r1
    ];

    c.bench_function("malloc", |b| {
        b.iter_batched(
            || {
                let validator = BulkValidator::with_bytes(bytes.clone());
                let mut vm = VM::new(validator.process_all_instructions().unwrap());

                // ldi
                vm.run_next(&mut System::new()).unwrap();

                (vm, System::new())
            },
            |(mut vm, mut handler)| {
                vm.run_next(&mut handler).unwrap();
            },
            BatchSize::SmallInput,
        )
    });
}

fn benchmark_malloci(c: &mut Criterion) {
    // malloci $r0, 32
    let bytes: Vec<u8> = vec![
        0b0000_1010, // malloci
        0b0010_0000, // 32
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0110_0000, // r0
    ];

    c.bench_function("malloci", |b| {
        b.iter_batched(
            || {
                let validator = BulkValidator::with_bytes(bytes.clone());
                let vm = VM::new(validator.process_all_instructions().unwrap());

                (vm, System::new())
            },
            |(mut vm, mut handler)| {
                vm.run_next(&mut handler).unwrap();
            },
            BatchSize::SmallInput,
        )
    });
}

fn benchmark_free(c: &mut Criterion) {
    // ldi 32, $r1
    // malloc $r0, $r1
    // free $r0
    let bytes: Vec<u8> = vec![
        0b0000_0011, // ldi
        0b0010_0000, // 32
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0111_0000, // r1
        0b0000_1001, // malloc
        0b0110_0111, // r0, r1
        0b0000_1011, // free
        0b0110_0000, // r0
    ];

    c.bench_function("free", |b| {
        b.iter_batched(
            || {
                let validator = BulkValidator::with_bytes(bytes.clone());
                let mut vm = VM::new(validator.process_all_instructions().unwrap());

                // ldi
                vm.run_next(&mut System::new()).unwrap();

                // malloc
                vm.run_next(&mut System::new()).unwrap();

                (vm, System::new())
            },
            |(mut vm, mut handler)| {
                vm.run_next(&mut handler).unwrap();
            },
            BatchSize::SmallInput,
        )
    });
}

fn benchmark_freea(c: &mut Criterion) {
    // ldi 32, $r1
    // malloc $r0, $r1
    // malloc $r0, $r1
    // free 0
    // free 1
    let bytes: Vec<u8> = vec![
        0b0000_0011, // ldi
        0b0010_0000, // 32
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0111_0000, // r1
        0b0000_1001, // malloc
        0b0110_0111, // r0, r1
        0b0000_1100, // freea
        0b0000_0000, // 0
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
    ];
    c.bench_function("freea", |b| {
        b.iter_batched(
            || {
                let validator = BulkValidator::with_bytes(bytes.clone());
                let mut vm = VM::new(validator.process_all_instructions().unwrap());

                // ldi
                vm.run_next(&mut System::new()).unwrap();

                // malloc
                vm.run_next(&mut System::new()).unwrap();

                (vm, System::new())
            },
            |(mut vm, mut handler)| {
                vm.run_next(&mut handler).unwrap();
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    benchmark_load_byte,
    benchmark_load_integer,
    benchmark_malloc,
    benchmark_malloci,
    benchmark_free,
    benchmark_freea
);

criterion_main!(benches);
