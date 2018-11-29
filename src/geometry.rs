


pub struct Site{
    pub nblist:Vec<usize>,
}

impl Site{
    pub fn new(x:&Vec<i32>, sizes:&Vec<i32>) -> Site {

        let mut nblist = Vec::new( );

        for dim in 0..sizes.len(){
            let mut xnb = x.clone();
            xnb[dim] = xnb[dim]+1;
            nblist.push(Lattice::coords2index(&xnb, &sizes));
            xnb[dim] = xnb[dim]-2;
            nblist.push(Lattice::coords2index(&xnb, &sizes));
        }
        Site { nblist:nblist}

    }

    
    
}

pub struct Lattice{
    pub sites:Vec<Site>,
    pub sizes:Vec<i32>,
    pub numsites:i32,
}

impl Lattice{
    pub fn new(sizes:&Vec<i32>) -> Lattice{
        let shere = sizes.clone();
        let mut sites:Vec<Site> = Vec::new();
        let mut numsites:i32 =  sizes.iter().product();
        for i in 0..numsites{
            let mut cx = Lattice::index2coords(i,&sizes);
            sites.push(Site::new(&cx,&sizes ));
        }
        Lattice {sites:sites, sizes:shere, numsites:numsites}
    }

    pub fn coords2index(x: &Vec<i32>, sizes:&Vec<i32>) -> usize{
        let mut i =0;
        let mut treatedsizes = 1;
        for dim in 0..x.len(){
            //periodic boundary conditions
            i = i + treatedsizes*((x[dim]+sizes[dim])%sizes[dim]);
            treatedsizes = treatedsizes*sizes[dim]; 
        }
        i as usize
    }

    pub fn index2coords(i: i32, sizes:&Vec<i32>) -> Vec<i32>{
        let mut treatedsizes = 1;
        let mut coords = vec![0; sizes.len()];
        for dim in 0..coords.len(){
            coords[dim] = (i/treatedsizes)%sizes[dim];
            treatedsizes = treatedsizes*sizes[dim]; 
        }
        coords
    }
}
