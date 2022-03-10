# Using Structs to Structure Related

## 구조체를 정의하고 초기화하기

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
struct Color(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("two@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    user1.email = String::from("anotheremail@example.com");
    let black = Color(0, 0, 0);

    println!("{}", user1.email);
    println!("{}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

```

어떤 필드도 없는 구조체 역시 정의할 수 있습니다! 이는 유닛 타입인 ()와 비슷하게 동작하고, 그 때문에 유사 유닛 구조체(unit-like structs)라 불립니다. 유사 유닛 구조체는 특정한 타입의 트레잇(trait)을 구현해야하지만 타입 자체에 데이터를 저장하지 않는 경우에 유용합니다. 트레잇(trait)에 대해서는 10장에서 더 살펴보도록 하겠습니다.

User 구조체 정의에서, &str 문자 슬라이스 타입 대신 String타입을 사용했습니다. 이는 의도적인 선택으로, 구조체 전체가 유효한 동안 구조체가 그 데이터를 소유하게 하고자 함입니다.

구조체가 소유권이 없는 데이터의 참조를 저장할수는 있지만, 10장에서 언급 될 라이프타임(lifetimes) 의 사용을 전제로 합니다. 라이프타임은 구조체가 존재하는동안 참조하는 데이터를 계속 존재할 수 있도록 합니다.

## 예제

```rust
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
```

## 파생 트레잇(derived trait)으로 유용한 기능 추가하기

```rust
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
}
```

{} 내에 :? 명시자를 집어넣는 것은 println!에게 Debug라 불리우는 출력 포맷을 사용하고 싶다고 말해줍니다. Debug는 개발자에게 유용한 방식으로 우리의 구조체를 출력할 수 있도록 해줘서 우리 코드를 디버깅 하는 동안 그 값을 볼수 있게 해주는 트레잇입니다.

## 메소드 문법

```rust
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

```

```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

이러한 자동 참조 동작은 메소드가 명확한 수신자-즉 self의 타입을 가지고 있기 떄문에 동작합니다. 수신자와 메소드의 이름이 주어질 때, 러스트는 해당 메소드가 읽는지 (&self) 혹은 변형시키는지 (&mut self), 아니면 소비하는지 (self)를 결정론적으로 알아낼 수 있습니다. 러스트가 메소드 수신자를 암묵적으로 빌리도록 하는 사실은 실사용 환경에서 소유권을 인간공학적으로 만드는 중요한 부분입니다.

## 더 많은 파라미터를 가진 메소드

또다른 Rectangle 인스턴스를 파라미터로 갖는 can_hold 메소드를 Rectangle 상에 구현하기

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
```

## 연관 함수

impl 블록의 또다른 유용한 기능은 self 파라미터를 갖지 않는 함수도 impl 내에 정의하는 것이 허용된다는 점입니다. 이를 연관 함수 (associated functions) 라고 부르는데, 그 이유는 이 함수가 해당 구조체와 연관되어 있기 때문입니다. 이들은 메소드가 아니라 여전히 함수인데, 이는 함께 동작할 구조체의 인스턴스를 가지고 있지 않아서 그렇습니다. 여러분은 이미 String::from 연관 함수를 사용해본 적이 있습니다.

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}
```
