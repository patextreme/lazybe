#[derive(Debug, Clone, lazybe::Dal)]
#[lazybe(table = "todo")]
pub struct Todo {
    #[lazybe(primary_key)]
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: Status,
    pub priority: Option<Priority>,
}

#[derive(Debug, Clone, lazybe::DalEnum)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, lazybe::DalEnum)]
pub enum Status {
    Backlog,
    Todo,
    Doing,
    Done,
}
