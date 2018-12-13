use std::env;
use std::{thread, time};
use std::io;
mod geometry;
mod models;
mod algorithms;
extern crate sdl;
extern crate rand;

//use rand::Rng;

use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::{Event, Key};

fn main() {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("rust-sdl demo - video", "rust-sdl");

  //  let mut rng = rand::thread_rng();
    let scale:isize = 1;
    let sx:isize =2400;
    let sy:isize =1200;
    let screen = match sdl::video::set_video_mode(sx*scale, sy*scale, 32,
                                                  &[SurfaceFlag::HWSurface],
                                                  &[VideoFlag::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };

   let upcolor = sdl::video::Color::RGB(0,0,0); 
   let downcolor = sdl::video::Color::RGB(250,250,250); 
   let sizes = vec![sx as i32,sy as i32];

   let mut lat = geometry::Lattice::new(&sizes);
   let mut ising = models::Ising::new(&lat);
   let mut alg = algorithms::Wolf::new(&ising,1./2.269185);

    let sleeptime = time::Duration::from_millis(100);
   println!("press f for fastforeward and t to change the temperature");
   println!("termalizing ...");
   for i in 0..200{
            alg.update(&mut ising);
        }
    'main: loop {
        //update the ising model
        for i in 0..5{
            alg.update(&mut ising);
        }
        let mut currentcoord = vec![0,0];
        for i in 0usize..sx as usize {
            for j in 0usize..sy as usize {
                currentcoord[0] =i as i32;
                currentcoord[1] =j as i32;
                let index = geometry::Lattice::coords2index(&currentcoord, &sizes);
                let spin = ising.spins[index];
                let mut ccolor = upcolor;
                if spin ==-1 {
                    ccolor =downcolor;
                }
                screen.fill_rect(Some(sdl::Rect {
                    x: (i as i16*scale as i16) as i16 ,
                    y: (j as i16*scale as i16) as i16,
                    w: scale as u16,
                    h: scale  as u16
                }),ccolor);
            }
        }
        screen.flip();
        thread::sleep(sleeptime);
        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit => break 'main,
                Event::None => break 'event,
                Event::Key(k, _, _, _)
                    if k == Key::T
                    => {println!("Enter new inverse Temperature:");
                        let mut tempnew = String::new();
                        io::stdin().read_line(&mut tempnew).expect("Failed to read line");
                        let tempnew: f64 = tempnew.trim().parse().expect("Please type a number!");
                        alg.set_temperature(tempnew);
                        break 'event
                        },
                Event::Key(k, _, _, _)
                    if k == Key::F
                    => {println!("Fast foreward by n steps, enter n:");
                        let mut n = String::new();
                        io::stdin().read_line(&mut n).expect("Failed to read line");
                        let n: i32 = n.trim().parse().expect("Please type a number!");
                        for i in 0..n{
                            alg.update(&mut ising);
                        }
                        break 'event
                    },
                Event::Key(k, _, _, _)
                    if k == Key::Escape
                    => {println!("quit...");
                        break 'main},
                _ => {}
            }
        }
    }
    sdl::quit();
}

fn main2() {
    //geometry

    let sizes = vec![160,50];
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





