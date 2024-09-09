# Goals

My goals with this is simply learn rust. If you wanna cooperate simply dm me in discord(i'm pretty offline of it sry).
<br>
I really prettend doing something like rust because my guy writing in rust is amazing, but because it helps a lot. I pretend doing some kind of safer type and type checker before calling transpiling.
<br>
Maybe, only maybe, i try to implement macros, but only for creating code during transpilation time.

## To do
### Function deffiitions such
```rust
rust
fn a(p1:i32, p2:i32) -> i32 {
    p1 + p2
}
```
transforms to
```js
js
function a(a, b) {
    return a + b;
}

d.ts
function a(p1:number, p2:number):number;
ts
function a(p1:number, p2:number):number{
    return a + b;
}
```
### Everything(or almost) to be expression
```rs
rust
let a = 5;
let b = a > 5 {
    12
}else {
    2
}
let c = a < 5 {
    println!("Hello world 5 is bigger");
    28
}else{
    println!("Hello world é o cara... a é maior, respeita o vascão");
    13
}
```
transforms to
```js
js

const a = 5; //rust lets are initially immutable
const b = a > 5 ? 12 : 2;
const c;
if a < 5 {
    println!("Hello world 5 is bigger"); //println because the idea is another syntax, not to transform rust to js
    //rust is being used only to show things in it i pretend adding.
    c = 28;
}else{
    println!("Hello world é o cara... a é maior, respeita o vascão");
    c = 13;
}

d.ts //i dojust to keep the pattern

ts

let a:number = 5;
let b:number = a > 5 ? 12 : 2;
let c:number;
if a < 5 {
    println!("Hello world 5 is bigger");
    c = 28;
}else{
    println!("Hello world é o cara... a é maior, respeita o vascão");
    c = 13;
}
```
### Better enums than ts ones because they suck a lot, like, a lot
```rs
rust
pub enum Stuff {
    Something,
    AnotherThing(A),
    TheThirdThing(B)
}

let a = get_stuff(...);
match a {
    Stuff::Something => println!("Its something"),
    Stuff::AnotherThing(a) => println!("{:?}", a),
    Stuff::TheThirdThing(b) => println!("{:?}", b)
}
```
transforms to
```js
js
const anotherthing = Symbol("anotherthing");
const thirdthing = Symbol("thirdthing");
class Stuff {
    static Something = Object.freeze(new this());
    
    static AnotherThing = class {
        static cmp(o) {
            return o?.tag === anotherthing;
        }
        constructor(t){
            this.data = t;
            return Object.freeze(this);
        }
        get tag(){
            return anotherthing;
        }
    }
    static TheThirdThing = class {
        static cmp(o) {
            return o?.tag === thirdthing;
        }
        constructor(t){
            this.data = t;
            return Object.freeze(this);
        }
        cmp(o) {
            return o.tag == this.tag;
        }
        get tag(){
            return thirdthing;
        }
    }
}
const a = get_stuff(...);
switch(a) {
    case Stuff.Something: {
        println!("Its something");
        break;
    }
    default: {
        if(Stuff.AnotherThing.cmp(a) && a insteanceof Stuff.AnotherThing)
            println!("{:?}",a.data);
        else if(Stuff.TheThirdThing.cmp(a) && a instanceof Stuff.TheThirdThing)
            println!("{:?}", b.data);
        else console.log("Probably imma find some way to throw before finish compile if happens to not handle everything");
    }
}

d.ts
ts
got no idea of how to do so yet
```

Im pretty lazy to write the rest of the rust/js/ts codes, later i rewrite this

### Transpilation of match
### Operator Overloading(just rewrite the methods when in js)
### Loops as expressions
### Interfaces, types, classes,
### Maybe macros
### Imma think in more
