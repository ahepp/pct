pub struct Context {
    pub mm: f32,
    pub time: f32,
    pub food: f32,
    pub cash: f32,
    pub day: u32,
    pub speed: f32,
    pub waketime: f32,
    pub bedtime: f32,
    pub food_per_day: f32,
    pub max_food: f32,
    pub cash_per_food: f32,
    pub towns: Vec<Town>,
    pub town_idx: usize,
}
impl Context {
    pub fn new() -> Self {
        let towns = vec![
            Town {
                name: "Campo".to_string(),
                mm: 1.4,
            },
            Town {
                name: "Lake Morena".to_string(),
                mm: 20.0,
            },
            Town {
                name: "Mount Laguna".to_string(),
                mm: 41.5,
            },
        ];
        Context {
            mm: 0.0,
            time: 8.0,
            food: 5.0,
            cash: 1000.0,
            day: 0,
            speed: 1.0,
            waketime: 8.0,
            bedtime: 20.0,
            food_per_day: 1.0,
            max_food: 10.0,
            cash_per_food: 1.0,
            towns,
            town_idx: 0,
        }
    }
}

pub struct Town {
    pub name: String,
    pub mm: f32,
}
