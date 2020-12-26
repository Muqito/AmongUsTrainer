mod hack_util;

use std::str;
use std::thread;

const NAMES: [&str; 10] = [
    "Panic disable all hax",
    "other", //1,8,9 either don't work or arent implemented
    "Speed",
    "No Kill Cooldown",
    "Max Kill Range",
    "Invis (Use Vent)",
    "Imp",
    "Fullbright",
    "Reveal Dead And Alive",
    "Reveal Imposters",
];

fn main() {
    let process = hack_util::attach("Among Us.exe");
    if process.m_h_process as u32 == 0 {
        println!("Couldn't find the among us process D:");
        std::process::exit(1);
    }
    let module = hack_util::Module::get_module("Among Us.exe", "GameAssembly.dll"); //For some reason it only lists the last 5 processes not sure why

    //let player_control_ptr = process.pointer_from_offsets(0, vec![gameaseembly_offset, 0x5C, 0x0]);
    //let game_options_data_ptr = process.pointer_from_offsets(0, vec![gameaseembly_offset, 0x5C, 0x4]);

    let mut was_done: [bool; 10] = [false; 10];
    let mut ogspeed: f32 = 1.0;

    let mut other: bool = false;
    let mut speed: bool = false;
    let mut cooldown: bool = false;
    let mut range: bool = false;
    let mut vent: bool = false;
    let mut imposter: bool = false;
    let mut fullbright: bool = false;

    help(other, speed, cooldown, range, vent, imposter, fullbright);

    loop {
        for x in 0..9 {
            if hack_util::pressed(0x30 + x) {
                if !was_done[x as usize] {
                    was_done[x as usize] = true;
                    match x {
                        0 => {
                            speed = false;
                            let speed_ptr = process
                                .pointer_from_offsets(module.m_dw_base, vec![0xDA3C30, 0x5C, 0x14])
                                + 0x14;
                            process.write_memory::<f32>(speed_ptr, ogspeed);
                            cooldown = false;
                            range = false;
                            vent = false;
                            imposter = false;
                            fullbright = false;
                            help(other, speed, cooldown, range, vent, imposter, fullbright);
                        }
                        //Closet speed hack (1.2x)
                        1 => other = !other,
                        //Blatant speed hack (2x)
                        2 => {
                            let speed_ptr = process
                                .pointer_from_offsets(module.m_dw_base, vec![0xDA3C30, 0x5C, 0x14])
                                + 0x14;
                            if !speed {
                                ogspeed = process.read_memory::<f32>(speed_ptr);
                                process.write_memory::<f32>(speed_ptr, 4.0)
                            } else {
                                process.write_memory::<f32>(speed_ptr, ogspeed)
                            }
                            speed = !speed;
                        }
                        //No Kill Cooldown
                        3 => cooldown = !cooldown,
                        //Long Kill distance
                        4 => range = !range,
                        //vent freemove
                        5 => vent = !vent,
                        //Imposter mode
                        6 => imposter = !imposter,
                        //Fullbright
                        7 => fullbright = !fullbright,
                        //List dead
                        8 => {} //dead(&process),
                        //List imposters
                        9 => {} //imposters(&process),
                        _ => {}
                    }
                    if x != 0 && x != 1 && x != 8 && x != 9 {
                        println!("Toggled mod [{}] {}", x, NAMES[x as usize]);
                    }
                }
            } else {
                was_done[x as usize] = false
            }
        }
        //Passive Hacks

        if cooldown {
            let kill_cooldown_ptr =
                process.pointer_from_offsets(module.m_dw_base, vec![0xDA5A84, 0x5C, 0x0]) + 0x44;
            process.write_memory::<f32>(kill_cooldown_ptr, 0.0);
        }

        let kill_range_ptr =
            process.pointer_from_offsets(module.m_dw_base, vec![0xDA5A84, 0x5C, 0x4]) + 0x40;
        if range {
            process.write_memory::<u32>(kill_range_ptr, 2);
        } else {
            process.write_memory::<u32>(kill_range_ptr, 1);
        }

        if vent {
            let vent_move_ptr =
                process.pointer_from_offsets(module.m_dw_base, vec![0xDA5A84, 0x5C, 0x0]) + 0x30;
            let in_vent_ptr =
                process.pointer_from_offsets(module.m_dw_base, vec![0xDA5A84, 0x5C, 0x0]) + 0x31;
            if process.read_memory::<u8>(in_vent_ptr) == 1 {
                process.write_memory::<u8>(vent_move_ptr, 2);
            }
        }

        let player_imposter_ptr =
            process.pointer_from_offsets(module.m_dw_base, vec![0xDA5A84, 0x5C, 0x0, 0x34]) + 0x28;

        if imposter {
            process.write_memory::<u8>(player_imposter_ptr, 1);
        } else {
            process.write_memory::<u8>(player_imposter_ptr, 0);
        }

        let vision_radius_ptr =
            process.pointer_from_offsets(module.m_dw_base, vec![0xDA5A84, 0x5C, 0x0, 0x54]) + 0x1C;

        if fullbright {
            process.write_memory::<f32>(vision_radius_ptr, 20.0);
        } else {
            process.write_memory::<f32>(vision_radius_ptr, 5.0);
        }
        thread::sleep_ms(20);
    }
}

//Instant Hacks
/*
fn imposters(process: &hack_util::Process) {
    for i in 1..9 {
        let player_imposter_ptr = process.pointer_from_offsets(
            module.m_dw_base,
            vec![0xDA5A84, 0x5C, 0x0, 0x24, 0x8, 0x10 + (0x4 * i)],
        ) + 0x28;
        let player_name_ptr = process.pointer_from_offsets(
            module.m_dw_base,
            vec![0xDA5A84, 0x5C, 0x0, 0x24, 0x8, 0x10 + (0x4 * i), 0xC],
        ) + 0xC;

        let imposter = process.read_memory::<u8>(player_imposter_ptr);
        let player_name = process.read_memory::<[u8; 10]>(player_name_ptr);

        let name = match str::from_utf8(&player_name) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        if imposter == 1 {
            println!("{} is an imposter!", name);
        }
    }
} */

/*
fn dead(process: &hack_util::Process, module: &hack_util::Module) {
    let mut total_dead = 0;

    for i in 1..9 {
        let player_dead_ptr = process.pointer_from_offsets(
            module.m_dw_base,
            vec![0xDA5A84, 0x5C, 0x0, 0x24, 0x8, 0x10 + (0x4 * i)],
        ) + 0x29;
        let player_name_ptr = process.pointer_from_offsets(
            module.m_dw_base,
            vec![0xDA5A84, 0x5C, 0x0, 0x24, 0x8, 0x10 + (0x4 * i), 0xC],
        ) + 0xC;

        let dead = process.read_memory::<u8>(player_dead_ptr);
        total_dead += dead;
        let player_name = process.read_memory::<[u8; 10]>(player_name_ptr);

        let name = match str::from_utf8(&player_name) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        if dead == 1 {
            println!("{} is alive.", name);
        } else {
            println!("{} is dead.", name);
        }
    }
    let total_alive = 9 - total_dead;
    println!("{}/9 are still alive.", total_alive);
}*/

fn help(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool, g: bool) {
    println!("[0] {}", NAMES[0]);
    println!("[1] <{}> {}", a, NAMES[1]);
    println!("[2] <{}> {}", b, NAMES[2]);
    println!("[3] <{}> {}", c, NAMES[3]);
    println!("[4] <{}> {}", d, NAMES[4]);
    println!("[5] <{}> {}", e, NAMES[5]);
    println!("[6] <{}> {}", f, NAMES[6]);
    println!("[7] <{}> {}", g, NAMES[7]);
    println!("[8] {}", NAMES[8]);
    println!("[9] {}", NAMES[9]);
}
