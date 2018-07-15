use std::env;
use std::net::UdpSocket;

extern crate nalgebra as na;
extern crate ncollide3d;
use na::{Isometry3, Vector3, Point3};
use ncollide3d::query::{Ray, RayCast};
use ncollide3d::shape::*;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Character {
    position: Vector3<f32>,
    velocity: Vector3<f32>,
    view_forward: Vector3<f32>,
}
impl Default for Character {
    fn default() -> Character {
        Character {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            view_forward: Vector3::new(1.0, 0.0, 0.0),
        }
    }
}

struct Gamestate {
    characters: Vec<Character>,
}

fn main() {
    let cuboid = Cuboid::new(Vector3::new(1.0, 2.0, 1.0));
    let ray_inside = Ray::new(na::origin::<Point3<f32>>(), Vector3::y());
    let ray_miss = Ray::new(Point3::new(2.0, 2.0, 2.0), Vector3::new(1.0, 1.0, 1.0));

    assert!(cuboid.toi_with_ray(&Isometry3::identity(), &ray_inside, true).unwrap() == 0.0);
    assert!(cuboid.toi_with_ray(&Isometry3::identity(), &ray_inside, false).unwrap() == 2.0);
    assert!(cuboid.toi_with_ray(&Isometry3::identity(), &ray_miss, false).is_none());
    assert!(cuboid.toi_with_ray(&Isometry3::identity(), &ray_miss, true).is_none());


    let server = env::args().len() == 2 && env::args().nth(1).unwrap() == "--server";
    if server {
        let socket = UdpSocket::bind("127.0.0.1:34254").expect("failed to bind address");
        let mut buf = [0; 100];
        let (_amt, src) = socket.recv_from(&mut buf).expect("failed to recv message");
        let mut character: Character = bincode::deserialize(&buf[..]).unwrap();
        character.position += character.velocity;
        let enc = bincode::serialize(&character).unwrap();
        socket.send_to(&enc, &src).expect("failed to send message");
    } else {
        let socket = UdpSocket::bind("127.0.0.1:34255").expect("failed to bind address");
        socket.connect("127.0.0.1:34254").expect("failed to connect");
        let mut character = Character{velocity: Vector3::new(1.0, 0.0, 0.0), ..Default::default()};
        println!("{:?}", character);
        socket.send(
            &bincode::serialize(&character).unwrap()
        ).expect("failed to send message");
        let mut buf = [0; 100];
        socket.recv(&mut buf).expect("failed to recv message");
        character = bincode::deserialize(&buf[..]).unwrap();
        println!("{:?}", character);
    }

    /*
    let mut g = Gamestate{
        characters: vec![],
    };
    loop {
        for x in &g.characters {
            println!("{:?}", x);
        }
        for x in &mut g.characters {
            x.position += x.velocity;
        }
    }
    */
}
