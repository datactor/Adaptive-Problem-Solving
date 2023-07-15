# Practical Byzantine Fault Tolerance (PBFT)
### Introduction to PBFT
PBFT(Practical Byzantine Fault Tolerance)는 악의적이거나 결함이 있는 노드가 있음에도 불구하고
노드 그룹(cluster)이 공유 상태에 대한 합의에 도달할 수 있도록 하는 분산 시스템용으로 설계된 합의 알고리즘이다.

PBFT는 1999년 [Miguel Castro와 Barbara Liskov의 논문](https://pmg.csail.mit.edu/papers/osdi99.pdf) 에서 처음 소개되었다.
그 이후로 높은 처리량과 내결함성을 요구하는 블록체인 및 기타 분산 시스템을 위한 대중적인 합의 알고리즘이 되었다.

PBFT는 분산 시스템이 중앙 시스템에 의존하지 않고 합의에 도달할 수 있는 방법을 제공하기 때문에 탈중앙화의 핵심 요소이다.
이로 인해 PBFT는 분산되고 신뢰할 수 없도록 설계된 블록체인 시스템에 특히 유용하다.
PBFT는 또한 네트워크에서 최대 1/3의 Byzantine 노드를 허용할 수 있다는 장점이 있어 강력하고 탄력적인 합의 알고리즘이 된다.

* PBFT의 맥락에서 `Byzantine`이라는 용어는 네트워크에서 악의적이거나 독단적으로 행동하는 노드를 의미한다.
  여기에는 유효하지 않은 메시지를 보내거나 정보를 유보하는 등 의도적으로 합의 프로세스를 방해하려는 노드가 포함된다.
* PBFT는 비잔틴 결함에 대해 탄력적으로 설계되었다. 즉, 네트워크 노드의 최대 1/3이 비잔틴 노드인 경우에도 계속해서 올바르게 작동할 수 있다.
  이것은 `Byzantine fault tolerance threshold` 또는 `f+1` 규칙으로 알려져 있다.
  여기서 `f`는 시스템이 허용할 수 있는 결함이 있는 노드의 최대 수를 나타내고 `+1`은 합의에 필요한 올바른 노드의 최소 수를 나타낸다.
* 예를 들어, 노드가 100개이고 비잔틴 내결함성 임계값이 33%인 네트워크에서 네트워크는 최대 33개의 비잔틴 노드를 허용하면서 여전히 합의에 도달할 수 있다.
  이는 최소 67개의 노드가 정확하고 정직하게 행동하는 한 네트워크가 합의를 유지하고 계속해서 올바르게 작동할 수 있음을 의미한다.
* Byzantine faults를 허용하는 기능은 PBFT의 핵심 기능이며 악의적인 노드가 있는 경우에도 합의를 유지할 수 있어야 하는
  블록체인 네트워크와 같은 분산 시스템을 위한 강력한 합의 알고리즘이다.

### Overview of Byzantine Fault Tolerance (BFT)
`Byzantine`이라는 용어는 `Byzantine Generals' Problem`에서 유래한 것으로, 장군들의 집단이 공동의 적을 공격하는 데 협력해야 하지만
일부 장군은 계획을 훼손하려는 traitor일 수 있다는 theoretical 문제이다.
이 문제는 노드의 분산 네트워크를 조정하는 문제와 유사하며 그 중 일부는 결함이 있거나 악의적일 수 있다는 점에서 유사하다.
다음 [Byzantine fault](https://en.wikipedia.org/wiki/Byzantine_fault) 에서 [analogy](https://namu.wiki/w/%EB%B9%84%EC%9E%94%ED%8B%B0%EC%9B%80%20%EC%9E%A5%EA%B5%B0%20%EB%AC%B8%EC%A0%9C) 와 함께 보자.
비잔틴 장군 문제는 negative-sum 게임의 문제이다. 즉, negative-sum 게임의 상황을 전제로 하며 합의 알고리즘을 통해서 positive-sum의 결과를 달성할 수 있도록 설계되었다.

BFT 알고리즘은 노드의 분산 네트워크가 Byzantine faults가 있는 경우에도 공유 상태에 대한 합의에 도달할 수 있도록 설계되었다.
이러한 알고리즘은 일반적으로 노드 간의 일련의 메시지 교환을 포함하며 각 노드는 현재 상태에 대한 정보를 요청하고 확인하기 위해 피어에 메시지를 보낸다.
노드는 암호화 메커니즘을 사용하여 메시지가 인증되고 악의적인 행위자가 노드를 조작할 수 없도록 한다.

BFT는 결함이 있거나 악의적인 노드가 있는 경우에도 계속해서 올바르게 작동할 수 있기 때문에 분산 시스템에 중요다.
이렇게 하면 공격자가 악용할 수 있는 시스템 전체 오류 또는 보안 취약점을 방지할 수 있다.
BFT는 실패 또는 악의적인 행동이 심각한 결과를 초래할 수 있는 금융 네트워크 또는 군사 명령 및 제어 시스템과 같은 중요한 시스템에 특히 중요하다.

그러나 BFT 알고리즘에도 한계가 있다. 한 가지 제한 사항은 내결함성을 달성하기 위해 네트워크에서 일정량의 redundancy가 필요하다는 것이다.
이는 리소스 측면에서 비용이 많이 들고 확장성을 제한할 수 있다.
또한 BFT 알고리즘은 일반적으로 네트워크의 노드 간에 특정 수준의 신뢰(67%)를 가정하며 실제 시나리오에서는 가정이 성립하지 않을 수 있다.

전반적으로 BFT는 강력하고 안전한 분산 시스템의 개발을 가능하게 하는 분산 컴퓨팅의 중요한 개념이다.
한계가 있지만 중요한 시스템의 복원력과 보안을 보장하는 중요한 도구로 사용된다.

### How PBFT works
PBFT의 알고리즘은 지정된 leader node(primary node)가 네트워크에 transaction block을 제안하도록 함으로써 작동한다.
그런 다음 다른 노드(replicas)는 제안된 tx block을 확인하고 validity에 vote한다.

PBFT 알고리즘은 네트워크의 일부 노드가 비잔틴인 경우에도 작동하도록 설계되었다.
즉, 임의로 작동하고 시스템 규칙을 따르지 않을 수 있다.
이를 수용하기 위해 PBFT는 네트워크 노드의 최대 1/3이 비잔틴일 수 있다고 가정한다.

#### PBFT Consensus Process
PBFT 합의 프로세스는 4단계(여기선 5단계로 나눴다)로 구성된다.

1. Request Phase: 클라이언트가 primary node에 요청을 보낸다. primary node는 tx block을 propose하라는 요청을 받는다.
2. Pre-prepare Phase: The primary node는 요청에 시퀀스 번호를 할당하고 제안된 블록에 대한 `pre-prepare` 메시지를 all secondary nodes(backup nodes)들로 broadcast한다.
3. Prepare Phase: backup node는 요청을 수신 및 확인하고, 유효하다면 네트워크의 다른 replicas들에 `prepare` 메시지를 보낸다.
4. Commit Phase: replicas들이 2/3의 replicas들로부터 `Prepare` 메시지를 수신하면 네트워크의 다른 노드에 `Commit` 메시지를 보낸다.  
5. Reply: replicas들이 2/3의 replicas들로부터 commit 메시지를 수신하면 블록이 커밋되었음을 나타내는 응답 메시지를 primary node로 보낸다.
   그러면 client는 노드 quorum의 응답을 기다린다.

합의 프로세스는 네트워크의 노드들이 비잔틴 결함이 있는 경우에도 모든 노드가 동일한 상태라면 동의하도록 설계되었다.

특수한 상황을 가정해보자.
네트워크에 있는 노드의 67%가 동일한 결과에 동의하지만 redundancy들이 동의하지 않는 경우 이는 합의 프로세스에 문제가 있음을 나타낸다.
일부 노드가 악의적으로 동작하거나 PBFT 알고리즘 구현에 버그 또는 오류가 있을 수 있다.

이러한 시나리오에서는 문제를 조사하고 해결하기 위한 단계를 수행해야 할 수 있다.
- 한 가지 유효한 접근 방식은 현재 primary 노드가 악의적으로 행동하거나 프로토콜을 따르지 않는 것으로 의심되는 경우
  네트워크가 새 primary 노드로 전환할 수 있도록 하는 PBFT의 메커니즘인 "view change"를 수행하는 것이다.  
  view change 중에 네트워크는 새로운 primary 노드를 선택하고 합의 프로세스의 새로운 라운드를 시작한다.
  새로운 primary 노드가 정직하고 프로토콜을 따르는 경우 네트워크는 합의에 도달하고 공유 상태에 동의할 수 있다.
- 또 다른 접근 방식은 내결함성을 높이고 잘못된 합의 가능성을 줄이기 위해 네트워크에 추가 redundancy를 도입하는 것이다.
  여기에는 네트워크에 더 많은 노드를 추가하거나 Sharding과 같은 기술을 사용하여 네트워크를 더 작은 하위 네트워크로 나누는 것이 포함될 수 있다.

#### PBFT Message Flow
PBFT 메시지 flow는 다음과 같다.

1. 클라이언트는 primary 노드에 요청을 보낸다.
2. primary 노드는 요청에 시퀀스 번호를 할당하고 백업 노드로 broadcast한다.
3. backup 노드는 요청을 확인하고 네트워크의 다른 노드에 "Pre-prepare" 메시지를 보낸다.
4. 나머지 노드들은 Pre-prepare 메시지를 확인하고 "prepare" 메시지를 네트워크의 다른 노드로 보낸다.
5. 노드가 노드의 2/3로부터 "prepare" 메시지를 수신하면 네트워크의 다른 노드에 "commit" 메시지를 보낸다.
6. 나머지 노드들은 commit 메시지의 유효성을 검사하고 ledger에 요청을 추가한다.

cryptographic 메카니즘을 사용하면 메시지가 인증되고 악의적인 행위자가 노드를 조작할 수 없도록 한다.

#### PBFT Security and Fault Tolerance
PBFT는 네트워크에서 암호화 메커니즘과 redundancy를 사용하여 강력한 보안 및 내결함성을 제공한다.
네트워크 노드의 최대 1/3이 비잔틴일 수 있다고 가정하면 PBFT는 결함이나 악의적인 동작이 있는 경우에도 계속해서 올바르게 작동할 수 있다.
그러나 PBFT에는 리소스 요구 사항(추가적인 redundancy 요구) 및 중앙 집중화 가능성(primary node에게 상당한 권한과 책임이 있다)과 같은 제한 사항과 취약점도 있다.

#### PBFT Performance Characteristics
PBFT는 분산 시스템에 강력한 내결함성과 합의를 제공하도록 설계되었다.
짧은 대기 시간과 높은 처리량, 확장성을 비롯한 크고 복잡한 네트워크가 있는 고성능 애플리케이션에 적합하다.
PBFT는 분산 시스템에서 성능과 확장성을 희생하지 않으면서 내결함성과 합의를 제공하도록 설계되었기 때문이다.

PBFT는 노드가 여러 요청을 병렬로 처리할 수 있는 파이프라이닝이라는 메커니즘을 사용하기 때문에 상대적으로 오버헤드가 적다.
PBFT에서 각 노드는 일련의 단계에서 들어오는 요청을 처리하며 각 단계는 합의 프로세스의 다른 단계를 나타낸다.
이를 통해 노드는 여러 작업을 동시에 수행할 수 있으며 각 요청에 필요한 전체 처리 시간을 줄이는 데 도움이 될 수 있다.

파이프라이닝은 블록체인 네트워크에서는 validator, 즉 각각의 노드들이 병렬로 각자의 연산을 수행하고
파이프라인을 이어서 터치하기 때문에 중앙 네트워크는 많은 연산을 홀로 처리할 필요가 없기 때문에 오버헤드가 적다.
이는 각 노드가 다른 노드가 작업을 완료할 때까지 기다리지 않고 동시에 여러 작업을 수행할 수 있으므로
합의 프로세스가 큰 오버헤드를 차지하지 않는다.

또한 PBFT는 계산상 효율적일 수 있는 메시지 유효성 검사를 위해 암호화 메커니즘을 사용한다.
PBFT는 표준 암호화 알고리즘과 메시지 유효성 검사 기술을 사용하여 합의 프로세스에 추가 오버헤드 또는 계산 복잡성을 도입하는 것을 방지할 수 있다.
여기서 말하는 추가적인 오버헤드 및 계산 복잡성을 방지한다는 뜻은 식별되지 않은 노드 및 악의적인 노드가 합의 프로세스에
접근하지 못하도록 막는다는 뜻이다. 즉 기술적인 효율성보다는 악의적인 공격에 대한 저항성을 의미한다.

PBFT는 높은 성능과 확장성을 저해하지 않으면서 Fault Tolerance를 제공하도록 설계되었지만,
실제로는 PBFT 기반 시스템의 확장성은 네트워크 토폴로지, 메시지 크기, 노드 처리 능력 및 기본 하드웨어 인프라를 비롯한 여러 요인에 따라 달라지기 때문에 
이와 같은 특성을 항상 달성할 수 있는 것은 아니다. 그렇기 때문에 기본 아키텍처와 인프라를 신중하게 설계하고 최적화 해야한다.

### Key features of PBFT
다음은 PBFT의 주요 keyword에 대한 내용의 요약본이다.

1. Fault tolerence: PBFT는 네트워크에 있는 노드의 최대 1/3이 결함이 있거나 악의적일 수 있다고 가정하여 분산 시스템에서 강력한 내결함성을 제공하도록 설계되었다.
2. Consensus: PBFT는 분산 시스템에서 합의를 달성하기 위한 메커니즘을 제공하여 모든 노드가 동일한 결과에 동의하도록 한다.
3. Low latency and high throughput: PBFT는 낮은 대기 시간과 높은 처리량을 저해하지 않도록 설계되어 고성능 애플리케이션에 적합하다.
4. Scalability: PBFT는 확장 가능하며 크고 복잡한 네트워크를 처리할 수 있다.
5. Security: PBFT는 메시지 검증을 위해 암호화 메커니즘을 사용하여 악의적인 공격으로부터 보호하고 합의 프로세스의 무결성을 보장한다.

## Understanding PBFT's Components
### Roles of nodes in PBFT
PBFT에는 클라이언트, redundency 및 primary node의 세 가지 유형의 노드가 있다.
- client: 네트워크에 요청을 제출하는 외부 user이다.
- redundency(replica): 시스템의 현재 상태 사본을 저장하고 합의 프로세스에 참여하는 노드이다.
- primary node: 합의 프로세스를 조정하고 모든 replicas가 동일한 결과에 동의하도록 한다(잠재적 중앙 집권 위험).

### consensus rounds
PBFT는 각각 여러 단계로 구성된 일련의 라운드를 통해 합의를 달성한다.
각 라운드 동안 primary 노드는 모든 replicas에 요청을 보낸 다음 요청을 처리하고 다시 primary 노드로 응답을 보낸다.
primary node가 충분한 응답을 받으면 요청을 시스템 상태로 commit하기 위해 모든 replica에 메시지를 보낼 수 있다.

### message types and format
PBFT는 `Pre-prepare`, `prepare`, `commit` 및 `view change` 메시지를 포함하여 여러 유형의 메시지를 사용한다.
이러한 메시지는 특정 형식을 가지며 처리 중인 요청 및 프로세스에 관련된 노드와 같은 합의 프로세스의 현재 상태에 대한 정보를 포함한다.

### message flow
PBFT의 메시지 흐름은 합의를 달성하고 내결함성을 보장하도록 일련의 sequence flow 규칙이 정해져 있다.([PBFT Message Flow](https://github.com/datactor/Rustic-data-solving/blob/main/forge/distributed_data_management/pbft.md#PBFT-Message-Flow) 참고)

### view changes
PBFT에서는 primary node가 실패하거나 악의적이 되더라도 합의 프로세스가 계속될 수 있도록 `view change`가 사용된다.
`view change` 중에 replicas들은 새로운 primary node를 선출하여 합의 프로세스를 조정하고 모든 replicas가 동일한 결과에 동의하도록 할 수 있다.

## Benefits of PBFT
### Achieving Fault Tolerance Without Sacrificing Performance and Scalability
PBFT의 주요 이점 중 하나는 분산 시스템에서 성능과 확장성을 희생하지 않고 내결함성을 달성할 수 있게 설계되었다.
PBFT는 네트워크의 각 노드가 a copy of the system state를 유지하고 합의 프로세스에 참여하여
모든 노드가 현재 상태에 동의하는지 확인하는 replicated state machine architecture를 사용하여 내결함성을 달성한다.
이를 통해 합의 프로세스는 모든 노드가 동일한 상태에 동의하도록 보장하기 때문에 비잔틴 결함이나 악의적인 노드가 있는 경우에도 시스템이 계속 작동할 수 있다.

동시에 PBFT는 성능과 확장성을 매우 강조한다.
예를 들어 PBFT는 파이프라이닝과 병렬 처리를 사용하여 각 요청에 필요한 처리 시간을 최소화한다.
이를 통해 노드는 다른 노드가 작업을 완료할 때까지 기다리지 않고 여러 작업을 동시에 수행할 수 있다.
또한 PBFT의 메시지 유효성 검사 메커니즘은 합의 프로세스에 불필요한 오버헤드나 복잡성을 도입하지 않도록 안전하게 설계되었습니다.

### Robustness against attacks
PBFT는 비잔틴 오류 및 악의적인 노드가 있는 경우에도 강력한 보안 및 내결함성을 제공한다.
네트워크에서 암호화 메커니즘과 redundency를 사용함으로써 PBFT는 공격에 저항하고 합의 프로세스가 쉽게 손상되지 않는다.

### Ease of implementation and maintenance
PBFT는 분산 시스템에서 구현 및 유지 관리가 상대적으로 쉽다.
합의 알고리즘은 잘 정의되고 표준화되어 있으며 PBFT를 새로운 시스템에 통합하는 데 사용할 수 있는 기존 구현 및 라이브러리가 있다([tendermint](https://crates.io/crates/tendermint)).
또한 PBFT에는 복잡하거나 특수한 하드웨어나 소프트웨어가 필요하지 않으므로 광범위한 개발자와 조직이 액세스할 수 있다.

## Limitations of PBFT
### High resource requirements
PBFT는 내결함성을 달성하기 위해 상당한 양의 replicas들이 필요하므로 증가된 네트워크 대역폭 및 처리 능력과 같은 높은 리소스 요구 사항이 발생할 수 있다.
이로 인해 리소스가 제한된 환경이나 저전력 장치에서 PBFT를 구현하는 것이 어려울 수 있다.

### Limited fault tolerance
PBFT는 네트워크에 있는 노드 중 1/3 이하만 Byzantine 노드일 경우에만 유효하다.
노드의 1/3 이상이 Byzantine 노드인 경우 합의 프로세스가 실패할 수 있으며 노드가 공통 상태에 동의하지 못할 수 있다.
또한 PBFT는 primary node를 대상으로 하는 공격과 같은 특정 유형의 공격에 취약하므로 완전한 내결함성을 제공하지는 않는다.

### Centralization risk
PBFT에서 primary node는 작업 순서를 선택하고 노드가 일치하도록 보장하기 때문에 합의 프로세스에서 중요한 역할을 한다.
이 경우 primary node가 단일 실패 지점이 되거나 공격 대상이 될 수 있으므로 중앙 집중화 위험이 발생할 수 있다.
또한 특정 노드가 다른 노드보다 더 자주 선택되거나 선호될 수 있으므로 primary node의 선택이 완전히 분산되지 않을 수 있다.

PBFT 기반 시스템에서 중앙 집중화의 잠재적 위험을 해결하려면 더 높은 수준의 분산화를 제공하는 대체 합의 알고리즘을 고려해야 할 수 있다.
PBFT의 대안으로 간주될 수 있는 몇 가지 대체 합의 알고리즘이 있다. 일부 인기 있는 항목은 다음과 같다.
1. 지분 증명(PoS): 검증자가 네트워크에서 보유하고 있는 코인의 수에 따라 선택되는 합의 알고리즘.
   PoW보다 에너지 효율적이며 더 높은 수준의 분산화를 허용한다.
2. Raft: 리더 선택과 로그 복제 단계로 나누어 합의 프로세스를 단순화하는 합의 알고리즘이다.
   이해하고 구현하기 쉬우며 많은 생산 시스템에서 사용된다.
3. 권한 증명(PoA): 네트워크를 유지하기 위해 신뢰할 수 있는 고정된 검증자 집합에 의존하는 합의 알고리즘이다.
   참가자들 사이에 높은 수준의 신뢰가 존재하는 상황에서 유용하다.

다음은 기술적인 대안들이다.
1. Sharding: 네트워크를 horizontally 분할하고 여러 샤드 또는 하위 네트워크 간에 트랜잭션 처리를 분산하는 데 사용되는 기술이다.
   이 접근 방식은 네트워크의 내결함성을 높이고 중앙 집중화 가능성을 줄일 수 있다.
2. 다자간 계산(Multi-party computation): 중앙 집중화 위험을 줄이는 데 사용할 수 있는 또 다른 기술이다.
   MPC를 사용하면 여러 당사자가 서로에게 입력 내용을 공개하지 않고 함수를 공동으로 계산할 수 있다.
   이는 여러 노드가 서로에게 민감한 정보를 공개하지 않고 공동 작업을 수행해야 하는 분산 시스템에서 유용할 수 있다([Millionaires' problem](https://en.wikipedia.org/wiki/Yao%27s_Millionaires%27_problem)).
3. 이러한 기술 외에도 로드 밸런서, 방화벽 및 침입 탐지 시스템과 같은 추가 기술을 사용하여 내결함성을 높이고 중앙 집중화 가능성을 줄일 수 있다.

## Implementations of PBFT
```rust
use tendermint_p2p::{Block, Vote};
use tendermint::{block::Height, consensus::Proposal, hash::Hash};

// Define the necessary data structures for PBFT
struct PbftBlock {
    height: Height,
    hash: Hash,
    proposal: Proposal,
    votes: Vec<Vote>,
}

struct PbftState {
    sequence_number: u64,
    current_block: Option<PbftBlock>,
}

impl PbftState {
    fn get_proposal(&self) -> Option<Proposal> {
        // Returns the proposal of the current block
        self.current_block.as_ref().map(|block| block.proposal.clone())
    }

    fn get_votes_count(&self) -> usize {
        // Returns the number of votes for the current block
        self.current_block.as_ref().map_or(0, |block| block.votes.len())
    }

    fn get_valid_votes(&self) -> Vec<Vote> {
        // Returns a vector containing the valid votes for the current block
        let mut valid_votes = vec![];
        if let Some(current_block) = &self.current_block {
            let votes_needed = 2 * current_block.votes.len() / 3;
            let mut count_map = std::collections::HashMap::<[u8; 20], usize>::new();
            for vote in &current_block.votes {
                let vote_bytes = bincode::serialize(vote).unwrap();
                let count = count_map.entry(vote.hash()).or_insert(0);
                *count += 1;
            }
            for vote in &current_block.votes {
                let vote_bytes = bincode::serialize(vote).unwrap();
                if count_map.get(&vote.hash()).unwrap_or(&0) >= &votes_needed {
                    valid_votes.push(vote.clone());
                }
            }
        }
        valid_votes
    }
    
    // Implement the PBFT consensus algorithm
    fn process_block(&mut self, block: Block) -> Result<(), ()> {
        let pbft_block = PbftBlock::from_block(block);
        if self.current_block.is_none() || pbft_block.height > self.current_block.as_ref().unwrap().height {
            self.current_block = Some(pbft_block.clone());
            self.sequence_number = 0;
            self.broadcast_preprepare(pbft_block.clone());
        }
        Ok(())
    }

    fn broadcast_preprepare(&mut self, block: PbftBlock) {
        // Broadcast the `pre-prepare` message to all nodes in the network
        let msg = format!("pre-prepare:{:?}", block);
        // send the message to all other nodes in the network
        // ...
        self.process_preprepare(msg);
    }

    fn process_preprepare(&mut self, msg: String) {
        // Validate the `pre-prepare` message and send a `prepare` message to other nodes
        let pbft_block = PbftBlock::from_string(msg)?;
        if pbft_block.height == self.current_block.as_ref().unwrap().height &&
            pbft_block.hash == self.current_block.as_ref().unwrap().hash &&
            pbft_block.proposal == self.current_block.as_ref().unwrap().proposal {
            // The `pre-prepare` message is valid, send a `prepare` message to other nodes
            let msg = format!("prepare:{:?}", pbft_block);
            // send the message to all other nodes in the network
            // ...
            self.process_prepare(msg);
        }
        Ok(())
    }

    fn process_prepare(&mut self, msg: String) {
        // Validate the `prepare` message and send a `commit` message to other nodes
        let pbft_block = PbftBlock::from_string(msg)?;
        if pbft_block.height == self.current_block.as_ref().unwrap().height &&
            pbft_block.hash == self.current_block.as_ref().unwrap().hash &&
            pbft_block.proposal == self.current_block.as_ref().unwrap().proposal {
            // The `prepare` message is valid, add it to the list of votes
            self.current_block.as_mut().unwrap().votes.push(pbft_block.votes[0].clone());
            if self.current_block.as_ref().unwrap().votes.len() == 2 / 3 {
                // We have enough votes, broadcast the `commit` message to other nodes
                let msg = format!("commit:{:?}", self.current_block.as_ref().unwrap());
                // send the message to all other nodes in the network
                // ...
                self.process_commit(msg);
            }
        }
        Ok(())
    }

    fn process_commit(&mut self, msg: String) -> Result<(), ()> {
        // Validate the `commit` message and finalize the block if valid
        let pbft_block = PbftBlock::from_string(msg)?;
        if pbft_block.height == self.current_block.as_ref().unwrap().height &&
            pbft_block.hash == self.current_block.as_ref().unwrap().hash &&
            pbft_block.proposal == self.current_block.as_ref().unwrap().proposal {
            // The `commit` message is valid, finalize the block
            self.current_block = None;
            self.sequence_number += 1;
        }
        Ok(())
    }
}
```
위의 구현은 PbftBlock struct 및 PbftState struct를 포함하는 PBFT(Practical Byzantine Fault Tolerance) 합의 알고리즘에 필요한 데이터 구조의 basic 버전을 정의한다.
PbftBlock struct에는 블록의 height, hash, proposal 및 vote를 필드로 갖는다.
또한 sequence_number 및 선택적 PbftBlock인 current_block이 포함되어 있다.

PbftState struct는 현재 블록의 proposal을 반환하는 get_proposal을 포함하여 PBFT 합의 알고리즘에 대한 여러 메서드를 구현한다.
현재 블록에 대한 투표 수를 반환하는 get_votes_count, get_valid_votes는 현재 블록에 대한 유효한 투표를 포함하는 벡터를 반환한다.

주요 PBFT 알고리즘(message flow)은 process_block, broadcast_preprepare, process_preprepare, process_prepare 및 process_commit 메서드에서 구현된다.
이러한 방법은 pre-prepare, prepare 및 commit 메시지를 포함하여 PBFT 알고리즘에서 일련의 메시지를 검증하고 처리한다.
이러한 방법은 또한 현재 블록에 투표를 추가하고 quorum 이상의 투표가 있을 때 블록을 마무리함으로써 PBFT 알고리즘의 상태를 업데이트한다.

위의 구현에서는 개선할 수 있는 몇가지 눈에 띄는 사항이 있다.

1. Lack of message broadcast: 현재 구현에서 broadcast_preprepare 및 process_prepare 메서드는 방법을 지정하지 않고 "네트워크의 다른 모든 노드"에 메시지를 보낸다.
   실제 구현에서 네트워크 계층은 모든 노드에 메시지를 브로드캐스팅해야 한다.
2. Lack of timeout handling: 구현에는 타임아웃을 처리하는 메커니즘이 없다. PBFT에서 타임아웃은 노드가 느리거나 결함이 있는 경우를 처리하는 데 사용되며 알고리즘이 올바르게 작동하는 데 필요하다.
3. Lack of view changes: 구현에는 primary 노드에 결함이 있거나 느린 경우를 처리하기 위해 PBFT에서 필요한 view change를 처리하는 메커니즘이 없다.
   실제 구현에서는 알고리즘이 견고하도록 view change를 구현해야 한다.
4. Lack of fault tolerance: 구현에는 결함이 있는 노드를 처리하는 메커니즘이 없다.
   실제 구현에서는 노드에 결함이 있거나 악의적인 경우에도 알고리즘이 올바르게 작동할 수 있도록 내결함성을 구현해야 한다.
5. 솔라나의 TowerBFT는 gossip 네트워크를 사용하여 메시지를 전달하는 반면
   위의 구현에서는 메시지가 네트워크의 모든 노드로 전송된다고 가정한다.

위의 사항은 실제 구현을 통해 처리하도록 하자.