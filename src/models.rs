use geometry;
use std::io::Write;
use std::io::stdout;


pub struct Ising<'a>{
 pub spins:Vec<i32>,
 pub lattice:&'a geometry::Lattice
}


impl<'a> Ising<'a>{
    pub fn new(lattice:&'a geometry::Lattice) -> Ising<'a>{
       let mut spins = vec![1; lattice.numsites as usize];
       Ising{spins:spins , lattice:lattice}
    }
    pub fn asci_print(& mut self){

        if self.lattice.sizes.len() == 2{
            print!("{}[2J", 27 as char);

            for x in 0..self.lattice.sizes[1]{
                for y in 0..self.lattice.sizes[0]{
                    let coords = vec![y,x];
                    let i = geometry::Lattice::coords2index(&coords, &self.lattice.sizes);
                    if self.spins[i as usize] == 1{
                        print!("\x1B[37m\u{25A0}");
                    } else {
                        print!("\x1B[30m\u{25A0}");
                    }
                }
                print!("\n")
            }
            print!("-------------------------------------\n");
            stdout().flush();
        }

    }
}

