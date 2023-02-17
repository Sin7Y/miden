use assembly::Assembler;
use criterion::{criterion_group, criterion_main, Criterion};
use miden::{ProofOptions};
use std::time::Duration;
use stdlib::StdLibrary;
use vm_core::ProgramInputs;
use log::{debug, error, info, logger, LevelFilter};

fn program_prover(c: &mut Criterion) {
    // env_logger::Builder::new()
    //         .format(|buf, record| writeln!(buf, "{}", record.args()))
    //         .filter_level(log::LevelFilter::Debug)
    //         .init();

    let mut group = c.benchmark_group("program_prover");

    // trace rows:
    // 2^18 = 1<<16
    // 2^20 = 1<<18

    let size = 1<<16;

    let program = format!(
        "begin 
            repeat.{}
                swap dup.1 add
            end
        end",
        size
    );
    println!("programlalala");

    let program = Assembler::new()
        .with_module_provider(StdLibrary::default())
        .compile(&program)
        .unwrap();

    group.bench_function("program_prover", |bench| {
        bench.iter(|| {
            
            let inputs = ProgramInputs::from_stack_inputs(&[0, 1]).unwrap();
        
            let (mut outputs, proof) = miden::prove(&program, &inputs, &ProofOptions::default()).unwrap();

            // if miden::verify(program.hash(), &vec![0, 1], &outputs, proof).is_ok() {
            //     println!("true");
            // } else {
            //     println!("wrong");
            // }
        });
    });

    group.finish();
}

criterion_group![
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = program_prover
];
criterion_main!(benches);
