extern crate chrono;
use chrono::naive::NaiveDate;

use crate::constantes::university as uni_constants;
use crate::constantes::EXTENSION_PATH;

extern crate serde_json;
use serde::{Deserialize, Serialize};

use std::{
    fs::{write, File},
    io::BufReader,
    str::FromStr,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Percentage {
    tests: f32,
    works: f32,
    mini: f32,
}

impl Percentage {
    fn build(tests: Option<f32>, works: Option<f32>, mini: Option<f32>) -> Percentage {
        let tests = tests.unwrap_or(uni_constants::MIN_PER);
        let works = works.unwrap_or(uni_constants::MIN_PER);
        let mini = mini.unwrap_or(uni_constants::MIN_PER);
        if tests + works + mini != uni_constants::MAX_PER
            && tests + works + mini != uni_constants::MIN_PER
        {
            // panic!("Error! The given values are incorrect");
            println!("Error! The given values are incorrect");
        }
        Percentage { tests, works, mini }
    }

    fn validation(&self) -> bool {
        self.mini + self.works + self.tests == uni_constants::MAX_PER
    }

    fn validation_number(number: f32) -> bool {
        number >= uni_constants::MIN_PER && number <= uni_constants::MAX_PER
    }

    pub fn validation_values(tests: f32, works: f32, mini: f32) -> bool {
        mini + works + tests == uni_constants::MAX_PER
            && Percentage::validation_number(tests)
            && Percentage::validation_number(works)
            && Percentage::validation_number(mini)
    }

    fn change(&mut self, tests: Option<f32>, works: Option<f32>, mini: Option<f32>) -> bool {
        let tests = tests.unwrap_or(0.0);
        let works = works.unwrap_or(0.0);
        let mini = mini.unwrap_or(0.0);
        if !Percentage::validation_values(tests, works, mini) {
            return false;
        }

        self.tests = tests;
        self.works = works;
        self.mini = mini;
        true
    }

    fn tests(&self) -> f32 {
        self.tests
    }

    fn works(&self) -> f32 {
        self.works
    }

    fn mini(&self) -> f32 {
        self.mini
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Evaluation {
    pub percentage: f32,
    pub date: Option<NaiveDate>,
}

impl Evaluation {
    pub fn build(percentage: f32, date: Option<NaiveDate>) -> Evaluation {
        Evaluation { percentage, date }
    }

    pub fn build_with_string(percentage: f32, date_str: String) -> Evaluation {
        let date = match NaiveDate::parse_from_str(&date_str, uni_constants::DATE_PARSE_FORMAT) {
            Ok(date) => Some(date),
            Err(err) => {
                println!("Error parsing string: {} to NaiveDate\n{}", date_str, err);
                None
            }
        };

        Evaluation { percentage, date }
    }

    pub fn parse_date_from_str(&mut self, date_str: String) -> bool {
        match NaiveDate::parse_from_str(&date_str, uni_constants::DATE_PARSE_FORMAT) {
            Ok(date) => self.date = Some(date),
            Err(err) => {
                println!("Error parsing string: {} to NaiveDate\n{}", date_str, err);
                return false;
            }
        };
        true
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject {
    pub name: String,
    pub tests: Option<Vec<Evaluation>>,
    pub works: Option<Vec<Evaluation>>,
    pub mini_tests_tpc: Option<Vec<Evaluation>>,
    pub percentage: Percentage,
    pub notes: Option<String>,
}

impl Subject {
    pub fn build(
        name: String,
        tests: Option<Vec<Evaluation>>,
        works: Option<Vec<Evaluation>>,
        mini_tests_tpc: Option<Vec<Evaluation>>,
        percentage: Percentage,
        notes: Option<String>,
    ) -> Subject {
        Subject {
            name,
            tests,
            works,
            mini_tests_tpc,
            percentage,
            notes,
        }
    }

    fn get_per_tests(&self) -> Option<Vec<f32>> {
        let tests = match &self.tests {
            None => return None,
            Some(tests) => tests,
        };

        let mut vec_per = Vec::new();
        for elem in tests {
            vec_per.push(elem.percentage);
        }
        Some(vec_per)
    }

    fn get_per_minis(&self) -> Option<Vec<f32>> {
        let minis = match &self.mini_tests_tpc {
            None => return None,
            Some(minis) => minis,
        };

        let mut vec_per = Vec::new();
        for elem in minis {
            vec_per.push(elem.percentage);
        }
        Some(vec_per)
    }

    fn get_per_works(&self) -> Option<Vec<f32>> {
        let works = match &self.works {
            None => return None,
            Some(works) => works,
        };

        let mut vec_per = Vec::new();
        for elem in works {
            vec_per.push(elem.percentage);
        }
        Some(vec_per)
    }

    fn add_notes(&mut self, note: String) {
        if note.is_empty() {
            panic!("Error! The note's string is empty");
        }
        match &mut self.notes {
            None => self.notes = Some(note),
            Some(notes) => {
                notes.push('\n');
                notes.push_str(note.as_str());
            }
        }
    }

    // adiciona simplesmente, nao verifica se as percentagens estao correstas
    fn add_test(&mut self, evaluation: Evaluation) {
        match &mut self.tests {
            None => self.tests = Some(vec![evaluation]),
            Some(vec_evaluations) => vec_evaluations.push(evaluation),
        };
    }

    // adiciona simplesmente, nao verifica se as percentagens estao correstas
    fn add_works(&mut self, evaluation: Evaluation) {
        match &mut self.works {
            None => self.works = Some(vec![evaluation]),
            Some(vec_evaluations) => vec_evaluations.push(evaluation),
        };
    }

    // adiciona simplesmente, nao verifica se as percentagens estao correstas
    fn add_minis(&mut self, evaluation: Evaluation) {
        match &mut self.mini_tests_tpc {
            None => self.mini_tests_tpc = Some(vec![evaluation]),
            Some(vec_evaluations) => vec_evaluations.push(evaluation),
        };
    }

    fn change_per_test(&mut self, vec_per: Option<Vec<f32>>) -> bool {
        let number = match &self.tests {
            Some(tests) => tests.len(),
            None => return false,
        };

        match &vec_per {
            None => {
                let per = uni_constants::MAX_PER / number as f32;
                match &mut self.tests {
                    None => return false,
                    Some(vec) => {
                        for evaluation in vec.iter_mut() {
                            evaluation.percentage = per;
                        }
                    }
                }
            }
            Some(vec) => {
                if number != vec.len() {
                    return false;
                }

                let mut i = 0;
                match &mut self.tests {
                    None => return false,
                    Some(vec_per) => {
                        for evaluation in vec_per.iter_mut() {
                            evaluation.percentage = vec[i];
                            i += 1;
                        }
                    }
                }
            }
        }
        true
    }

    fn change_per_works(&mut self, vec_per: Option<Vec<f32>>) -> bool {
        let number = match &self.works {
            Some(works) => works.len(),
            None => return false,
        };

        match &vec_per {
            None => {
                let per = uni_constants::MAX_PER / number as f32;
                match &mut self.works {
                    None => return false,
                    Some(vec) => {
                        for evaluation in vec.iter_mut() {
                            evaluation.percentage = per;
                        }
                    }
                }
            }
            Some(vec) => {
                if number != vec.len() {
                    return false;
                }

                let mut i = 0;
                match &mut self.works {
                    None => return false,
                    Some(vec_per) => {
                        for evaluation in vec_per.iter_mut() {
                            evaluation.percentage = vec[i];
                            i += 1;
                        }
                    }
                }
            }
        }
        true
    }

    fn change_per_mini_tests_tpc(&mut self, vec_per: Option<Vec<f32>>) -> bool {
        let number = match &self.mini_tests_tpc {
            Some(mini_tests_tpc) => mini_tests_tpc.len(),
            None => return false,
        };

        match &vec_per {
            None => {
                let per = uni_constants::MAX_PER / number as f32;
                match &mut self.mini_tests_tpc {
                    None => return false,
                    Some(vec) => {
                        for evaluation in vec.iter_mut() {
                            evaluation.percentage = per;
                        }
                    }
                }
            }
            Some(vec) => {
                if number != vec.len() {
                    return false;
                }

                let mut i = 0;
                match &mut self.mini_tests_tpc {
                    None => return false,
                    Some(vec_per) => {
                        for evaluation in vec_per.iter_mut() {
                            evaluation.percentage = vec[i];
                            i += 1;
                        }
                    }
                }
            }
        }
        true
    }

    //TODO wtf
    fn change_date_tests(&mut self, date: String, index: usize) -> bool {
        match &mut self.tests {
            None => return false,
            Some(test) => {
                let evaluation = test.get_mut(index).unwrap_or(return false);
                evaluation.parse_date_from_str(date);
            }
        }
        true
    }

    fn change_date_works(&mut self, date: String, index: usize) -> bool {
        match &mut self.works {
            None => return false,
            Some(works) => {
                let evaluation = works.get_mut(index).unwrap_or(return false);
                evaluation.parse_date_from_str(date);
            }
        }
        true
    }

    fn change_date_mini_tests_tpcs(&mut self, date: String, index: usize) -> bool {
        match &mut self.mini_tests_tpc {
            None => return false,
            Some(mini_tests_tpc) => {
                let evaluation = mini_tests_tpc.get_mut(index).unwrap_or(return false);
                evaluation.parse_date_from_str(date);
            }
        }
        true
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct University {
    pub subjects: Vec<Subject>,
}

impl University {
    pub fn build() -> University {
        University {
            subjects: Vec::new(),
        }
    }

    pub fn valid_subject(&mut self, name: &String) -> bool {
        for subject in &self.subjects {
            if subject.name.eq(name) {
                return true;
            }
        }
        false
    }

    pub fn add_subject(&mut self, name: &String) -> bool {
        if self.valid_subject(name) {
            return false;
        }

        self.subjects.push(Subject::build(
            String::from(name),
            None,
            None,
            None,
            Percentage::build(None, None, None),
            None,
        ));
        true
    }

    pub fn add_test(&mut self, name: String, evaluation: Evaluation) -> bool {
        for subject in &mut self.subjects {
            if subject.name.eq(&name) {
                subject.add_test(evaluation);
                return true;
            }
        }
        false
    }

    pub fn add_mini_tests_tpc(&mut self, name: String, evaluation: Evaluation) -> bool {
        for subject in &mut self.subjects {
            if subject.name.eq(&name) {
                subject.add_minis(evaluation);
                return true;
            }
        }
        false
    }

    pub fn add_works(&mut self, name: String, evaluation: Evaluation) -> bool {
        for subject in &mut self.subjects {
            if subject.name.eq(&name) {
                subject.add_works(evaluation);
                return true;
            }
        }
        false
    }

    // change per

    pub fn change_test(&mut self, name: String, percentage: Option<Vec<f32>>) -> bool {
        for subject in &mut self.subjects {
            if subject.name.eq(&name) {
                return subject.change_per_test(percentage);
            }
        }
        false
    }

    pub fn change_mini_tests_tpc(&mut self, name: String, percentage: Option<Vec<f32>>) -> bool {
        for subject in &mut self.subjects {
            if subject.name.eq(&name) {
                return subject.change_per_mini_tests_tpc(percentage);
            }
        }
        false
    }

    pub fn change_works(&mut self, name: String, percentage: Option<Vec<f32>>) -> bool {
        for subject in &mut self.subjects {
            if subject.name.eq(&name) {
                return subject.change_per_works(percentage);
            }
        }
        false
    }

    pub fn add_notes(&mut self, name: String, note: String) -> bool {
        for subject in &mut self.subjects {
            if subject.name.eq(&name) {
                subject.add_notes(note);
                return true;
            }
        }
        false
    }

    // functions to add date to some evaluation

    pub fn change_date(
        &mut self,
        name: String,
        where_evaluation: String,
        date: String,
        index: usize,
    ) -> bool {
        for subject in &mut self.subjects {
            if subject.name.eq(&name) {
                match where_evaluation.to_ascii_lowercase().as_str() {
                    "test" => return subject.change_date_tests(date, index),
                    "work" => return subject.change_date_works(date, index),
                    "mini" => return subject.change_date_mini_tests_tpcs(date, index),
                    _ => return false,
                }
            }
        }
        false
    }

    pub fn university_to_json(&self, name_file: &str) {
        let mut path = String::from(uni_constants::PATH);
        path.push_str(name_file);
        path.push_str(EXTENSION_PATH);

        let university = serde_json::to_string_pretty(&self).unwrap();
        write(path, &university).expect("Error write University struct on json file");
    }

    pub fn json_to_university(name_file: &str) -> University {
        let mut path = String::from(uni_constants::PATH);
        path.push_str(name_file);
        path.push_str(EXTENSION_PATH);

        let f = match File::open(&path) {
            Ok(file) => file,
            Err(_) => {
                File::create(path).unwrap();
                return University::build();
            }
        };
        let buf_reader = BufReader::new(f);

        let university: University = match serde_json::from_reader(buf_reader) {
            Ok(university) => university,
            Err(err) => {
                println!("\nError reading json file {} :\n {}", path, err);
                University::build()
            }
        };
        university
    }
}
