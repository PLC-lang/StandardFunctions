use std::time::Duration;

use common::compile_with_native;
use iec61131std::timers::TimerParams;
use inkwell::context::Context;
use rusty::runner::run;

// Import common functionality into the integration tests
mod common;

use common::add_std;

#[repr(C)]
#[derive(Default, Debug)]
struct MainType {
    value: bool,
    tp_inst: TimerParams,
    tp_out: bool,
    tp_et: iec61131std::timers::Time,
}

///TP Time Diagram
/*
    ┌───────────────────────────────────────────────────────┐
    │ TP                                                    │
    │                                                       │
    │                                                       │
    │                                                       │
    │                ┌───────────────┐     ┌┐  ┌┐           │
    │                │               │     ││  ││           │
    │       IN       │               │     ││  ││           │
    │          ──────┘               └─────┴┴──┴┴───        │
    │               t0               t1    t2  t3           │
    │                                                       │
    │                                                       │
    │                ┌─────┐               ┌─────┐          │
    │                │     │               │     │          │
    │       Q        │     │               │     │          │
    │          ──────┘     └───────────────┘     └─────     │
    │               t0     t0+TP          t2     t2+TP      │
    │                                                       │
    │                                                       │
    │                                                       │
    │                                                       │
    │                                                       │
    │       PT                                              │
    │       │        ──────┐               ──────┐          │
    │       │             /│                     │          │
    │       │            / │                    /│          │
    │ ET    │           /  │                   / │          │
    │       │          /   │                  /  │          │
    │       │         /    │                 /   │          │
    │       └────────/     └────────────────/    └──────────┤
    │       0       t0     t0+TP           t2    t2+TP      │
    │                                                       │
    │                                                       │
    └───────────────────────────────────────────────────────┘

*/
#[test]
fn tp_true_for_time() {
    let prog = r#"
        PROGRAM main
            VAR_INPUT
                value : BOOL;
            END_VAR
            VAR
                tp_inst : TP;
                tp_out  : BOOL;
                tp_et   : TIME;
            END_VAR
            tp_inst(IN := value, PT := T#10ms, Q => tp_out, ET => tp_et);

        END_PROGRAM
    "#;

    let source = add_std!(prog, "timers.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = MainType::default();
    //On first call, out is true, et is 0
    main_inst.value = true;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    //After 5ms, out is true, et is 5ms
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(5));
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 5_000_000);
    //At 10ms, out is true, et is 10ms
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(5));
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 10_000_000);
    //After 15ms, out is false, et is 0/
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(5));
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
}

#[test]
fn tp_does_not_retrigger_on_constant_input() {
    let prog = r#"
        PROGRAM main
            VAR_INPUT
                value : BOOL;
            END_VAR
            VAR
                tp_inst : TP;
                tp_out  : BOOL;
                tp_et   : TIME;
            END_VAR
            tp_inst(IN := value, PT := T#10ms, Q => tp_out, ET => tp_et);

        END_PROGRAM
    "#;

    let source = add_std!(prog, "timers.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);

    let mut main_inst = MainType::default();
    //On first call, out is true, et is 0
    main_inst.value = true;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    //At 10ms, out is true, et is 10ms
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(10));
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 10_000_000);
    //After 15ms, out is false, et is 0/
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(5));
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    //After 20ms, out is false, et is 0/
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(5));
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
}

#[test]
fn tp_not_interrupted_by_signal_change() {
    let prog = r#"
        PROGRAM main
            VAR_INPUT
                value : BOOL;
            END_VAR
            VAR
                tp_inst : TP;
                tp_out  : BOOL;
                tp_et   : TIME;
            END_VAR
            tp_inst(IN := value, PT := T#10ms, Q => tp_out, ET => tp_et);
        END_PROGRAM
    "#;

    let source = add_std!(prog, "timers.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = MainType::default();

    //On first call with true, out is true, et is 0
    main_inst.value = true;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);

    //advance 1 ms
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(1));
    //call timer with false
    main_inst.value = false;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    //Verify that the timer is still running
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 1_000_000);
    // advance by 1 ms
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(1));
    //call timer with true
    main_inst.value = true;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    //assert that the signal was not interrupted
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 2_000_000);
}

/* TON Timing Diagram


    ┌────────────────────────────────────────────────────────────┐
    │ TON                                                        │
    │                                                            │
    │                                                            │
    │                                                            │
    │                ┌──────────┐    ┌───┐   ┌──────────┐        │
    │                │          │    │   │   │          │        │
    │       IN       │          │    │   │   │          │        │
    │          ──────┘          └────┘   └───┘          └────    │
    │               t0          t1   t2  t3  t4         t5       │
    │                                                            │
    │                                                            │
    │                      ┌────┐                  ┌────┐        │
    │                      │    │                  │    │        │
    │       Q              │    │                  │    │        │
    │          ────────────┘    └──────────────────┘    └─────   │
    │                   t0+TP   t1               t4+TP  t5       │
    │                                                            │
    │                                                            │
    │                                                            │
    │                                                            │
    │                                                            │
    │       PT                                                   │
    │       │              /───┐                                 │
    │       │             /    │                   /────┐        │
    │       │            /     │        /│        /     │        │
    │ ET    │           /      │       / │       /      │        │
    │       │          /       │      /  │      /       │        │
    │       │         /        │     /   │     /        │        │
    │       └────────/         └────/    └────/         └──      │
    │       0       t0        t1   t2    t3             t5       │
    │                                                            │
    │                                                            │
    └────────────────────────────────────────────────────────────┘



*/

#[test]
fn ton_returns_true_after_time_preset() {
    let prog = r#"
        PROGRAM main
            VAR_INPUT
                value : BOOL;
            END_VAR
            VAR
                tp_inst : TON;
                tp_out  : BOOL;
                tp_et   : TIME;
            END_VAR
            tp_inst(IN := value, PT := T#10ms, Q => tp_out, ET => tp_et);
        END_PROGRAM
    "#;

    let source = add_std!(prog, "timers.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = MainType::default();
    // Value true First call -> false
    main_inst.value = true;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    // Value true After 5ms -> false
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(5));
    main_inst.value = true;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 5_000_000);
    // Value true After 10ms -> false
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(5));
    main_inst.value = true;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 10_000_000);
    // Value true After 15ms -> true
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(5));
    main_inst.value = true;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    // Value false after 20ms -> false
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(5));
    main_inst.value = false;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
}

#[test]
fn ton_counts_elapsed_time_while_waiting() {
    let prog = r#"
        PROGRAM main
            VAR_INPUT
                value : BOOL;
            END_VAR
            VAR
                tp_inst : TON;
                tp_out  : BOOL;
                tp_et   : TIME;
            END_VAR
            tp_inst(IN := value, PT := T#10ms, Q => tp_out, ET => tp_et);
        END_PROGRAM
    "#;

    let source = add_std!(prog, "timers.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = MainType::default();
    // Value true, counter starts at 0
    main_inst.value = true;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    // Value true after 5ms counter at 5ms
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(5));
    main_inst.value = true;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 5_000_000);
    // Value false after 6ms counter at 0ms (stopped)
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(1));
    main_inst.value = false;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
}

#[test]
fn ton_waits_again_after_turining_off() {
    let prog = r#"
        PROGRAM main
            VAR_INPUT
                value : BOOL;
            END_VAR
            VAR
                tp_inst : TON;
                tp_out  : BOOL;
                tp_et   : TIME;
            END_VAR
            tp_inst(IN := value, PT := T#9ms, Q => tp_out, ET => tp_et);
        END_PROGRAM
    "#;

    let source = add_std!(prog, "timers.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = MainType::default();
    // Value true First call -> false
    main_inst.value = true;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    // Value true After 5ms -> false
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(5));
    main_inst.value = true;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 5_000_000);
    // Value true After 10ms -> true
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(5));
    main_inst.value = true;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0); //Elapsed time set to 0 since timer is finished
                                    // Value false After 15ms -> false
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(5));
    main_inst.value = false;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    // Value true after 20ms -> false
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(5));
    main_inst.value = true;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    // Value true after 30ms -> true
    iec61131std::timers::test_time_helpers::MockClock::advance(Duration::from_millis(10));
    main_inst.value = true;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
}

#[test]
fn toff_turns_of_after_time_preset() {}
