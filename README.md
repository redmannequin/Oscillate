# Oscillate
![](https://github.com/redmannequin/Oscillate/workflows/Rust/badge.svg)

A scripting language for audio production written in rust with a focus on simplicity and extendability.

## Building

Use [cargo](https://www.rust-lang.org/tools/install) to build oscillate.

```bash
cargo build --release
```

## Basic Syntax Overview

* variable decelerations 
```
a: 2;
```
* arrays
```
[1,2,3];
[1;3] == [1,1,1];
```
* if statment
```
if condition { 
  ... 
} else if condition {
  ...
} else {
  ...
} 
```
* functions
```
fn name(p1,...,pn) {
  ...
  return;
}
```
* objects
```
define name {
  ...
}
```
* example: A sine wave with a delay
```
define Buffer {
  size : PARAM(0);
  data: [size];
  count: 0;
  start: 0;

  fn push(x) {
    if (self.count == self.size) {
      new_data: [self.size*2];
      while x: self.pop() {
        new_data.push(x);
      }
      self.size: size*2;
      self.data: new_data;
    }
    self.count: self.count + 1;
    self.data[self.count] = x;
  }

  fn pop() {
    if self.count == 0 { return; }
    data: self.data[(self.start+self.count)%self.size];
    self.count: self.count-1;
    return data;
  }

  fn remove(index) {
    if index == 0 {
      data: self.data[self.start];
      self.start: (self.start+1)%self.size;
      return data;
    } else {
      unimplemented;
    }
  }
}

define SizedFIFO {
    sig: INPUT,
    size: INPUT;
    data: Buffer { size; };

    fn push(value) { 
      if self.data.count == self.size { 
        self.data.remove(0);
      }
      self.data.push(value); 
    }

    INIT { self.push(sig); }
    RUN { self.push(sig); }
    OUTPUT { self.data.remove(0); }
}

define Sin {
    t: INPUT;
    freq: INPUT;
    phase: INPUT::Default(0);
    vol: INPUT::Default(0);

    OUTPUT { self.vol*self.sin(2*PI*self.freq*self.t); }
}

define Delay {
    sig: INPUT;
    time: INPUT::Default(1);
    buf: SizedFIFO { in: sig; size: time; };
    
    INIT { 
      for i in 0..(time) {
        self.buf.push(0);
      }
      self.buf.push(self.sig);
    }
    
    OUTPUT { self.buf; }
    
}

define SoundA {
    t: PARAM::Default(0);
    note: INPUT::Default(Notes::A4) 

    a: {
        a1: Sin {
            t: self.t;
            freq: self.note;
            phase: 0;
            vol: 0;
        }

        a2: Delay { 
            in: a1;
            time: 0.5; 
        }

        OUPUT { a2; }
    }

    RUN {
        self.t: self.t + 1;
    }

    OUTPUT_CH_1 { self.a; }
    OUTPUT_CH_2 { self.a; }
}
```

## License
[MIT](https://choosealicense.com/licenses/mit/)
