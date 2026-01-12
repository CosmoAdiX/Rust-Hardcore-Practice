#[derive(Debug)]
struct Triangle {
    base: u16,
    height: u16,
}

impl Triangle {
    // associative functions


    /* SO THIS IS ASSOCIATIVE FUNCTION THAT BASICALLY HELPS YOUR TO RE-USE THE FUNCTIONS AND METHODS ALL OVER THE CODESPACE. */
    fn new(new_base: u16, new_height: u16) -> Self {
        Self {
            base: new_base,
            height: new_height,
        }
    }
}

pub fn impl_function() {


    println!("\n |-->   This is an impl function and this is how we use it...");

    let new_tria = Triangle::new(120,66);

    println!(" |-->   {:?}", new_tria);

    
}

