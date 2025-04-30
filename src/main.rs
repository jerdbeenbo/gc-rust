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
    is not being referenced anymore, and works entirely at runtime.

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
        collect - Run your garbage collection process
        show - Display the current state of your memory pool
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
use std::{env, io::{self, Read}};

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
fn alloc(cells: &mut Vec<Cell>, req_data: i32) -> Option<usize>{

    //Find first avaliable cell to be used
    for i in 0..cells.len() {
        if cells[i].freed == true {

            //Store the data at the index position i
            cells[i] = Cell {
                data: Some(req_data),
                reference_count: 1,
                freed: false,
                is_root: false,
                references_cell: None
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
}

fn configure_roots(cells: &Vec<Cell>, a: usize, b: usize) -> Vec<usize> {

    //Arbitrarily assign cells on either end as root (this was declared
    //at execution as a command line param)
    let pos_x: usize = a;
    let pos_y: usize = b;

    vec![pos_x, pos_y]      //Return it as vector
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

    //Listen for user input, and act based on commands
    //Stop listening when the user signals to run the mark-and-sweep collection
    let mut listening: bool = true;
    while listening {
        //while accepting commands
        let mut input: String = String::new();                   //Create a new string variable each iteration to store the users input
        io::stdin()                                        //access the standard input stream
            .read_line(&mut input)      //Read what the user types and store it in input
            .expect("Unable to read Stdin");                //On fail, panic with msg
        
        println!("{}", input);
    
    }
    
}
