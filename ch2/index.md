# Programming a Guessing Game

## Review Code

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

**line 1**

```rust
use std::io;
```

러스트는 모든 프로그램의 스코프에 prelude 내의 타입들을 가져옵니다. 만약 여러분이 원하는 타입이 prelude에 없다면 use문을 활용하여 명시적으로 그 타입을 가져와야 합니다. std::io는 사용자의 입력을 받는 것을 포함하여 io와 관련된 기능들을 제공합니다.

**line 3**

```rust
fn main() {
```

1장에서 보았듯이 main 함수는 프로그램의 진입점입니다.

**line 8**

```rust
let mut guess = String::new();
```

러스트에서 변수는 기본적으로 불변입니다. 다음 예시는 변수 앞에 mut을 이용하여 가변변수를 만드는 법을 보여줍니다.

::new에 있는 ::는 new가 String 타입의 연관함수 임을 나타냅니다. 연관함수는 하나의 타입을 위한 함수이며, 이 경우에는 하나의 String 인스턴스가 아니라 String 타입을 위한 함수입니다. 몇몇 언어에서는 이것을 정적 메소드 라고 부릅니다.

new 함수는 새로운 빈 String을 생성합니다. new 함수는 새로운 값을 생성하기 위한 일반적인 이름이므로 많은 타입에서 찾아볼 수 있습니다.

**line 10:11**

```rust
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

만약 프로그램 시작점에 use std::io가 없다면 함수 호출 시 std::io::stdin처럼 작성해야 합니다. stdin 함수는 터미널의 표준 입력의 핸들(handle)의 타입인 std::io::Stdin의 인스턴스를 돌려줍니다.

코드의 다음 부분인 .read_line(&mut guess)는 사용자로부터 입력을 받기 위해 표준 입력 핸들에서 .read_line(&mut guess) 메소드를 호출합니다. 또한 read_line에 &mut guess 를 인자로 하나 넘깁니다.

read_line은 사용자가 표준 입력에 입력할 때마다 입력된 문자들을 하나의 문자열에 저장하므로 인자로 값을 저장할 문자열이 필요합니다. 그 문자열 인자는 사용자 입력을 추가하면서 변경되므로 가변이어야 합니다.

&는 코드의 여러 부분에서 데이터를 여러 번 메모리로 복사하지 않고 접근하기 위한 방법을 제공하는 참조자 임을 나타냅니다. 참조자는 복잡한 특성으로서 러스트의 큰 이점 중 하나가 참조자를 사용함으로써 얻는 안전성과 용이성입니다. 이 프로그램을 작성하기 위해 참조자의 자세한 내용을 알 필요는 없습니다. 4장에서 참조자에 대해 전체적으로 설명할 것입니다. 지금 당장은 참조자가 변수처럼 기본적으로 불변임을 알기만 하면 됩니다. 따라서 가변으로 바꾸기 위해 &guess가 아니라 &mut guess로 작성해야 합니다.

`read_line`은 우리가 인자로 넘긴 문자열에 사용자가 입력을 저장할 뿐 아니라 하나의 값을 돌려 줍니다. 여기서 돌려준 값은 `io::Result` 입니다. 러스트는 표준 라이브러리에 여러 종류의 Result 타입을 가지고 있습니다. 제네릭 Result이나 io:Result가 그 예시입니다.

Result 타입은 열거형(enumerations)로써 enums 라고 부르기도 합니다. 열거형은 정해진 값들만을 가질 수 있으며 이러한 값들은 열거형의 variants 라고 부릅니다. 6장에서 열거형에 대해 더 자세히 다룹니다.

Result의 variants는 Ok와 Err입니다. Ok는 처리가 성공했음을 나타내며 내부적으로 성공적으로 생성된 결과를 가지고 있습니다. Err는 처리가 실패했음을 나타내고 그 이유에 대한 정보를 가지고 있습니다.

이러한 Result는 에러 처리를 위한 정보를 표현하기 위해 사용됩니다. Result 타입의 값들은 다른 타입들처럼 메소드들을 가지고 있습니다. io::Result 인스턴스는 expect 메소드를 가지고 있습니다. 만약 io::Result 인스턴스가 Err일 경우 expect 메소드는 프로그램이 작동을 멈추게 하고 expect에 인자로 넘겼던 메세지를 출력하도록 합니다. 만약 read_line 메소드가 Err를 돌려줬다면 그 에러는 운영체제로부터 생긴 에러일 경우가 많습니다. 만약 io::Result가 Ok 값이라면 expect는 Ok가 가지고 있는 결과값을 돌려주어 사용할 수 있도록 합니다. 이 경우 결과값은 사용자가 표준 입력으로 입력했던 바이트의 개수입니다.

만약 expect를 호출하지 않는다면 컴파일은 되지만 경고가 나타납니다.

```
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `std::result::Result` which must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default
```

러스트는 read_line가 돌려주는 Result 값을 사용하지 않았음을 경고하며 일어날 수 있는 에러를 처리하지 않았음을 알려줍니다. 이 경고를 없애는 옳은 방법은 에러를 처리하는 코드를 작성하는 것이지만 만약 문제가 발생했을 때 프로그램이 멈추길 바란다면 expect를 사용할 수 있습니다. 9장에서 에러가 발생했을 때 이를 처리하는 방법에 대해 배웁니다.

### println! 변경자(placeholder)를 이용한 값 출력

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

### 크레이트(Crate)를 사용하여 더 많은 기능 가져오기

러스트는 아직 표준 라이브러리에 임의의 값을 생성하는 기능이 없습니다. 하지만 러스트 팀에서는 rand 크레이트를 제공합니다.

Crate는 러스트 코드의 패키지입니다. 우리가 만들고 있는 프로젝트는 실행이 가능한 binary crate 입니다. rand crate는 다른 프로그램에서 사용되기 위한 용도인 library crate 입니다.

rand를 사용하는 코드를 작성하기 전에 Cargo.toml 을 수정하여 rand 크레이트를 의존 리스트에 추가해야 합니다.
[dependencies] 절의 시작 바로 아래에 다음 내용을 추가하세요.

```
[dependencies]

rand = "0.8.3"
```

## Code 2

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

use 라인인 use rand::Rng를 추가합니다. Rng는 정수 생성기가 구현한 메소드들을 정의한 trait이며 해당 메소드들을 이용하기 위해서는 반드시 스코프 내에 있어야 합니다. 10장에서 trait에 대해 더 자세히 다룰 것입니다.

또한 우리는 중간에 두 개의 라인을 추가합니다. rand::thread_rng 함수는 OS가 시드(seed)를 정하고 현재 스레드에서만 사용되는 특별한 정수생성기를 돌려 줍니다. 다음으로 우리는 get_range 메소드를 호출합니다. 이 메소드는 Rng trait에 정의되어 있으므로 use rand::Rng 문을 통해 스코프로 가져올 수 있습니다. gen_range 메소드는 두 개의 숫자를 인자로 받고 두 숫자 사이에 있는 임의의 숫자를 생성합니다. 하한선은 포함되지만 상한선은 제외되므로 1부터 100 사이의 숫자를 생성하려면 1과 101을 넘겨야 합니다.

크레이트에서 어떤 trait를 사용하고 어떤 함수나 메소드들을 호출하는 것을 아는 것은 단순히 아는 것 이 아닙니다. 각각의 크레이트의 문서에서 사용 방법을 제공합니다. Cargo의 또다른 멋진 특성은 cargo doc --open 명령어로써 로컬에서 여러분의 모든 의존 패키지들이 제공하는 문서들을 빌드해서 브라우저에 표시해 줍니다. 만약 rand 크레이트의 다른 기능들에 흥미가 있다면 cargo doc --open을 실행하고 왼쪽의 사이드바에 rand를 클릭하세요.

## Code 3

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

guess와 secret_number를 비교하기 위해 작성합니다.
이전 값을 새 값으로 섀도잉합니다.

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

## Code 4

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```
