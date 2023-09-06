fn main() {
    // x over here is of type `function item`
    // function item has zero size
    // they are just used to uniquely identify
    // a function by the compiler
    let x = boo;
    println!("{}", std::mem::size_of_val(&x));
    // over here, the function item:x is coerced into a function pointer
    bar(x);

    // a `function pointer` type implements all the three traits: Fn, FnMut and FnOnce
    // since the `self` in this case is just the function pointer and they have no `state` as such
    // - they don't have any references nor do they have any lifetimes associated with them, as
    // opposed to for eg: &str where it is a reference to a string stored somewhere and has lifetime associated with it
    gim(x, x, x);

    let mut z = String::new();

    let c = || ();
    // 'c' can be passed into bar because a closure that does not capture anything from it's environment is coerced into a function pointer
    bar(c);

    let c = || print!("{z}");
    // let d = || z.clear();
    /*
    we can think that the compiler would implement the closure as something like this,
    struct FClosure {
     // used to capture the environment
     // in this case, we only need the immutable reference to z as we are just printing it
     z: &String
    }

    impl Fn for FClosure {
      call(&self) {
        // here, the code from the closure body will be copy pasted
        print!("{self.z}");
    }
    }

    the same will be for a closure that needs a mutable reference and ownership, in those cases, it will
    implement FnMut and FnOnce
    */

    quox(c);
    // d cannot be used since it will need mutable reference to use `z.clear`
    // but it can be used if we say that F implements FnMut in the quox definition
    // quox(d);
}

// anything that implements Fn also implements FnOnce and FnMut
// rough implementation of the traits
/*
  impl FnOnce for F:FnMut {
    call(self){
        F::call(&mut self)
}
  impl FnMut for F:Fn {
    call(&mut self){
        F::call(&self)
}
 Fn implements FnMut
 FnMut implements FnOnce
 => Fn implements FnOnce
}

*/

fn boo() {}

// this over here is a function pointer
fn bar(f: fn()) {
    println!("{}", std::mem::size_of_val(&f))
}

fn gim<F, X, Y>(f: F, f2: X, f3: Y)
where
    F: Fn(),
    X: FnMut(),
    Y: FnOnce(),
{
}

fn quox<F>(f: F)
where
    F: Fn(),
{
}
