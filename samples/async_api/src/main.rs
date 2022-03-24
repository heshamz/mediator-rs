#![allow(dead_code)]
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use mediator::{AsyncMediator, AsyncRequestHandler, DefaultAsyncMediator, Event, Request};

#[derive(Debug, Clone)]
struct User {
    id: Uuid,
    name: String,
}

struct UserService(Vec<User>);
type SharedUserService = Arc<Mutex<UserService>>;

struct CreateUserRequest(String);
impl Request<User> for CreateUserRequest {}

struct GetAllUsersRequest;
impl Request<Vec<User>> for GetAllUsersRequest {}

#[derive(Clone)]
struct UserCreatedEvent(User);
impl Event for UserCreatedEvent {}

struct CreateUserRequestHandler(SharedUserService, DefaultAsyncMediator);
#[mediator::async_trait]
impl AsyncRequestHandler<CreateUserRequest, User> for CreateUserRequestHandler {
    async fn handle(&mut self, req: CreateUserRequest) -> User {
        let mut service = self.0.lock().await;
        let user = User {
            id: Uuid::new_v4(),
            name: req.0.clone(),
        };

        service.0.push(user.clone());
        self.1.publish(UserCreatedEvent(user.clone())).await.expect("publish failed");
        user
    }
}

struct GetAllUsersRequestHandler(SharedUserService);
#[mediator::async_trait]
impl AsyncRequestHandler<GetAllUsersRequest, Vec<User>> for GetAllUsersRequestHandler {
    async fn handle(&mut self, _: GetAllUsersRequest) -> Vec<User> {
        let service = self.0.lock().await;
        service.0.clone()
    }
}

#[tokio::main]
async fn main() {
    let service = Arc::new(Mutex::new(UserService(vec![])));
    let total_users = Arc::new(Mutex::new(0_usize));

    let s = service.clone();
    let mut mediator = DefaultAsyncMediator::builder()
        .add_handler_deferred(|m| CreateUserRequestHandler(service.clone(), m))
        //.add_handler(GetAllUsersRequestHandler(service.clone()))
        .subscribe_fn_with(total_users.clone(), |event: UserCreatedEvent, total: Arc<Mutex<usize>>| async move {
            println!("User created: {:?}", event.0.name);
            let mut lock = total.lock().await;
            *lock += 1;
        })
        .add_handler(move |req: GetAllUsersRequest| async move {

            let service = s.lock().await;
            let users = service.0.clone();
            users
        })
        .build();

    mediator.send(CreateUserRequest("John".to_string())).await.unwrap();
    mediator.send(CreateUserRequest("Jane".to_string())).await.unwrap();

    let users = mediator.send(GetAllUsersRequest).await.unwrap();
    println!("{:#?}", users);
    println!("Total users: {}", total_users.lock().await);
}
