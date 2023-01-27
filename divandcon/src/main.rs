fn main() {
    println!("recursion은 새로운 stack frame을 생성하고 완료될시 결과를 반환한다.\n\
    반환된 결과를 이전 스택프레임의 값으로 놓으면서 생성된 스택프레임이 popped off 된다\n\
    즉, 완료되기 전까지 모든 스택프레임은 popped off되지 않는다.\n\
    (last 스택프레임이 완료되어야 이전 스택프레임이 완료됨. 순차적으로 완료된다)");
}