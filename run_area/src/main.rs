#[derive(Debug)]
struct Circle {
    radius: f32,
}
impl Circle {
    fn new(rs: f32) -> Circle {
        Circle {
            radius: rs
        }
    } 
}

#[derive(Debug)]
struct Triangle {
    base: f32,
    height: f32,    
}
impl Triangle {
    fn new(be: f32, ht: f32) -> Triangle {
        Triangle {
            base: be,
            height: ht,
        }
    }
}

#[derive(Debug)]
struct Square {
    width: f32,
    height: f32,
}
impl Square {
    fn new(wh: f32, ht: f32) -> Square {
        Square {
            width: wh,
            height: ht,
        }
    }
}


trait RunArea {
    #![allow(non_snake_case)]
    fn get_Area(&self) -> f32;
}
impl RunArea for Circle {
    fn get_Area(&self) -> f32 {
        3.14 * self.radius.powf(2.0)  
    }
}

impl RunArea for Triangle {
    fn get_Area(&self) -> f32 {
        (self.base * self.height) / 2.0  
    }
}

impl RunArea for Square {
    fn get_Area(&self) -> f32 {
        self.width * self.height  
    
    }
}





fn main() {
    #![allow(non_snake_case)]
     let r = Circle::new(5.0);
     println!("{:?}, 面积 = {}", r,r.get_Area());
     
     let t = Triangle::new(5.0, 10.0);
     println!("{:?}, 面积 = {}",t,t.get_Area());
     
     let l = Square::new(5.0, 10.0);
     println!("{:?}, 面积 = {}",l,l.get_Area());
    
    
   
      
      
      
  
    
}

