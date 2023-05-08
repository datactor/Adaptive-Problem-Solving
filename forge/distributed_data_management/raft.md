# Rust Raft

## 1. Introduction
### 1.1. Overview of Rust Raft
Rust Raft는 strong consistent를 유지하면서 node failure 및 네트워크 파티션을 허용할 수 있는 분산 시스템을 구축하기 위한 프레임워크를 제공한다.
Raft 알고리즘의 모듈식 구현을 제공하여 분산 응용 프로그램의 개발 프로세스를 간소화하고 확장 가능한 구성 요소와 configurable parameters를 제공한다.

### 1.2. Background on Raft Consensus Algorithm
Raft는 Paxos 및 PBFT와 같은 기타 합의 알고리즘의 complexity와 구현 문제를 해결하기 위해 도입되었다.
이해하기 쉽고, 단순성에 중점을 둔 Raft는 클러스터에 있는 대부분의 노드에 결함이 없다는 가정하에 작동한다.
leader-based 접근 방식을 통해 합의를 달성하며, leader는 follower 노드 전체에서 로그 복제 프로세스를 관리할 책임이 있으므로
결국 모든 노드가 동일한 operations order에 동의하는 것을 보장한다.

Raft의 주요 technical attributes:
1. Leader Election: Raft는 randomized timers와 투표 프로세스를 통해 leader를 선출하므로, 한 번에 하나의 leader만 활성화 된다.
2. Log Replication: Leader는 로그에 새 entries를 append하고 이를 follower에게 복제한다. follower는 etries 수신을 확인하고 상태 머신에 적용한다.
3. Log Consistency: Raft는 새 항목을 수락하기 전에 indexes와 follower's logs가 leader's logs와 일치하는 것을 보장하기 위해 log 일관성을 강화시킨다. 
4. Cluster Membership changes: Raft는 2단계 configuration 변경 프로세스를 통해 클러스터에서 노드의 동적 추가 제거를 지원하여 안전성과 가용성을 유지한다.

### 1.3. Rust Language and its Benefits for Distributed Systems
Rust는 분산 시스템을 구축할 때 다음과 같은 주요 기술 속성을 통해 몇 가지 이점을 제공한다.

성능: Rust의 효율적인 메모리 관리, zero-cost abstractions 및 assembly level의 제어를 통해 개발자는 높은 처리량과 낮은 대기 시간을 위해 애플리케이션을 최적화할 수 있다.
안전성: type system과 함께 ownership, borrowing, lifetime과 같은 Rust의 memory safe features는 data race, null pointer dereferences, buffer overflows를 포함하여
분산 시스템에서 일반적인 오류를 방지하는 데 도움이 된다.
동시성: 동시성, async/await 및 병렬성에 대한 Rust의 내장 지원은 fearless 동시성 모델과 함께 동시 및 병렬 코드의 개발을 단순화한다.
생태계: Raft consensus 알고리즘, serialization 라이브러리 및 네트워킹 프레임워크의 다양한 구현을 포함하여
Rust에서 사용 가능한 분산 시스템 구축 라이브러리 및 도구 모음들이 늘고 있다.

Rust의 이점과 Raft 합의 알고리즘의 단순성을 활용함으로써 Rust Raft는 효율적이고 안전하며 신뢰할 수 있는 분산 시스템을 개발하기 위한
강력한 플랫폼을 제공하는 동시에 특정 요구 사항에 맞게 조정할 수 있는 유연성을 제공한다.

## 2. Rust Raft Core Concepts
### 2.1. Nodes and Servers
Raft cluster는 서로 합의를 달성하기 위해 서로 통신하는 여러 노드(서버)로 구성된다.
각 노드는 다음 세 가지 상태 중 하나일 수 있다.
1. Leader: log entries 복제 관리 및 client requests 처리를 담당하는 노드이다. Raft cluster에는 한 번에 하나만 존재할 수 있다.
2. Follower: leader의 instructions를 수동적으로 따르고, log entries를 복제하는 노드이다. 
3. Candidate: leader노드가 실패했다고 판단될 때 leadership을 위해 실행 중인 노드이다.

노드들은 RPCs(Remote Procedure Calls)를 통해 통신하고 정보를 교환하며 consensus를 유지한다.

### 2.2. Leader Election
leader election은 현재 leader가 실패하거나 cluster 초기화 중에 새로운 리더가 선택되는 과정이다.
Raft는 무작위 timer와 투표를 통해 leader를 선출한다.  

follwer가 leader로부터 정해진 시간안에 어떠한 통신도 받지 못하면, follower는 후보 상태로 전환되고 선거가 시작된다.
이것은 leader가 의심스러운 상태로 여겨지기 때문에 교체 프로세스를 준비한다는 것을 뜻하지만, 네트워크나 follower의 문제일 가능성도 존재한다.
follower가 문제일 가능성도 있는데 그러한 follower를 후보로 올리는 이유는 voting과정 중에 다른 노드와의 투표 요청 통신이 필요하기 때문에
follower의 통신 불량이라면, 다른 노드와의 통신에 실패할 가능성이 높아 리더로 선출될 가능성이 매우 낮아진다.
Raft 알고리즘은 다수결 원칙에 따라 leader를 선출하므로, 클러스터의 과반수 노드가 정상적으로 작동하고 소통할 수 있다면,
선거 과정에서 문제가 있는 follower가 leader로 선출되는 것을 방지할 수 있다.  
이는 Raft 알고리즘이 안정성과 가용성을 유지하는데 중요한 역할을 하는 이유이기도 하다

### 2.3. Log Replication
leader는 새 log entries를 log에 추가하고 모든 follower nodes에 복제한다.
followers들은 entries 수신을 확인하고 그들의 state machines에 적용한다.
log 복제는 모든 노드가 같은 log 항목을 갖고 해당 state machines가 동일한 상태에 도달하도록 보장한다.

### 2.4. Log Compaction and Snapshotting
log가 커짐에 따라 로그를 압축하고 그들을 더 효율적인 포맷으로 상태를 저장해야 한다.
Log compaction은 필요한 저장공간을 줄이고 시스템의 성능을 향상시킨다.
Snapshotting은 state machine의 현재 상태에 대한 snapshot을 만드는 프로세스로,
새 노드의 시작점으로 사용하거나 장애 발생 시 복구 메커니즘으로 사용할 수 있다. 

### 2.5. Cluster Membership Changes
Raft는 클러스터에서 동적으로 노드를 추가하고 제거하는 것을 지원하여,
환경의 변화에 맞게 확장하고 적응할 수 있도록 한다.  
클러스터 멤버십 변경은 전황 중에 시스템의 안전과 가용성을 보장하는 2단계 프로세스를 사용한다.
Leader는 configuration 변경 프로세스를 관리하고 새로운 configuration이 적용되기 전에
모든 노드에 committed되고 복제되도록 한다.

## 3. Getting Started with Rust Raft
Raft consensus 알고리즘의 context에서 Raft의 components들이 Raft architecture에
어떻게 적용되는지에 대해 생각해보자.

### 3.1. Integrating Rust Raft with Raft Concepts
Rust Raft는 Raft architecture의 개념에 해당하는 몇가지 핵심 components와 함께 consensus 알고리즘 구현을 제공한다.

1. `Config`: Raft의 노드별 매개변수에 해당하는 Raft 노드의 configuration options를 나타낸다.
2. `Raft`: leader election의 구현, log replication 및 cluster membership 관리를 구현하는 Raft 노드를 나타낸다.
3. `Storage`: Raft의 로그 항목 및 메타데이터에 대한 저장소 인터페이스를 정의하여 데이터 저장소를 커스터마이징할 수 있다.
4. `StateMachine`: 분산시스템의 동작을 정의하는 Raft의 log 항목이 적용되는 상태 머신을 나타낸다.
5. `Transport`: Raft 노드에 대한 통신 인터페이스를 정의하여 메시지 전송 메커니즘을 사용자 정의할 수 있다.

이러한 components들이 Raft architecutre에 어떻게 맞춰지는지 이해함으로써, 분산 시스템에서 효과적으로 Raft를 활용하는 것을 목표로 하자.

### 3.2. Exploring Rust Raft's Features and Flexibility
Raft의 module식 디자인과 flexibility를 통해 다양한 사례와 요구사항에 맞게 조정할 수 있다.

- Custom Storage: in-memory starage, disk-based sorage 또는 distributed storage systems like Amazon S3 or Google Cloud Storage와
  같은 다양한 storage 엔진을 사용하도록 Storage trait을 implement한다.
- Custom State Machine: distributed system's 동작을 정의하고, app의 논리에 따라 commited log entries를 처리할 수 있도록 StateMachine trait을 구현한다.
- TCP, UDP, gRPC나 HTTP 같은 프로토콜 메커니즘을 사용하도록 Transport trait을 구현한다.

Rust Raft를 사용하면 Raft consensus 알고리즘의 단순함과 robustness를 누리면서 특정 요구사항에 맞게 구현을 자유롭게 조정할 수 있다.

## 4. Comparing Raft with Alternative Consensus Algorithms
| Consensus Algorithm | Understandability | Performance                      | Fault Tolerance  | Use Cases                                                         |
|---------------------|-------------------|----------------------------------|------------------|-------------------------------------------------------------------|
| Raft                | High              | Moderate                         | CrashFault       | Distributed databases, configuration management, key-value stores |
| Paxos               | Low               | High                             | Crash Fault      | Distributed databases, distributed file systems, cloud computing  |
| PBFT                | Moderate          | High (in low-latency networks)   | Byzantine Fault  | 	Distributed ledgers, blockchain, critical infrastructure systems |
