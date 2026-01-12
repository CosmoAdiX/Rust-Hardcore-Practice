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
    // Method
    fn area_traingle(&mut self) -> f64 {
        (self.base as f64 * self.height as f64) / 2_f64
    }
    
}

pub fn impl_function() {


    println!("\n |-->   This is an impl function and this is how we use it...");


    // Now we can use our initialization of the triangle from anywhere by Triangle::new() and saving it in the variable and all.
    let mut new_tria = Triangle::new(27,37);

    // println!(" |-->   {:?}", new_tria);


    let v = new_tria.area_traingle();

    println!(" |-->    Soo the area of the triangle is: {} unit.", v);
    
}


