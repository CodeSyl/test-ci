use uuid::Uuid;

// Application use case
pub trait AbstractRepository {
    fn find_all(&self) -> i32;
}
// database adapter
#[derive(Debug)]
pub struct WayRepository {
    pub dbname: String,
}

impl AbstractRepository for WayRepository {
    fn find_all(&self) -> i32 {
        2
    }
}

impl WayRepository {
    pub fn new(dbname: String) -> Self {
        Self { dbname }
    }
}

pub struct ClimberChallenge<'a> {
    pub repo: &'a dyn AbstractRepository,
}

impl<'a> ClimberChallenge<'a> {
    pub fn new(repo: &'a dyn AbstractRepository) -> Self {
        Self { repo }
    }
}
pub trait ClimberChallengeInterface {
    fn climber_success_way(&self, way_number: i32) -> i32;
}

impl<'a> ClimberChallengeInterface for ClimberChallenge<'a> {
    fn climber_success_way(&self, way_number: i32) -> i32 {
        let climbers = self.repo.find_all();

        println!("climbers {:?}", climbers);

        way_number
    }
}

// Domain entity
#[derive(Debug, Default)]
pub struct Climber {
    pub id: Uuid,
    pub name: String,
    pub username: String,
}

impl Climber {
    pub fn new(name: String, username: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            username,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::ClimberChallenge;
    use crate::core::ClimberChallengeInterface;
    use crate::core::WayRepository;

    #[test]
    fn test_for_pipeline() {
        let way = WayRepository::new("test_db".to_string());
        let cc = ClimberChallenge { repo: &way };
        let success = cc.climber_success_way(1);

        println!("Main display result {:#?}", success);

        assert_eq!(success, 1);
    }
}
