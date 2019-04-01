#[macro_use] extern crate text_io;

fn main() {
  let mut array: [[i32; 6]; 6] = [[0; 6]; 6];

  for i in 0..6 {
    for j in 0..6 {
      let n: i32 = read!();
      array[i][j] = n;
    }
  }

  let mut max = -63; // The minimum value that could be represented
  for i in 1..5 {
    for j in 1..5 {
      let a = array[i - 1][j - 1];
      let b = array[i - 1][j    ];
      let c = array[i - 1][j + 1];
      let d = array[i    ][j    ];
      let e = array[i + 1][j - 1];
      let f = array[i + 1][j    ];
      let g = array[i + 1][j + 1];

      let sum = a + b + c + d + e + f + g;
      if sum > max {
        max = sum;
      }
    }
  }

  println!("{}", max);
}
