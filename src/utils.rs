pub fn project_xyz_2_uvw(xyz : [f32;3]) -> [f32;3] {
    //orthographic projection where the camera is 30degrees above the xy plane
    let (s30, c30) = ((30.0 as f32)/180.0 * 3.14159).sin_cos();
    let u = xyz[0];
    let v = xyz[1]*s30 + xyz[2]*c30;
    let w = -xyz[1] * c30;
    [u, v, w]
}

#[derive(Clone)]
pub struct TimeTaggedValue<T> 
where
T : Clone,
{
    pub value : T,
    pub duration : f32,
}

pub struct TimeTaggedStack<T>
where
T: std::fmt::Debug + Clone,
{
    pub stack : Vec<TimeTaggedValue<T>>,
    pub max_size : usize,
}

impl<T> TimeTaggedStack<T>
where
T : std::fmt::Debug + Clone,
{
    pub fn new(max_size : usize) -> Self {
        Self{
            stack : Vec::with_capacity(max_size),
            max_size,
        }
    }

    pub fn pop(&mut self) -> Option<TimeTaggedValue<T>> {
        self.stack.pop()
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


//like time tagged stack but with max_duration
pub struct DurativeStack<T>
where
T: std::fmt::Debug + Clone, 
{
    pub stack : Vec<TimeTaggedValue<T>>,
    pub max_size : usize,
    pub max_duration : f32,
}

impl<T> DurativeStack<T>
where
T : std::fmt::Debug + Clone, 
{
    pub fn new(max_size : usize, max_duration : f32) -> Self {
        Self{
            stack : Vec::with_capacity(max_size),
            max_size,
            max_duration,
        }
    }

    pub fn pop(&mut self) -> Option<TimeTaggedValue<T>> {
        self.stack.pop()
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