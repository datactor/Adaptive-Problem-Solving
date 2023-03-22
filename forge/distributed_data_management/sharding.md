# Sharding

## Intro
Sharding은 분산 데이터베이스에서 데이터를 여러 서버 또는 노드에 분할하여 주어진 resource로
데이터베이스의 horizontal-scailing(scale-out)을 가능하게 하는 데 사용되는 기술이다.
데이터를 더 작고 관리하기 쉬운 조각으로 나누고 여러 서버에 분산함으로써 sharding은 시스템의 성능,
availability 및 scalability를 향상시킬 수 있다.

sharding된 데이터베이스에서 각 shard는 전체 dataset의 subset을 저장하는 역할을 하며
각 shard는 다른 서버나 노드에서 관리할 수 있다. 데이터를 찾고 액세스하기 위해 sharding된 db는
데이터의 특정 특성 또는 속성을 기반으로 데이터를 특정 shard에 매핑하는 shard key를 사용한다.

## How Sharding Works
1. 샤딩은 큰 데이터 세트를 샤드라고 하는 더 작고 관리하기 쉬운 조각으로 분할합니다. 각 샤드는 일반적으로 데이터의 하위 집합을 포함하며 별도의 서버 또는 노드에 저장됩니다. 효과적인 샤딩의 핵심은 성능과 확장성을 최적화하는 방식으로 데이터를 분할하는 것입니다. 이는 지리적 위치, 사용자 활동 또는 데이터 유형과 같은 다양한 요소를 기반으로 수행할 수 있습니다.
2. 데이터가 샤드로 분할되면 여러 서버 또는 노드에 분산됩니다. 각 샤드는 일반적으로 중복성과 가용성을 보장하기 위해 여러 서버에 복제됩니다. 즉, 한 서버에 장애가 발생해도 다른 서버에서 데이터에 계속 액세스할 수 있습니다. 데이터를 분산하기 위해 샤딩 시스템은 일관된 해싱 또는 범위 분할과 같은 다양한 기술을 사용하여 각 샤드가 서버 전체에 고르게 분산되도록 합니다.
3. 샤딩된 시스템의 데이터에 액세스하는 것은 기존 데이터베이스보다 더 복잡할 수 있습니다. 클라이언트가 데이터를 요청할 때 시스템은 먼저 데이터가 저장된 샤드를 결정해야 합니다. 여기에는 일반적으로 메타데이터 저장소 또는 인덱스를 쿼리하여 데이터 위치를 조회하는 작업이 포함됩니다. 샤드가 식별되면 클라이언트는 적절한 서버에 쿼리하여 데이터를 검색할 수 있습니다. 일부 샤딩 시스템은 캐싱 또는 로드 밸런싱을 사용하여 성능을 개선하고 데이터에 액세스하는 데 필요한 쿼리 수를 줄입니다.

## The Benefits of Sharding
- Describe how sharding can improve scalability
- Explain how it can improve availability and reliability
- Outline how it can improve system performance

## The Challenges of Sharding
- Describe some of the challenges that come with sharding, such as data consistency and management
- Explain how Solana addresses these challenges
- Outline some of the trade-offs that come with sharding, such as increased complexity and reduced flexibility

## Sharding in Blockchain DB
Sharding을 사용하는 솔라나의 DB를 살펴보자.
솔라나는 고성능 분산형 블록체인 플랫폼으로 샤딩을 사용하여 높은 처리량과 낮은 트랜잭션 지연을 가능하게 한다.
Solana에서 샤딩은 "account sharding"이라는 기술을 사용하여 구현된다.

솔라나의 account sharding은 트랜잭션 정보가 포함된 계정을 더 작은 하위 집합으로 분할하고 이를
여러 노드에 분산시키는 작업을 포함한다. 솔라나 네트워크의 각 노드는 총 계정의 하위 집합을 관리할
책임이 있다. 이를 통해 Solana는 네트워크에 많은 수의 노드가 있는 경우에도 높은 트랜잭션 처리량과
낮은 대기 시간을 달성할 수 있다.

account 정보를 찾고 액세스하기 위해 Solana는 Pubkey와 해시 값의 조합을 사용한다.
Pubkey는 account를 식별하는 데 사용되며 해시 값은 account 관리를 담당하는 네트워크의 노드를 결정하는 데 사용된다.

솔라나의 계정 sharding은 확장성 향상(scale-out), 자원 활용도 향상, 보안 강화 등 몇 가지 장점이 있다.
그러나 복잡성 증가 및 shard key의 신중한 관리 필요성과 같은 몇 가지 문제도 발생한다.

전반적으로 샤딩은 분산 데이터베이스의 sharding은 확장성을 향상시키는 중요한 기술이며,
솔라나에서 sharding을 사용하면 탈중앙화 네트워크에서 높은 트랜잭션 처리량과 짧은 대기 시간을 달성하는 데 효과가 있음을 알 수 있다.

## Conclusion
- Summarize the main points of the document
- Provide some final thoughts on sharding in Solana and large-scale systems in general