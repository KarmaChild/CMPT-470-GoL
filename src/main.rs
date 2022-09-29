mod game_of_life;


fn main() {

    let a = game_of_life::initialize_grid(game_of_life::read_file_to_string_buffer("input.txt"),game_of_life::make_grid(100,100));
    let b = game_of_life::make_grid(100,100);


    game_of_life::write_to_file(game_of_life::game_of_life(a,b),"output.txt");

}
