use std::pin::Pin;

use crate::interface::solver::RemoteSolverClient;
use crate::interface::solver::SolverClient;
use crate::problem_handler::ProblemRepository;
use crate::problem_handler::ProblemRow;
use crate::problems::problem_kind::Problem;
use crate::test_template::Test;
use anyhow::Result;
use contracts::SolveResponse;

pub struct ProblemService {
    repo: ProblemRepository,
    solve_client: RemoteSolverClient,
}

impl ProblemService {
    pub async fn default() -> Self {
        Self {
            repo: ProblemRepository::new("sqlite:./data/problems.db")
                .await
                .expect("failed to create database"),
            solve_client: RemoteSolverClient::new("127.0.0.1:4000"),
        }
    }
}

impl ProblemService {
    pub async fn get<P: Problem>(&self) -> Result<ProblemRow> {
        let data = P::create();
        let data_string = format!("{:?}", data)
            .strip_prefix('"')
            .unwrap()
            .strip_suffix('"')
            .unwrap()
            .to_string();
        println!("data = {:?}", data);
        println!("data_string = {:?}", data_string);
        println!("len = {}", data_string.len());
        let request = P::into_request(data);
        let answer = self.solve_client.solve(request).await?;
        let a = match answer {
            SolveResponse::Solved(v) => v,
            SolveResponse::BadData(message) => return Err(anyhow::anyhow!(message)),
            SolveResponse::Fault => return Err(anyhow::anyhow!("Failed to get problem")),
        };

        let id = self.repo.insert((data_string.clone(), a)).await?;
        Ok(ProblemRow::new(id.try_into()?, a, data_string))
    }
    pub async fn validate(&self, id: i64, answer: i64) -> Result<bool> {
        let problem = self.query(id).await?;
        Ok(problem.validate(answer))
    }
    pub async fn query(&self, id: i64) -> Result<ProblemRow> {
        let problem = self.repo.get(id).await?;
        Ok(problem)
    }
}

impl Test for ProblemService {
    fn test_object() -> Pin<Box<dyn Future<Output = Self> + Send>> {
        Box::pin(async move {
            let repo = ProblemRepository::test_object().await;
            let solve_client = RemoteSolverClient::default();
            Self { repo, solve_client }
        })
    }
}

#[cfg(test)]
mod tests {
    use contracts::SolveRequest;

    type SqlxResult<T> = sqlx::Result<T>;

    use super::*;
    use rand::Rng;

    struct TestProblem {
        id: Option<i64>,
        data: String,
        answer: i64,
    }

    impl TestProblem {
        fn id(&self) -> SqlxResult<i64> {
            match self.id {
                Some(id) => Ok(id),
                None => Err(sqlx::Error::InvalidArgument("None".to_string())),
            }
        }

        fn set_id(&mut self, id: i64) {
            self.id = Some(id);
        }

        fn answer(&self) -> i64 {
            self.answer
        }

        fn data(&self) -> String {
            self.data.clone()
        }
    }
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
        let service = ProblemService::test_object().await;
        let problem1 = service.get::<TestProblem>().await.unwrap();
        let problem2 = service.get::<TestProblem>().await.unwrap();
        let problem3 = service.get::<TestProblem>().await.unwrap();
        assert_eq!(problem1.id, 1);
        assert_eq!(problem2.id, 2);
        assert_eq!(problem3.id, 3);

        let p1ans = problem1.data.len();
        let p2ans = problem2.data.len();
        let p3ans = problem3.data.len();

        let validation1 = service.validate(problem1.id, p1ans as i64).await.unwrap();
        let validation2 = service.validate(problem2.id, p2ans as i64).await.unwrap();
        let validation3 = service.validate(problem3.id, p3ans as i64).await.unwrap();

        assert!(validation1);
        assert!(validation2);
        assert!(validation3);
    }
}
