/*
    This is a barebones garbage collector (mark and sweep) for Rust, demonstrating the
    advantages of Rust's ownership and borrowing features compared against a garbage collector implementation.
    My findings from this experiement are explored in a report, accompanied within the repo.

    What does a garbage collector do?
        - Manages memory for us semi-automatically (using explicit rules)
        - It tracks which objects are still being used and referenced
        - It reclaims memory from objects that no longer accessible
        - All done automatically without explicitly freeing memory manually

    This is a functional, albiet very small garbage collector that manages memory through
    the use of a memory pool (Vec of Cell), it tracks what memory is in use, reclaims memory that
    is not being referenced anymore. However, think of this more as an educational demonstration than a standalone
    working implementation of a mark and sweep garbage collector

    However, it is mostly a demonstration that operates within Rust, with controlled memory and
    only with a primitive data type.

    This is a essentially a working memory management system that is existing alongside
    Rust's already established memory system. This garabage collector manages its own
    'universe' of memory (Vec<Cell>)

    Authored by Jarred Jenkins
    https://github.com/jerdbeenbo
*/

//Have this program running and then run commands from another window through
//a command-interface post program or something
/*
    Commands:
        allocate <value> - Create a new cell with a specific value
        reference <from> <to> - Make one cell reference another
        root <index> - Mark a specific cell as a root
        unroot <index> - Remove root status from a cell
        gc - Run your garbage collection process
        show - Display the current state of your memory pool
        populate - Populate remaining cells with data that will be sweeped
        help - List available commands
        exit - End the program

    After each action, show memory state:
        -> Which cells contain objects
        -> What references exist between objects
        -> Which objects are roots (directly reachable from variables)
        -> Which objects are currently reachable

    After running garbage collections:
        -> Show which objects were marked as reclaimed
*/

//For collecting arguments from the user
use std::io::{self};
use rand::{prelude::*, rng};

//A cell of memory that will be stored in a vector -> making up a greater "memory pool"
#[derive(Clone)]
struct Cell {
    data: Option<i32>,              //Actual data within the memory pool...
                                    //  ...stored as an option as the default data value should be None
    reference_count: i32,           //Is this object still being referenced? (amount of references)
    freed: bool,                    //False || in use (referenced), True || not in use (de-referenced)
    is_root: bool,                  //Declares whether or not this is a root (static) entrance variable
    references_cell: Option<usize>  //The index of a cell this cell references
}

//Implementation for a Cell
impl Cell {

    //Creates a new cell with default values
    fn new() -> Cell {
        Cell {
            data: None,
            reference_count: 0,
            freed: true,
            is_root: false,
            references_cell: None
        }
    }

    // Clone the current Cell
    fn get_cell(&self) -> Cell {
        Cell {
            data: self.data,
            reference_count: self.reference_count,
            freed: self.freed,
            is_root: self.is_root,
            references_cell: self.references_cell
        }
    }
}

//Run once at the start during of the program to create a memory pool ->
//which is essentially just a Vec of Cell, with size n specified when the function is called.
fn init_pool(size: usize) -> Vec<Cell> {

    //Create instance of a default cell
    let default_cell = Cell::new();

    //Set up memory pool with just default implementations of cells
    let cells: Vec<Cell> = vec![default_cell; size];

    cells   //Return cells
}

//Searches through the cells vec and finds a cell that is not in use, and assigns it the memory that is requested
//to be stored here. (At this stage, only supports storing i32 primitive values)
//Return an index that points to the location in memory that the data is stored
//Takes a mutable reference to the memory pool so it can update and iterate on it.
fn alloc(cells: &mut Vec<Cell>, req_data: i32, reference: usize) -> Option<usize>{

    println!("Receiving data value {}", req_data);
    println!("Receiving reference root {}", reference);


    //Find first avaliable cell to be used
    for i in 0..cells.len() {
        if cells[i].freed == true {

            //Store the data at the index position i
            cells[i] = Cell {
                data: Some(req_data),
                reference_count: 1,
                freed: false,
                is_root: false,
                references_cell: Some(reference)
            };

            return Some(i);     //Return the index
        }
    }
    None    //-> Return None as there is no memory freely avaliable for storing any data at the moment
}

//frees the data at the pointer index position
//by deleting the stored information there, and replaces it with a default cell value
fn free(cells: &mut Vec<Cell>, pointer: usize) {
    cells[pointer] = Cell::new();

    println!("Cell {} was freed, and is now ready for use again", pointer);
}

//configure 2 cells to root
fn configure_roots(cells: &mut Vec<Cell>, a: usize, b: usize) {

    //error handle
    if a > 19 || b > 19 {
        //set values to default
        //Unfree them as they'll have values (soon)
        println!("One value was out of bounds, using defaults...");
        cells[0].is_root = true;
        cells[0].freed = false;
        cells[1].is_root = true;
        cells[1].freed = false;

        println!("cells {} and {} are now the roots", 0, 19);
    }
    else {
        //Assign the cells as roots that were chosen by the user
        //Unfree them as they'll have values (soon)
        cells[a].is_root = true;
        cells[a].freed = false;
        cells[b].is_root = true;
        cells[b].freed = false;

        println!("cells {} and {} are now the roots", a, b);
    }
}

//unroot all cells
fn unroot(cells: &mut Vec<Cell>) {
    //loop over cells and unroot all
    for i in 0..cells.len() {
        if cells[i].is_root == true {
            cells[i].is_root == false;

            println!("cell {} unrooted", i);
        }
    }

    println!();     //Add a space
}

//populate any anymaining cells with data that is not referencing anything (these will be sweeped)
fn populate_remaining(cells: &mut Vec<Cell>) {
    //loop through and populate all free cells
    let rng: i32 = rng().random_range(0..1000);    //Generate a random arbitrary int value

    for i in 0..cells.len() {
        if cells[i].freed == true {
            //Cell is free
            cells[i].data = Some(rng);      //Assign some arbitrary data (exact val, not important)
            cells[i].freed = false;         //This cell now has data occupying it

            println!("Cell {} has been populated", i);
        }
    }

    println!();
}

//Function to view the current state of the memory cells
fn view_state(cells: &Vec<Cell>) {

    //just print each cell
    for i in 0..cells.len() {
        print!("\nCell |{}|:
        \n1. Has data?: {}
        \n2. Is free?: {}
        \n3. Is root?: {}
        \n4. Ref amt: {}
        \n5. References?: {}"
    
    , i, cells[i].data.is_some(), cells[i].freed, cells[i].is_root, cells[i].reference_count, cells[i].references_cell.is_some());
    }
}

//Processes messages
//<a> pass in a usise value to print predetermined, lengthly messages (such as a welcome)
//<b> pass in smaller, custom messages from outside of this function
fn show_message(a: Option<usize>, b: Option<String>) {
    
    let welcome: &str = "GCed-Rust Demonstration
    \n1. Run --help to see a list of commands.";

    if a.is_some() {    //Boolean operator to see if a carries a value
        match a {
            Some(1) => println!("{}", welcome),
            _ => println!("invalid: use --help to configure commands")  //For none or default
        }
    }
    else {
        let msg = b.unwrap();       //Unwrap msg
        println!("{}", msg)         //Print custom message
    }

}

//The marking phase of the garbage collector
fn mark(cells: &Vec<Cell>) -> Vec<usize>{

    //Loop through all the cells and record their index positions if they are not a root or 
    //dont reference another cell
    let mut r: Vec<usize> = Vec::new();
    for i in 0..cells.len() {
        if cells[i].reference_count == 0 && cells[i].is_root == false {
            r.push(i);
        }
    }


    //return a vector of index positions to sweep (free)
    r
}

//The sweeping phase of the garbage collector (free any memory cell that isn't referencing anything or is being referenced)
fn sweep(cells: &mut Vec<Cell>, sweep_list: Vec<usize>) {
    //free (sweep) all the cells are position usize

    //run the free function on each cell
    for sweep in sweep_list {
        free(cells, sweep);
    }
}

//Begin the garbage collection
fn collect(cells: &mut Vec<Cell>) {

    //'mark' cells to be freed (sweeped)
    let sweep_list: Vec<usize> = mark(&cells);

    sweep(cells, sweep_list);
}

//The program will randomly create references between memory cells with
//real data
fn create_references(cells: &mut Vec<Cell>, times_to_run: usize) {
    let mut rng = rand::rng();

    //keep track of what cells are roots
    let mut roots: Vec<usize> = Vec::new();

    //keep track of the data stored in them
    let mut data: Vec<i32> = Vec::new();

    //set data of root memory cells
    for i in 0..cells.len() {
        if cells[i].is_root {
            
            //Create and store data
            let _data = rng.random_range(1..50);
            data.push(_data);

            //Assign data to mem cell
            cells[i].data = Some(_data);

            //store index of root
            roots.push(i);
        }
    }
    //assign a new value that is a product (makes reference to) one of the root cells
    //choose which root
    let root = rng.random_range(0..roots.len());

    //TODO: This currently just spams the same value in multiple memory cells, change this up
    //for now and for pure demonstration purposes, it is fine and will work, but is predictable and boring
    for i in 0..times_to_run {
        let cell_index = alloc(cells, (data[root] as i32) * (data[root] as i32), roots[root]);

        //print if it was a success or not
        if cell_index.is_some() {
            println!("Success! Cell with value {} was created at pos {} referencing root {}", (data[root] as i32) * (data[root] as i32), cell_index.unwrap(), roots[root]);
        }
    }
    println!(); //Add a line
}

fn parse_param_to_usize(param: Option<&&str>, default: usize) -> usize {
    match param {
        Some(value) => {
            // Try to parse the string to a number
            match value.trim().parse::<usize>() {
                Ok(number) => number,  // Successfully parsed
                Err(_) => {
                    println!("Warning: Could not parse '{}' as a number. Using default: {}", value, default);
                    default  // Use default if parsing fails
                }
            }
        },
        None => {
            default  // Use default if no parameter provided
        }
    }
}

//Main input loop of the program, listen for commands from the user
fn listen(listening: bool, cells: &mut Vec<Cell>) {
    while listening {
        //while accepting commands
        let mut input: String = String::new();                   //Create a new string variable each iteration to store the users input
        io::stdin()                                        //access the standard input stream
            .read_line(&mut input)      //Read what the user types and store it in input
            .expect("Unable to read Stdin");                //On fail, panic with msg
            
        let input: Vec<&str> = input.split(' ').collect();           //remove whitespace
        //Get the first command
        let command: &str = input[0];
        //Commands can take up to 2 inputs
        let fparam: Option<&&str> = input.get(1);    //&& reference to a reference
        let sparam: Option<&&str> = input.get(2);    //&& reference to a reference

        //these parameters will always be cell index position, so make adjustments
        let index1 = parse_param_to_usize(fparam, 0);  // Default to 0 if parameter missing or invalid
        let index2 = parse_param_to_usize(sparam, cells.len() - 1);  // Default to last cell if missing

        //Seperate values

        match command.trim() {
            "--help" => println!("\nAvaliable Commands:
            \n1. --root <cell_index_pos>(0-19) <cell_index_pos>(0-19)
            \n2. --unroot
            \n3. --create_ref <amount_of_times>
            \n4. --state
            \n5. --populate
            \n6. --gc
            \n7. --exit"),         //Print a the accepted list of commands
            "--root" => configure_roots(cells, index1, index2),             //Root cells, or default a: 0, b: len-1
            "--unroot" => unroot(cells),                                         //Unroot all
            "--create_ref" => create_references(cells, index1),     //Run as many times as specified
            "--gc" => collect(cells),                                           //Run the garbage collector (mark and sweep)
            "--state" => view_state(cells),
            "--exit" => println!("Exiting"),
            "--populate" => populate_remaining(cells),
            _ => println!("Unknown command. Type 'help' for assistance.")       //Default if command doesn't match
        }
    }
}


fn main() {

    //1. Create a memory pool
    /*
        A memory pool, AKA memory allocator or memory management pool, is a
        software or hardware structure used to manage dynamic memory allocation
        in a computer program.
        Used to efficiently allocate and deallocate memory for data structures
        and objects during program execution. It is a pre-allocated region
        of memory that is divided into fixed-size blocks. Memory pools are a form
        of dynamic memory allocation that offers a number of advantages over
        traditional methods such as malloc and free found in C systems programming.
    */

    //Fixed-size Memory Pool of Memory Cells stored in a vec (the vector IS the memory pool)
    //This would be comparible to the heap
    let mut cells: Vec<Cell> = init_pool(20);

    let msg: usize = 1;                         //Welcome message
    show_message(Some(msg), None);         //Run the initial message

    //Listen for user input, and act based on commands
    //Stop listening when the user signals to run the mark-and-sweep collection
    let mut listening: bool = true;
    //main loop of the program | listen for commands from the user
    listen(listening, &mut cells);
    
}
