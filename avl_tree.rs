use std::cmp;
use std::cmp::TotalOrd;

enum AVLNode<T> {
  // left, right, height, value
  Branch(~AVLNode<T>, ~AVLNode<T>, int, T),
  Nil
}

impl<T:TotalOrd> AVLNode<T> {
  fn in_order_traverse(&self, f: |&T|) {
    match self {
      &Branch(ref left, ref right, _, ref value) => {
        left.in_order_traverse(|x| f(x));
        f(value);
        right.in_order_traverse(|x| f(x));
      },
      &Nil => {}
    }
  }

  // immutable insert function since mutable is horrible
  fn insert(self, insValue: T) -> AVLNode<T> {
    match self {
      Branch(left, right, height, value) => {
        let (newLeft, newRight) = match insValue.cmp(&value) {
          Equal => (left, right),
          Less => (~left.insert(insValue), right),
          Greater => (left, ~right.insert(insValue))
        };
        let newHeight = cmp::max(newLeft.get_height(), newRight.get_height()) + 1;

        // if newHeight is not old height balance could have changed
        Branch(newLeft, newRight, newHeight, value).balance()
      }
      Nil => {
        Branch(~Nil, ~Nil, 0, insValue)
      }
    }
  }

  fn balance(self) -> AVLNode<T> {
    let balance = self.get_balance();
    match self {
      Branch(left, right, height, value) => {
        if balance > 1 {
          if left.get_balance() == -1 {
            return Branch(~left.left_rotate(), right, height, value).right_rotate();
          } else {
            return Branch(left, right, height, value).right_rotate();
          }
        } else if balance < -1 {
          if right.get_balance() == 1 {
            return Branch(left, ~right.right_rotate(), height, value).left_rotate();
          } else {
            return Branch(left, right, height, value).left_rotate();
          }
        } else {
          return Branch(left, right, height, value);
        }
      },
      Nil => return Nil
    }
  }

  fn left_rotate(self) -> AVLNode<T> {
    match self {
      Branch(left, right, height, value) => {
        match right {
          ~Branch(pivleft, pivright, pivheight, pivval) => {
            // the math is weird but this works
            let left_height = cmp::max(left.get_height(), pivleft.get_height()) + 1;
            let root_height = cmp::max(left_height + 1, pivright.get_height());
            return Branch(~Branch(left, pivleft, left_height, value), pivright, root_height, pivval);
          },
          ~Nil => fail!("nope")
        }
      },
      Nil => fail!("not even once")
    }
  }


  fn right_rotate(self) -> AVLNode<T> {
    match self {
      Branch(left, right, height, value) => {
        match left {
          ~Branch(pivleft, pivright, pivheight, pivval) => {
            let right_height = cmp::max(right.get_height(), pivright.get_height()) + 1;
            let root_height = cmp::max(right_height + 1, pivleft.get_height());

            return Branch(pivleft, ~Branch(pivright, right, right_height, value), root_height, pivval);
          },
          ~Nil => fail!("no")
        }
      },
      Nil => fail!("not even once")
    }
  }

  fn get_balance(&self) -> int {
    match self {
      &Branch(ref left, ref right, _, _) => left.get_height() - right.get_height(),
      &Nil => 0
    }
  }

  fn get_height(&self) -> int {
    match self {
      &Branch(_, _, height, _) => height,
      &Nil => -1
    }
  }

  // please be log(n)
  fn get_depth(&self) -> int {
    match self {
      &Branch(ref left, ref right, _, _) => cmp::max(left.get_depth(), right.get_depth())+1,
      &Nil => 0
    }
  }


  fn find(self, findValue: T) -> bool {
    match self {
      Branch(left, right, _, value) => {
        if value < findValue {
          return left.find(findValue);
        } else if value > findValue {
          return right.find(findValue);
        } else {
          return true;
        }
      },
      Nil => {
        return false;
      }
    }
  }

  fn good(&self) -> bool {
    match self {
      &Branch(ref left, ref right, height, ref value) => {
        match left {
          &~Branch(_, _, _, ref leftValue) => {
            if leftValue > value {
              println!("{:?} is not less than {:?}", leftValue, value);
              return false;
            }
          },
          &~Nil => {}
        }
        match right {
          &~Branch(_, _, _, ref rightValue) => {
            if rightValue < value {
              println!("{:?} is not greater than {:?}", rightValue, value);
              return false;
            }
          },
          &~Nil => {}
        }
        let trueHeight = cmp::max(left.get_depth(), right.get_depth());
        if trueHeight != height {
          println!("incorrect height {}, should be {}", height, trueHeight);
          return false;
        }

        return left.good() && right.good();
      },
      &Nil => return true
    }
  }
}

#[test]
fn test_left_rotate() {
  let tree = Branch(~Branch(~Nil, ~Branch(~Nil, ~Nil, 0, 4), -1, 3), ~Nil, 2, 5);
  assert_eq!(tree.left_rotate(), Branch(~Branch(~Nil, ~Branch(~Nil, ~Nil, 0, 4), -1, 3), ~Nil, 2, 5));
}



fn main() {
  let mut tree : AVLNode<int> = Branch(~Nil, ~Nil, 0, 0);
  for i in range(0,8192) {
    tree = tree.insert(i);
    if !tree.good() {
      break;
    }
  }
  println!("depth: {:?}", tree.get_depth());
  // println!("tree: {:?}", tree);

  // let left_right_tree = Branch(~Branch(~Nil, ~Branch(~Nil, ~Nil, 0, 4), -1, 3), ~Nil, 2, 5);
  // tree = Branch(~Nil, ~Branch(~Nil, ~Branch(~Nil, ~Nil, 0, 3), -1, 2), -2, 1);
  // println!("tree: {:?}", tree);
  // println!("tree: {:?}", tree.left_rotate());
  // println!("left_rotate left child: {:?}", Branch(~Branch(~Nil, ~Branch(~Nil, ~Nil, 0, 4), -1, 3).left_rotate(), ~Nil, 2, 5));
  // println!("bal?: {:?}", tree.balance());
  // println!("bal: {:?}", Branch(~Branch(~Nil, ~Branch(~Nil, ~Nil, 0, 4), -1, 3).left_rotate(), ~Nil, 2, 5).right_rotate());
}

