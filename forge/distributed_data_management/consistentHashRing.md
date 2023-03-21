# Consistent Hashing
Consistent Hashing은 노드가 추가되거나 제거될 때 다시 매핑해야 하는
키의 수를 최소화하면서 부하의 균형을 맞추는 방식으로 노드 집합에 데이터를
배포하는 방법을 제공하는 분산 해싱 기술이다.  
Consistent Hashing을 사용하면 해시 함수를 사용해 키가 원의 한 점에 매핑되고
각 노드도 동일한 원의 한 점에 매핑된다. 그런 다음 해시 링은 원을 파티션으로 균등하게 분할하며 형성되며 각
파티션은 해시 값 범위를 나타낸다. 모든 노드는 해시 값을 기반으로 링에 저장될 수 있으며 키는 해당 해시 값 범위에 속하는 노드에 매핑된다.
이렇게 하면 각 노드가 특정 범위의 키를 담당하고 전체 키 분포가 노드 간에 균등하게 균형을 이루게 된다.  
1997년 논문 "Consistent Hashing and Random Trees: Distributed Caching Protocols for Relieving Hot Spots on the World Wide Web"에서
Karger 등에 의해 처음 소개되었다.

![conHashRing](../img/conHashRing.png)

## Background
Consistent Hashing은 데이터 파티셔닝 및 노드 멤버십을 관리하기 위해 대규모 분산 시스템에서 일반적으로 사용되는 분산 해시 테이블이다.
Consistent Hashing을 사용하면 전체 데이터 배포에 영향을 주지 않고 시스템에서 노드를 추가하거나 제거할 수 있다.

## How Consistent Hashing Works
Consistent Hashing에서 각 노드에는 일반적으로 해시 함수를 사용하여 생성되는
고유 식별자가 할당된다. key space는 일련의 간격으로 파티션되며 각 간격은
시스템의 노드 중 하나에 할당된다.  

새로운 키가 시스템에 삽입되면 해시 함수를 사용해 키를 원의 한 지점에 매핑하고
이 지점에서 시계 방향으로 회전해 간격을 소유한 노드를 키 저장을 담당하는 노드로 선택한다.  

노드가 시스템에 추가되거나 시스템에서 제거되면 해당 노드에 할당된 키만 다시 매핑하면 된다.
즉 시스템에 노드가 추가되었다면 해당 노드에 할당된 키만 해당 노드에 매핑, 노드를 제거한다면 해당 노드에
할당된 키들만 재할당을 하면 된다. 다른 노드와 키들은 영향을 받지 않는다.
이것은 원에 있는 간격의 소유권이 해시 함수에 의해 결정되고 노드는 자신의 ID를 기반으로 원의 일부에 대한 소유권을 갖기 때문이다.
따라서 deterministic한 방식으로 적은 수의 키만 다시 원에 매핑하도록 재할당하면 되고 나머지 키는 영향을 받지 않고
시스템의 나머지 노드에서 계속 서비스를 제공한다.

1. Key Distribution
   - 각 노드에는 해시 함수를 사용하여 고유한 식별자가 할당된다.
   - 키 공간은 시스템의 노드에 할당된 간격으로 분할된다.

2. Node Addition and Removal
   - 노드를 추가하거나 제거할 때 노드에 할당된 키만 다시 매핑하면 된다.
   - 노드는 자신의 ID에 따라 원의 간격에 대한 소유권을 갖는다.
   - intervals의 소유권은 해시 함수에 의해 결정된다.
   - 소수의 키만 결정적으로 재할당하면 된다.

3. Key Lookup
   - 새 키가 삽입되면 해시 함수를 사용하여 키를 원의 point에 매핑한다.
   - 시계 방향으로 노드 소유 interval이 키 저장을 담당한다.
   - 정렬된 키 목록에서 이진 검색을 사용하여 효율적으로 조회할 수 있다.

## Advantages of Consistent Hashing
Consistent Hashing static hashing 및 dyn hashing과 같은 기존 해싱 기술에 비해 몇 가지 이점을 제공한다.

### Load Balancing
Consistent Hashing은 부하가 시스템의 노드 전체에 고르게 분산되도록 한다.
이는 키를 원에 매핑하고 각 노드에 원의 일부를 할당함으로써 달성된다.
이렇게 하면 각 노드가 거의 동일한 수의 키를 담당하므로 load balancing에 큰 도움이 된다.

### Scalability
Consistent Hashing은 확장성이 뛰어나고 많은 수의 노드를 쉽게 처리할 수 있다.
이는 노드를 추가하거나 제거할 때 적은 수의 키만 다시 매핑하면 되므로 이동해야 하는 데이터의 양이 최소화되기 때문이다.

### Fault Tolerance
Consistent Hashing은 노드 손실이 해당 노드에 할당된 키에만 영향을 미치므로 높은 수준의 내결함성을 제공한다.
나머지 키는 영향을 받지 않고 시스템의 나머지 노드에서 계속 제공된다.

### Consistency and Availability
Consistent Hashing은 eventual consistency을 제공한다. 즉, 시스템의 모든 노드가 결국 일관된 상태로 수렴된다.
그러나 노드들은 다른 시간에 다른 데이터 views를 가질 수 있으므로 strong consistency는 보장하지 않는다.

앞서 언급한 것과 같이 Consistent Hashing은 일부 노드가 실패하더라도 시스템이 계속 작동하므로 고가용성을 제공한다.
그러나 많은 수의 노드가 손실되면 부하의 불균형과 성능 저하가 발생할 수 있다.

## Implementation of Consistent Hashing
Consistent Hashing은 hash tables, balanced trees, or linked lists와 같은
다양한 data structures를 사용하여 구현할 수 있다. 가장 일반적인 구현은 fixed number of slots이 있는 circular hash table이다.

새 노드가 시스템에 추가되면 원의 한 지점으로 해시되는 임의의 ID가 할당된다.
그런 다음 노드는 자신의 ID와 시계 방향 이웃의 ID 사이의 원 부분에 대한 소유권을 갖는다.
키가 시스템에 삽입되면 원의 한 지점으로 해시되고 이 지점에서 시계 방향으로 간격을 소유한 노드가 키 저장을 담당한다.

### Node ID Generation
새 노드가 시스템에 추가되면 일반적으로 해시 함수를 사용하여 생성되는 고유 식별자가 할당된다.
식별자는 노드가 원 전체에 고르게 분포되도록 무작위로 선택된다.
노드 ID를 생성하는 데 사용되는 해시 함수는 충돌을 방지하기 위해 암호학적으로 안전해야 한다.

### Key Hashing
새 키가 시스템에 삽입되면 노드 ID 생성에 사용된 것과 동일한 해시 함수를 사용하여 해시된다.
해시 함수의 출력은 키를 원의 한 점에 매핑하는 데 사용되는 64비트 정수이다.
이 시점부터 시계 방향 간격을 소유한 노드가 키 저장을 담당한다.

### Node Assignment
원의 간격에 노드를 할당하는 것은 노드 ID를 생성하는 데 사용되는 해시 함수에 의해 결정된다.
각 노드는 자신의 ID와 시계 방향 이웃의 ID 사이의 원 부분에 대한 소유권을 갖는다.
이렇게 하면 각 노드가 키의 일부를 담당하고 부하가 노드 전체에 고르게 분산된다.
노드가 시스템에 추가되거나 시스템에서 제거되면 해당 노드에 할당된 키만 다시 매핑하면 되므로
이동해야 하는 데이터의 양이 최소화된다.

### Replication
Consistent Hashing에서는 여러 노드를 원의 동일한 간격에 할당하여 복제를 수행할 수 있다.
이렇게 하면 노드 중 하나에 장애가 발생해도 다른 노드가 인계받아 요청을 계속 처리할 수 있다.  
NOTE_ 그러나 이것은 일부 노드가 다른 노드보다 더 많은 수의 간격을 담당할 수 있으므로 부하의 불균형을 초래할 수 있다.

### Virtual Nodes
가상 노드는 일관된 해싱에서 로드 밸런싱을 개선하고 노드를 추가하거나 제거할 때 다시 매핑해야 하는 키 수를 줄이는 데 사용된다.
가상 노드는 동일한 물리적 노드에 여러 식별자를 할당하여 생성된다.
이렇게 하면 로드가 노드 전체에 고르게 분산되고 노드가 추가되거나 제거될 때 다시 매핑해야 하는 키 수가 줄어든다.

## The ConsistentHashRing Struct
Consistent Hash Ring은 원형 링으로 표시되며 각 노드는 해시 값을 기준으로 링의 한 지점에 표시된다.
링에 키를 저장하기 위해 키는 먼저 링의 한 지점으로 해시된다. 그러면 키가 해시된 지점에서 시계방향으로 링에 나타나는 첫 번째 노드에 키가 저장된다.
시계 방향으로 노드가 없으면(키를 저장할 적절한 노드를 찾지 못한 채 해시 링에 대한 루프가 완료되었다면) 루프에서 만난 마지막 노드 바로 다음 노드인,
해시 링의 첫 번째 노드에 키가 저장된다.

다음은 hash table과 circular linked list를 사용한 Consistent Hashing의 예이다. (written in Rust)
```rust
pub struct ConsistentHashRing {
    nodes: HashMap<u64, String>,
    sorted_keys: Vec<u64>,
}

impl ConsistentHashRing {
    pub fn new() -> Self {
        ConsistentHashRing {
            nodes: HashMap::new(),
            sorted_keys: Vec::new(),
        }
    }

    pub fn add(&mut self, node_id: &str, replicas: usize) {
        for i in 0..replicas {
            let key = Self::hash(&(node_id.to_owned() + &i.to_string()));
            self.nodes.insert(key, node_id.to_owned());
            self.sorted_keys.push(key);
        }
        self.sorted_keys.sort();
    }

    pub fn remove(&mut self, node_id: &str) {
        let mut remove_indices = Vec::new();
        for (i, key) in self.sorted_keys.iter().enumerate() {
            let node = self.nodes.get(&key).unwrap();
            if node == node_id {
                remove_indices.push(i);
            }
        }
        for i in remove_indices.iter().rev() {
            self.sorted_keys.remove(*i);
        }
        self.nodes.retain(|_, v| v != node_id);
    }

    pub fn get_node(&self, key: &str) -> Option<String> {
        if self.nodes.is_empty() {
            return None;
        }
        let hash = Self::hash(key);
        let pos = match self.sorted_keys.binary_search(&hash) {
            Ok(pos) => pos,
            Err(pos) => {
                if pos == self.sorted_keys.len() {
                    0
                } else {
                    pos
                }
            }
        };
        Some(self.nodes.get(&self.sorted_keys[pos]).unwrap().clone())
    }

    fn hash(key: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }
}
```
ConsistentHashRing struct는 노드를 저장하기 위해 해시 테이블(HashMap)을 사용하고 키를 저장하기 위해 정렬된 벡터를 사용하는
Consistent hashing의 구현이다. 이 구현에 사용된 해시 함수는 키를 원의 점에 매핑하고 노드는 ID와 원의 시계 방향 이웃 ID 사이의 점 범위에 속하는
키를 저장하는 역할을 한다.

add 메서드는 해시 테이블에 새 노드를 추가하고 오름차순으로 정렬된 벡터에 해당 키를 추가한다.  
remove 메서드는 해시 테이블과 정렬된 벡터 모두에서 노드를 제거한다.  
get_node 메서드는 정렬된 벡터에서 해당 키를 찾기 위해 binary_search를 사용하여 주어진 키를 담당하는 노드를 반환한다.  

이 구현에 사용되는 해시 함수의 선택은 성능 및 부하 분산 속성에 매우 중요하다.
해시 함수는 노드 전체에 키를 균등하게 배포하고 노드가 거의 동일한 수의 키를 담당하도록 한다.
사용된 해시 함수는 분산 속성이 우수하고 빠르며 충돌률이 낮아야 한다.

이 구현을 통해 dyn membership이 있는 분산 시스템에서 주어진 키를 담당하는 노드를 효율적으로 조회할 수 있다.
그러나 노드가 추가되거나 제거될 때 해시 링을 재조정해야 하는 등 몇 가지 단점이 있으며,  
매번 해시링을 재조정한다는 것은 매 accountID의 업데이트에 대해 sort를 수행한다는 의미로 대규모 시스템에서 치명적일 수 있다.
이러한 단점을 해결하기 위해 Rendezvous Hashing 또는 Jump Hashing과 같은 Consistent Hashing의
다른 구현이 제안되었다. 이러한 구현은 rebalancing의 필요성을 최소화하고 시스템의 성능 및 load balancing을 개선하는 것을 목표로 한다.

## Use Cases
Consistent Hashing은 contents delivery networks, 분산 데이터베이스 및 P2P 네트워크와 같은 다양한 분산 시스템에서 사용된다.
노드 수가 많고 동적이며 로드를 예측할 수 없는 시스템에서 특히 유용하다.

## Limitations
Consistent Hashing 기술에는 시스템을 설계하고 구현할 때 고려해야 할 몇 가지 limitation 사항이 있다.
주요 risk중 하나는 핫스팟 및 부하 불균형(데이터 지역성)의 데이터 분포 가능성이다.
핫스팟은 노드가 불균형적으로 많은 수의 키를 담당할 때 발생하며, 이는 가상 노드와 같은 기술을 사용하여 로드를 보다 균등하게 분산함으로써
완화할 수 있다. 또한 일부 시나리오에서는 데이터 분포가 노드 전체에 고르게 분산되지 않아 성능에도 영향을 미칠 수 있다.  

Consistent Hashing은 또한 일부 응용 프로그램에서 필요할지도 모르는 strong consistency을 보장하지 않는다.
마지막으로 해시 함수의 선택은 시스템의 성능 및 로드 밸런싱 속성에 상당한 영향을 미칠 수 있다.  
일부 해시 함수는 다른 함수보다 더 많은 충돌을 생성하여 핫스팟 가능성이 높아지고 로드 밸런싱이 감소할 수 있으며,
또 다른 해시 함수는 분산 속성이 더 우수하여 로드 밸런싱 및 성능이 향상될 수 있다.
따라서 최적의 성능과 로드 밸런싱을 보장하려면 특정 사용 사례에 적합한 해시 함수를 신중하게 선택하는 것이 중요하다.

## Comparison with Other Techniques
Consistent Hashing은 분산 해싱에 사용되는 여러 기술 중 하나이다.
다른 기술로는 static hashing, dynamic hashing, and rendezvous hashing, jump hashing등이 있다.
이러한 각 기술에는 고유한 강점과 약점이 있으며 기술 선택은 시스템의 특정 요구 사항에 따라 다르다.
그럼에도 다른 기술과 비교할 때 Consistent Hashing은 높은 확장성, 내결함성 및 로드 밸런싱 속성을 제공한다.