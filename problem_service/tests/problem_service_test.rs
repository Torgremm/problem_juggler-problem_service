use contracts::{SolveRequest, ValidationResponse};
use problem_service::problems::problem_kind::Problem;

use rand::Rng;
mod common;

struct TestProblem;

impl Problem for TestProblem {
    type Data = String;
    fn create() -> String {
        let mut rng = rand::rng();
        let count = rng.random_range(5..10);
        std::iter::repeat_n('0', count).collect()
    }
    fn into_request(data: String) -> SolveRequest {
        SolveRequest::TestProblem { data }
    }
}
#[tokio::test]
async fn insert_shouldwork() {
    let service = common::get_test_service().await;
    let problem1 = service.get::<TestProblem>().await.unwrap();
    let problem2 = service.get::<TestProblem>().await.unwrap();
    let problem3 = service.get::<TestProblem>().await.unwrap();

    assert!(problem2.id > problem1.id);
    assert!(problem3.id > problem2.id);

    let p1ans = problem1.data.len();
    let p2ans = problem2.data.len();
    let p3ans = problem3.data.len();

    let validation1 = service.validate(problem1.id, p1ans as i64).await.unwrap();
    let validation2 = service.validate(problem2.id, p2ans as i64).await.unwrap();
    let validation3 = service.validate(problem3.id, p3ans as i64).await.unwrap();

    assert!(validation1 == ValidationResponse::Valid);
    assert!(validation2 == ValidationResponse::Valid);
    assert!(validation3 == ValidationResponse::Valid);
}
