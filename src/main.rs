use std::env;
use std::{thread, time};
mod geometry;
mod models;
mod algorithms;

fn main() {
    //geometry

    let sizes = vec![40,20];
    //let mut w = World::new(&sizes,1./2.269);
    let mut lat = geometry::Lattice::new(&sizes);
    let mut ising = models::Ising::new(&lat);
    let mut alg = algorithms::Wolf::new(&ising, 1./2.69);
    for i in 0..1000{
        alg.update(&mut ising);
    }
    loop{
        for i in 0..20{
            alg.update(&mut ising);
        }
        ising.asci_print();
        let sleeptime = time::Duration::from_millis(1000);
        thread::sleep(sleeptime);
    }
}


//#[cfg(test)]
//mod test{
//  use super::*;
//  #[test]
//  fn test_index_coords(){
//      //check all positive coordinates
//      let sizes = vec![4,3,7];
//      for i in 0..(4*3*7){
//          let coords = Site::index2coords(i, &sizes);
//          let itest = Site::coords2index(&coords,&sizes);
//          assert_eq!(i as usize,itest);
//      }
//      let sizes = vec![2,2];
//      let xa = vec![1,0];
//      let ia = Site::coords2index(&xa,&sizes);
//      let xb = vec![-1,0];
//      let ib = Site::coords2index(&xb,&sizes);
//      assert_eq!(ia,ib);

//  }
//  #[test]
//  fn test_nblist(){
//      let sizes = vec![2,3,4];
//      let x0 = vec![0,0,0];
//      let s0 = Site::new(&x0,&sizes);
//      let nbc1 = vec![1,0,0];
//      let nbc2 = vec![0,1,0];
//      let nbc3 = vec![0,2,0];
//      let nbc4 = vec![0,0,1];
//      let nbc5 = vec![0,0,3];
//      let nbi1 = Site::coords2index(&nbc1, &sizes);
//      let nbi2 = Site::coords2index(&nbc2, &sizes);
//      let nbi3 = Site::coords2index(&nbc3, &sizes);
//      let nbi4 = Site::coords2index(&nbc4, &sizes);
//      let nbi5 = Site::coords2index(&nbc5, &sizes);
//      println!("{:?}",s0.nblist);
//      assert_eq!(s0.nblist[0], nbi1);
//      assert_eq!(s0.nblist[1], nbi1);
//      assert_eq!(s0.nblist[2], nbi2);
//      assert_eq!(s0.nblist[3], nbi3);
//  }
//  #[test]
//  fn test_update(){
//      let sizes = vec![3,4,6];
//      let mut w = World::new(&sizes, 1.);
//      w.wolf_update();
//  }
//}





