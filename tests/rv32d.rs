#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate rvemu;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn fld_rd_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x93, 0x0f, 0x20, 0x00, // addi x31, x0, 2
            0x13, 0x0f, 0x40, 0x00, // addi x30, x0, 4
            0x87, 0xbf, 0x0f, 0x00, // fld f31, 0(x31)
        ],
    };

    cpu.start(&mut mem);

    // x0-x31
    let expected_x = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        4, 2,
    ];
    // f0-f30
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
    // f31
    assert_eq!(0xbf8700400f130020, cpu.fregs[31].to_bits());
}

#[wasm_bindgen_test]
pub fn fsd_rs2_offset_rs1() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0x93, 0x0f, 0x20, 0x00, // addi x31, x0, 2
            0x13, 0x0f, 0x40, 0x00, // addi x30, x0, 4
            0x27, 0xb0, 0xff, 0x01, // fsd f31, 0(x31)
            0x87, 0xbf, 0x0f, 0x00, // fld f31, 0(x31)
        ],
    };

    cpu.start(&mut mem);

    // x0-x31
    let expected_x = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        4, 2,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fmaddd_rd_rs1_rs2_rs3() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xc3, 0x0f, 0xdf, 0xe3, // fmadd.d f31, f30, f29, f28
        ],
    };

    cpu.fregs[28] = -0.5;
    cpu.fregs[29] = 4.2;
    cpu.fregs[30] = 1.2;

    cpu.start(&mut mem);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, 4.2, 1.2, 4.54,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fmsubd_rd_rs1_rs2_rs3() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xc7, 0x0f, 0xdf, 0xe3, // fmsub.d f31, f30, f29, f28
        ],
    };

    cpu.fregs[28] = -0.5;
    cpu.fregs[29] = 4.2;
    cpu.fregs[30] = 1.2;

    cpu.start(&mut mem);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, 4.2, 1.2, 5.54,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fnmaddd_rd_rs1_rs2_rs3() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xcb, 0x0f, 0xdf, 0xe3, // fnmadd.d f31, f30, f29, f28
        ],
    };

    cpu.fregs[28] = -0.5;
    cpu.fregs[29] = 4.2;
    cpu.fregs[30] = 1.2;

    cpu.start(&mut mem);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, 4.2, 1.2, -5.54,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fnmsubd_rd_rs1_rs2_rs3() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xcf, 0x0f, 0xdf, 0xe3, // fnmsub.d f31, f30, f29, f28
        ],
    };

    cpu.fregs[28] = -0.5;
    cpu.fregs[29] = 4.2;
    cpu.fregs[30] = 1.2;

    cpu.start(&mut mem);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, 4.2, 1.2, -4.54,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn faddd_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x0f, 0xdf, 0x03, // fadd.d f31, f30, f29
        ],
    };

    cpu.fregs[29] = 4.2;
    cpu.fregs[30] = 2.5;

    cpu.start(&mut mem);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, 2.5, 6.7,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fsubd_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x0f, 0xdf, 0x0b, // fsub.d f31, f30, f29
        ],
    };

    cpu.fregs[29] = 4.2;
    cpu.fregs[30] = 2.8;

    cpu.start(&mut mem);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, 2.8, -1.4,
    ];
    for (i, e) in expected.iter().enumerate() {
        // TODO: workaround for floating point precision problem
        assert_eq!(*e, (cpu.fregs[i] * 10.0).round() / 10.0);
    }
}

#[wasm_bindgen_test]
pub fn fmuld_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x0f, 0xdf, 0x13, // fmul.d f31, f30, f29
        ],
    };

    cpu.fregs[29] = 4.2;
    cpu.fregs[30] = -1.2;

    cpu.start(&mut mem);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, -1.2, -5.04,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fdivd_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x0f, 0xdf, 0x1b, // fdiv.d f31, f30, f29
        ],
    };

    cpu.fregs[29] = -1.2;
    cpu.fregs[30] = 4.2;

    cpu.start(&mut mem);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.2, 4.2, -3.5,
    ];
    for (i, e) in expected.iter().enumerate() {
        // TODO: workaround for floating point precision problem
        assert_eq!(*e, (cpu.fregs[i] * 10.0).round() / 10.0);
    }
}

#[wasm_bindgen_test]
pub fn fsgnjd_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x0f, 0xdf, 0x23, // fsgnj.d f31, f30, f29
        ],
    };

    cpu.fregs[29] = -1.2;
    cpu.fregs[30] = 4.2;

    cpu.start(&mut mem);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.2, 4.2, -4.2,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fsgnjnd_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x1f, 0xdf, 0x23, // fsgnjn.d f31, f30, f29
        ],
    };

    cpu.fregs[29] = -1.2;
    cpu.fregs[30] = 4.2;

    cpu.start(&mut mem);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.2, 4.2, 4.2,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fsgnjxd_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x2f, 0xdf, 0x23, // fsgnjx.d f31, f30, f29
        ],
    };

    cpu.fregs[29] = -1.2;
    cpu.fregs[30] = 4.2;

    cpu.start(&mut mem);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.2, 4.2, -4.2,
    ];
    for (i, e) in expected.iter().enumerate() {
        // TODO: workaround for floating point precision problem
        assert_eq!(*e, (cpu.fregs[i] * 10.0).round() / 10.0);
    }
}

#[wasm_bindgen_test]
pub fn fmind_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x0f, 0xdf, 0x2b, // fmin.d f31, f30, f29
        ],
    };

    cpu.fregs[29] = 4.2;
    cpu.fregs[30] = -1.2;

    cpu.start(&mut mem);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, -1.2, -1.2,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fmaxd_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x1f, 0xdf, 0x2b, // fmax.d f31, f30, f29
        ],
    };

    cpu.fregs[29] = 4.2;
    cpu.fregs[30] = -1.2;

    cpu.start(&mut mem);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, -1.2, 4.2,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fcvtsd_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x0f, 0x1f, 0x40, // fcvt.s.d f31, f30
        ],
    };

    cpu.fregs[30] = -1.2;

    cpu.start(&mut mem);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.2, -1.2,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fcvtds_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x0f, 0x0f, 0x42, // fcvt.d.s f31, f30
        ],
    };

    cpu.fregs[30] = -1.2;

    cpu.start(&mut mem);

    // f0-f31
    let expected = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.2, -1.2,
    ];
    for (i, e) in expected.iter().enumerate() {
        // TODO: workaround for floating point precision problem
        assert_eq!(*e, (cpu.fregs[i] * 10.0).round() / 10.0);
    }
}

#[wasm_bindgen_test]
pub fn fsqrtd_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x0f, 0x0f, 0x5a, // fmax.d f31, f30
        ],
    };

    cpu.fregs[30] = 4.2;

    cpu.start(&mut mem);

    // f0-f31
    let expected = [
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        4.2,
        2.04939015319192,
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fled_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x0f, 0xdf, 0xa3, // fle.d f31, f30, f29
        ],
    };

    cpu.fregs[29] = 4.2;
    cpu.fregs[30] = 4.2;

    cpu.start(&mut mem);

    // x0-x31
    let expected_x = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, 4.2, 0.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fltd_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x1f, 0xdf, 0xa3, // flt.d f31, f30, f29
        ],
    };

    cpu.fregs[29] = 4.2;
    cpu.fregs[30] = -1.2;

    cpu.start(&mut mem);

    // x0-x31
    let expected_x = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, -1.2, 0.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn feqd_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x2f, 0xdf, 0xa3, // feq.d f31, f30, f29
        ],
    };

    cpu.fregs[29] = 4.2;
    cpu.fregs[30] = 4.2;

    cpu.start(&mut mem);

    // x0-x31
    let expected_x = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2, 4.2, 0.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fcvtwd_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x8f, 0x0f, 0xc2, // fcvt.w.d x31, f31 (rm: 000)
        ],
    };

    cpu.fregs[31] = -4.2;

    cpu.start(&mut mem);

    // x0-x31
    let expected_x = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, -4,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -4.2,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fcvtwud_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x8f, 0x1f, 0xc2, // fcvt.wu.d x31, f31 (rm: 000)
        ],
    };

    cpu.fregs[31] = 4.2;

    cpu.start(&mut mem);

    // x0-x31
    let expected_x = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 4,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fcvtdw_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x8f, 0x0f, 0xd2, // fcvt.d.w x31, f31 (rm: 000)
        ],
    };

    cpu.xregs[31] = -4;

    cpu.start(&mut mem);

    // x0-x31
    let expected_x = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, -4,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -4.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fcvtdwu_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x8f, 0x1f, 0xd2, // fcvt.d.wu x31, f31 (rm: 000)
        ],
    };

    cpu.xregs[31] = 4;

    cpu.start(&mut mem);

    // x0-x31
    let expected_x = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 4,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}

#[wasm_bindgen_test]
pub fn fclassd_rd_rs1_rs2() {
    let mut cpu = rvemu::cpu::Cpu::new();
    let mut mem = rvemu::memory::Memory {
        dram: vec![
            0xd3, 0x9f, 0x0f, 0xe2, // fclass.d x31, f31
        ],
    };

    cpu.fregs[31] = std::f64::INFINITY;

    cpu.start(&mut mem);

    // x0-x31
    let expected_x = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 7,
    ];
    // f0-f31
    let expected_f = [
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        std::f64::INFINITY,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs[i]);
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs[i]);
    }
}
