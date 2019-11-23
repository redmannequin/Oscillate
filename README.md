# Oscillate

A scripting language for audio production written in rust with a focus on simplicity and extendability.

## Building

Use [cargo](https://www.rust-lang.org/tools/install)(rusts package manager) to build oscillate.

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
    if (count == size) {
      new_data: [size*2];
      new_data.copy(data);
      data: new_data;
    }
    count += 1;
    data[count] = x;
  }

  fn pop() {
    if count == 0 { return; }
    data: data[(start+count)%size];
    count--;
    return data;
  }

  fn remove(index) {
    if index == 0 {
      data: data[start];
      start = (start+1)%size;
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
      if data.count == size { 
        data.remove(0);
      }
      data.push(value); 
    }

    INIT { self.push(sig); }
    RUN { self.push(sig); }
    OUTPUT { data.remove(0); }
}

define Sin {
    t: INPUT;
    freq: INPUT;
    phase: INPUT::Default(0);
    vol: INPUT::Default(0);

    OUTPUT { vol*sin(2*PI*freq*t); }
}

define Delay {
    sig: INPUT;
    time: INPUT::Default(1);
    buf: SizedFIFO { in: sig; size: time; };
    
    INIT { 
      for i in 0..(time) {
        buf.push(0);
      }
      buf.push(sig);
    }
    
    OUTPUT { buf; }
    
}

define SoundA {
    t: PARAM::Default(0);
    note: INPUT::Default(Notes::A4) 

    a: {
        a1: Sin {
            t: t;
            freq: note;
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
        t += 1;
    }

    OUTPUT_CH_1 { a; }
    OUTPUT_CH_2 { a; }
}
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
