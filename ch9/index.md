# ch9: Error Handling

## panic!과 함께하는 복구 불가능한 에러

가끔씩 여러분의 코드에서 나쁜 일이 일어나고, 이에 대해 여러분이 할 수 있는 것이 없을 수도 있습니다. 이러한 경우를 위하여 러스트는 panic! 매크로를 가지고 있습니다. 이 매크로가 실행되면, 여러분의 프로그램은 실패 메세지를 출력하고, 스택을 되감고 청소하고, 그 후 종료됩니다. 이런 일이 발생하는 가장 흔한 상황은 어떤 종류의 버그가 발견되었고 프로그래머가 이 에러를 어떻게 처리할지가 명확하지 않을 때 입니다.

```
기본적으로, panic!이 발생하면, 프로그램은 되감기(unwinding) 를 시작하는데, 이는 러스트가 패닉을 마주친 각 함수로부터 스택을 거꾸로 훑어가면서 데이터를 제거한다는 뜻이지만, 이 훑어가기 및 제거는 일이 많습니다. 다른 대안으로는 즉시 그만두기(abort) 가 있는데, 이는 데이터 제거 없이 프로그램을 끝내는 것입니다. 프로그램이 사용하고 있던 메모리는 운영체제에 의해 청소될 필요가 있을 것입니다. 여러분의 프로젝트 내에서 결과 바이너리가 가능한 작아지기를 원한다면, 여러분의 Cargo.toml 내에서 적합한 [profile] 섹션에 panic = 'abort'를 추가함으로써 되감기를 그만두기로 바꿀 수 있습니다. 예를 들면, 여러분이 릴리즈 모드 내에서는 패닉 상에서 그만두기를 쓰고 싶다면, 다음을 추가하세요:
```

```rust
fn main() {
    panic!("crash and burn");
}
```

직접 panic을 발생시키는 코드

## panic! 백트레이스 사용하기

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

여기서 우리는 벡터의 100번째 요소(0부터 시작하여 100번째)에 접근하기를 시도하고 있지만, 벡터는 오직 3개의 요소만 가지고 있습니다. 이러한 상황이면 러스트는 패닉을 일으킬 것입니다. []를 사용하는 것은 어떤 요소를 반환하기를 가정하지만, 유효하지 않은 인덱스를 넘기게 되면 러스트가 반환할 올바른 요소는 없습니다.

이러한 상황에서 C와 같은 다른 언어들은 여러분이 원하는 것이 아닐지라도, 여러분이 요청한 것을 정확히 주려고 시도할 것입니다: 여러분은 벡터 내에 해당 요소와 상응하는 위치의 메모리에 들어 있는 무언가를 얻을 것입니다. 설령 그 메모리 영역이 벡터 소유가 아닐지라도 말이죠. 이러한 것을 버퍼 오버리드(buffer overread) 라고 부르며, 만일 어떤 공격자가 읽도록 허용되어선 안 되지만 배열 뒤에 저장된 데이터를 읽어낼 방법으로서 인덱스를 다룰 수 있게 된다면, 이는 보안 취약점을 발생시킬 수 있습니다.

RUST_BACKTRACE=1 cargo run를 하면 backtrace를 할 수 있다!

## Result와 함께하는 복구 가능한 에러

2장의 “Result 타입으로 잠재된 실패 다루기” 절에서 Result 열거형은 다음과 같이 Ok와 Err라는 두 개의 variant를 갖도록 정의되어 있음을 상기하세요:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {}", error);
        }
    };
}
```

에러를 발생시키고 패턴매칭으로 처리

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```

error의 종류가 NotFound일 경우 File::create를 실행, 이 또한 에러 처리를 한다.

### Shortcuts for Panic on Error: unwrap and expect

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

error인 경우 panic!매크로 발생

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

unwrap과 비슷하나 에러 메세지 전달해줌

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

에러를 상위에 전파하기 위해 ?를 사용함. OK로 resolve

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

축약버전

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

fs::read_to_string 자체가 Result이므로 propagation됨

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```

?연산자는

- Result값을 반환하는 함수에 쓸 수 있다. error 전파
- Option을 반환하는 함수에도 쓰일 수 있다. none 전파
- main함수 안에서는 쓰지 못한다. Main함수의 리턴값이 에러면 안되기 때문

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

main에서 Result 반환하는 꼼수

## To panic! or Not to panic!

어떤 경우에 panic을 발생시킬 것인가 말 것인가.

에러 처리 결정권을 호출자에게 주자. Result를 리턴하자

### Examples, Prototype Code, and Tests

간단한 프로토타입 코드를 짜보자. 너무 꼼꼼히 에러처리를 하기 힘듬 -> unwrap, expect로 짜다가
실제 개발을 할 때 적절히 에러 처리를 하자

테스트 코드 작성도 마찬가지

### Cases in Which You Have More Information Than the Compiler

실패할 가능성이 거의 없는 코드인 경우 컴파일러는 그 정보를 모른다.
그냥 간단히 unwrap()으로 처리

### Guidelines for Error Handling

- 코드가 잘못된 상태가 될 경우 panic을 발생시키는 것이 좋다.
  - 잘못된 상태란 예상치 못한 결과, 전제 조건이 깨지는 결과가 발생한 경우
- 어떨 땐 잘되고 어떨 땐 잘 안되는 경우는 Result를 반환하여 처리하는 것이 좋다.
- 어떤 정보를 인코딩하기 힘든 타입인 경우 panic을 발생시키는 것이 좋다.
- 값에 대한 input validation이 깨진 경우 panic을 발생시키는 것이 좋다
  - 이에 대해, 코드를 사용하는 사람에게 보여줄 문서를 잘 작성하는 것이 좋겠다.

### Creating Custom Types for Validation

```rust
loop {
        // --snip--

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            // --snip--
    }
```

위와 같은 사용자의 입력이 1부터 100까지 여야 할 때

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

구조체를 생성하는 함수에 validation을 해서 조건에 부합하지 않을 경우 panic을 발생시킨다.

