#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};


#[contracttype]
#[derive(Clone, Debug)]
pub struct Student {
    id: u64, 
    nim: String,
    nama: String,
}


const STUDENT_DATA: Symbol = symbol_short!("STUDENT");

#[contract]
pub struct StudentContract;

#[contractimpl]
impl StudentContract {
    
    pub fn get_students(env: Env) -> Vec<Student> {
        
        return env.storage().instance().get(&STUDENT_DATA).unwrap_or(Vec::new(&env));
    }

    
    pub fn create_student(env: Env, nim: String, nama: String) -> String {
      
        let mut students: Vec<Student> = env.storage().instance().get(&STUDENT_DATA).unwrap_or(Vec::new(&env));
    
        let student = Student {
        id: env.prng().gen::<u64>(),
        nim: nim,
        nama: nama,
        };
        
        students.push_back(student);
        
        env.storage().instance().set(&STUDENT_DATA, &students);
        return String::from_str(&env, "Student added successfully");
    }

    pub fn delete_student(env: Env, id: u64) -> String {
      
        let mut students: Vec<Student> = env.storage().instance().get(&STUDENT_DATA).unwrap_or(Vec::new(&env));
   
        for i in 0..students.len() {
            if students.get(i).unwrap().id == id {
                students.remove(i);

                env.storage().instance().set(&STUDENT_DATA, &students);
                return String::from_str(&env, "Student deleted");
            }
        }
        return String::from_str(&env, "Student not found");
    }
}

mod test;
