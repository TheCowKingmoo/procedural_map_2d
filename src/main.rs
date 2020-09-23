use rand::prelude::*;

static TYPE_ARRAY: &'static [char] = &['*', '+', '-', '~', '$', '^'];
static TYPE_WEIGHT_ARRAY: &'static [u32] = &[20, 10, 15, 15, 20, 20];
static TYPE_DEBT_ARRAY: &'static [u32] = &[5, 10, 3, 4, 3, 2];

const HEIGHT: usize = 128;
const WIDTH: usize = 128;
const EMPTY_TERRAIN: char = 'e';

fn main() {
    generate_node_map();
}

/*
 unweighted random values assigned and printed
*/
fn absolute_random() {
    let mut rng = rand::thread_rng();
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let random_value: u32 = rng.gen_range(0, TYPE_ARRAY.len() as u32);
            print!("{}", TYPE_ARRAY[random_value as usize]);
        }
        println!();
    }
}

/*
  grabs the index for a char out of TYPE_ARRAY based on the rand value given
  and the value based on TYPE_WEIGHT_ARRAY

  eg: is rng_value == 45
  first loop - > 45 > 40 -> true -> loop again
  second loop -> 45 > 40+10 -> false -> return i (which is 1)
*/

fn get_index_from_rand(rng_value: u32) -> usize {
    let mut total = 0;
    for i in 0..TYPE_WEIGHT_ARRAY.len() {
        total = total + TYPE_WEIGHT_ARRAY[i];
        if total > rng_value {
            return i;
        }
    }
    return 0;
}

/*
 Point Struct
   x - the x coord of the map
   y - the y coord of the map
   terrain - what char to assign the point
   debt - the weight other Points will use to determine if they need to continue the terrain or use a new one
        - if a point gets its terrain based off of a neighbor WIDTH value will be neighbor.debt - 1 to help lower
        - the next chance of getting the terrain again
*/
#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
    terrain: char,
    debt: u32,
}

/*
 Drives map generation
*/
fn generate_node_map() {
    let mut map_array: [[Point; WIDTH]; HEIGHT] = [[Point {
        x: 0,
        y: 0,
        terrain: EMPTY_TERRAIN,
        debt: 0,
    }; WIDTH]; HEIGHT];

    for i in 0..WIDTH + 2 {
        print!("-");
    }
    println!();

    for i in 0..HEIGHT {
        print!("|");
        for j in 0..WIDTH {
            map_array[i][j].x = i;
            map_array[i][j].y = j;
            assign_terrain(&mut map_array, i as u32, j as u32);
            print!("{}", map_array[i][j].terrain);
        }
        println!("|");
    }

    for i in 0..WIDTH + 2 {
        print!("-");
    }
}

/*
  Determines what terrain to assign based on points to the left or above
*/

fn assign_terrain(map: &mut [[Point; WIDTH as usize]; HEIGHT as usize], x: u32, y: u32) {
    let mut rng = rand::thread_rng();
    let mut random_value: u32 = rng.gen_range(0, 100);
    let mut x_debt: bool = false;
    let mut y_debt: bool = false;

    let i_x: usize = x as usize;
    let i_y: usize = y as usize;

    if x != 0 {
        // mul debt by 10 to treat as percentage
        if 10 * map[i_x - 1][i_y].debt > random_value {
            x_debt = true;
        }
    }

    if !x_debt && y != 0 {
        // regen rng otherwise a higher x debt will always beat the lower y debt
        random_value = rng.gen_range(0, 100);
        // mul debt by 10 to treat as percentage
        if 10 * map[i_x][i_y - 1].debt > random_value {
            y_debt = true;
        }
    }

    if x_debt {
        //copy x-1 neighbor terrain

        map[i_x][i_y].terrain = map[i_x - 1][i_y].terrain;
        map[i_x][i_y].debt = map[i_x - 1][i_y].debt - 1;
    } else if y_debt {
        // copy y-1 neighbor terrain
        map[i_x][i_y].terrain = map[i_x][i_y - 1].terrain;
        map[i_x][i_y].debt = map[i_x][i_y - 1].debt - 1;
    } else {
        // copy no one - gen your own thing
        let index = get_index_from_rand(random_value);
        map[i_x][i_y].terrain = TYPE_ARRAY[index];
        map[i_x][i_y].debt = TYPE_DEBT_ARRAY[index];
    }
}
