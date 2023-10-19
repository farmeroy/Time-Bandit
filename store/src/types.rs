use sqlx::FromRow;
#[derive(Debug, FromRow)]
pub struct TaskWithEvents {
    pub task: Task,
    pub events: Option<Vec<Event>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub details: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Event {
    pub id: i32,
    pub task_id: i32,
    pub notes: Option<String>,
    pub time_stamp: String,
    pub duration: i32,
}

#[derive(Debug, FromRow)]
pub struct EventWithTaskName {
    pub event: Event,
    pub task_name: String,
}