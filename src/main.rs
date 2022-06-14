use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() -> std::io::Result<()> {
    let mut labels = HashMap::new();
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();
    let file = File::open(filename.clone())?;
    let contents = BufReader::new(file);
    let re = Regex::new("^+").unwrap();
    let mut i = 0;
    for line in contents.lines() {
        let mut line = line.unwrap();
        line = line.replace(",", " ");
        line = line.replace("\t", " ");
        line = line.replace(":", " : ");
        line = re.replace_all(&line, "").to_string();
        let re_2 = Regex::new(" +").unwrap();
        let instruction: Vec<&str> = re_2.split(&line).collect();
        if instruction[1] == ":" {
            labels.insert(instruction[0].to_string(), i);
            i += 1;
        }
    }
    let file_2 = File::open(filename.clone())?;
    let contents_2 = BufReader::new(file_2);
    i = 0;
    for line in contents_2.lines() {
        let mut line = line.unwrap();
        line = line.replace(",", " ");
        let re_3 = Regex::new("\t+").unwrap();
        re_3.replace(&line, " ").to_string();
        line = line.replace(":", " : ");
        line = re.replace_all(&line, "").to_string();
        let re_2 = Regex::new(" +").unwrap();
        let instruction: Vec<&str> = re_2.split(&line).collect();
        let op;
        let mut f2;
        let mut f3;
        let mut f4;
        //let mut f5;
        if instruction[1] == ":" {
            op = instruction[2].clone().to_string();
            f2 = instruction[3].clone().to_string();
            f3 = instruction[4].clone().to_string();
            f4 = instruction[5].clone().to_string();
            //f5 = instruction[6].clone().to_string();
        } else {
            op = instruction[0].clone().to_string();
            f2 = instruction[1].clone().to_string();
            f3 = instruction[2].clone().to_string();
            f4 = instruction[3].clone().to_string();
            //f5 = instruction[4].clone().to_string();
        }
        if op == "add" {
            p_b(6, 0);
            p_r3(&mut f2, &mut f3, &mut f4);
            p_b(11, 0);
            print!("\n");
        } else if op == "addi" {
            p_b(6, 1);
            p_r2i(&mut f2, &mut f3);
            p_b(16, f4.parse::<isize>().unwrap());
            print!("\n");
        } else if op == "sub" {
            p_b(6, 0);
            p_r3(&mut f2, &mut f3, &mut f4);
            p_b(11, 2);
            print!("\n");
        } else if op == "lui" {
            p_b(6, 3);
            p_r2i(&mut f2, &mut "r0".to_string());
            p_b(11, 2);
            print!("\n");
        } else if op == "and" {
            p_b(6, 0);
            p_r3(&mut f2, &mut f3, &mut f4);
            p_b(11, 8);
            print!("\n");
        } else if op == "andi" {
            p_b(6, 4);
            p_r2i(&mut f2, &mut f3);
            p_b(16, f4.parse::<isize>().unwrap());
            print!("\n");
        } else if op == "or" {
            p_b(6, 0);
            p_r3(&mut f2, &mut f3, &mut f4);
            p_b(11, 9);
            print!("\n");
        } else if op == "ori" {
            p_b(6, 5);
            p_r2i(&mut f2, &mut f3);
            p_b(16, f4.parse::<isize>().unwrap());
            print!("\n");
        } else if op == "xor" {
            p_b(6, 0);
            p_r3(&mut f2, &mut f3, &mut f4);
            p_b(11, 10);
            print!("\n");
        } else if op == "xori" {
            p_b(6, 6);
            p_r2i(&mut f2, &mut f3);
            p_b(16, f4.parse::<isize>().unwrap());
            print!("\n");
        } else if op == "nor" {
            p_b(6, 0);
            p_r3(&mut f2, &mut f3, &mut f4);
            p_b(11, 11);
            print!("\n");
        } else if op == "sll" {
            p_b(6, 0);
            p_r3(&mut f2, &mut f3, &mut "r0".to_string());
            p_b(5,f4.parse::<isize>().unwrap());
            p_b(6, 16);
            print!("\n");
        } else if op == "srl" {
            p_b(6, 0);
            p_r3(&mut f2, &mut f3, &mut "r0".to_string());
            p_b(5,f4.parse::<isize>().unwrap());
            p_b(6, 17);
            print!("\n");
        } else if op == "sra" {
            p_b(6, 0);
            p_r3(&mut f2, &mut f3, &mut "r0".to_string());
            p_b(5,f4.parse::<isize>().unwrap());
            p_b(6, 18);
            print!("\n");
        } else if op == "lw" {
            p_b(6, 16);
            p_r2i(&mut f2, &mut base(&mut f3));
            p_b(16, dpl(&mut f3).parse::<isize>().unwrap());
            print!("\n");
        } else if op == "lh" {
            p_b(6, 18);
            p_r2i(&mut f2, &mut base(&mut f3));
            p_b(16, dpl(&mut f3).parse::<isize>().unwrap());
            print!("\n");
        } else if op == "lb" {
            p_b(6, 20);
            p_r2i(&mut f2, &mut base(&mut f3));
            p_b(16, dpl(&mut f3).parse::<isize>().unwrap());
            print!("\n");
        } else if op == "sw" {
            p_b(6, 24);
            p_r2i(&mut f2, &mut base(&mut f3));
            p_b(16, dpl(&mut f3).parse::<isize>().unwrap());
            print!("\n");
        } else if op == "sh" {
            p_b(6, 26);
            p_r2i(&mut f2, &mut base(&mut f3));
            p_b(16, dpl(&mut f3).parse::<isize>().unwrap());
            print!("\n");
        } else if op == "sb"{
            p_b(6, 28);
            p_r2i(&mut f2, &mut base(&mut f3));
            p_b(16, dpl(&mut f3).parse::<isize>().unwrap());
            print!("\n");
        } else if op == "beq" {
            p_b(6, 32);
            p_r2b(&mut f2, &mut f3);
            p_b(16, labels[&f4] - i - 1);
            print!("\n");
        } else if op == "bne" {
            p_b(6, 33);
            p_r2b(&mut f2, &mut f3);
            p_b(16, labels[&f4] - i - 1);
            print!("\n");
        } else if op == "blt" {
            p_b(6, 34);
            p_r2b(&mut f2, &mut f3);
            p_b(16, labels[&f4] - i - 1);
            print!("\n");
        } else if op == "ble" {
            p_b(6, 35);
            p_r2b(&mut f2, &mut f3);
            p_b(16, labels[&f4] - i - 1);
            print!("\n");
        } else if op == "j" {
            p_b(6, 40);
            p_b(26, labels[&f2]);
            print!("\n");
        } else if op == "jal"{
            p_b(6, 41);
            p_b(26, labels[&f2]);
            print!("\n");
        } else if op == "jr"{
            p_b(6, 42);
            p_r3(&mut "r0".to_string(), &mut f2, &mut "r0".to_string());
            p_b(11, 0);
            print!("\n");
        } else {
            println!("ERROR: Illegal Instruction");
        }
        i += 1;
    }
    Ok(())
}

fn p_b(digits: usize, num: isize) {
    if num >= 0 {
        print!("{:>0digits$b}", num);
    } else {
        print!("{}", &num.to_string().as_str()[..(64 - digits)]);
    }
}
fn p_r3(rd: &mut String, rs: &mut String,rt: &mut String) {
    *rs = rs.replace("r", "");
    p_b(5, rs.parse::<isize>().unwrap());
    *rt = rt.replace("r", "");
    p_b(5, rt.parse::<isize>().unwrap());
    *rd = rd.replace("r", "");
    p_b(5, rd.parse::<isize>().unwrap());
}
fn p_r2i(rt: &mut String, rs: &mut String) {
    *rs = rs.replace("r", "");
    p_b(5, rs.parse::<isize>().unwrap());
    *rt = rt.replace("r", "");
    p_b(5, rt.parse::<isize>().unwrap());
}
fn p_r2b(rs: &mut String, rt: &mut String) {
    *rs = rs.replace("r", "");
    p_b(5, rt.parse::<isize>().unwrap());
    *rt = rt.replace("r", "");
    p_b(5, rt.parse::<isize>().unwrap());
}
fn base(addr: &mut String) -> String{
    let re = Regex::new(".*(").unwrap();
    re.replace(addr, "").to_string();
    *addr = addr.replace(")", "").to_string();
    addr.clone()
}
fn dpl(addr: &mut String) -> String {
    let re= Regex::new("(.*)").unwrap();
    re.replace(addr, "").to_string();
    addr.clone()
}