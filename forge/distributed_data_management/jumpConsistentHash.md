# Jump Consistent Hashing
Jump Consistent Hashing은 효율적인 데이터 배포 및 로드 밸런싱을 위해 분산 시스템에서 사용되는 기술이다.
JumpConsistent Hashing은 지정된 수의 버킷에 키를 균일하게 분배하는 해싱 알고리즘으로,
2014년 John Lamping과 Eric Veach의 연구 논문에서 처음 소개되었다.
이 알고리즘은 일관된 해시 링을 사용하여 노드 클러스터 전체에 데이터를 배포한다.
노드 전체에 부하를 고르게 분산하고 리소스를 효율적으로 사용하는 등 기존의 Consistent Hashing 에 비해 몇 가지 이점을 제공한다.

Jump Consistent Hashing는 키를 해시 값에 매핑하는 데 Robert Jenkins의 해시 함수를 사용한다.
이 알고리즘은 일련의 해시 값을 가상 버킷 세트로 나누고 버킷 사이를 이동하여 노드에 키를 할당하는 방식으로 작동한다.
Jump Consistent Hashing은 핫스팟의 위험 감소 및 로드 밸런싱 개선을 포함하여 기존의 일관된 해싱에 비해 몇 가지 장점이 있다.
실제로 Jump Consistent Hashing은 다양한 실제 응용 프로그램에서 사용할 수 있으며 다양한 기술을 통해 최적화할 수 있다.
그러나 유연성 부족 및 복잡성 증가와 같은 몇 가지 제한 사항과 고려해야 할 잠재적인 단점이 있다.
전반적으로 Jump Consistent Hashing은 분산 시스템에서 효율적인 데이터 배포를 위한 강력한 도구이다.

## 1. Background
### Overview of distributed systems and the importance of efficient data distribution
분산 시스템은 대량의 데이터를 여러 시스템으로 나누어 처리하는 데 사용된다.
분산 시스템의 주요 이점은 단일 시스템보다 더 큰 처리 능력과 저장 용량을 제공한다는 것이다.
분산 시스템은 여러 노드가 문제에 대해 병렬로 작업할 수 있도록 하여 병렬성과 확장성을 모두 제공할 수 있으며,
이로 인해 처리능력이 향상될 수 있다. 즉, scale-out과 scale-up의 효과를 모두 누릴 수 있다.
그러나 분산 시스템을 구축하고 유지하는 것은 어려울 수 있다.
데이터가 적절하게 분산되고 시스템이 대량의 트래픽을 처리할 수 있도록 신중한 설계 및 관리가 필요하기 때문이다.

효율적인 데이터 분산은 분산 시스템의 확장성과 내결함성을 유지하는 데 필수적이다.
반면 일관되지 않은 데이터 분산은 핫스팟과 불균형 로드로 이어질 수 있으며, 이는 성능 문제와 심지어 시스템 오류까지 유발할 수 있다.
Consistent Hashing은 시스템 전체에 데이터를 균등하게 배포하는데 사용되는 일반적인 기술이지만 몇 가지 제한 사항이 있다.

### Brief explanation of traditional consistent hashing and its limitations
Consistent Hashing은 각 노드가 키 범위를 담당하는 원형 공간의 노드에 키를 매핑하여 작동한다.
이 접근 방식을 사용하면 부하가 시스템의 노드 전체에 고르게 분산되지만, Consistent hashing에는 핫스팟 가능성 및
시스템에서 노드를 추가하거나 제거할 때 일반적으로 노드를 순환 목록으로 다시 정렬해야 한다.
새로운 노드가 추가되거나 기존 노드가 제거될 때마다 전체 목록을 다시 해싱해야 하므로, 노드가 많은 대규모 서비스에서는 비효율적일 수 있다.

핫스팟은 적은 수의 키가 불균형적으로 많은 수의 노드에 매핑될 때 발생하며 이로 인해 로드 분산이 불균형해지고 시스템 성능이 저하될 수 있다.
이전에 해당 노드에 매핑된 키를 다른 노드에 다시 매핑해야 하므로 시스템에서 노드를 추가하거나 제거할 때 리해싱이 필요하다.
이 프로세스는 특히 대규모 분산 시스템에서 시간이 많이 걸리고 자원 집약적일 수 있다.

이러한 제한 사항을 해결하기 위해 Jump Consistent Hashing과 같은 대체 해싱 기술이 제안되었다.
Jump Consistent Hashing은 키를 노드에 매핑하는 다른 접근 방식을 사용하므로 재해싱이 필요하지 않고 핫스팟의 가능성이 줄어든다.
이 기술은 효율적인 데이터 배포가 시스템 성능과 확장성을 유지하는 데 중요한 대규모 분산 시스템에서 특히 유용하다.

## 2. Robert Jenkins' Hash Function
JumpConsistent Hash Ring의 구현은 1997년 논문 "Hash Functions for Hash Table Lookup"에서
Robert Jenkins가 제안한 알고리즘의 변형을 사용한다.

### Overview of the hash function used in JumpConsistent Hashing
Jenkins의 해시 함수는 입력 키를 받아 32비트 해시 값을 출력으로 생성하는 비암호화 해시 함수이다.
이 함수는 입력 키를 해시 값으로 변환하기 위해 비트 시프트와 비트 연산의 조합을 사용한다.
해시 함수는 상대적으로 간단하고 빠르지만 여전히 키에 대한 해시 값의 좋은 분포를 제공한다.
https://burtleburtle.net/bob/hash/evahash.html
https://en.wikipedia.org/wiki/Jenkins_hash_function

### Explanation of how the hash function maps keys to hash values
Jenkins의 해시 함수를 사용하여 키를 해시 값에 매핑하기 위해 이 함수는 키를 입력으로 사용하고 해시 값을 출력으로 생성한다.
그런 다음 해시 값은 Jump Consistent Hashing 알고리즘에서 가상 버킷에 키를 할당하는 데 사용된다.
해시 함수는 가능한 값의 32비트 범위에 고르게 분포되는 무작위 모양의 해시 값을 생성하도록 설계되어 가상 버킷에 키를 고르게 분포시키는 데 도움이 된다.

일련의 해시 값을 0에서 n-1까지 번호가 매겨진 가상 버킷 세트로 나눈다고 생각해보자.
1. 0에서 9까지 번호가 매겨진 총 10개의 가상 버킷이 있다.
2. 이러한 가상 버킷 중 하나에 "hello world" 키를 할당하려고 한다.
3. 먼저 Jenkins의 해시 함수를 사용하여 키에 대한 32비트 해시 값을 계산한다.
4. 이 값이 0x3e4a5a57이라고 가정한다.
5. 다음으로 이 해시 값을 사용하여 32비트 해시 값을 32비트 왼쪽으로 이동하고,
   원래 32비트 해시 값과 비트 단위 OR 연산하여 64비트 해시 값을 계산한다.
6. 이것은 0x000000003e4a5a57의 64비트 해시 값을 제공한다.
7. 그런 다음 이 64비트 해시 값을 총 가상 버킷 수(이 경우 10)로 나누어 인덱스 i를 얻는다.
   이 예에서 0x000000003e4a5a57를 10으로 나눈 값은 104506018이고 나머지는 3이므로 i는 3이다.
8. 마지막으로 "hello world" 키를 가상 버킷 번호 3에 할당한다.

다음은 위의 내용대로 구현한 Jenkins의 one-at-a-time hash algorithm이다.
```rust
fn jenkins_hash(key: &str) -> u32 {
   let mut hash: u32 = 0;
   for byte in key.bytes() {
      hash = hash.wrapping_add(byte as u32);
      hash = hash.wrapping_add(hash << 10);
      hash ^= hash >> 6; // XOR operation (hash += hash >> 6)
   }
   hash = hash.wrapping_add(hash << 3);
   hash ^= hash >> 11;
   hash = hash.wrapping_add(hash << 15);
   hash
}

// Jump Consistent Hashing function
fn jump_consistent_hash(key: &str, num_buckets: u32) -> u32 {
   let hash = jenkins_hash(key) as u64;
   let mut index = 0;
   let mut next = 1;
   let num_buckets = num_buckets as u64;

   while index < num_buckets {
      index = next;
      next <<= 1;
      next += 1;
   }

   let mut bucket = (hash.wrapping_mul(index)) >> 32;
   if bucket >= num_buckets {
      bucket = (hash.wrapping_mul(next)) >> 32;
   }

   bucket as u32
}

fn main() {
   let key = "hello world";
   let num_buckets = 10;
   let bucket = jump_consistent_hash(key, num_buckets);

   println!("Key '{}' maps to virtual bucket {}.", key, bucket);
}
```
이 예제에서 jenkins_hash 함수는 문자열 키를 입력으로 사용하고 32비트 해시 값을 반환하는 Jenkins의 해시 함수를 구현한다.
'jump_consistent_hash' 함수는 이 해시 함수를 사용하여 키에 대한 64비트 해시 값을 계산한 다음 지정된 총 버킷 수를 기반으로 가상 버킷에 매핑한다.
마지막으로 main 함수는 jump_consistent_hash를 사용하여 키를 가상 버킷에 매핑하는 방법을 보여준다.

Jenkins의 해시 함수는 Jump Consistent Hashing 알고리즘의 핵심 구성 요소이며,
이를 통해 알고리즘이 키를 가상 버킷에 효율적으로 매핑하고 데이터를 분산 시스템 전체에 고르게 배포할 수 있다.

## 3. How JumpConsistent Hashing Works
Jump Consistent Hashing은 키를 노드에 매핑하는 데 기존의 Consistent Hashing과 다른 접근 방식을 사용한다.
Jump Consistent Hashing은 원형 공간의 노드에 키를 매핑하는 대신 키를 가상 bucket 집합으로 분할된 해시 값에 매핑한다.
그런 다음 bucket 사이를 이동하여 노드에 키를 할당한다. 이렇게 하면 다시 해싱할 필요가 없으며 핫스팟의 가능성이 크게 줄어든다.

### Overview of the algorithm and its components
JumpConsistent Hashing의 알고리즘은 해시 함수와 가상 bucket에 키를 할당하는 방법의 두 가지 주요 구성요소로 구성된다.

Robert Jenkins의 해시 함수에서 생성된 해시 값을 키로 입력으로 사용하고,
키는 가상 해시 테이블에서 버킷에 할당하기 위해 매핑하는 방식으로 사용된다.
그런 다음 알고리즘은 현재 키와 해시 테이블의 버킷 수를 기반으로 다른 버킷으로 "jump"한다.
이 프로세스는 키가 저장되는 최종 버킷이 선택될 때까지 반복된다.

### Detailed explanation of the hash function and its role in the algorithm
여기서 사용되는 해시 함수는 위에서 설명한 Robert Jenkins가 제안한 one-at-a-time hash algorithm의 변형이다.
입력 키를 사용하고 비트 시프트와 비트 연산의 조합을 사용하여 32비트 해시 값을 출력으로 생성한다.
해시 함수는 가능한 값의 32비트 범위에 고르게 분산되는 임의 모양의 해시 값(avalanche)을 생성하도록 설계되어
가상 버킷 전체에 키를 고르게 분산하는 데 도움이 된다.

다음은 JumpConsistent Hash Ring의 작동 방식이다.

1. 해싱할 버킷 수, N, 키를 선택한다.
2. 64비트 정수 해시 함수(Jenkins Hash)를 사용하여 키의 해시 값을 계산한다.
3. 해시 값을 사용하여 초기 버킷을 결정한 다음 알고리즘이 키가 저장된 최종 버킷에 도달할 때까지 버킷 수에 따라 버킷 사이를 "Jump"한다.
4. 버킷이 존재하지 않는 경우 알고리즘이 기존 버킷을 분할하여 생성한다. 이는 버킷 수가 변경되더라도 load balance를 유지하는 데 도움된다.
5. 버킷이 이미 있는 경우 버킷에 키를 넣는다. 노드가 추가되거나 제거될 때 키의 재배포가 일관된 해싱 접근 방식을 사용하여 수행된다.
   여기서 키의 작은 부분만 새 버킷에 다시 매핑하면 되고, 나머지 키는 원래 버킷에 남아 있을 수 있다.

JumpConsistent Hash Ring의 중요한 기능 중 하나는 버킷 수가 변경되더라도 버킷 전체에 균일한 키 배포를 보장한다는 것이다.
즉, 클러스터에 새 노드가 추가되거나 기존 노드가 제거되면 시스템의 전체 균형을 유지하는 방식으로 키가 재분배된다.

## 4. The Advantages of JumpConsistent Hashing
JumpConsistent Hashing은 기존의 Consistent Hashing에 비해 몇 가지 주요 이점을 제공한다.
### Explanation of JumpConsistent Hashing and its key advantages
#### 1) Load balancing
JumpConsistent Hashing은 버킷 수가 변경되더라도 가상 버킷 전체에 키를 균등하게 분배한다.
이렇게 하면 시스템 전체에서 균형 잡힌 로드를 유지하고 핫스팟 가능성을 줄일 수 있다.
#### 2) Fast Key Lookup
JumpConsistent 해싱은 키를 가상 버킷에 매핑하는 단일 해시 함수만 필요하므로 기존의 일관된 해싱보다 빠르고 간단하다.
#### 3) Minimal Rebalancing
JumpConsistent Hashing은 "Jump" 접근 방식을 사용하여 키를 가상 버킷에 매핑하므로 비용이 많이 드는 재해싱이 필요하지 않으며,
시스템에서 노드를 추가하거나 제거할 때 재조정해야 하는 데이터의 양을 줄인다.

### Comparison of JumpConsistent Hashing to traditional consistent hashing
#### 1) Circular Space vs Virtual Buckets
JumpConsistent Hashing은 가상 버킷을 사용하여 키를 노드에 매핑하는 반면 기존의 Consistent Hashing은 키를 원형 공간의 노드에 매핑한다.
#### 2) Number of Hash Functions
JumpConsistent 해싱에는 하나의 해시 함수만 필요한 반면, 기존의 Consistent Hashing은 일반적으로 여러 개의 해시 함수가 필요하다.
#### 3) Rebalancing
기존의 Consistent은 시스템에서 노드를 추가하거나 제거할 때 키의 많은 부분을 재해싱 및 재매핑해야 한다.
그러나 JumpConsistent 해싱은 새 노드가 추가되면 알고리즘은 기존 가상 버킷을 반으로 나누고 절반을 새 노드에 할당할 수 있다.
해당 가상 버킷에 원래 할당된 키의 일부만 새 노드에 다시 매핑하면 되고 나머지 키는 원래 버킷에 남을 수 있다.
마찬가지로 노드가 제거되면 해당 가상 버킷은 이웃의 가상 버킷과 병합될 수 있으며 다시 매핑할 키의 작은 부분만 필요하므로 더 효율적이다.

예를 들면, 기존의 consistent hashing은 node가 업데이트 될때마다 리스트를 순환 정렬 해야 하거나,
키 일부의 재해싱 및 재매핑을 요구할 수 있다. 이것은 대규모 서비스에서 잦은 update가 있을 경우 매우 큰 비중의 리소스를 점유하므로
높은 서비스 레이턴시 및 시스템 복잡성 증가 등의 여러 측면에서 성능에 문제가 될 가능성이 있다.  
대조적으로 JumpConsistent Hashing은 Rebalancing에 보다 효율적인 접근 방식을 사용하여 재매핑해야 하는 데이터의 양을 줄이고 
위와 같은 잠재적인 성능 문제를 방지한다.
