# RSA(Rivest-Shamir-Adleman) vs ECC(Elliptic curve cryptography)
공개키 암호화는 정보의 기밀성과 무결성을 보호하기 위해 널리 사용되는 암호화 방식이다.
RSA(Rivest-Shamir-Adleman)와 ECC(Elliptic curve cryptography)는 공개키 암호화 알고리즘 중 가장 대표적인 두 가지이다.
각각 다른 수학적 원리와 개념을 기반으로 하며, 장단점이 다르기 때문에 서로 다른 적절한 상황에 사용되고 있다.

## 1. RSA: Classic public key cryptography
RSA 알고리즘은 1977년에 Ron Rivest, Adi Shamir, Leonard Adleman에 의해 개발된 고전적인 공개키 암호화 알고리즘이다.
RSA는 큰 소수의 곱셈과 인수분해의 난이도에 기반하여 안전성을 제공한다.

### How RSA works
1. 키 생성: RSA는 두 개의 서로 다른 소수(`p`와 `q`)를 선택하고,
   이 두 소수를 곱한 값을 `N`으로 정의한다. 또한, `p-1 * q-1`(최소공배수)를 `φ(N)`으로 정의한다.
2. public key와 private key 생성: RSA는 `1 < e < φ(N)` 및 `φ(N)`과 서로소(`gcd(e, φ(N)) = 1`)인 값을 선택하여 공개키`e`를 생성하고,
   개인키`d`는 `e * d ≡ 1(mod φ(N))`을 만족하는 값이다(`modular multiplicative inverse of e modulo φ(N)`).
3. 암호화: 암호화할 평문(`M`)을 숫자로 변환한 뒤, 공개키`e`와 `N`을 사용하여 암호화한다.
   암호화는 평문의 값을 `e`번 곱한 뒤, `N`으로 나눈 나머지를 계산한다(`C = M^e mod N`).
4. 복호화: 암호화된 암호문을 개인키`d`와 `N`을 사용하여 복호화한다.
   복호화는 암호문을 `d`번 곱한 뒤, `N`으로 나눈 나머지`N`를 계산한다(`M = C^d mod N`).

### RSA in Rust
```rust
extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::{One, Zero};

// RSA 키 생성 함수
fn generate_rsa_key() -> (BigUint, BigUint, BigUint) {
    let p = BigUint::from(61u32); // p: prime 1
    let q = BigUint::from(53u32); // q: prime 2
    let n = &p * &q; // n: modulus
    let phi = (&p - BigUint::one()) * (&q - BigUint::one()); // phi(N)

    let e = BigUint::from(17u32); // e: pubkey
    let d = e.mod_inverse(&phi).unwrap(); // d: privatekey

    (e, d, n)
}

// RSA 암호화 함수
fn rsa_encrypt(message: &BigUint, e: &BigUint, n: &BigUint) -> BigUint {
    message.modpow(e, n)
}

// RSA 복호화 함수
fn rsa_decrypt(ciphertext: &BigUint, d: &BigUint, n: &BigUint) -> BigUint {
    ciphertext.modpow(d, n)
}

fn main() {
    let message = BigUint::from(42u32);
    let (e, d, n) = generate_rsa_key();

    let ciphertext = rsa_encrypt(&message, &e, &n);
    let plaintext = rsa_decrypt(&ciphertext, &d, &n);

    println!("Plaintext: {}", plaintext);
    println!("Ciphertext: {}", ciphertext);
}
```

### Advantages of RSA
1. 수학적 기초 알고리즘이 간단하고 보안성이 검증되었다.
2. 대칭키 암호화에 비해 더 안전하며, 중간에 키가 유출되더라도 안전성을 유지한다.
3. 암호화 및 복호화 과정에서 속도가 느릴 수 있지만, 키 교환과 인증 등 다양한 보안 프로토콜에 사용된다.

### Disadvantages of RSA
1. 키 길이에 따라 암호화 및 복호화 속도가 크게 달라진다.
2. key의 크기가 클수록 연산이 복잡해지고 메모리 요구량이 늘어난다.

## 2. ECC: Efficient and strong public key cryptography
ECC(Elliptic curve cryptography) 알고리즘은 타원곡선 이론을 기반으로 한 공개키 암호화 알고리즘이다.
ECC는 RSA와 비교해 키 길이가 짧으면서도 같은 수준의 보안성을 제공하는 특징을 가지고 있다.

### How ECC works
1. 타원곡선 선택: ECC는 타원곡선 위에서 작업을 수행하기 때문에, 암호화에 사용할 타원곡선을 선택한다.
2. 키 생성: ECC는 소수 개수의 곱셈을 사용하여 공개키와 개인키를 생성한다.
   privatekey는 무작위로 선택된 private value `d`이며, pubkey는 privatekey와 타원곡선 위에서의 곡선의 기준점 `P`에 `d`와 곱셈 연산을 통해 계산된다(`Q = d * P`).
3. 암호화 및 복호화: ECC는 공개키와 개인키를 사용하여 암호화와 복호화를 수행한다.
   암호화는 공개키와 평문을 곡선의 한 지점으로 변환하고 곱셈을 통해 계산되고, 복호화는 이 프로세스를 반대로 수행하여 계산된다.

### ECC in Rust
```rust
extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{One, Zero};

// ECC struct
#[derive(Debug, Clone)]
struct ECPoint {
    x: Option<BigUint>,
    y: Option<BigUint>,
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

// ECC gen 함수
fn generate_ecc_key() -> (ECPoint, BigUint) {
    let a = BigUint::from(2u32); // 타원곡선 파라미터 a
    let b = BigUint::from(3u32); // 타원곡선 파라미터 b
    let p = BigUint::from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16).unwrap(); // 유한체 p (secp256k1의 p 값)

    let base_point = ECPoint {
        x: Some(BigUint::from_str_radix("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap()), // 기저점 G의 x 좌표
        y: Some(BigUint::from_str_radix("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap()), // 기저점 G의 y 좌표
        a: a.clone(),
        b: b.clone(),
        p: p.clone(),
    };

    let private_key = BigUint::from(12345u32); // 개인키

    (base_point, private_key)
}

// ECC mul 연산
fn ecc_multiply(point: &ECPoint, scalar: &BigUint) -> ECPoint {
    let mut result = point.clone();
    let mut scalar_binary = scalar.to_radix_be(2);

    scalar_binary.reverse();
    scalar_binary.pop(); // MSB 제거

    for bit in scalar_binary {
        result = ecc_double(&result);
        if bit == 1 {
            result = ecc_add(&result, &point);
        }
    }

    result
}

// ECC add 연산
fn ecc_add(p: &ECPoint, q: &ECPoint) -> ECPoint {
    let p_x = p.x.clone().unwrap();
    let p_y = p.y.clone().unwrap();
    let q_x = q.x.clone().unwrap();
    let q_y = q.y.clone().unwrap();
    let p_a = p.a.clone();
    let p_b = p.b.clone();
    let p_p = p.p.clone();

    let mut result = ECPoint {
        x: None,
        y: None,
        a: p_a.clone(),
        b: p_b.clone(),
        p: p_p.clone(),
    };

    if p_x == q_x && p_y == q_y {
        let m = (BigUint::from(3u32) * p_x.clone().pow(2u32) + p_a.clone())
            .mod_floor(&p_p)
            .mod_inverse(&p_p)
            .unwrap();

        let mut x = (m.clone().pow(2u32) - p_x.clone() - q_x.clone()).mod_floor(&p_p);
        x = x.mod_floor(&p_p);

        let mut y = (m.clone() * (p_x.clone() - x.clone()) - p_y.clone()).mod_floor(&p_p);
        y = y.mod_floor(&p_p);

        result.x = Some(x);
        result.y = Some(y);
    } else {
        let m = (q_y.clone() - p_y.clone())
            .mod_floor(&p_p)
            .mod_inverse(&p_p)
            .unwrap();

        let mut x = (m.clone().pow(2u32) - p_x.clone() - q_x.clone()).mod_floor(&p_p);
        x = x.mod_floor(&p_p);

        let mut y = (m.clone() * (p_x.clone() - x.clone()) - p_y.clone()).mod_floor(&p_p);
        y = y.mod_floor(&p_p);

        result.x = Some(x);
        result.y = Some(y);
    }

    result
}

// ECC double 연산
fn ecc_double(p: &ECPoint) -> ECPoint {
    let p_x = p.x.clone().unwrap();
    let p_y = p.y.clone().unwrap();
    let p_a = p.a.clone();
    let p_p = p.p.clone();

    let mut result = ECPoint {
        x: None,
        y: None,
        a: p_a.clone(),
        b: p.b.clone(),
        p: p_p.clone(),
    };

    let m = (BigUint::from(3u32) * p_x.clone().pow(2u32) + p_a.clone())
        .mod_floor(&p_p)
        .mod_inverse(&p_p)
        .unwrap();

    let mut x = (m.clone().pow(2u32) - p_x.clone() - p_x.clone()).mod_floor(&p_p);
    x = x.mod_floor(&p_p);

    let mut y = (m.clone() * (p_x.clone() - x.clone()) - p_y.clone()).mod_floor(&p_p);
    y = y.mod_floor(&p_p);

    result.x = Some(x);
    result.y = Some(y);

    result
}

fn main() {
    let message = BigUint::from(42u32);
    let (base_point, private_key) = generate_ecc_key();

    let public_key = ecc_multiply(&base_point, &private_key);
    let ciphertext = ecc_multiply(&base_point, &message.to_bigint().unwrap());
    let plaintext = ecc_multiply(&ciphertext, &private_key).x.unwrap();

    println!("Plaintext: {}", plaintext);
    println!("Ciphertext: {:?}", ciphertext);
}
```

### Advantages of ECC
1. RSA와 비교해 같은 수준의 보안성을 제공하는 키 길이가 짧은 알고리즘이며, 때문에 계산 및 저장에 소요되는 자원이 적다.
2. 암호화 및 복호화 속도가 빠르고 작은 디바이스나 네트워크 통신 환경에서 유용하다.

### Disadvantages of ECC
1. ECC의 구현과 보안 설정에는 정확성이 요구되고, 적절하지 않은 구현은 보안상 취약점을 야기할 수 있다.
2. ECC에 대한 근본적인 증명이 어렵기 때문에, 블랙박스로 취급되는 경우가 있다.

## 3. RSA vs ECC
RSA와 ECC는 공개키 암호화의 기본 원리를 활용한 암호화 알고리즘으로, 각각 장단점이 있다.
아래는 RSA와 ECC를 비교한 특징이다:

- RSA는 수학적인 기초가 간단하고 보안성이 검증되었으며, 안전한 키 교환과 인증에 사용됩니다. 하지만 키 길이에 따라 속도와 메모리 요구량이 크게 달라진다.
- ECC는 키 길이가 짧아서 계산 및 저장에 소요되는 자원이 적고, 같은 수준의 보안성을 제공한다.
  빠른 암호화 및 복호화 속도를 가지며 작은 디바이스나 네트워크 통신 환경에 적합하다.
  하지만 구현과 보안 설정에 정확성이 요구되며, 부적절한 구현은 보안상 취약점을 야기할 수 있다.

RSA와 ECC는 각각의 특징과 요구사항에 따라 선택되어야 한다. RSA는 전통적이고 널리 사용되는 알고리즘이며, ECC는 효율적이고 강력한 알고리즘이다.
적용되는 환경과 요구되는 보안 수준을 고려하여 적절한 선택을 해야한다.

### Security Strength
ECC는 일반적으로 RSA에 비해 strenth-per-bit가 더 높다.
예를 들어 256비트 ECC 키는 대략 3072비트 RSA 키와 동등한 것으로 간주된다.

### Performance
ECC operations는 일반적으로 동일한 보안 수준에서 RSA에 비해 더 빠르고 메모리와 전력을 적게 사용한다.
이는 IoT장치와 같은 제한된 환경에서 특히 유용하다.

### Quantum Computing Threat
RSA와 ECC는 이론적으로 양자 컴퓨터를 이용한 공격에 취약하다.
포스트 양자 암호화는 양자 컴퓨팅 공격으로부터 안전한 암호화 알고리즘에 중점을 둔 분야이다.

### Adoption and Compatibility
RSA는 더 오랜 기간 동안 사용되었으며 광범위한 지원을 받았지만 ECC의 효율성 때문에 밀리고 있다.
일부 환경에서는 RSA만 지원할 수 잇으며 이전 시스템은 최신 타원 곡선을 지원하지 않을 수도 있다.

### Standards and Recommendations
NIST(National Institute of Standards and Technology)는 modern systems들에 대해 ECC로 전환할 것을 권장하고 있다.
그러나 안전한 parameters와 curves를 선택하는 것은 보안을 위해 매우 중요하게 고려되어야 할 사항이다.

### Conclusion
RSA와 ECC는 모두 데이터 보안을 위한 강력한 도구이지만 서로 다른 목적과 시나리오를 제공한다.
RSA의 단순성과 광범위한 지원은 호환성을 위한 안전한 선택이다. 반대로 ECC의 효율성은 성능이 제한된 환경에 적합하다.
RSA와 ECC 중에서 결정할 때 필요한 보안 강도, 성능, 호환성과 같은 요소를 모두 고려해야 한다.
그렇지만 시스템이 RSA에 큰 의존도가 없으며, ECC를 선택할 수 있는 옵션 있는 경우에는 ECC가 권장되는 것은 사실인 것 같다.
때문에 ECC를 채택하되 RSA 및 양자 암호화로 확장할 수 있게 열어두는 것이 합리적일 듯 하다.