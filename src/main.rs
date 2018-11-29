use std::env;
use std::{thread, time};

use std::io::Write;
use std::io::stdout;
extern crate rand;
use rand::prelude::*;

struct Site{
    spin:i32,
    nblist:Vec<usize>,
    isdiscovered:bool
}

impl Site{
    pub fn new(x:&Vec<i32>, sizes:&Vec<i32>) -> Site {

        let mut nblist = Vec::new( );

        for dim in 0..sizes.len(){
            let mut xnb = x.clone();
            xnb[dim] = xnb[dim]+1;
            nblist.push(Site::coords2index(&xnb, &sizes));
            xnb[dim] = xnb[dim]-2;
            nblist.push(Site::coords2index(&xnb, &sizes));
        }
        Site {spin:1, nblist:nblist, isdiscovered:false}

    }

    fn coords2index(x: &Vec<i32>, sizes:&Vec<i32>) -> usize{
        let mut i =0;
        let mut treatedsizes = 1;
        for dim in 0..x.len(){
            //periodic boundary conditions
            i = i + treatedsizes*((x[dim]+sizes[dim])%sizes[dim]);
            treatedsizes = treatedsizes*sizes[dim]; 
        }
        i as usize
    }

    fn index2coords(i: i32, sizes:&Vec<i32>) -> Vec<i32>{
        let mut treatedsizes = 1;
        let mut coords = vec![0; sizes.len()];
        for dim in 0..coords.len(){
            coords[dim] = (i/treatedsizes)%sizes[dim];
            treatedsizes = treatedsizes*sizes[dim]; 
        }
        coords
    }
    
}

struct World{
    sites:Vec<Site>,
    sizes:Vec<i32>,
    numsites:i32,
    rng:rand::prelude::ThreadRng,
    beta:f64,
    padd:f64
}

impl World{
    pub fn new(sizes:&Vec<i32>, beta:f64) -> World{
        let shere = sizes.clone();
        let mut sites:Vec<Site> = Vec::new();
        let mut numsites:i32 =  sizes.iter().product();
        let mut rng = rand::thread_rng();
        for i in 0..numsites{
            let mut cx = Site::index2coords(i,&sizes);
            sites.push(Site::new(&cx,&sizes ));
        }
        let padd = 1. - (-2.0*beta).exp();
        World {sites:sites, sizes:shere, numsites:numsites, rng:rng,beta:beta, padd:padd}
    }

    pub fn wolf_update(&mut self) {
        // select a random starting point
        let r:f64 = self.rng.gen();
        let ri:usize = (r*self.numsites as f64 ) as usize;
        self.sites[ri].isdiscovered=true;
        // keep track of discovered sites to reset the isdiscovered property
        let mut discoveredsites = vec![ri];
        let mut queue:Vec<usize> = vec![ri];
        let mut level = 0;
        while queue.len() != 0{
            level = level+1;
            //take a site from the queue:
            let xs:usize = queue.pop().unwrap();
            // include its neighbours with probability padd if they are aligned
            let nblist = self.sites[xs].nblist.clone();
            for nb in nblist{
                if ((!self.sites[nb].isdiscovered) &&
                         self.sites[nb].spin == self.sites[ri].spin){
                    let r:f64 =self.rng.gen();
                    if r < self.padd{
                        //println!("{}",r);
                        queue.push(nb);
                        self.sites[nb].isdiscovered = true;
                        discoveredsites.push(nb);
                    }
                }
            }
            self.sites[xs].spin = -self.sites[xs].spin;
            //flip the spin at xs
        }
        //println!("level = {}",level);
        //reset the isdiscovered
        for xd in discoveredsites{
            self.sites[xd].isdiscovered= false;
        }
    }

    pub fn asci_print(&mut self){

        if self.sizes.len() == 2{
            print!("{}[2J", 27 as char);
            println!("beta: {}, padd:{}", self.beta, self.padd);

            for x in 0..self.sizes[1]{
                for y in 0..self.sizes[0]{
                    let coords = vec![y,x];
                    let i = Site::coords2index(&coords, &self.sizes);
                    if self.sites[i as usize].spin == 1{
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


#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_index_coords(){
        //check all positive coordinates
        let sizes = vec![4,3,7];
        for i in 0..(4*3*7){
            let coords = Site::index2coords(i, &sizes);
            let itest = Site::coords2index(&coords,&sizes);
            assert_eq!(i as usize,itest);
        }
        let sizes = vec![2,2];
        let xa = vec![1,0];
        let ia = Site::coords2index(&xa,&sizes);
        let xb = vec![-1,0];
        let ib = Site::coords2index(&xb,&sizes);
        assert_eq!(ia,ib);

    }
    #[test]
    fn test_nblist(){
        let sizes = vec![2,3,4];
        let x0 = vec![0,0,0];
        let s0 = Site::new(&x0,&sizes);
        let nbc1 = vec![1,0,0];
        let nbc2 = vec![0,1,0];
        let nbc3 = vec![0,2,0];
        let nbc4 = vec![0,0,1];
        let nbc5 = vec![0,0,3];
        let nbi1 = Site::coords2index(&nbc1, &sizes);
        let nbi2 = Site::coords2index(&nbc2, &sizes);
        let nbi3 = Site::coords2index(&nbc3, &sizes);
        let nbi4 = Site::coords2index(&nbc4, &sizes);
        let nbi5 = Site::coords2index(&nbc5, &sizes);
        println!("{:?}",s0.nblist);
        assert_eq!(s0.nblist[0], nbi1);
        assert_eq!(s0.nblist[1], nbi1);
        assert_eq!(s0.nblist[2], nbi2);
        assert_eq!(s0.nblist[3], nbi3);
    }
    #[test]
    fn test_update(){
        let sizes = vec![3,4,6];
        let mut w = World::new(&sizes, 1.);
        w.wolf_update();
    }
}



fn main() {
    //geometry

    let sizes = vec![800,240];
    //let mut w = World::new(&sizes,1./2.269);
    let mut w = World::new(&sizes,1./2.269);
    for i in 0..1000{
        w.wolf_update();
    }
    loop{
        for i in 0..20{
            w.wolf_update();
        }
        w.asci_print();
        let sleeptime = time::Duration::from_millis(1000);
        thread::sleep(sleeptime);
    }
}

