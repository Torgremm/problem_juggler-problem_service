use crate::interface::solver::RemoteSolverClient;
use crate::interface::solver::SolverClient;
use crate::problem_handler::ProblemRepository;
use crate::problem_handler::ProblemRow;
use crate::problems::count_islands::CountIslands;
use crate::problems::largest_window::LargestWindow;
use crate::problems::problem_kind::DBColumn;
use crate::problems::problem_kind::Problem;
use crate::problems::size_of_island::SizeOfIsland;
use anyhow::Result;
use contracts::ProblemRequest;
use contracts::SolveResponse;
use contracts::ValidationResponse;

pub struct ProblemService {
    repo: ProblemRepository,
    solve_client: RemoteSolverClient,
}

impl ProblemService {
    pub fn new(repo: ProblemRepository, solve_client: RemoteSolverClient) -> Self {
        Self { repo, solve_client }
    }
}

impl ProblemService {
    pub async fn default() -> Self {
        Self {
            repo: ProblemRepository::new("sqlite::memory:")
                .await
                .expect("failed to create database"),
            solve_client: RemoteSolverClient::new("127.0.0.1:4000"),
        }
    }
}

impl ProblemService {
    pub async fn get_dispatch(&self, req: ProblemRequest) -> Result<ProblemRow> {
        match req {
            ProblemRequest::LargestWindowInArray => Ok(self.get::<LargestWindow>().await?),
            ProblemRequest::UnimplementedProblem => todo!(),
            ProblemRequest::TestProblem => todo!(),
            ProblemRequest::SizeOfIsland => Ok(self.get::<SizeOfIsland>().await?),
            ProblemRequest::CountIslands => Ok(self.get::<CountIslands>().await?),
        }
    }
}

impl ProblemService {
    pub async fn get<P: Problem>(&self) -> Result<ProblemRow> {
        let data = P::create();
        let data_string = data.to_db_entry();
        let request = P::into_request(data);
        let answer = self.solve_client.solve(request).await?;
        let a = match answer {
            SolveResponse::Solved(v) => v,
            SolveResponse::BadData(message) => return Err(anyhow::anyhow!(message)),
            SolveResponse::Fault => {
                return Err(anyhow::anyhow!("Solver was unable to solve the problem"))
            }
        };

        let id = self.repo.insert((&data_string, a)).await?;
        Ok(ProblemRow::new(id.try_into()?, a, data_string))
    }
    pub async fn validate(&self, id: i64, answer: i64) -> Result<ValidationResponse> {
        let problem = self.query(id).await?;
        Ok(problem.validate(answer))
    }
    pub async fn query(&self, id: i64) -> Result<ProblemRow> {
        let problem = self.repo.get(id).await?;
        Ok(problem)
    }
}
