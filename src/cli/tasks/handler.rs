use crate::auth::context::ResolvedContext;
use crate::cli::tasks::command::TaskCommand;
use crate::cli::tasks::env::TaskEnv;
use crate::cli::tasks::{
    activity, bulk, comment, crud, duplicate, field, label, link, list, mine, quick, relation,
    repeat, transfer,
};
use crate::errors::KaneoError;

pub async fn run(cmd: TaskCommand, ctx: &ResolvedContext) -> Result<(), KaneoError> {
    let client = ctx.client()?;
    let env = TaskEnv::new(&client, ctx);

    match cmd {
        TaskCommand::List(args) => list::run(args, &env).await,
        TaskCommand::Get(args) => crud::get(args, &env).await,
        TaskCommand::Create(args) => crud::create(args, &env).await,
        TaskCommand::Update(args) => crud::update(args, &env).await,
        TaskCommand::Delete(args) => crud::delete(args, &env).await,
        TaskCommand::Status(args) => quick::status(args, &env).await,
        TaskCommand::Priority(args) => quick::priority(args, &env).await,
        TaskCommand::Assign(args) => quick::assign(args, &env).await,
        TaskCommand::MoveTask(args) => quick::move_task(args, &env).await,
        TaskCommand::Comment { command } => comment::run(command, &env).await,
        TaskCommand::TaskLabel { command } => label::run(command, &env).await,
        TaskCommand::Relation { command } => relation::run(command, &env).await,
        TaskCommand::Link { command } => link::run(command, &env).await,
        TaskCommand::Field(args) => field::run(args, &env).await,
        TaskCommand::Activity(args) => activity::run(args, &env).await,
        TaskCommand::Repeat(args) => repeat::run(args, &env).await,
        TaskCommand::Duplicate(args) => duplicate::run(args, &env).await,
        TaskCommand::Bulk(args) => bulk::run(args, &env).await,
        TaskCommand::Mine(args) => mine::run(args, &env).await,
        TaskCommand::Export(args) => transfer::export(args, &env).await,
        TaskCommand::Import(args) => transfer::import(args, &env).await,
    }
}
