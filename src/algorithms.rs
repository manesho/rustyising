extern crate rand;
use algorithms::rand::prelude::*;

use geometry;
use models;


pub struct Wolf{
    rng:rand::prelude::ThreadRng,
    isdiscovered:Vec<bool>,
    padd:f64
}
impl Wolf{
    pub fn new(m:&models::Ising, beta:f64) -> Wolf {
        let mut isdiscovered = vec![false; m.lattice.numsites as usize];
        let padd = 1. - (-2.0*beta).exp();
        Wolf{rng:rand::thread_rng(), isdiscovered:isdiscovered, padd:padd}
    }
    pub fn set_temperature(&mut self, b:f64){
        self.padd=1. - (-2.0*b).exp();
    }
pub fn update(&mut self, m:&mut models::Ising) {
    // select a random starting point
    let r:f64 = self.rng.gen();
    let ri:usize = (r*m.lattice.numsites as f64 ) as usize;
    self.isdiscovered[ri]=true;
    // keep track of discovered sites to reset the isdiscovered property
    let mut discoveredsites = vec![ri];
    let mut queue:Vec<usize> = vec![ri];
    let mut level = 0;
    while queue.len() != 0{
        level = level+1;
        //take a site from the queue:
        let xs:usize = queue.pop().unwrap();
        // include its neighbours with probability padd if they are aligned
        let nblist = m.lattice.sites[xs].nblist.clone();
        for nb in nblist{
            if ((!self.isdiscovered[nb]) &&
                     m.spins[nb] == m.spins[ri]){
                let r:f64 =self.rng.gen();
                if r < self.padd{
                    //println!("{}",r);
                    queue.push(nb);
                    self.isdiscovered[nb] = true;
                    discoveredsites.push(nb);
                }
            }
        }
        m.spins[xs] = -m.spins[xs];
        //flip the spin at xs
    }
    //println!("level = {}",level);
    //reset the isdiscovered
    for xd in discoveredsites{
        self.isdiscovered[xd]= false;
    }
}
}

