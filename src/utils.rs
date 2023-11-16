pub struct TimeTaggedValue<T> {
    pub value : T,
    pub duration : f32,
}

pub struct TimeTaggedStack<T>
where
T: std::fmt::Debug, 
{
    pub stack : Vec<TimeTaggedValue<T>>,
    pub max_size : usize,
}

impl<T> TimeTaggedStack<T>
where
T : std::fmt::Debug, 
{
    pub fn new(max_size : usize) -> Self {
        Self{
            stack : Vec::with_capacity(max_size),
            max_size,
        }
    }
    
    pub fn push(&mut self, value : T) {
        if self.stack.len() == self.max_size {
            self.stack.remove(0);
        }
        self.stack.push(TimeTaggedValue{value : value, duration : 0.0});
    }

    pub fn update(&mut self, delta_time : f32) {
        for time_tagged_value in self.stack.iter_mut() {
            time_tagged_value.duration += delta_time;
        }
    }
}


pub struct DurativeStack<T>
where
T: std::fmt::Debug, 
{
    pub stack : Vec<TimeTaggedValue<T>>,
    pub max_size : usize,
    pub max_duration : f32,
}

impl<T> DurativeStack<T>
where
T : std::fmt::Debug, 
{
    pub fn new(max_size : usize, max_duration : f32) -> Self {
        Self{
            stack : Vec::with_capacity(max_size),
            max_size,
            max_duration,
        }
    }
    
    pub fn push(&mut self, value : T) {
        if self.stack.len() == self.max_size {
            self.stack.remove(0);
        }
        self.stack.push(TimeTaggedValue { value: value, duration: 0.0 });
    }

    pub fn update(&mut self, delta_time : f32) {
        let mut keep_values = 0;
        for time_tagged_value in self.stack.iter_mut().rev() {
            time_tagged_value.duration += delta_time;
            if time_tagged_value.duration > self.max_duration {
                break
        } else {
            keep_values += 1;
        }
        }

        let popout_values = self.stack.len() - keep_values;
        for _ in 0..popout_values {
            self.stack.remove(0);
        }
    }
}
