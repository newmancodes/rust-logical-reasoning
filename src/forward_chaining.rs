use crate::horn_clause::HornClause;

pub struct ForwardChaining {
    knowledge_base: Vec<HornClause>
}

impl ForwardChaining {
    pub fn new(knowledge_base: Vec<HornClause>) -> Self {
        ForwardChaining {
            knowledge_base
        }
    }

    pub fn entails(self, n: i8) -> bool {
        if n <= 0 {
            // TODO : Return useful error
            println!("n is too small");
            return false;
        }
  
        for clause in self.knowledge_base.iter() {
            let mut clause_iterator = clause.iter();
            while let Some(j) = clause_iterator.next() {
                if *j < -n || *j > n {
                    println!("n is too small to cover encountered literals");
                    // TODO : Return useful error
                    return false;
                }
            }
        }

        let mut model = Vec::new();
        for _ in 0..n+1 {
            model.push(false);
        }

        let mut fix_point = false;

        while !fix_point {
            fix_point = true;

            for clause in self.knowledge_base.iter() {
                let mut all_true = true;

                let mut clause_iterator = clause.iter();
                while let Some(j) = clause_iterator.next() {
                    if *j < 0 && !model.get(-*j as usize).unwrap() {
                        all_true = false;
                        break;
                    }
                }

                if all_true {
                    let mut goal_clause = true;

                    let mut clause_iterator = clause.iter();
                    while let Some(j) = clause_iterator.next() {
                            if *j > 0 {
                            goal_clause = false;

                            if !model[*j as usize] {
                                model[*j as usize] = true;
                                fix_point = false;
                            }
                            
                            break;
                        }
                    }

                    if goal_clause {
                        return false;
                    }
                }
            }
        }

        // TODO : return the model
        true
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn aima_figure_7pt16_has_a_model() {
        // arrange
        let fc = ForwardChaining::new(vec![HornClause::new(vec![-5, -6]), HornClause::new(vec![-3, -4, 5]), HornClause::new(vec![-2, -3, 4]), HornClause::new(vec![-1, -5, 3]), HornClause::new(vec![-1, -2, 3]), HornClause::new(vec![1]), HornClause::new(vec![2])]);

        // act
        let has_model = fc.entails(6);

        // assert
        assert_eq!(true, has_model);
    }
}