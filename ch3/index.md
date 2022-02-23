# Common Programming Concepts

## 변수와 가변성

let

기본적으로 불변, mut 키워드로 가변

## 상수

const

항상 불변, 데이터 타입 명시 필요

## 섀도잉

섀도잉 있음

## 데이터 타입

### 스칼라

| Length | Signed | Unsigned |
| ------ | ------ | -------- |
| 8bit   | i8     | u8       |
| 16bit  | i16    | u16      |
| 32bit  | i32    | u32      |
| 64bit  | i64    | u64      |
| arch   | isize  | usize    |

isize와 usize는 컴퓨터 환경에 따라 결정된다.

### 정수 리터럴

| Number literals | Example     |
| --------------- | ----------- |
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |

### 부동 소수점

f32, f64
기본은 f64

### Boolean

부우울

### 문자

char

```rust
let c = 'z';
```

### 복합 타입

#### 튜플

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

튜플은 단일 요소를 위한 복합계로 고려되었기에 변수 tup에는 튜플 전체가 bind 됩니다. 개별 값을 튜플의 밖으로 빼내오기 위해서는, 패턴 매칭을 사용하여 튜플의 값을 구조해체 시키면 됩니다.

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    println!("The value of y is: {}", tup.2);
}
```

마치 js의 구조분해 할당과 같은 그것

#### 배열

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

벡터보다 배열이 유용한 경우는 데이터를 heap보다 stack에 할당하는 것을 원하거나. 항상 고정된 요소를 갖는다고 확신하고 싶을 때입니다.
c++ array or vector?
out of range error 주의
Rust는 유효하지 않은 메모리에 엑세스를 허용하나 즉시 종료함으로써 이런 종류의 오류로부터 사용자를 보호한다.

## 함수 동작 원리

```rust
fn foo(x: i32, y: i32) -> i32{
  x+y
}
```

### 구문과 표현식

구문은 반환값이 없다.

```rust
fn main() {
    let x = (let y = 6);
}
```

표현식은 값을 산출한다.

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y); //4
}
```

x + 1을 x + 1; 이라고 하는 순간 구문이 되기 때문에 ()처럼 빈 튜플로 표시된다.

## 주석

//와 문서화 주석이 있음
/\*\*/ 없음

## 제어문

### if

조건은 반드시 boolean이어야 한다.

### let구문에서 if 사용하기

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```

Rust는 타입을 런타임에 정의되도록 할 수 없다.

```rust
let number = if condition { 5 } else { 'six' }; //안됨
```

### 반복문과 반복

```rust
loop{}

while condition {}

for element in a.iter()

fn main() {
    for number in (1..4).rev() {
        println!("{}", number);
    }
}
```
