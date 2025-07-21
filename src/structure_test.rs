struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}

pub(crate) fn main() {
    // let mut user1 =build_user(String::from("2494946808@qq.com"),String::from("huashen666"));
    // user1.email = String::from("huashen666@666.com");
    //
    // let  user2 = User{
    //     email:String::from("huashen@example.com"),
    //     ..user1
    // };

    let rect = Rectangle {
        width:30,
        height: 50,
    };
    // dbg!(rect);
    let area = rect.area();
    println!("Area: {:?}", area);

    let rect1 = Rectangle{
        width:10,
        height:30
    };

    let rect2 = Rectangle{
        width:70,
        height:140
    };

    println!("Can rect hold rect1? {}", rect.can_hold(&rect1));
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
}

fn build_user(email:String, username:String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.width > other.width && self.height > other.height{
            return true;
        }
        false
    }
}

