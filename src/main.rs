use std::arch::x86_64::__cpuid;
use std::cmp::PartialEq;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::io::{stdout, Write};
use crossterm::{
    cursor,
    execute,
    terminal,
};
use std::time::{Duration, Instant};
use std::thread;

//Physics Constants
const GRAVITY: f32 = 9.81;

//Frame Generation Constants
const FPS: i16 = 250;

//Chunk Structure
struct Chunk {
    global_pos: i32,
    // data is store in an X,Y format. The first array is columns and the inner array is rows
    data: [[i8;32];32],
}

enum MoveDir{
    Left,
    Right,
}
fn main() {



    let mut rng = rand::rng();
    let seed: u64 = rng.random_range(u64::MIN..=u64::MAX);

    let mut srng = StdRng::seed_from_u64(seed);
    const TOTAL_LOOPS: i32 = 100000;
    let mut _loops: i32 = 0;

    let _generated_chunks_postve: Vec<Chunk> = Vec::new();
    let _generated_chunks_negtve: Vec<Chunk> = Vec::new();


    let _program_start = Instant::now();

    let mut first_generation = true;

    let mut chunk_to_draw = generate_chunk(&mut srng, seed, first_chunk(), MoveDir::Left);

    loop{
        if _loops >= TOTAL_LOOPS{
            break;
        }
        let _frame_start = Instant::now();
        println!("Frame start: {:?}", _frame_start);

        chunk_to_draw = generate_chunk(&mut srng ,seed, chunk_to_draw, MoveDir::Left);

        draw_chunk(&chunk_to_draw);

        let _frame_time= Instant::now() - _frame_start;
        println!("Frame time: {:?}", _frame_time);
        if _frame_time < Duration::from_millis(4) {
            let _sleep_time = Duration::from_millis(4) - _frame_time;
            thread::sleep(_sleep_time);
            println!("Sleep for: {:?}", _sleep_time);
        }
        _loops += 1;
        first_generation = false;
        println!("Generation: {}", first_generation);
        print!("{esc}c", esc = 27 as char);

        // print!("Generated seed: {}", seed);
        // stdout().flush().unwrap();
    }

    // println!("{:?}", generate_chunk(seed, first_chunk(), MoveDir::Left).data);


    // println!("Done in: {}", _program_start.elapsed().as_millis());

}

fn generate_chunk(srng: &mut StdRng, seed: u64, current_chunk: Chunk, move_dir: MoveDir) -> Chunk{

    let mut new_chunk: Chunk = Chunk{global_pos: 0, data: [[0;32];32]};
    let mut _first_elem: bool = true;
    new_chunk.global_pos = current_chunk.global_pos + if matches!(move_dir,MoveDir::Right) {1} else {-1};

    let mut start_height: i8 = 0;

    if matches!(move_dir,MoveDir::Left) {
        for (i, elem) in current_chunk.data[0].iter().enumerate() {
            if *elem != 0 {
                start_height = i as i8;
            }
        }
    }
    else{
        for (i, elem) in current_chunk.data[new_chunk.data.len() - 1].iter().enumerate() {
            if *elem != 0 {
                start_height = i as i8;
            }
        }
    }

    let mut block_height: i8 = ((start_height + srng.random_range(-1..=1)).clamp(0,31));



    new_chunk.data[0]
        [(block_height) as usize]
        = 1;

    for line in new_chunk.data.iter_mut(){
        if _first_elem{
            _first_elem = false;
            // println!("{}",_first_elem);
            continue;
        }

        let next_block = (block_height + srng.random_range(-1..=1)).clamp(0,31);
        block_height = next_block;
        line[next_block as usize] = 1;
        // println!("{:?}", line);
        // println!("{}",_first_elem)

    }

    for col in new_chunk.data.iter_mut() {
        let mut ground = false;

        for (i,e) in col.iter_mut().enumerate() {
            if *e == 1{
                ground = true
            }
            if ground{
                *e = 1;
            }
        }
    }



    return new_chunk;
}

fn first_chunk() -> Chunk{

    let mut first_chunk = Chunk{global_pos: 0, data: [[0;32];32]};
    for row in first_chunk.data.iter_mut(){
        row[15] = 1;
    }
    return first_chunk;
}

fn draw_chunk(chunk: &Chunk) {

    let mut output: [String; 32] = std::array::from_fn(|_| String::from(" "));

    for (i, row) in chunk.data.iter().enumerate() {
        for (e, elem) in row.iter().enumerate() {
            if *elem == 0 {
                output[e].push(' ');
            }
            else{
                output[e].push('▪');
            }
        }
    }

    for (i,e) in output.iter_mut().enumerate(){
        e.push('\n');
    }

    for i in output.iter() {
        print!("{}", i);
    }
}