use std::io::prelude::*;
use std::fs::File;

pub fn decompile(source_path: &str) {
    let mut source = match File::open(source_path) {
        Err(why) => panic!("Couldn't open file {}{}", source_path, why),
        Ok(file) => file
    };
    let mut buffer = Vec::new();
    source.read_to_end(&mut buffer).expect("Couldn't read file!");

    let mut instructions = Vec::new();

    let mut i = 0;
    while i < buffer.len() {
        let opcode: u16 = (buffer[i] as u16) << 8 | buffer[i + 1] as u16;
        instructions.push(translate_opcode(opcode));
        i += 2;
    }

    let mut file = match File::create(format!("{}{}", source_path, ".source")) {
        Err(why) => panic!("couldn't create file: {}", why),
        Ok(file) => file,
    };

    match file.write_all(instructions.join("\n").as_bytes()) {
        Err(why) => panic!("couldn't write to file: {}", why),
        Ok(_) => (),
    };
}

fn translate_opcode(opcode: u16) -> String {
    let nnn = opcode & 0x0FFF;
    let kk = (opcode & 0x00FF) as u8;
    let n = (opcode & 0x000F) as u8;

    let x = ((opcode & 0x0F00) >> 8) as usize;
    let y = ((opcode & 0x00F0) >> 4) as usize;

    return get_opcode_structure(opcode)
        .replace("nnn", &nnn.to_string()[..])
        .replace("kk", &kk.to_string()[..])
        .replace("n", &n.to_string()[..])
        .replace("x", &x.to_string()[..])
        .replace("y", &y.to_string()[..]);
}

fn get_opcode_structure(opcode: u16) -> String {
    let op_1 = (opcode & 0xF000) >> 12;
    let op_2 = (opcode & 0x0F00) >> 8;
    let op_3 = (opcode & 0x00F0) >> 4;
    let op_4 = opcode & 0x000F;

    match (op_1, op_2, op_3, op_4) {
        (0, 0, 0xE, 0) => return "CLS".to_string(),
        (_, _, 0xE, 0xE) => return "RET".to_string(),
        (1, _, _, _) => return "JP nnn".to_string(),
        (2, _, _, _) => return "CALL nnn".to_string(),
        (3, _, _, _) => return "SE Vx, kk".to_string(),
        (4, _, _, _) => return "SNE Vx, kk".to_string(),
        (5, _, _, 0) => return "SE Vx, Vy".to_string(),
        (6, _, _, _) => return "LD Vx, kk".to_string(),
        (7, _, _, _) => return "ADD Vx, kk".to_string(),
        (8, _, _, 0) => return "LD Vx, Vy".to_string(),
        (8, _, _, 1) => return "OR Vx, Vy".to_string(),
        (8, _, _, 2) => return "AND Vx, Vy".to_string(),
        (8, _, _, 3) => return "XOR Vx, Vy".to_string(),
        (8, _, _, 4) => return "ADD Vx, Vy".to_string(),
        (8, _, _, 5) => return "SUB Vx, Vy".to_string(),
        (8, _, _, 6) => return "SHR Vx, Vy".to_string(),
        (8, _, _, 7) => return "SUBN Vx, Vy".to_string(),
        (8, _, _, 0xE) => return "SHL Vx".to_string(),
        (9, _, _, 0) => return "SNE Vx, Vy".to_string(),
        (0xA, _, _, _) => return "LD I, nnn".to_string(),
        (0xB, _, _, _) => return "JP V0, nnn".to_string(),
        (0xC, _, _, _) => return "RND Vx, kk".to_string(),
        (0xD, _, _, _) => return "DRW Vx, Vy, n".to_string(),
        (0xE, _, 9, 0xE) => return "SKP Vx".to_string(),
        (0xE, _, 0xA, 1) => return "SKNP Vx".to_string(),
        (0xF, _, 0, 7) => return "LD Vx, DT".to_string(),
        (0xF, _, 0, 0xA) => return "LD Vx, K".to_string(),
        (0xF, _, 1, 5) => return "LD DT, Vx".to_string(),
        (0xF, _, 1, 8) => return "LD ST, Vx".to_string(),
        (0xF, _, 1, 0xE) => return "ADD I, Vx".to_string(),
        (0xF, _, 2, 9) => return "LD F, Vx".to_string(),
        (0xF, _, 3, 3) => return "LD B, Vx".to_string(),
        (0xF, _, 5, 5) => return "LD [I], Vx".to_string(),
        (0xF, _, 6, 5) => return "LD Vx, [I]".to_string(),
        (_, _, _, _) => return format!("{:X}", opcode),
    }
}