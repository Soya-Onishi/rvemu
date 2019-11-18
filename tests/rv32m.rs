#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate rvemu;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn mul_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addi x31, x0, 3
        0x93, 0x0f, 0x30, 0x00,
        // addi x30, x0, -5
        0x13, 0x0f, 0xb0, 0xff,
        // mul x29, x30, x31
        0xb3, 0x0e, 0xff, 0x03,
    ];

    cpu.start(&mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -5, 3];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn div_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addi x31, x0, 3
        0x93, 0x0f, 0x30, 0x00,
        // addi x30, x0, -5
        0x13, 0x0f, 0xb0, 0xff,
        // div x29, x30, x31
        0xb3, 0x4e, 0xff, 0x03,
    ];

    cpu.start(&mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -5, 3];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn divu_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addi x31, x0, 3
        0x93, 0x0f, 0x30, 0x00,
        // addi x30, x0, 5
        0x13, 0x0f, 0x50, 0x00,
        // div x29, x30, x31
        0xb3, 0x4e, 0xff, 0x03,
    ];

    cpu.start(&mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 5, 3];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn rem_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addi x31, x0, 3
        0x93, 0x0f, 0x30, 0x00,
        // addi x30, x0, -5
        0x13, 0x0f, 0xb0, 0xff,
        // rem x29, x30, x31
        0xb3, 0x6e, 0xff, 0x03,
    ];

    cpu.start(&mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2, -5, 3];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}

#[wasm_bindgen_test]
pub fn remu_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = vec![
        // addi x31, x0, 3
        0x93, 0x0f, 0x30, 0x00,
        // addi x30, x0, 5
        0x13, 0x0f, 0x50, 0x00,
        // remu x29, x30, x31
        0xb3, 0x7e, 0xff, 0x03,
    ];

    cpu.start(&mut mem);

    let expected =
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 5, 3];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.regs[i]);
    }
}