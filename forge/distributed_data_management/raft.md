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
### 4.1. Raft vs. Paxos
Paxos는 1990년대 후반에 개발된 consensus 알고리즘이다. 이것은 제대로 이해하고 구현하기 어렵다는 이유로
대안으로 Raft가 개발되었다고 한다.
이 둘의 핵심적인 유사한 원칙은  Leader Election, Log Replication, Safety와 같은 분산 시스템에서 필요한 기본 개념에 있다.
하지만 세부적인 구현방식에서 차이가 있으며, 각각의 장단점이 있다.

- Understandability: Raft는 Paxos보다 이해하기 쉽도록 설계되었다. 두 알고리즘 모두 유사한 원칙을 기반으로 하지만,
  Raft의 설계는 프로세스를 단순화하고 추론하기 쉽게 만든다.
- Performance: Paxos는 일반적으로 특히 메시지 loss가 많거나, 높은 network latency 시나리오에서 Raft보다 성능이 뛰어나다.
  그러나 이로 인해 complexity가 증가한다.
- Fault Tolerance: 두 알고리즘 모두 Crash Fault Tolerance(CFT)가 있다. 즉 노드가 충돌하고 나중에 복구되는 오류를 처리할 수 있다.
  - CFT vs BFT?
    Crash fault tolerance (CFT)는 분산 시스템에서 노드가 갑자기 멈추는 등의 예기치 않은 동작이 발생해도 시스템이 정상적으로 동작하도록
    하는 방법을 말한다. 즉, CFT는 시스템의 일부 노드가 비정상적으로 종료되는 상황에서도 전체 시스템이 동작을 멈추지 않도록 하는 것이다.  
    반면, 다른 종류의 fault tolerance는 노드의 동작이 예상한 대로 수행되지 않는 경우를 다루는 것이다. 예를 들어, 노드가 부분적으로 동작하거나,
    메시지를 전송할 때 오류가 발생하는 경우 등이 있다. 이러한 상황에서는 CFT보다 더 많은 작업이 필요하다.  
    또한, 이러한 더 일반적인 fault tolerance 기술은 Byzantine fault tolerance (BFT)이라고도한다. BFT는 노드의 악의적인 동작,
    즉 노드가 악의적인 목적으로 잘못된 메시지를 전송하거나 다른 노드를 공격하는 경우에도 시스템이 동작을 멈추지 않도록 보장하는 방법을 말한다.
    요약하자면, CFT는 예상치 못한 동작으로 인한 시스템 장애를 방지하는 데 초점을 맞추고, BFT는 악의적인 동작으로 인한 장애를 방지하는 데 초점을 맞춘다.
  두 알고리즘 모두 노드가 임의의 동작을 나타낼 수 있는 Byzantine faults를 처리하지 않는다.
- Use cases: 두 알고리즘 모두 distributed database, cloud computing env에서 사용하기에 적합하다.

### 4.2. Raft vs. PBFT
PBFT는 이름에서부터 알 수 있듯이 Byzantine faults를 견딜 수 있도록 설계된 합의 알고리즘이다.

- Understandability: PBFT는 Raft보다 복잡하지만 Paxos보다 일반적으로 간단하다고 평가된다.
- Performance: PBFT는 low-latency 네트워크에서 고성능을 달성할 수 있다. 그러나 네트워크 latency가 증가하면
  성능이 저하된다.
- Use cases: PBFT는 높은 수준의 security와 fault tolerance를 요구하는 distributed ledgers, blockchain systems,
  critical infrastructure systems들에서 일반적으로 사용된다.

### 4.3. Comparison Table: Raft, Paxos, and PBFT
| Consensus Algorithm | Understandability | Performance                      | Fault Tolerance  | Use Cases                                                         |
|---------------------|-------------------|----------------------------------|------------------|-------------------------------------------------------------------|
| Raft                | High              | Moderate                         | CrashFault       | Distributed databases, configuration management, key-value stores |
| Paxos               | Low               | High                             | Crash Fault      | Distributed databases, distributed file systems, cloud computing  |
| PBFT                | Moderate          | High (in low-latency networks)   | Byzantine Fault  | 	Distributed ledgers, blockchain, critical infrastructure systems |

## 5. Implementing Rust Raft in Real-world Scenarios
### 5.1. Key-Value Store
std::collections::HashMap은 단일 머신에서 실행되는 메모리 기반의 Key-Value 데이터 구조로,
일반적으로 빠른 읽기 및 쓰기 성능을 가지고 있다. 이러한 HashMap은 단일 노드에서만 작동하며, 노드 간의 분산 작업을 처리하지 않는다.

반면에, Raft를 사용하여 구현한 분산 Key-Value store는 여러 노드에서 실행되는 분산 시스템으로, 데이터가 여러 노드에 분산 저장되어 있다.
이러한 분산 시스템에서 Raft 알고리즘은 Leader 선출 및 분산된 데이터의 복제와 일관성을 유지하기 위해 필요한 작업을 수행한다.

따라서, Raft를 사용한 분산 Key-Value store는 높은 가용성과 확장성을 제공할 수 있으며,
시스템 장애 및 노드 장애에 대한 내결함성(fault tolerance)도 가지고 있다.

반면, std::collections::HashMap은 단일 노드에서만 작동하며, 노드 간 통신, 복제 및 데이터 일관성을 처리하지 않는다.
성능 측면에서는, 분산 Key-Value store에서는 Raft 알고리즘의 Overhead(일관성을 유지하기 위한 데이터 복제, 로그 쓰기 및 검증,
Leader 선출 및 클러스터 노드 간 통신 등)로 인해 더 느리다.
또한, 데이터가 여러 노드에 분산되어 있기 때문에, 특정 키에 대한 읽기 작업이 네트워크 지연으로 인해 더 많은 시간이 걸릴 수 있다.
그러나 높은 가용성과 내결함성(fault tolerance)을 제공하는 이점은 특정 시나리오에서 이러한 성능 이슈를 상쇄할 수 있다.

다음은 distributed key-value store를 구현하는 간단한 예이다.

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::RwLock;
use std::sync::Arc;

/// 1. Define the state machine and its operations:
#[derive(Debug, Serialize, Deserialize)]
pub enum StateMachineCmd {
    Put { key: String, value: String },
    Get { key: String },
    Delete { key: String },
}

/// 2. implement the state machine
pub struct KeyValueStore {
    store: HashMap<String, String>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        KeyValueStore {
            store: HashMap::new(),
        }
    }

    pub fn apply_cmd(&mut self, cmd: StateMachineCmd) -> Option<String> {
        match cmd {
            StateMachineCmd::Put { key, value } => {
                self.store.insert(key, value);
                None
            }
            StateMachineCmd::Get { key } => self.store.get(&key).cloned(),
            StateMachineCmd::Delete { key } => self.store.remove(&key),
        }
    }
}

/// 4. Create a config.rs file for Raft node configuration
pub struct NodeConfig {
  pub id: u64,
  pub addr: SocketAddr,
}

pub fn get_config() -> Vec<NodeConfig> {
  vec![
    NodeConfig {
      id: 1,
      addr: "127.0.0.1:9001".parse().unwrap(),
    },
    NodeConfig {
      id: 2,
      addr: "127.0.0.1:9002".parse().unwrap(),
    },
    NodeConfig {
      id: 3,
      addr: "127.0.0.1:9003".parse().unwrap(),
    },
  ]
}

/// 5. 클라이언트 요청을 Raft 리더에게 전달하기 위한 논리를 구현. 이 단계에는 Raft 리더에게 요청을 보내기 위한 클라이언트 인터페이스 생성이 포함된다.
/// 그런 다음 리더는 명령을 follower에게 복제한다. 명령이 커밋되면 상태 시스템에 적용할 수 있다.
async fn handle_client_request(
    raft: &raft::Raft<StateMachineCmd>,
    state_machine: &mut KeyValueStore,
    cmd: StateMachineCmd,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    // Propose the command to the Raft cluster.
    let proposal = bincode::serialize(&cmd)?;
    let response = raft.send_command(proposal).await?;

    // If the command was committed, apply it to the state machine.
    if response.committed {
        Ok(state_machine.apply_cmd(cmd))
    } else {
        Err("Command not committed".into())
    }
}
```
