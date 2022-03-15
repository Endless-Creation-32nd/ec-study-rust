# ch6: Enums and Pattern Matching

## 열거형 정의하기

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

지금으로써는 실제 IP 주소 데이터를 저장할 방법이 없습니다. 단지 어떤 종류 인지만 알 뿐입니다.

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

각 variant 는 다른 타입과 다른 양의 연관된 데이터를 가질 수 있습니다. 구조체로는 이렇게 할 수 없습니다. 열거형은 이런 경우를 쉽게 처리합니다:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

```rust
struct Ipv4Addr {
    // details elided
}

struct Ipv6Addr {
    // details elided
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

열거형과 구조체의 차이점

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // 유닛 구조체
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 튜플 구조체
struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체
```

각기 다른 타입을 갖는 여러 개의 구조체를 사용한다면, 이 메시지 중 어떤 한 가지를 인자로 받는 함수를 정의하기 힘들 것입니다. Message 열거형은 하나의 타입으로 이것이 가능합니다.

```rust
impl Message {
    fn call(&self) {
        // 메소드 내용은 여기 정의할 수 있습니다.
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

구조체에 impl 을 사용해서 메소드를 정의한 것처럼, 열거형에도 정의할 수 있습니다. 여기 Message 열거형에 에 정의한 call 이라는 메소드가 있습니다:

## Option 열거형과 Null 값 보다 좋은 점들.

이번 절에서는 표준 라이브러리에서 열거형으로 정의된 또 다른 타입인 Option 에 대한 사용 예를 살펴볼 것입니다. Option 타입은 많이 사용되는데, 값이 있거나 없을 수도 있는 아주 흔한 상황을 나타내기 때문입니다. 이 개념을 타입 시스템의 관점으로 표현하자면, 컴파일러가 발생할 수 있는 모든 경우를 처리했는지 체크할 수 있습니다. 이렇게 함으로써 버그를 방지할 수 있고, 이것은 다른 프로그래밍 언어에서 매우 흔합니다.

프로그래밍 언어 디자인은 가끔 어떤 특성들이 포함되었는지의 관점에서 생각되기도 하지만, 포함되지 않은 특성들도 역시 중요합니다. 러스트는 다른 언어들에서 흔하게 볼 수 있는 null 특성이 없습니다. Null 은 값이 없다는 것을 표현하는 하나의 값입니다. null 을 허용하는 언어에서는, 변수는 항상 두 상태중 하나가 될 수 있습니다: null 혹은 null 이 아님.

null 값으로 발생하는 문제는, null 값을 null 이 아닌 값처럼 사용하려고 할 때 여러 종류의 오류가 발생할 수 있다는 것입니다. null이나 null이 아닌 속성은 어디에나 있을 수 있고, 너무나도 쉽게 이런 종류의 오류를 만들어 냅니다.

문제는 실제 개념에 있기보다, 특정 구현에 있습니다. 이와 같이 러스트에는 null 이 없지만, 값의 존재 혹은 부재의 개념을 표현할 수 있는 열거형이 있습니다. 이 열거형은 Option<T> 이며, 다음과 같이 표준 라이브러리에 정의되어 있습니다:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Option<T> 열거형은 매우 유용하며 기본적으로 포함되어 있기 때문에, 명시적으로 가져오지 않아도 사용할 수 있습니다. 또한 variants 도 마찬가지입니다: Option:: 를 앞에 붙이지 않고, Some 과 None 을 바로 사용할 수 있습니다. Option<T> 는 여전히 일반적인 열거형이고, Some(T) 과 None 도 여전히 Option<T> 의 variants 입니다.

<T> 는 러스트의 문법이며 아직 다루지 않았습니다. 제너릭 타입 파라미터이며, 제너릭에 대해서는 10 장에서 더 자세히 다룰 것입니다. 지금은 단지 <T> 가 Option 열거형의 Some variant 가 어떤 타입의 데이터라도 가질 수 있다는 것을 의미한다는 것을 알고 있으면 됩니다. 여기 숫자 타입과 문자열 타입을 갖는 Option 값에 대한 예들이 있습니다:

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

Some 이 아닌 None 을 사용한다면, Option<T> 이 어떤 타입을 가질지 러스트에게 알려줄 필요가 있습니다. 컴파일러는 None 만 보고는 Some variant 가 어떤 타입인지 추론할 수 없습니다.

Some 값을 얻게 되면, 값이 있다는 것과 Some 이 갖고 있는 값에 대해 알 수 있습니다. None 값을 사용하면, 어떤 면에서는 null 과 같은 의미를 갖게 됩니다: 유효한 값을 갖지 않습니다. 그렇다면 왜 Option<T> 가 null 을 갖는 것보다 나을까요?

간단하게 말하면, Option<T> 와 T (T 는 어떤 타입이던 될 수 있음)는 다른 타입이며, 컴파일러는 Option<T> 값을 명확하게 유효한 값처럼 사용하지 못하도록 합니다. 예를 들면, 아래 코드는 Option<i8> 에 i8 을 더하려고 하기 때문에 컴파일되지 않습니다:

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
//error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is
//not satisfied
```

주목하세요! 실제로, 이 에러 메시지는 러스트가 Option<i8> 와 i8 를 어떻게 더해야 하는지 모른다는 것을 의미하는데, 둘은 다른 타입이기 때문입니다. 러스트에서 i8 과 같은 타입의 값을 가질 때, 컴파일러는 항상 유효한 값을 갖고 있다는 것을 보장할 것입니다. 값을 사용하기 전에 null 인지 확인할 필요도 없이 자신 있게 사용할 수 있습니다. 단지 Option<i8> 을 사용할 경우엔 (혹은 어떤 타입 이건 간에) 값이 있을지 없을지에 대해 걱정할 필요가 있으며, 컴파일러는 값을 사용하기 전에 이런 케이스가 처리되었는지 확인해 줄 것입니다.

다르게 얘기하자면, T 에 대한 연산을 수행하기 전에 Option<T> 를 T 로 변환해야 합니다. 일반적으로, 이런 방식은 null 과 관련된 가장 흔한 이슈 중 하나를 발견하는데 도움을 줍니다: 실제로 null 일 때, null 이 아니라고 가정하는 경우입니다.

null 이 아닌 값을 갖는다는 가정을 놓치는 경우에 대해 걱정할 필요가 없게 되면, 코드에 더 확신을 갖게 됩니다. null 일 수 있는 값을 사용하기 위해서, 명시적으로 값의 타입을 Option<T> 로 만들어 줘야 합니다. 그다음엔 값을 사용할 때 명시적으로 null 인 경우를 처리해야 합니다. 값의 타입이 Option<T> 가 아닌 모든 곳은 값이 null 아 아니라고 안전하게 가정할 수 있습니다. 이것은 null을 너무 많이 사용하는 문제를 제한하고 러스트 코드의 안정성을 높이기 위한 러스트의 의도된 디자인 결정사항입니다.

그럼 Option<T> 타입인 값을 사용할 때, Some variant 에서 T 값을 어떻게 가져와서 사용할 수 있을까요? Option<T> 열거형에서 다양한 상황에서 유용하게 사용할 수 있는 많은 메소드들이 있습니다; 문서에서 확인할 수 있습니다. Option<T> 의 메소드들에 익숙해지는 것은 러스트를 사용하는데 매우 유용할 것입니다.

일반적으로, Option<T> 값을 사용하기 위해서는 각 variant 를 처리할 코드가 필요할 것입니다. Some(T) 값일 경우만 실행되는 코드가 필요하고, 이 코드는 안에 있는 T 를 사용할 수 있습니다. 다른 코드에서는 None 값일 때 실행되는 코드가 필요가 하기도 하며, 이 코드에서는 사용할 수 있는 T 값이 없습니다. match 표현식은 제어 흐름을 위한 구분으로, 열거형과 함께 사용하면 이런 일들을 할 수 있습니다: 열거형이 갖는 variant 에 따라 다른 코드를 실행할 것이고, 그 코드는 매칭 된 값에 있는 데이터를 사용할 수 있습니다.

## match 흐름 제어 연산자

```rust
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

### Option<T>를 이용하는 매칭

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### \_ 변경자(placeholder)

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

## if let을 사용한 간결한 흐름 제어

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```
