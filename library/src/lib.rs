pub fn fibonacciSequence(n: u32) -> Vec<u32> {
    let mut Seq = vec![0, 1];
    for i in 2..=n as usize {
        let nextTerm = Seq[i - 1] + Seq[i - 2];
        Seq.push(nextTerm);
    }
    Seq
}
