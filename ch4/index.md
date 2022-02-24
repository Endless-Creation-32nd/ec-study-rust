# Understanding Ownership

소유권(Ownership)은 러스트의 가장 유니크한 특성이며, 러스트가 가비지 콜렉터 없이 메모리 안정성 보장을 하게 해줍니다. 그러므로, 소유권이 러스트 내에서 어떻게 동작하는지 이해하는 것은 중요합니다.

## 소유권이 무엇일까

메모리를 관리하자

GC를 쓰는 언어도 있다. 직접 해제해야 하는 언어도 있다. 러스트에선 소유권으로 관리한다.

### 소유권 규칙

1. 러스트의 각각의 값은 해당값의 오너(owner)라고 불리우는 변수를 갖고 있다.
2. 한번에 딱 하나의 오너만 존재할 수 있다.
3. 오너가 스코프 밖으로 벗어나는 때, 값은 버려진다(dropped).

### 변수의 스코프

변수는 스코프 안에서 유효하다.

### String 타입

스트링 리터럴이 이미 있지만 이것은 immutable이다.

```rust
fn main() {
let mut s = String::from("hello");

s.push_str(", world!"); // push_str()은 해당 스트링 리터럴을 스트링에 붙여줍니다.

println!("{}", s); // 이 부분이 `hello, world!`를 출력할 겁니다.
}
```

String은 mutable할 수 있는데 리터럴 문자열은 왜 변할 수 없을까요?

바로 메모리를 쓰는 방식에 있습니다.

### 메모리와 할당

스트링 리터럴은 내용을 컴파일 타임에 알 수 있으므로 텍스트가 최종 실행파일에 직접 하드코딩 되어있다.
String 타입은 변경 가능하고 커질 수 있는 텍스트를 지원하기 위해 만들어졌다. 따라서 힙에 메모리를 할당받아 저장할 필요가 있고 런타임에 운영체제로부터 메모리가 요청되어야 한다. 그리고 String의 사용이 끝났을 때 운영체제에게 메모리를 반남할 방법이 필요하다.

러스트는 변수가 스코프 밖으로 벗어나면 자동으로 drop을 호출하여 메모리를 반환한다. 항상

### 변수와 데이터가 상호작용하는 방법: 이동(move)

```rust
fn main() {
let s1 = String::from("hello");
let s2 = s1;
}
```

흔히 얕은복사, 깊은복사와 비슷한 개념

![](https://doc.rust-lang.org/book/img/trpl04-02.svg)

s1, s2모두 스코프 밖으로 벗어나게 되면 둘다 같은 메모리를 해제하려 하기 때문에 double free라고 알려진 오류가 발생할 수 있다. 이는 memory curruption의 원인이 된다.

러스트는 s1이 더이상 유효하지 않다고 간주하면서 이 문제를 해결한다. 하지만

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1); //에러 발생
```

이러한 러스트의 동작 방식 때문에 러스트는 `얕은 복사`라는 말 대신에 `이동`이라고 부른다.

### 변수와 데이터가 상호작용하는 방법: 클론

깊은 복사

```rust
fn main() {
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
}
```

### 스택에만 있는 데이터: 복사

스택에 있는 데이터는 그냥 복사됨

```rust
fn main() {
  let x = 5;
  let y = x;

  println!("x = {}, y = {}", x, y);
}
```

러스트는 정수형과 같이 스택에 저장할 수 있는 타입에 대해 달수 있는 Copy 트레잇이라고 불리우는 특별한 어노테이션(annotation)을 가지고 있습니다 (트레잇에 관해서는 10장에서 더 자세히 보겠습니다). 만일 어떤 타입이 Copy 트레잇을 갖고 있다면, 대입 과정 후에도 예전 변수를 계속 사용할 수 있습니다. 러스트는 만일 그 타입 혹은 그 타입이 가지고 있는 부분 중에서 Drop 트레잇을 구현한 것이 있다면 Copy 트레잇을 어노테이션 할 수 없게끔 합니다. 만일 어떤 타입이 스코프 밖으로 벗어났을 때 어떤 특수한 동작을 필요로 하고 우리가 그 타입에 대해 Copy 어노테이션을 추가한다면, 컴파일 타임 오류를 보게 됩니다. Copy 어노테이션을 여러분의 타입에 어떤 식으로 추가하는지 알고 싶다면, 부록 C의 파생 가능한 트레잇(Derivable Traits)을 보세요.

그래서 어떤 타입이 Copy가 될까요? 여러분은 주어진 타입에 대해 확신을 하기 위해 문서를 확인할 수도 있겠지만, 일반적인 규칙으로서 단순한 스칼라 값들의 묶음은 Copy가 가능하고, 할당이 필요하거나 어떤 자원의 형태인 경우 Copy를 사용할 수 없습니다. Copy가 가능한 몇가지 타입을 나열해 보겠습니다:

- u32와 같은 모든 정수형 타입들
- true와 false값을 갖는 부울린 타입 bool
- f64와 같은 모든 부동 소수점 타입들
- Copy가 가능한 타입만으로 구성된 튜플들. (i32, i32)는 Copy가 되지만, (i32, String)은 안됩니다.

### 소유권과 함수

함수에게 값을 넘기는 의미론(semantics)은 값을 변수에 대입하는 것과 유사합니다. 함수에게 변수를 넘기는 것은 대입과 마찬가지로 이동하거나 복사될 것입니다. Listing 4-7은 변수가 스코프 안으로 들어갔다 밖으로 벗어나는 것을 주석과 함께 보여주는 예입니다:

```rust
fn main() {
    let s = String::from("hello");  // s가 스코프 안으로 들어왔습니다.

    takes_ownership(s);             // s의 값이 함수 안으로 이동했습니다...
                                    // ... 그리고 이제 더이상 유효하지 않습니다.
    let x = 5;                      // x가 스코프 안으로 들어왔습니다.

    makes_copy(x);                  // x가 함수 안으로 이동했습니다만,
                                    // i32는 Copy가 되므로, x를 이후에 계속
                                    // 사용해도 됩니다.

} // 여기서 x는 스코프 밖으로 나가고, s도 그 후 나갑니다. 하지만 s는 이미 이동되었으므로,
  // 별다른 일이 발생하지 않습니다.

fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어왔습니다.
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는
  // 해제되었습니다.

fn makes_copy(some_integer: i32) { // some_integer이 스코프 안으로 들어왔습니다.
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않습니다.

```

### 반환 값과 스코프

값의 반환 또한 소유권을 이동시킵니다. Listing 4-7과 비슷한 주석이 달린 예제를 하나 봅시다:

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership은 반환값을 s1에게
                                        // 이동시킵니다.

    let s2 = String::from("hello");     // s2가 스코프 안에 들어왔습니다.

    let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back 안으로
                                        // 이동되었고, 이 함수가 반환값을 s3으로도
                                        // 이동시켰습니다.

} // 여기서 s3는 스코프 밖으로 벗어났으며 drop이 호출됩니다. s2는 스코프 밖으로
  // 벗어났지만 이동되었으므로 아무 일도 일어나지 않습니다. s1은 스코프 밖으로
  // 벗어나서 drop이 호출됩니다.

fn gives_ownership() -> String {             // gives_ownership 함수가 반환 값을
                                             // 호출한 쪽으로 이동시킵니다.

    let some_string = String::from("hello"); // some_string이 스코프 안에 들어왔습니다.

    some_string                              // some_string이 반환되고, 호출한 쪽의
                                             // 함수로 이동됩니다.
}

// takes_and_gives_back 함수는 String을 하나 받아서 다른 하나를 반환합니다.
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프
                                                      // 안으로 들어왔습니다.

    a_string  // a_string은 반환되고, 호출한 쪽의 함수로 이동됩니다.
}
```

## 참조자(References)와 빌림(Borrowing)

앞 절의 마지막에 등장한 튜플을 이용하는 이슈는 String을 호출하는 함수 쪽으로 반환함으로써 calculate_length를 호출한 이후에도 여전히 String을 이용할 수 있도록 하는 것인데, 그 이유는 String이 calculate_length 안쪽으로 이동되었기 때문입니다.

여기 값의 소유권을 넘기는 대신 개체에 대한 참조자(reference)를 인자로 사용하는 calculate_length 함수를 정의하고 이용하는 방법이 있습니다:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

이 엠퍼센드(&) 기호가 참조자이며, 이는 여러분이 어떤 값을 소유권을 넘기지 않고 참조할수 있도록 해줍니다.

![](https://doc.rust-lang.org/book/img/trpl04-05.svg)

### 가변 참조자(Mutable References)

참조자로 값을 바꾸려먼 변수를 mut로 바꿔주어야 한다.

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

하지만 가변 참조자는 딱 한가지 큰 제한이 있습니다: 특정한 스코프 내에 특정한 데이터 조각에 대한 가변 참조자를 딱 하나만 만들 수 있다는 겁니다. 아래 코드는 실패할 겁니다:

이러한 제한이 가지는 이점은 바로 러스트가 컴파일 타임에 데이터 레이스(data race)를 방지할 수 있도록 해준다는 것입니다.

데이터 레이스는 아래에 정리된 세 가지 동작이 발생했을때 나타나는 특정한 레이스 조건입니다:

1. 두 개 이상의 포인터가 동시에 같은 데이터에 접근한다.
2. 그 중 적어도 하나의 포인터가 데이터를 쓴다.
3. 데이터에 접근하는데 동기화를 하는 어떠한 메커니즘도 없다.

항상 우리는 새로운 스코프를 만들기 위해 중괄호를 사용하는데, 이는 그저 동시에 만드는 것이 아니게 해줌으로써, 여러 개의 가변 참조자를 만들 수 있도록 해줍니다.

```rust
fn main() {
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // 여기서 r1은 스코프 밖으로 벗어났으므로, 우리는 아무 문제 없이 새로운 참조자를 만들 수 있습니다.

let r2 = &mut s;
}
```

### 댕글링 참조자(Dangling References)

포인터가 있는 언어에서는 자칫 잘못하면 댕글링 포인터(dangling pointer) 를 만들기 쉬운데, 댕글링 포인터란 어떤 메모리를 가리키는 포인터를 보존하는 동안, 그 메모리를 해제함으로써 다른 개체에게 사용하도록 줘버렸을 지도 모를 메모리를 참조하고 있는 포인터를 말합니다. 이와는 반대로, 러스트에서는 컴파일러가 모든 참조자들이 댕글링 참조자가 되지 않도록 보장해 줍니다: 만일 우리가 어떤 데이터의 참조자를 만들었다면, 컴파일러는 그 참조자가 스코프 밖으로 벗어나기 전에는 데이터가 스코프 밖으로 벗어나지 않을 것임을 확인해 줄 것입니다.

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    //= help: this function's return type contains a borrowed value, but there is no
    //value for it to be borrowed from
    //= help: consider giving it a 'static lifetime
    let s = String::from("hello");

    &s
}
```

이 오류 메세지는 우리가 아직 다루지 못한 특성을 인용하고 있습니다: 바로 라이프타임(lifetime) 입니다. 라이프타임에 대한 것은 10장에서 자세히 다룰 것입니다. 하지만 여러분이 라이프타임에 대한 부분을 무시한다면, 이 메세지는 이 코드가 왜 문제인지를 알려줄 열쇠를 쥐고 있습니다:

```rust
fn dangle() -> &String { // dangle은 String의 참조자를 반환합니다

    let s = String::from("hello"); // s는 새로운 String입니다

    &s // 우리는 String s의 참조자를 반환합니다.
} // 여기서 s는 스코프를 벗어나고 버려집니다. 이것의 메모리는 사라집니다.
  // 위험하군요!
```

s가 dangle안에서 만들어졌기 때문에, dangle의 코드가 끝이나면 s는 할당 해제됩니다. 하지만 우리는 이것의 참조자를 반환하려고 했습니다. 이는 곧 이 참조자가 어떤 무효화된 String을 가리키게 될 것이란 뜻이 아닙니까! 별로 안 좋죠. 러스트는 우리가 이런 짓을 못하게 합니다.

여기서의 해법은 String을 직접 반환하는 것입니다:

## 슬라이스(Slices)

소유권을 갖지 않는 또다른 데이터 타입은 슬라이스입니다. 슬라이스는 여러분이 컬렉션(collection) 전체가 아닌 컬렉션의 연속된 일련의 요소들을 참조할 수 있게 합니다.

여기 작은 프로그래밍 문제가 있습니다: 스트링을 입력 받아 그 스트링에서 찾은 첫번째 단어를 반환하는 함수를 작성하세요. 만일 함수가 공백문자를 찾지 못한다면, 이는 전체 스트링이 한 단어라는 의미이고, 이때는 전체 스트링이 반환되어야 합니다.

```rust

#![allow(unused)]
fn main() {
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
}
```

이 프로그램은 아무런 오류 없이 컴파일되고, s.clear()을 호출한 뒤 word를 사용한다 해도 역시 컴파일될 것입니다. word는 s의 상태와 전혀 연결되어 있지 않으므로, word는 여전히 값 5를 담고 있습니다. 우리는 첫번째 단어를 추출하고자 하기 위해 s와 값 5를 사용할 수 있지만, word에 5를 저장한 뒤 s의 내용물이 변경되었기 때문에 이러한 사용은 버그가 될 것입니다.

### 스트링 슬라이스

```rust
fn main() {
  let s = String::from("hello world");

  let hello = &s[0..5];
  let world = &s[6..11];
}
```

이는 전체 String의 참조자를 갖는 것과 비슷하지만, 추가적으로 [0..5]라는 코드가 붙어 있습니다. 전체 String에 대한 참조자 보다는, String의 일부분에 대한 참조자입니다. start..end 문법은 start부터 시작하여 end를 포함하지 않는 연속된 범위를 기술합니다.

우리는 대괄호 내에 [starting_index..ending_index]를 특정한 범위를 이용하여 슬라이스를 만들 수 있는데, 여기서 starting_index는 슬라이스에 포함되는 첫번째 위치이고 ending_index는 슬라이스에 포함될 마지막 위치보다 1을 더한 값입니다. 내부적으로 슬라이스 데이터 구조는 시작 위치와 슬라이스의 길이를 저장하는데, 이 길이 값은 ending_index에서 starting_index를 뺀 값입니다. 따라서 let world = &[6..11];의 경우, world는 s의 6번째 바이트를 가리키고 있는 포인터와 길이값 5를 갖고 있는 슬라이스가 될 것입니다.

![](https://doc.rust-lang.org/book/img/trpl04-06.svg)

러스트의 .. 범위 문법을 사용하여, 여러분이 만일 첫번째 인덱스(즉 0)에서부터 시작하길 원한다면, 두 개의 마침표 전의 값은 생략할 수 있습니다. 다시 말하면, 아래의 두 줄은 동일한 표현입니다:

```rust
fn main() {
  let s = String::from("hello");

  let slice = &s[0..2];
  let slice = &s[..2];
}
```

비슷한 이치로, 만일 여러분의 슬라이스가 String의 마지막 바이트까지 포함한다면, 여러분은 끝의 숫자를 생략할 수 있습니다. 이는 아래 두 줄의 표현이 동일하다는 의미입니다:

```rust
fn main() {
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
}
```

이 모든 정보를 잘 기억하시고, first_word가 슬라이스를 반환하도록 다시 작성해봅시다. “스트링 슬라이스”를 나타내는 타입은 &str로 씁니다:

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // Error!

    println!("the first word is: {}", word);
}
```

빌림 규칙에서 우리가 만일 무언가에 대한 불변 참조자를 만들었을 경우, 가변 참조자를 만들 수 없다는 점을 상기해보세요. clear 함수가 String을 잘라낼 필요가 있기 때문에, 이 함수는 가변 참조자를 갖기 위한 시도를 할 것이고, 이는 실패하게 됩니다. 러스트는 우리의 API를 사용하기 쉽게 해줄 뿐만 아니라 이러한 종류의 오류 전체를 컴파일 타임에 제거해 줍니다!

**스트링 리터럴은 슬라이스입니다**

스트링 리터럴이 바이너리 안에 저장된다고 하는 얘기를 상기해봅시다. 이제 슬라이스에 대해 알았으니, 우리는 스트링 리터럴을 적합하게 이해할 수 있습니다:

```rust
fn main() {
  let s = "Hello, world!";
}
```

여기서 s의 타입은 &str입니다: 이것은 바이너리의 특정 지점을 가리키고 있는 슬라이스입니다. 이는 왜 스트링 리터럴이 불변인가도 설명해줍니다; &str은 불변 참조자이기 때문입니다.

**파라미터로서의 스트링 슬라이스**

```rust
fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
```

더 경험이 많은 러스트인이라면 대신 아래와 같이 작성하는데, 그 이유는 &String과 &str 둘 모두에 대한 같은 함수를 사용할 수 있도록 해주기 때문입니다.

만일 우리가 스트링 슬라이스를 갖고 있다면, 이를 바로 넘길 수 있습니다. String을 갖고 있다면, 이 String의 전체 슬라이스를 넘길 수 있습니다. 함수가 String의 참조자 대신 스트링 슬라이스를 갖도록 정의하는 것은 우리의 API를 어떠한 기능적인 손실 없이도 더 일반적이고 유용하게 해줍니다:

### 그 밖의 슬라이스들

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

이 슬라이스는 &[i32] 타입을 갖습니다. 이는 스트링 슬라이스가 동작하는 방법과 똑같이, 슬라이스의 첫번째 요소에 대한 참조자와 슬라이스의 길이를 저장하는 방식으로 동작합니다. 여러분은 다른 모든 종류의 컬렉션들에 대하여 이런 종류의 슬라이스를 이용할 수 있습니다. 벡터에 대해서 8장에서 이야기할 때 이러한 컬렉션에 대해 더 자세히 다루겠습니다.

## 정리

소유권, 빌림, 그리고 슬라이스의 개념은 러스트 프로그램의 메모리 안정성을 컴파일 타임에 보장하는 것입니다. 러스트 언어는 다른 시스템 프로그래밍 언어와 같이 여러분의 메모리 사용에 대한 제어권을 주지만, 데이터의 소유자가 스코프 밖으로 벗어났을 때 소유자가 자동적으로 데이터를 버리도록 하는 것은 곧 여러분이 이러한 제어를 위해 추가적인 코드 작성이나 디버깅을 하지 않아도 된다는 뜻입니다.

소유권은 러스트의 다른 수많은 부분이 어떻게 동작하는지에 영향을 주므로, 이 책의 남은 부분 전체에 걸쳐 이 개념들에 대해 더 이야기할 것입니다. 다음 장으로 넘어가서 데이터들을 함께 그룹짓는 struct를 보겠습니다.
