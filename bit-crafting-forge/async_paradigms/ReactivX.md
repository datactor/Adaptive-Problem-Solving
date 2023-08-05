# ReactiveX

## 1. Introduction

ReactiveX는 비동기 및 이벤트 기반 프로그램을 작성하기 위한 라이브러리이다. Observable 시퀀스를 사용하여 프로그램을 구성하는 데 필요한 도구를 제공한다. ReactiveX는 observer 패턴을 확장하여 데이터 및/또는 이벤트의 시퀀스를 지원하며, 이를 사용하면 low-level의 스레딩, 동기화, 스레드 안전성, 동시 데이터 구조, Non-blocking I/O와 같은 고려사항들을 추상화하여 선언적으로 시퀀스를 함께 구성할 수 있는 연산자를 추가해 준다.

## 2. Why ReactiveX?

Reactive Extensions은 말그대로, 라이브러리가 반응형 프로그래밍 패러다임에 기반을 두고 있음을 의미하며, `Extensions`는 이를 확장하여 비동기 데이터 스트림을 제어할 수 있는 강력한 도구 세트를 제공함을 의미한다.

`Reactive`는 시스템이 데이터의 변화나 이벤트에 반응하는 방식을 의미한다. 이는 `reactive programming` 개념에서 비롯되었는데, 이것은 주로 데이터 흐름과 이벤트의 전파를 중점으로 하는 프로그래밍 패러다임이다. 기본적으로 데이터의 변화가 전체 시스템에 전파되어 결과가 자동으로 업데이트되는 방식이다. 이는 비동기적이고 이벤트 기반의 시스템을 더 효율적이고 관리하기 쉽게 만들어 준다.

`Extensions`는 `ReactiveX`가 기존의 reactive programming 개념을 확장하고, 보다 강력하고 유연하게 만들어 준다는 것을 의미합니다. ReactiveX는 이 개념을 이용하여 비동기 데이터 스트림을 강력하게 제어하는 도구를 제공합니다.

따라서 `ReactiveX`라는 이름은 기본적으로 `반응형 프로그래밍의 확장`이라는 의미를 내포하고 있다.

때때로 `functional reactive programming`이라고 불리기도 하지만 이는 오해의 소지가 있다. `ReactivX`는 함수형이기도 하고, 반응형이기도 하지만, `functional reactive programming`은 완전히 다른 영역이다.가장 중요한 차이점 중 하나는 함수형 반응형 프로그래밍은 시간에 따라 지속적으로 변하는 선형적인 값들을 다루는 반면, ReactiveX는 시간에 따라 발생하는 개별적인 값들을 다룬다는 것이다. 즉 선형적이지 않다.

함수형 반응형 프로그래밍은 시간에 다라 연속적으로 변화하는 값을 처리한다. 예를 들어 시간에 따라 변화하는 특정 센서에서의 읽기 값이나 주가와 같은 데이터를 처리할 수 있다.
이러한 데이터 스트림은 시간에 따라 변화하는 값들의 연속적인 시퀀스로 볼 수 있으며, 이 값들은 시간에 따라 연속적으로 변화한다.

반면에 ReactiveX는 이산적인 값들을 다룬다. 이는 값이 특정 시점에 발생하는 개별적인 이벤트들로 구성되어 있음을 의미한다. 예를 들어 유저가 버튼을 클릭하거나 텍스트 필드에 입력을 하는 등의 행동은 각각 이벤트를 생성하며, 이러한 이벤트들은 시간에 따라 분산되어 발생한다. ReactiveX는 이러한 이벤드들을 감지하고 적절하게 반응하는 프로그래밍 패러다임이다.

요약하면 함수형 반응형 프로그래밍은 하나의 객체의 선형적인 변화를 감지할 때 유용하고,
ReactiveX는, 여러 객체가 이산적으로 생성되는 이벤트를 감지할 때 유용하다.

## Observables and Observers

`Observable`은 말 그대로 Observer 패턴에서 착안한 것으로, 이 패턴은 객체의 상태 변화를 관찰하고 이에 반응하는 객체를 모델링 하는 데 사용된다. `ReactiveX`에서는 `Observable` 개념을 사용해 이 패턴을 확장하였고, 이를 통해 비동기 데이터 스트림을 모델링하고, 이러한 데이터 스트림을 효율적으로 처리하는 기능을 제공하고 있다.

### 1) Observerble의 기능

`Observable`은 데이터의 스트림을 생성하고 발행하는 역할을 한다. 이 데이터 스트림은 0개 이상의 데이터 항목과 최종적으로 성공(success) 또는 실패(error)를 알리는 신호로 구성된다. `Observer`는 이러한 데이터 스트림을 구독(subscribe)하고, 데이터 항목이 발행될 때마다 특정 작업을 수행하게 된다.

### 2) Observable의 장점

비동기 처리: `Observable`은 비동기 이벤트 처리를 쉽게 만들어 준다. 이를 통해 복잡한 콜백 구조를 피하고, 비동기 이벤트를 마치 동기적인 `array`를 처리하는 것처럼 쉽게 다룰 수 있다.

합성 가능성: `Observable`은 다른 Observable들과 합성할 수 있다. 이를 통해 복잡한 비동기 로직을 쉽게 구현할 수 있다.

코드의 가독성과 유지 보수성: `Observable`을 사용하면, 비동기 코드를 더욱 선언적인 방식으로 작성할 수 있습니다. 이로 인해 코드의 가독성과 유지 보수성이 향상된다.

### 3) Observable의 단점

디버깅: `Observable`이 생성하는 비동기 흐름은 디버깅하기가 어려울 수 있다. 오류가 발생했을 때, 그 원인을 찾기 위해 전체 이벤트 체인을 추적해야 할 수도 있다.

### 4) Observable의 용도

`Observable`은 다음과 같은 다양한 용도로 사용될 수 있다.

- 비동기 이벤트 처리: `Observable`은 HTTP 요청, 타이머 이벤트, 사용자 입력 등과 같은 비동기 이벤트를 처리하는 데 유용하다.

- 복잡한 이벤트 로직 구현: `Observable`을 사용하면, 복잡한 이벤트 조건을 구현하는 데 유용하하. 예를 들어, 두 개 이상의 이벤트 스트림을 결합하거나, 특정 조건을 만족하는 이벤트만 필터링하거나, 이벤트 스트림의 특정 시간 동안의 이벤트를 모으는 등의 작업을 쉽게 수행할 수 있다.

- 백엔드 서비스 구축: `Observable`은 높은 성능의 백엔드 서비스를 구축하는 데 유용하다. 예를 들어, 여러 동시 요청을 처리하고, 각 요청에 대한 응답을 조율하는 데 사용될 수 있다.

요약하면, `ReactiveX`의 `Observable` 모델은 비동기 이벤트 스트림을 array와 같이 데이터 항목 컬렉션에 사용하는 간단하고 합성 가능한 작업과 같은 방식으로 처리할 수 있게 해준다. 이로 인해 콜백의 복잡한 구조에서 벗어나 코드의 가독성을 높이고 버그를 줄일 수 있다. 예를 들어, Java의 `Future`와 같은 기법은 비동기 실행의 단일 레벨에 대해 사용하기 쉽지만, 중첩될 때 betrivial한 복잡성이 추가된다. 반면에 `ReactiveX` `Observables`는 비동기 데이터의 흐름과 시퀀스를 구성하기 위한 것이다.

또한 `ReactiveX` `Observables`는 단일 스칼라 값만 발행하는 것이 아니라, 값의 시퀀스 또는 tokio의 `unbounded`와 같은 무한 스트림까지 지원한다. `Observable`은 이러한 모든 사용 사례에 사용할 수 있는 단일 추상화이다.

## 3. Why Use ReactiveX?

### 1) 복잡한 비동기 로직 처리

ReactiveX는 비동기 프로그래밍에 대한 복잡성을 줄이는 데 크게 도움이 된다. 예를 들어, 우리는 종종 네트워크 요청, 사용자 입력, 파일 I/O 등과 같은 여러 비동기 `Future`들을 동시에 처리해야 할 때가 있다. 이러한 `Task`들은 종종 다른 `Task`들과 의존성을 가지거나, 서로 다른 시간에 완료될 수 있으며, 이로 인해 코드는 복잡해지고 에러를 유발할 수 있다. `ReactiveX`를 사용하면, 이러한 작업들을 `Observable`` 시퀀스로 모델링하고, 비동기 이벤트를 마치 동기적인 collection을 다루는 것처럼 쉽게 처리할 수 있다.

예를 들어, 우리가 애플리케이션에서 여러 HTTP 요청을 처리해야 한다고 가정해보자. 이 요청들은 서로 의존적일 수도 있고, 각 요청이 완료되는 시간이 다를 수 있다. 전통적인 콜백 패턴을 사용하면, 이러한 복잡한 비동기 흐름을 관리하기가 매우 어려울 수 있다. 이를 `콜백 지옥`이라고 부르는데, 이는 중첩된 콜백으로 인해 코드가 불필요하게 복잡해지고 가독성이 떨어지는 상황을 말한다.

`ReactiveX`를 사용하면, 이러한 비동기 요청을 Observable 객체로 캡슐화할 수 있고, 이를 통해 비동기 이벤트를 마치 array의 요소를 다루는 것처럼 쉽게 처리할 수 있다.

```javascript
// Create an Observable that gets data from each URL
const request1$ = rxjs.ajax.getJSON('https://api.example.com/data1');
const request2$ = rxjs.ajax.getJSON('https://api.example.com/data2');

// Use Observable's `forkJoin` operator to wait for all requests to complete, then merge the results.
rxjs.forkJoin([request1$, request2$])
  .subscribe(([data1, data2]) => {
    console.log('Data1:', data1, 'Data2:', data2);
  });
```

### 2) 여러 이벤트 소스 조율

`Event driven programming`은 사용자 인터페이스, 네트워크 통신, 하드웨어 제어 등 다양한 도메인에서 자주 사용되는 프로그래밍 패러다임이다. `ReactiveX`는 이벤트 스트림을 첫 클래스 객체로 취급하여, 여러 이벤트 소스를 쉽게 합성하고, 이벤트의 발생을 `observe`하고, 반응하는 코드를 작성할 수 있게 한다. 예를 들어, 여러 센서의 데이터를 동시에 추적하고 그들의 출력을 합성해야 하는 IoT 시스템의 경우, `ReactiveX`의 접근 방식은 매우 유용할 수 있다.

앞서 설명한 대로, `ReactiveX`는 여러 이벤트 소스를 쉽게 조율하는데 도움이 된다. 예를 들어, IoT 시스템에서 센서 A와 B의 데이터를 모두 관찰하고, 두 센서의 데이터가 특정 조건을 만족할 때 알림을 보내야 한다고 가정해보자.

`ReactiveX`를 사용하면, 각 센서의 데이터를 `Observable` 스트림으로 표현하고, 이러한 스트림을 결합하여 복잡한 조건을 쉽게 구현할 수 있다.

```javascript
// data streams sensors A and B.
const sensorA$ = getSensorAStream();
const sensorB$ = getSensorBStream();

// Combine the latest data from both sensors using the `combineLatest` operator.
rxjs.combineLatest(sensorA$, sensorB$)
  .pipe(
    // Use the `filter` operator to select only data that meets certain conditions.
    rxjs.operators.filter(([dataA, dataB]) => dataA > 10 && dataB < 20),
  )
  .subscribe(([dataA, dataB]) => {
    console.log('Alert: SensorA and SensorB satisfy the condition.');
  });
```

### 3) 고성능 백엔드 서비스 구축

`ReactiveX`는 비동기 I/O를 쉽게 관리할 수 있도록 도와주며, 고성능 백엔드 서비스를 구축하는데 큰 장점을 제공한다. 예를 들어, 서버는 종종 많은 양의 동시 요청을 처리해야 하며, 각 요청은 데이터베이스 쿼리, 외부 API 호출, 파일 시스템 작업 등의 여러 비동기 작업을 수행할 수 있다.

`ReactiveX`는 이러한 요청을 병렬로 처리하고, 응답을 적절하게 조정하는 데 도움이 된다.

서버가 여러 동시 요청을 처리하고, 각 요청이 데이터베이스 쿼리와 외부 API 호출을 필요로 한다고 가정해보자.

`ReactiveX`를 사용하면, 이러한 비동기 작업을 `Observable` 스트림으로 표현하고, 이들을 병렬로 처리하고, 결과를 쉽게 조율할 수 있다.

```javascript
// A function that performs a database query and external API call for each request.
function handleRequest(req) {
  const query$ = performDatabaseQuery(req);
  const apiCall$ = makeExternalApiCall(req);

  // Wait for both async operations to complete using the `forkJoin` operator.
  return rxjs.forkJoin([query$, apiCall$]);
}

// Call the `handleRequest` function for every incoming request.
incomingRequests$.pipe(
  // Perform asynchronous operations in parallel using the `mergeMap` operator.
  rxjs.operators.mergeMap(req => handleRequest(req)),
)
.subscribe(([queryResult, apiResult]) => {
  console.log('Query Result:', queryResult, 'API Result:', apiResult);
});
```

이들 모두 `ReactiveX`가 비동기 데이터 스트림을 더 쉽게 다루고 이해할 수 있게 해주는 측면들이다. 이로 인해 코드는 더욱 선언적이고, 읽기 쉬워지며, 유지 관리가 더 쉬워진다.

### 4) real-time 애플리케이션 개발

실시간 앱에서는 데이터가 지속적으로 변경되고, 이러한 변경사항을 사용자에게 실시간으로 반영해야 한다. 예를 들어, 주식 거래 앱에서는 주식 가격이 계속해서 변화하고, 이러한 가격 변화를 사용자에게 실시간으로 보여줘야 한다.

`ReactiveX`를 사용하면, 이러한 실시간 데이터 스트림을 쉽게 모델링하고 처리할 수 있다. 데이터가 도착할 때마다 알려주는 `Observable`을 만들어서, 새로운 데이터가 도착할 때마다 UI를 업데이트할 수 있다.

```javascript
// Create an Observable that emits stock prices
const stockPrice$ = getStockPriceStream();

// Subscribe to the Observable and update the UI whenever a new price is emitted
stockPrice$.subscribe(price => {
  updateUiWithNewPrice(price);
});
```

이렇게 `Reactive`를 사용하면 실시간 앱의 데이터 처리 로직을 간결하고 이해하기 쉬운 코드로 표현할 수 있다. 이로 인해 코드의 복잡도가 감소하고, 디버깅이 쉬워진다.

### 5) Reactive Programming의 확장성(Rust: ?)

`ReactiveX`는 다양한 언어를 지원한다. `Java`, `JavaScript`, `C#`, `Python`, `Swift` 등 다양한 언어에서 `ReactiveX 라이브러리를 사용할 수 있다. 이로 인해 한 언어에서 다른 언어로 코드를 포팅하는 데 드는 비용을 줄일 수 있고, 여러 플랫폼에서 동일한 프로그래밍 패러다임을 사용할 수 있다.

이외에도 `ReactiveX` 데이터 스트림 처리를 위한 다양한 연산자를 제공한다. 이러한 연산자를 활용하면, 데이터 스트림의 변환, 필터링, 결합, 에러 처리 등 복잡한 데이터 처리 로직을 쉽게 구현할 수 있다.

결론적으로, `ReactiveX` 복잡한 비동기 데이터 스트림 처리를 위한 강력한 도구이다. 비동기 프로그래밍의 복잡성을 줄이고, 코드의 가독성과 유지 보수성을 향상시키며, 더 나은 사용자 경험을 제공하기 위해 `ReactiveX` 사용해 보는 것을 고려해볼 만하다. Rust에서 어떻게 풀어나가는 것이 좋을지, 대안책은 어떤 것들이 있을지 고민해 보자.

## 4. Key Concepts of ReactiveX

This section defines and explains the key concepts and terms used in ReactiveX.

## 5. Disadvantages of reactiveX

## 6. Core Operators in ReactiveX

This section explains the core operators used in ReactiveX.

## 7. Examples and Use Cases of ReactiveX

This section provides some key use cases and code examples of ReactiveX.

## Conclusion

This section summarizes the main content of the document and re-emphasizes the importance of ReactiveX.

## References and Further Reading

This section provides references used in the document and resources for further learning.