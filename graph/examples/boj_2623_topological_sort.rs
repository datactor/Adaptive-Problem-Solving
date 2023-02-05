// https://www.acmicpc.net/problem/2623
// DAG(topological sort)
// O(n + m)

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
    collections::VecDeque,
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

// impl<'a> impl 블록에 라이프타임 매개변수가 있음을 명시
// 함수 대신 메서드를 사용하면 메서드가 impl된 type의 내부 데이터에 액세스할 수 있으므로 캡슐화 및 데이터 숨김이 가능.
// 이렇게 하면 형식의 동작에 대해 더 쉽게 추론하고 사용 방법에 대한 제한을 적용할 수 있음
// (private 구조체나 열거형에 메서드를 통해 접근 가능하지만 직접 데이터에 접근할 수 없게 하기)
//
// 또한 메서드는 다른 형식 매개 변수 또는 참조 lifetime에 대해 다른 impl로 overload될 수 있지만(self) 함수는 그렇지 않음.
// 이를 통해 특정 사용 사례에 특화될 수 있는 일반 동작을 구현하는 데 더 큰 유연성이 허용됨.
//
// 요약하면 메서드는 구현 및 데이터와 밀접하게 연결된 특정 type에 대한 동작을 정의하는 방법을 제공하는 반면
// 함수는 특정 유형에 연결되지 않은 동작을 정의하는 보다 일반적인 방법을 제공함.
// impl과 trait을 결합하면 객체 지향 디자인 패턴(다형성)을 적용할 수 있음. 그렇지만 우리에겐 enum이 있다.

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Scanner {
        Scanner {
            input: s.split_ascii_whitespace(),
        }
    }

    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut scanner = Scanner::new(&input);
    let (n, m) = (scanner.read::<usize>(), scanner.read::<usize>());
    let mut graph = vec![vec![]; n+1]; // 노드 seq 정렬. atomic은 아님
    let mut indegree = vec![0; n+1]; // 위상 정렬

    for _ in 0..m {
        let num = scanner.read::<usize>();
        let sub: Vec<usize> = (0..num).map(|_| scanner.read::<usize>()).collect();
        for j in 0..num-1 {
            graph[sub[j]].push(sub[j+1]); // 선행 노드별로 후행 노드를 push
            indegree[sub[j+1]] += 1; // 차수 기입
        }
    }


    let mut dq = VecDeque::new();
    for i in 1..n+1 {
        if indegree[i] == 0 {
            dq.push_back(i) // dq에 차수가 0인 노드를 우선 기입한다.
        }
    }

    let mut result = vec![];
    while !dq.is_empty() {
        let num = dq.pop_front().unwrap(); // 선입 선출
        result.push(num);
        for i in &graph[num] { // 선출된 노드를 그래프서 찾고 후행 노드를 모두 불러옴
            indegree[*i] -= 1; // 불러온 후행 노드의 차수를 줄인다
            if indegree[*i] == 0 { // 차수를 줄였을 때 0일 경우 dq에 넣는다.
                dq.push_back(*i)
            }
        }
    }

    // 모든 노드가 불러질 수 없다면 보조 pd들의 seq들이 상충된다는 것으로
    // (result의 elements수가 node수보다 적다면 DAG가 아니라는 뜻으로(단방향 비순환그래프가 아니라 cycle이 있다는 뜻))
    // error처리한다.
    if result.len() != n {
        writeln!(output, "0")?;
    } else {
        for i in result {
            writeln!(output, "{}", i)?;
        }
    }

    Ok(())
}