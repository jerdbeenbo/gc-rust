/*
    This is a barebones "virtual" garbage collector (mark and sweep) for Rust, demonstrating the
    advantages of Rust's ownership and borrowing features compared against a garbage collector implementation.

    What does a garbage collector do?
        - Manages memory for us semi-automatically (using explicit rules)
        - It tracks which objects are still being used and referenced
        - It reclaims memory from objects that no longer accessible
        - All done automatically without explicitly freeing memory manually

    This is a functional, albeit very small garbage collector that manages memory 
    through the use of a memory pool (implemented as a Vec<Cell>). It tracks which memory 
    is in use and reclaims memory that is no longer referenced. However, think of this more 
    as an educational demonstration than a standalone working implementation of a 
    mark-and-sweep garbage collector, as this GC operates on top of Rust's memory management 
    rather than directly managing allocations to the physical heap. Instead, we abstract 
    this away and use a Vector as a "virtual heap."

    It is mostly a demonstration that operates within Rust, with controlled memory and
    only with a primitive data type (i32).

    This is a essentially a working memory management system that is existing alongside
    Rust's already established memory system. This garabage collector manages its own
    'universe' of memory (Vec<Cell>)

    Authored by Jarred Jenkins
    https://github.com/jerdbeenbo
*/

//TODO: Need to update references to support a DFS Mark traversal system

//For collecting arguments from the user
use rand::prelude::*;
use std::{collections::VecDeque, io::{self}, vec};

//Structures
//A cell of memory that will be stored in a vector -> making up a greater "memory pool"
#[derive(Clone)]
struct Cell {
    data: Option<i32>, //Actual data within the memory pool...
    //  ...stored as an option as the default data value should be None
    reference_count: i32,           //Is this object still being referenced? (amount of references)
    freed: bool,                    //False || in use (referenced), True || not in use (de-referenced)
    is_root: bool,                  //Declares whether or not this is a root (static) entrance variable
    by_ref: Vec<usize>,             //Determins what cell(s) reference this cell
    will_ref: Vec<usize>,           //The index of a cell this cell calls reference to
    marked: bool,                   //Flag to signal if the cell has been marked for keeping. Any cell that is not marked will be sweeped
}

//Implementation for a Cell
impl Cell {
    //Creates a new cell with default values
    fn new() -> Cell {
        Cell {
            data: None,                 //Cell starts with no data
            reference_count: 0,         //Cell starts with no references
            freed: true,                //Cell starts as free, avaliable for use
            is_root: false,             //By default, cell is not a root
            by_ref: Vec::new(),         //This cell is referenced by
            will_ref: Vec::new(),       //References None cell
            marked: false,              //If the cell has been marked for keeping. Any cell that is not marked will be sweeped
        }
    }
}

//Enum to define error behaviour
#[derive(Debug)]
enum AllocError {
    Occupied,           //Target space is occupied
    NoFreeMemory,       //No free space was found to allocate memory
    DataIsFree,         //The data is not in use, cannot mutate it
}

///Index Result which is the return type for allocation functions
/// Either, it was successful and it returns the index position <usize>
/// Otherwise, it was unsuccessful -> where we return an Allocation Error specified enum above.
type IndexResult = Result<usize, AllocError>;

//Macro to abstract away what allocation function to actually use, just pass in parameters and the macro will decide which arm to match
/// Allocates memory in the memory pool with different patterns:
///
/// # Patterns
///
/// ## Pattern 0: Just data
/// ```
/// malloc!(cells, data)
/// ```
/// Allocates data in the first available cell with no references.
/// This value would be swept by the garbage collector if unreferenced.
///
/// ## Pattern 1: Automatic free allocation
/// ```
/// malloc!(cells, data, reference_to)
/// ```
/// Allocates data with a reference to another cell.
///
/// ## Pattern 2: Specific allocation
/// ```
/// malloc!(cells, data, reference, pos)
/// ```
/// Allocates data at a specific position with a reference to another cell.
///
/// # Arguments
///
/// * `cells` - A mutable reference to the memory pool vector
/// * `data` - The value to store in the cell
/// * `reference_to` - Optional reference to another cell index
/// * `pos` - Optional specific position to allocate at
///
/// # Returns
///
/// * `IndexResult` - Result containing either the allocated index or an allocation error
///
/// # Examples
///
/// ```
/// // Allocate data with no references
/// let index = malloc!(cells, 42);
///
/// // Allocate data with a reference to cell at index 0
/// let index = malloc!(cells, 42, Some(0));
///
/// // Allocate data at position 5 with a reference to cell at index 0
/// let index = malloc!(cells, 42, Some(0), 5);
/// ```
macro_rules! malloc {
    // Pattern 0 Just data - find first available cell with no reference
    ($cells:expr, $data:expr) => {
        free_alloc($cells, $data, None)   //Allocate data in memory that has no references
                                                //... this value would be sweeped by the garbage collector
    };

    //Pattern 1 (Automatic, first free-allocation)
    ($cells:expr, $data:expr, $reference_to:expr) => {
        //Three parameters, call free_alloc
        free_alloc($cells, $data, $reference_to)
    };

    //Pattern 2 (specific-allocation)
    ($cells:expr, $data:expr, $reference:ident, $pos:expr) => {
        //Four parameters, call spec_alloc
        spec_alloc($cells, $data, $reference, $pos)
    };
}

//Run once at the start during of the program to create a memory pool ->
//which is essentially just a Vec of Cell, with size n specified when the function is called.
fn init_pool(size: usize) -> Vec<Cell> {
    //Create instance of a default cell
    let default_cell = Cell::new();

    //Set up memory pool with just default implementations of cells
    let cells: Vec<Cell> = vec![default_cell; size];

    cells //Return cells
}

//Searches through the cells vec and finds a cell that is not in use, and assigns it the memory that is requested
//to be stored here. (At this stage, only supports storing i32 primitive values)
//Return an index that points to the location in memory that the data is stored
//Takes a mutable reference to the memory pool so it can update and iterate on it.
fn free_alloc(cells: &mut Vec<Cell>, req_data: i32, ref_to: Option<usize>) -> IndexResult {    
    
    //Find first avaliable cell to be used
    for i in 0..cells.len() {
        if cells[i].freed == true {
            //Store the data at the index position i
            cells[i] = Cell {
                data: Some(req_data),
                reference_count: 1,
                freed: false,
                is_root: false,
                by_ref: vec![],                     //Initially, no cells will reference this cell
                will_ref: if ref_to.is_some() {
                    vec![ref_to.unwrap()]           //Reference was provided at allocation            
                }
                else {
                    vec![]                          //Empty vector, no reference was provided at allocation
                },                                          
                marked: false,
            };

            return Ok(i); //If successful, return index I as position stored
        }
    }
    Err(AllocError::NoFreeMemory) //-> Retern no free memory as an error
}

//Allocates at a specific memory position
fn spec_alloc(cells: &mut Vec<Cell>, req_data: i32, reference: Option<usize>, store_pos: usize) -> IndexResult {
   
   let mut ref_amt: i32;
   //derive reference amt
   if reference.is_some() {
        ref_amt = 1;
   } else {
        ref_amt = 0;
   }
    
    //check if memory is allocated
    if cells[store_pos].freed == true {
        //the memory is free for use
        //store the data
        cells[store_pos] = Cell {
            data: Some(req_data),
            reference_count: ref_amt,
            freed: false,
            is_root: false,
            will_ref: if reference.is_some() {
                vec![reference.unwrap()]            //Reference was provided at allocation
            } else {
                vec![]                              //No reference was provided at allocation
            },
            by_ref: vec![],                         //Start with no cell referencing this cell
            marked: false,
        };

        return Ok(store_pos);
    }

    Err(AllocError::Occupied) //Return none as the memory position is not free, handle this by freeing pos at call
}

//frees the data at the pointer index position
//by deleting the stored information there, and replaces it with a default cell value
fn free(cells: &mut Vec<Cell>, pointer: usize) {
    cells[pointer] = Cell::new(); //Use new impl for cell to create a default cell (default state for a free cell awaiting assignment)

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
        cells[0].marked = true;     //Mark this cell to not be swept, as it is a root.
        cells[1].is_root = true;
        cells[1].freed = false;
        cells[1].marked = true;     //Mark this cell to not be swept, as it is a root.

        println!("cells {} and {} are now the roots", 0, 19);
    } else {
        //Assign the cells as roots that were chosen by the user
        //Unfree them as they'll have values (soon)
        cells[a].is_root = true;
        cells[a].freed = false;
        cells[a].marked = true;     //Mark this cell to not be swept, as it is a root.
        cells[b].is_root = true;
        cells[b].freed = false;
        cells[b].marked = true;     //Mark this cell to not be swept, as it is a root.

        println!("cells {} and {} are now the roots", a, b);
    }
}

//unroot all cells
fn unroot(cells: &mut Vec<Cell>) {
    //loop over cells and unroot all
    for i in 0..cells.len() {
        if cells[i].is_root == true {
            cells[i].is_root = false;

            println!("cell {} unrooted", i);
        }
    }

    println!();         //Print a blank line at the end of the func
}

//populate any anymaining cells with data that is not referencing anything (these will be sweeped)
fn populate_remaining(cells: &mut Vec<Cell>) {
    //loop through and populate all free cells
    let mut rng = rand::rng();
    let random_val: i32 = rng.random_range(0..1000);    //Generate a random arbitrary int value

    for i in 0..cells.len() {
        if cells[i].freed == true {
            //Cell is free
            cells[i].data = Some(random_val);           //Assign some arbitrary data (exact val, not important)
            cells[i].freed = false;                     //This cell now has data occupying it

            println!("Cell {} has been populated", i);
        }
    }

    println!();         //Print a blank line at the end of the func
}

//Function to view the current state of the memory cells
fn view_state(cells: &Vec<Cell>) {
    //just print each cell
    for i in 0..cells.len() {
        print!(
"Cell |{}|:
    1. Has data?: {}
    2. Is free?: {}
    3. Is root?: {}
    4. Ref amt: {}
    5. Ref Other?: {}
    6. Ref By?: {}
    7. MARKED: {}\n",
            i,                              //Cell position
            cells[i].data.is_some(),        //Does this cell currently store any data?
            cells[i].freed,                 //Is this cell free?
            cells[i].is_root,               //Is this cell a root?
            cells[i].reference_count,       //How many references does this cell have <inclusive>
            !cells[i].will_ref.is_empty(),   //Displays if this cell is referening another cell
            !cells[i].by_ref.is_empty(),     //Displays if this cell is referenced by another cell
            cells[i].marked,
        );
    }
}

//Processes messages
//<a> pass in a usise value to print predetermined, lengthly messages (such as a welcome)
//<b> pass in smaller, custom messages from outside of this function
fn show_message(a: Option<usize>, b: Option<String>) {
    let welcome: &str = "GCed-Rust Demonstration
    \n1. Run --help to see a list of commands.";

    if a.is_some() {
        //Boolean operator to see if a carries a value
        match a {
            Some(1) => println!("{}", welcome),
            _ => println!("invalid: use --help to configure commands"), //For none or default
        }
    } else {
        let msg = b.unwrap(); //Unwrap msg
        println!("{}", msg) //Print custom message
    }
}


///Function that is used to handle cell viability on creating references, can take n cells to check
fn cell_viability(cells: &Vec<Cell>, _cells: &Vec<usize>) -> IndexResult {

    //Check if the cells are free (i.e. not in use)
    for cell_index in _cells {
        if cells[*cell_index].freed {
            //If the cell IS free, then we shouldn't be returning a reference
            return Err(AllocError::DataIsFree);
        }
    }

    //If no errors were found, return 1
    Ok(1)
}

fn assign_reference(cells: &mut Vec<Cell>, c1pos: usize, c2pos: usize) {

    //Assign reference between two cells
    /*
        -> c1pos WILL REFERENCE c2pos
        therefore, c2pos will be referenced BY c1pos
     */

    //Check if the data can be used
    let cells_to_check: Vec<usize> = vec![c1pos, c2pos];
    let result: IndexResult = cell_viability(&cells, &cells_to_check);

    //Boolean flag
    let mut check: bool = false;

    //Perform action or report error
    match result {
        Ok(val) => check = true,                        //Boolean flag to progress the function
        Err(why) => println!("{}", match why {
            AllocError::Occupied
                => "Space is occupied",                         //Report error
            AllocError::NoFreeMemory
                => "No free memory avaliable",
            AllocError::DataIsFree
                => "The memory was free, not suitable for use",
        }),
    }

    //Only create references if allowed
    if check {
        //Cell 1
        cells[c1pos].reference_count = cells[c1pos].reference_count + 1;        //Increase reference count
        if !cells[c1pos].will_ref.contains(&c2pos) {                            //...only add reference if it doesn't already exist
            cells[c1pos].will_ref.push(c2pos);                                  //Push c2pos into vector of references
        }

        //Cell 2
        cells[c2pos].reference_count = cells[c2pos].reference_count + 1;        //Increase reference count
        if !cells[c2pos].by_ref.contains(&c1pos) {                              //...only add reference if it doesn't already exist
            cells[c2pos].by_ref.push(c1pos);                                    //Push c1pos into vector of references
        }
    }

}

///Runs the marking (Non-recursive stack-based DFS) algorithm on all cells of memory on the virtual heap.
/// #### Parameters
/// `cells` -> requires a mutable reference to the cells vector of type `Vec<Cell>`
/// #### Example usage
/// ```
/// mark(cells);
/// ```
/// Does not return anything, as it mutates the cells directly and marks their `marked` boolean flag
fn mark(cells: &mut Vec<Cell>) {
    //get root index position
    let mut roots: Vec<usize> = Vec::new();
    for i in 0..cells.len() {
        if cells[i].is_root {
            roots.push(i);
        }
    }

    //Traverse the graph (DFS) and mark them with a mark bit flag
    //Left->Right traversal Vertical first horizontal next
    
    //Start at left-most root (index 0 of the roots vector), then sequentially move along roots until all cells are marked as traversed
    //The by_ref field will be how we fallback recursively
    //Follow the will_ref until a dead end

    //TODO: Handle Reference BY, if the value is still being referenced by another cell BUT it itself
    //doesnt reference a cell, it shouldn't be swept. (currently it is)

    let mut stack: VecDeque<usize> = VecDeque::new();

    for root in roots {
        //Beginning at the root cell, begin updating cells
        //Root <usize> is our index link into the cells heap memory pool
        if cells[root].will_ref.is_empty() {
            //Cell doesn't reference anything
            continue;           //Specifically specifiy to continue for readability...
        }
        else {
            //-> traverse its references

            //Initialise variables for current and next position
            let mut current_pos: usize = root;

            //Ensure root is marked (Roots should be marked when they are made)
            if !cells[current_pos].marked {
                //if it is not marked, fix and mark here
                cells[current_pos].marked = true;
            }

            //Add adjacent nodes into stack
            for node in 0..cells[current_pos].will_ref.len() {
                
                //Record the nodes
                stack.push_back(cells[current_pos].will_ref[node]);
            }

            //Start traversing along the stack nodes
            for i in 0..stack.len() {             //will_ref is a vector of cells that current_pos references

                //Visited this node, pop it from the stack
                stack.pop_front();

                //This cell is still in use (is still being referenced)
                //mark as safe to keep
                cells[stack[i]].marked = true;

                //Now check if the cell also has its OWN list of referenced cells
                if !cells[stack[i]].will_ref.is_empty() {
                    // This cell has it's own list of references, continue further down the graph

                    //move cell position
                    current_pos = stack[i];

                    //Add adjacent nodes into stack
                    for node in 0..cells[current_pos].will_ref.len() {
                        
                        //Record the nodes
                        stack.push_back(cells[current_pos].will_ref[node]);
                    }
                }
            }

        }
    }
}   

        // //Loop through all the cells and record their index positions if they are not a root or
    // //dont reference another cell
    // let mut r: Vec<usize> = Vec::new();
    // for i in 0..cells.len() {
    //     if cells[i].reference_count == 0 && cells[i].is_root == false {
    //         r.push(i);
    //     }
    // }

    // //return a vector of index positions to sweep (free)
    // r 

/// The sweeping phase of the garbage collector (free any memory cell that isn't referencing anything or is being referenced)
/// #### Example Cell To Be Swept (Freed)
/// ```
/// Cell 
/// {
///     data: <...>
///     reference_count: <...>
///     freed: <...>
///     is_root: <...>
///     by_ref: <...>
///     will_ref: <...>
///     marked: false,      // <- This cell is not marked to keep, and therefore it is determined to not be in use anymore          
/// }
/// ```
fn sweep(cells: &mut Vec<Cell>) {
    //free (sweep) all the cells are position usize

    //run the free function on each cell that is not marked
    for i in 0..cells.len() {
        if !cells[i].marked {
            free(cells, i);        //pass in cell index position
        }
    }
}

/// This function runs the entire garbage collection algorithm.
/// ### Logic flow
/// This function runs these two commands.
/// ```
/// mark() -> sweep();
/// ```
/// And does not return anything, allowing it to be called within a matching arm during the user input phase.
fn collect(cells: &mut Vec<Cell>) {
    //'mark' cells to be freed (sweeped)
    mark(cells);

    //Sweep unreferenced and no longer in use cells
    sweep(cells);
}

//The program will randomly create references between memory cells with
//real data
fn create_free_ref(cells: &mut Vec<Cell>, times_to_run: usize) {
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
        let index = malloc!(cells, (data[root] as i32) * (data[root] as i32), Some(roots[root]));   //First free allocation

        match index {
            Ok(index) => println!("Cell at position {} was used", index),   //Report to the console what index was used
            Err(why) => println!("{}", match why {
                AllocError::Occupied
                    => "Space is occupied",     //Report error
                AllocError::NoFreeMemory
                    => "No avaliable memory found",
                AllocError::DataIsFree
                    => "The memory was free, not suitable for use",
            }),
        }
    }
    println!(); //Add a line
}

fn parse_param_to_usize(param: Option<&&str>, default: usize) -> usize {
    match param {
        Some(value) => {
            // Try to parse the string to a number
            match value.trim().parse::<usize>() {
                Ok(number) => number, // Successfully parsed
                Err(_) => {
                    println!(
                        "Warning: Could not parse '{}' as a number. Using default: {}",
                        value, default
                    );
                    default // Use default if parsing fails
                }
            }
        }
        None => {
            default // Use default if no parameter provided
        }
    }
}

//Function for handling allocation from prompt
//TODO: some tasks to expand here
fn handle_prompt_allocation(cells: &mut Vec<Cell>, index: usize) {
    let mut rng: ThreadRng = rand::rng();
    let data: i32 = rng.random_range(0..50);                                    //Generate some arbitrary data TODO: actually handle data

    let index = malloc!(cells, data, None, index);  //Handle no references TODO: Meanful connection of references

    match index {
        Ok(index) => println!("Cell at position {} was used", index),   //Report to the console what index was used
        Err(why) => println!("{}", match why {
            AllocError::Occupied
                => "Space is occupied",                                         //Report error
            AllocError::NoFreeMemory
                => "No free memory avaliable",
            AllocError::DataIsFree
                => "The memory was free, not suitable for use",
        }),
    }
}

//Main input loop of the program, listen for commands from the user
fn listen(listening: bool, cells: &mut Vec<Cell>) {
    while listening {
        //while accepting commands
        let mut input: String = String::new(); //Create a new string variable each iteration to store the users input
        io::stdin() //access the standard input stream
            .read_line(&mut input) //Read what the user types and store it in input
            .expect("Unable to read Stdin"); //On fail, panic with msg

        let input: Vec<&str> = input.split(' ').collect();      //remove whitespace
                                                                //Get the first command
        let command: &str = input[0];
        //Commands can take up to 2 inputs
        let fparam: Option<&&str> = input.get(1);       //&& reference to a reference
        let sparam: Option<&&str> = input.get(2);       //&& reference to a reference

        //these parameters will always be cell index position, so make adjustments
        let index1 = parse_param_to_usize(fparam, 0); // Default to 0 if parameter missing or invalid
        let index2 = parse_param_to_usize(sparam, cells.len() - 1); // Default to last cell if missing

        //Seperate values

        match command.trim() {
            "--help" => println!(
                "\nAvaliable Commands:
    1. --root <cell_index_pos>(0-19) <cell_index_pos>(0-19)
    2. --unroot
    3. --arb_ref <amount_of_times>
    4. --link_ref <Cell 1> *references...->* <Cell 2>
    5. --alloc_at <Cell>
    6. --state
    7. --populate
    8. --gc
    9. --exit"
            ), //Print a the accepted list of commands
            "--root" => configure_roots(cells, index1, index2), //Root cells, or default a: 0, b: len-1
            "--unroot" => unroot(cells),                        //Unroot all
            "--arb_ref" => create_free_ref(cells, index1), //Run as many times as specified
            "--gc" => collect(cells), //Run the garbage collector (mark and sweep)
            "--state" => view_state(cells),
            "--exit" => println!("Exiting"),
            "--populate" => populate_remaining(cells),
            "--alloc_at" => handle_prompt_allocation(cells, index1),
            "--link_ref" => assign_reference(cells, index1, index2),    //Cell 1 references Cell 2
            _ => println!("Unknown command. Type 'help' for assistance."), //Default if command doesn't match
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
    /*
    A true heap would use actual memory addresses and pointers.
    This implementation is a simulation of heap behavior within Rust's safe memory model.
    Therefore we handle 'pointers' as just index positions of this vector <usize>
     */
    let mut cells: Vec<Cell> = init_pool(20);

    let msg: usize = 1; //Welcome message
    show_message(Some(msg), None); //Run the initial message

    //Listen for user input, and act based on commands
    //Stop listening when the user signals to run the mark-and-sweep collection
    let mut listening: bool = true;
    //main loop of the program | listen for commands from the user
    listen(listening, &mut cells);
}