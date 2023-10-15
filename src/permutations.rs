
fn permutations(s: &String) -> Vec<String> {
  match s.len() {
      0 => return Vec::new(),
      1 => {
          return vec!(s.clone()).to_vec();
      }
      2 => {
          let reverse = s.clone().chars().rev().collect::<String>();
          return vec!(s.clone(), reverse).to_vec();
      }
      _ => {
          let mut result: Vec<String> = Vec::new();
          for i in 0..s.len() {
              let mut s = s.clone();
              let char = s.remove(i);
              let mut permutations = permutations(&s);
              for j in 0..permutations.len() {
                  permutations[j].push(char);
              }
              result.append(&mut permutations);
          }
          return result;
      }
  }
}
