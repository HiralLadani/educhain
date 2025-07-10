// use ic_cdk::{api, caller, query, update};
// use candid::{CandidType, Deserialize};
// use std::cell::RefCell;
// use std::collections::HashMap;

// type Principal = candid::Principal;

// // Thread-local storages
// thread_local! {
//     static STUDENTS: RefCell<HashMap<Principal, StudentProfile>> = RefCell::new(HashMap::new());
//     static COURSES: RefCell<HashMap<u64, Course>> = RefCell::new(HashMap::new());
//     static ENROLLMENTS: RefCell<Vec<Enrollment>> = RefCell::new(Vec::new());
//     static PENDING_REQUESTS: RefCell<Vec<CourseRequest>> = RefCell::new(Vec::new());
//     static DAO_PROPOSALS: RefCell<Vec<DaoProposal>> = RefCell::new(Vec::new());
//     static BANNED_INSTRUCTORS: RefCell<Vec<Principal>> = RefCell::new(Vec::new());
//     static REMOVED_STUDENTS: RefCell<Vec<Principal>> = RefCell::new(Vec::new());
//     static CONFIG: RefCell<TokenConfig> = RefCell::new(TokenConfig { reward_per_course: 10, cost_to_enroll: 5 });
// }

// // Data models
// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub struct StudentProfile {
//     name: String,
//     roll_no: String,
//     email: String,
// }

// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub struct Course {
//     id: u64,
//     title: String,
//     description: String,
//     instructor: Principal,
// }

// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub struct CourseRequest {
//     id: u64,
//     title: String,
//     description: String,
//     instructor: Principal,
//     instructor_name: String,
// }

// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub struct Enrollment {
//     course_id: u64,
//     student: Principal,
//     student_name: String,
//     roll_no: String,
//     passed: Option<bool>,
// }

// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub struct DaoProposal {
//     text: String,
//     yes_votes: u32,
//     no_votes: u32,
// }

// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub struct TokenConfig {
//     reward_per_course: u64,
//     cost_to_enroll: u64,
// }

// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub struct PlatformStats {
//     total_students: u64,
//     total_courses: u64,
//     certificates_issued: u64,
// }

// // Student functions
// #[update]
// fn update_student_profile(name: String, roll_no: String, email: String) -> String {
//     let me = caller();
//     STUDENTS.with(|s| s.borrow_mut().insert(me, StudentProfile { name, roll_no, email }));
//     "Profile updated".into()
// }

// #[update]
// fn enroll_in_course(course_id: u64) -> String {
//     let me = caller();
//     if REMOVED_STUDENTS.with(|r| r.borrow().contains(&me)) {
//         return "Student removed.".into();
//     }
//     let profile = STUDENTS.with(|s| s.borrow().get(&me).cloned());
//     if let Some(p) = profile {
//         ENROLLMENTS.with(|e| {
//             if e.borrow().iter().any(|en| en.course_id == course_id && en.student == me) {
//                 "Already enrolled.".into()
//             } else {
//                 e.borrow_mut().push(Enrollment {
//                     course_id,
//                     student: me,
//                     student_name: p.name,
//                     roll_no: p.roll_no,
//                     passed: None,
//                 });
//                 "Enrolled successfully.".into()
//             }
//         })
//     } else {
//         "Set profile first.".into()
//     }
// }

// #[update]
// fn drop_out_of_course(course_id: u64) -> String {
//     let me = caller();
//     ENROLLMENTS.with(|e| {
//         let mut list = e.borrow_mut();
//         let before = list.len();
//         list.retain(|en| !(en.course_id == course_id && en.student == me));
//         if list.len() < before { "Dropped out".into() } else { "Not enrolled".into() }
//     })
// }

// #[query]
// fn browse_courses() -> Vec<Course> {
//     COURSES.with(|c| c.borrow().values().cloned().collect())
// }

// // Instructor functions
// #[update]
// fn request_new_course(title: String, description: String, instructor_name: String) -> CourseRequest {
//     let req = CourseRequest {
//         id: api::time() as u64,
//         title,
//         description,
//         instructor: caller(),
//         instructor_name,
//     };
//     PENDING_REQUESTS.with(|p| p.borrow_mut().push(req.clone()));
//     req
// }

// #[query]
// fn list_my_courses() -> Vec<Course> {
//     let me = caller();
//     COURSES.with(|c| c.borrow().values().filter(|x| x.instructor == me).cloned().collect())
// }

// #[query]
// fn list_enrolled_students(course_id: u64) -> Vec<Enrollment> {
//     ENROLLMENTS.with(|e| e.borrow().iter().filter(|en| en.course_id == course_id).cloned().collect())
// }

// #[update]
// fn mark_pass(course_id: u64, student: Principal) -> String {
//     mark_student(course_id, student, true)
// }

// #[update]
// fn mark_fail(course_id: u64, student: Principal) -> String {
//     mark_student(course_id, student, false)
// }

// fn mark_student(course_id: u64, student: Principal, pass: bool) -> String {
//     ENROLLMENTS.with(|e| {
//         if let Some(en) = e.borrow_mut().iter_mut().find(|en| en.course_id == course_id && en.student == student) {
//             en.passed = Some(pass);
//             if pass { "Marked pass".into() } else { "Marked fail".into() }
//         } else {
//             "Not found".into()
//         }
//     })
// }

// // Admin functions
// #[update]
// fn approve_course_request(id: u64) -> String {
//     PENDING_REQUESTS.with(|p| {
//         let mut list = p.borrow_mut();
//         if let Some(pos) = list.iter().position(|x| x.id == id) {
//             let req = list.remove(pos);
//             COURSES.with(|c| c.borrow_mut().insert(req.id, Course {
//                 id: req.id,
//                 title: req.title,
//                 description: req.description,
//                 instructor: req.instructor,
//             }));
//             "Approved".into()
//         } else { "Not found".into() }
//     })
// }

// #[query]
// fn list_pending_requests() -> Vec<CourseRequest> {
//     PENDING_REQUESTS.with(|p| p.borrow().clone())
// }

// #[update]
// fn add_dao_proposal(text: String) -> String {
//     DAO_PROPOSALS.with(|d| d.borrow_mut().push(DaoProposal { text, yes_votes: 0, no_votes: 0 }));
//     "Added".into()
// }

// #[update]
// fn vote_on_proposal(index: u64, yes: bool) -> String {
//     DAO_PROPOSALS.with(|d| {
//         if let Some(p) = d.borrow_mut().get_mut(index as usize) {
//             if yes { p.yes_votes += 1; } else { p.no_votes += 1; }
//             "Voted".into()
//         } else { "Not found".into() }
//     })
// }

// #[query]
// fn view_dao_proposals() -> Vec<DaoProposal> {
//     DAO_PROPOSALS.with(|d| d.borrow().clone())
// }

// #[update]
// fn ban_instructor(instructor: Principal) -> String {
//     BANNED_INSTRUCTORS.with(|b| b.borrow_mut().push(instructor));
//     "Banned".into()
// }

// #[update]
// fn remove_student(student: Principal) -> String {
//     REMOVED_STUDENTS.with(|r| r.borrow_mut().push(student));
//     "Removed".into()
// }

// #[update]
// fn set_token_reward(amount: u64) -> String {
//     CONFIG.with(|c| c.borrow_mut().reward_per_course = amount);
//     "Set".into()
// }

// #[update]
// fn set_cost_to_enroll(amount: u64) -> String {
//     CONFIG.with(|c| c.borrow_mut().cost_to_enroll = amount);
//     "Set".into()
// }

// #[query]
// fn get_platform_stats() -> PlatformStats {
//     PlatformStats {
//         total_students: STUDENTS.with(|s| s.borrow().len() as u64),
//         total_courses: COURSES.with(|c| c.borrow().len() as u64),
//         certificates_issued: ENROLLMENTS.with(|e| e.borrow().iter().filter(|en| en.passed == Some(true)).count() as u64),
//     }
// }

// // Export Candid
// ic_cdk::export_candid!();
use ic_cdk::{init,api, caller, query, update};
use std::cell::RefCell;
use std::collections::HashMap;
use candid::{CandidType, Deserialize, Principal};
//use ic_cdk::api::http::{HttpResponse, HttpRequest};
// Thread-local storages
// IMPORTANT: Data in thread_local! is NOT persistent across canister upgrades or restarts.
// For production, you MUST use ic_stable_structures for persistent storage.
thread_local! {
    static USER_ROLES: RefCell<HashMap<Principal, Role>> = RefCell::new(HashMap::new());
    static STUDENTS: RefCell<HashMap<Principal, StudentProfile>> = RefCell::new(HashMap::new());
    static COURSES: RefCell<HashMap<u64, Course>> = RefCell::new(HashMap::new());
    static ENROLLMENTS: RefCell<Vec<Enrollment>> = RefCell::new(Vec::new());
    static PENDING_REQUESTS: RefCell<Vec<CourseRequest>> = RefCell::new(Vec::new());
    static DAO_PROPOSALS: RefCell<Vec<DaoProposal>> = RefCell::new(Vec::new());
    static BANNED_INSTRUCTORS: RefCell<Vec<Principal>> = RefCell::new(Vec::new());
    static REMOVED_STUDENTS: RefCell<Vec<Principal>> = RefCell::new(Vec::new());
    static CONFIG: RefCell<TokenConfig> = RefCell::new(TokenConfig { reward_per_course: 10, cost_to_enroll: 5 });
}

/// Initializer function for the canister.
/// Sets the deployer of the canister as the initial Admin.
#[init]
fn init() {
    let deployer = caller();
    USER_ROLES.with(|m| {
        m.borrow_mut().insert(deployer, Role::Admin);
    });
    ic_cdk::println!("Canister initialized. Deployer {:?} set as Admin.", deployer);
}

// Helper function to check if a principal has Admin privileges.
fn is_admin(p: Principal) -> bool {
    USER_ROLES.with(|m| matches!(m.borrow().get(&p), Some(Role::Admin)))
}

// Helper function to assert that the caller is an Admin.
fn require_admin() {
    let me = caller();
    assert!(me != Principal::anonymous(), "Authentication required: Caller is anonymous.");
    assert!(is_admin(me), "Authorization failed: Only admins can call this method.");
    ic_cdk::println!("Admin {:?} successfully called an admin-only method.", me);
}
// #[query]
// fn http_request(req: HttpRequest) -> HttpResponse {
//     // Here, serve your static files or index.html
//     HttpResponse {
//         status_code: 200,
//         headers: vec![("Content-Type".to_string(), "text/html".to_string())],
//         body: include_bytes!("index.html").to_vec(),
//         upgrade: None,
//         streaming_strategy: None,
//     }
// }
/// All possible roles a user can hold.
#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq)]
pub enum Role {
    Admin,
    Student,
    Professor,
    Guest,
}

// ──────────────────────────────────────────────────────────────
// User / Role Management Functions
// ──────────────────────────────────────────────────────────────

/// Registers the caller as a user. If the caller does not have a role yet, they are assigned Role::Guest.
/// If they already have a role, their existing role is preserved.
#[update]
fn register_user() {
    let me = caller();
    // Ensure anonymous principal cannot register
    assert!(me != Principal::anonymous(), "Cannot register with an anonymous principal.");

    USER_ROLES.with(|m| {
        m.borrow_mut().entry(me).or_insert(Role::Guest);
    });
    ic_cdk::println!("Principal {:?} registered or confirmed.", me);
}

/// Returns the role of the current caller.
/// If the caller's principal is not found in the USER_ROLES map, Role::Guest is returned.
#[query]
fn my_role() -> Role {
    let me = caller();
    USER_ROLES.with(|m| m.borrow().get(&me).cloned().unwrap_or(Role::Guest))
}

/// Admin-only: Assigns a new role to a specified user principal.
/// The caller must have the Admin role.
#[update]
fn assign_role(user: Principal, role: Role) {
    require_admin();
    USER_ROLES.with(|m| {
        m.borrow_mut().insert(user, role.clone());
    });
    ic_cdk::println!("Admin {:?} assigned role {:?} to user {:?}.", caller(), role, user);
}

/// Admin-only: Enumerates all registered users and their assigned roles.
/// The caller must have the Admin role.
#[query]
fn list_users() -> Vec<(Principal, Role)> {
    require_admin();
    USER_ROLES.with(|m| {
        m.borrow()
            .iter()
            .map(|(p, r)| (*p, r.clone()))
            .collect()
    })
}

/// Public: Returns a list of every known Principal that has interacted with the canister (i.e., has an entry in USER_ROLES).
#[query]
fn list_principals() -> Vec<Principal> {
    USER_ROLES.with(|m| m.borrow().keys().cloned().collect())
}

/// Returns the caller's Principal ID.
#[query]
fn whoami() -> Principal {
    caller()
}

// ──────────────────────────────────────────────────────────────
// Data Models
// ──────────────────────────────────────────────────────────────

/// Represents a student's profile information.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StudentProfile {
    pub name: String,
    pub roll_no: String,
    pub email: String,
}

/// Represents a course offered on the platform.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Course {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub instructor: Principal, // Principal of the instructor
}

/// Represents a request by an instructor to add a new course.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct CourseRequest {
    pub id: u64, // Unique ID for the request
    pub title: String,
    pub description: String,
    pub instructor: Principal, // Principal of the requesting instructor
    pub instructor_name: String, // Name of the requesting instructor
}

/// Represents a student's enrollment in a course.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Enrollment {
    pub course_id: u64,
    pub student: Principal, // Principal of the enrolled student
    pub student_name: String, // Name of the enrolled student (for display)
    pub roll_no: String, // Roll number of the enrolled student (for display)
    pub passed: Option<bool>, // Whether the student passed the course (None if not yet graded)
}

/// Represents a DAO (Decentralized Autonomous Organization) proposal.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct DaoProposal {
    pub text: String,
    pub yes_votes: u32,
    pub no_votes: u32,
}

/// Configuration for token rewards and costs.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TokenConfig {
    pub reward_per_course: u64, // Tokens rewarded to instructor for course completion
    pub cost_to_enroll: u64, // Tokens required for a student to enroll
}

/// Platform statistics.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct PlatformStats {
    pub total_students: u64,
    pub total_courses: u64,
    pub certificates_issued: u64,
}

// ──────────────────────────────────────────────────────────────
// Student Functions
// ──────────────────────────────────────────────────────────────

/// Allows a student to update their profile information.
#[update]
fn update_student_profile(name: String, roll_no: String, email: String) -> String {
    let me = caller();
    // Ensure not anonymous and has a student/guest role to allow profile creation
    assert!(me != Principal::anonymous(), "Authentication required.");

    STUDENTS.with(|s| s.borrow_mut().insert(me, StudentProfile { name, roll_no, email }));
    ic_cdk::println!("Student {:?} updated profile.", me);
    "Profile updated successfully.".into()
}

/// Allows a student to enroll in a course.
#[update]
fn enroll_in_course(course_id: u64) -> String {
    let me = caller();
    assert!(me != Principal::anonymous(), "Authentication required.");

    // Prevent removed students from enrolling
    if REMOVED_STUDENTS.with(|r| r.borrow().contains(&me)) {
        return "Enrollment failed: Student has been removed from the platform.".into();
    }

    let profile = STUDENTS.with(|s| s.borrow().get(&me).cloned());
    if let Some(p) = profile {
        // Check if course exists
        let course_exists = COURSES.with(|c| c.borrow().contains_key(&course_id));
        if !course_exists {
            return "Enrollment failed: Course not found.".into();
        }

        ENROLLMENTS.with(|e| {
            // Check if already enrolled
            if e.borrow().iter().any(|en| en.course_id == course_id && en.student == me) {
                "Enrollment failed: Already enrolled in this course.".into()
            } else {
                e.borrow_mut().push(Enrollment {
                    course_id,
                    student: me,
                    student_name: p.name,
                    roll_no: p.roll_no,
                    passed: None, // Not graded yet
                });
                ic_cdk::println!("Student {:?} enrolled in course {}.", me, course_id);
                "Enrolled successfully.".into()
            }
        })
    } else {
        "Enrollment failed: Please set up your student profile first (update_student_profile).".into()
    }
}

/// Allows a student to drop out of a course.
#[update]
fn drop_out_of_course(course_id: u64) -> String {
    let me = caller();
    assert!(me != Principal::anonymous(), "Authentication required.");

    ENROLLMENTS.with(|e| {
        let mut list = e.borrow_mut();
        let initial_len = list.len();
        // Retain only enrollments that are NOT for this course and this student
        list.retain(|en| !(en.course_id == course_id && en.student == me));
        if list.len() < initial_len {
            ic_cdk::println!("Student {:?} dropped out of course {}.", me, course_id);
            "Dropped out of course successfully.".into()
        } else {
            "Drop out failed: Not enrolled in this course.".into()
        }
    })
}

/// Public: Allows anyone to browse available courses.
#[query]
fn browse_courses() -> Vec<Course> {
    COURSES.with(|c| c.borrow().values().cloned().collect())
}

// ──────────────────────────────────────────────────────────────
// Instructor Functions
// ──────────────────────────────────────────────────────────────

/// Allows a professor to request a new course to be added to the platform.
#[update]
fn request_new_course(title: String, description: String, instructor_name: String) -> CourseRequest {
    let me = caller();
    assert!(me != Principal::anonymous(), "Authentication required.");

    // Prevent banned instructors from requesting courses
    if BANNED_INSTRUCTORS.with(|b| b.borrow().contains(&me)) {
        ic_cdk::trap("Course request failed: You are a banned instructor.");
    }

    let req = CourseRequest {
        id: api::time() as u64, // Use current time as a simple unique ID
        title,
        description,
        instructor: me,
        instructor_name,
    };
    PENDING_REQUESTS.with(|p| p.borrow_mut().push(req.clone()));
    ic_cdk::println!("Instructor {:?} requested new course: {}.", me, req.title);
    req
}

/// Allows an instructor to list the courses they are teaching.
#[query]
fn list_my_courses() -> Vec<Course> {
    let me = caller();
    assert!(me != Principal::anonymous(), "Authentication required.");

    COURSES.with(|c| c.borrow().values().filter(|x| x.instructor == me).cloned().collect())
}

/// Allows an instructor to view all students enrolled in a specific course they teach.
#[query]
fn list_enrolled_students(course_id: u64) -> Vec<Enrollment> {
    let me = caller();
    assert!(me != Principal::anonymous(), "Authentication required.");

    // Ensure the caller is indeed the instructor of this course
    let is_my_course = COURSES.with(|c| c.borrow().get(&course_id).map_or(false, |course| course.instructor == me));
    if !is_my_course {
        ic_cdk::trap("Access denied: You are not the instructor of this course.");
    }

    ENROLLMENTS.with(|e| {
        e.borrow()
            .iter()
            .filter(|en| en.course_id == course_id)
            .cloned()
            .collect()
    })
}

/// Allows an instructor to mark a specific student as having passed a course.
#[update]
fn mark_pass(course_id: u64, student: Principal) -> String {
    mark_student(course_id, student, true)
}

/// Allows an instructor to mark a specific student as having failed a course.
#[update]
fn mark_fail(course_id: u64, student: Principal) -> String {
    mark_student(course_id, student, false)
}

// Helper function to mark a student's status (pass/fail) in a course.
fn mark_student(course_id: u64, student: Principal, pass: bool) -> String {
    let me = caller();
    assert!(me != Principal::anonymous(), "Authentication required.");

    // Ensure the caller is indeed the instructor of this course
    let is_my_course = COURSES.with(|c| c.borrow().get(&course_id).map_or(false, |course| course.instructor == me));
    if !is_my_course {
        return "Authorization failed: You are not the instructor of this course.".into();
    }

    ENROLLMENTS.with(|e| {
        if let Some(en) = e.borrow_mut().iter_mut().find(|en| en.course_id == course_id && en.student == student) {
            en.passed = Some(pass);
            ic_cdk::println!("Instructor {:?} marked student {:?} in course {}: {}.", me, student, course_id, if pass {"Passed"} else {"Failed"});
            if pass { "Student marked as passed successfully.".into() } else { "Student marked as failed successfully.".into() }
        } else {
            "Marking failed: Enrollment not found for this student in this course.".into()
        }
    })
}

// ──────────────────────────────────────────────────────────────
// Admin Functions
// ──────────────────────────────────────────────────────────────

/// Admin-only: Approves a pending course request, adding it to the list of active courses.
#[update]
fn approve_course_request(id: u64) -> String {
    require_admin();
    PENDING_REQUESTS.with(|p| {
        let mut list = p.borrow_mut();
        if let Some(pos) = list.iter().position(|x| x.id == id) {
            let req = list.remove(pos);
            let title = req.title.clone();
            COURSES.with(|c| c.borrow_mut().insert(req.id, Course {
                id: req.id,
                title:  title.clone(),
                description: req.description,
                instructor: req.instructor,
            }));
            ic_cdk::println!("Admin {:?} approved course request {}: {}.", caller(), req.id, title);
            "Course request approved and course added.".into()
        } else {
            "Approval failed: Course request not found.".into()
        }
    })
}

/// Admin-only: Lists all pending course requests.
#[query]
fn list_pending_requests() -> Vec<CourseRequest> {
    require_admin();
    PENDING_REQUESTS.with(|p| p.borrow().clone())
}

/// Admin-only: Adds a new DAO proposal.
/// In a real DAO, proposals would likely come from community voting or a more complex submission process.
#[update]
fn add_dao_proposal(text: String) -> String {
    require_admin(); // For this simplified example, only admin can add proposals
    DAO_PROPOSALS.with(|d| d.borrow_mut().push(DaoProposal { text, yes_votes: 0, no_votes: 0 }));
    ic_cdk::println!("Admin {:?} added DAO proposal.", caller());
    "DAO proposal added successfully.".into()
}

/// Allows any authenticated user to vote on an existing DAO proposal.
#[update]
fn vote_on_proposal(index: u64, yes: bool) -> String {
    let me = caller();
    assert!(me != Principal::anonymous(), "Authentication required.");

    DAO_PROPOSALS.with(|d| {
        if let Some(p) = d.borrow_mut().get_mut(index as usize) {
            if yes { p.yes_votes += 1; } else { p.no_votes += 1; }
            ic_cdk::println!("Principal {:?} voted {} on proposal {}.", me, if yes {"Yes"} else {"No"}, index);
            "Vote cast successfully.".into()
        } else {
            "Voting failed: Proposal not found.".into()
        }
    })
}

/// Public: Allows anyone to view all current DAO proposals and their vote counts.
#[query]
fn view_dao_proposals() -> Vec<DaoProposal> {
    DAO_PROPOSALS.with(|d| d.borrow().clone())
}

/// Admin-only: Bans an instructor by adding their Principal to a list.
/// Banned instructors cannot request new courses.
#[update]
fn ban_instructor(instructor: Principal) -> String {
    require_admin();
    BANNED_INSTRUCTORS.with(|b| {
        if !b.borrow().contains(&instructor) {
            b.borrow_mut().push(instructor);
            ic_cdk::println!("Admin {:?} banned instructor {:?}.", caller(), instructor);
            "Instructor banned successfully.".into()
        } else {
            "Instructor already banned.".into()
        }
    })
}

/// Admin-only: Removes a student by adding their Principal to a list.
/// Removed students cannot enroll in courses.
#[update]
fn remove_student(student: Principal) -> String {
    require_admin();
    REMOVED_STUDENTS.with(|r| {
        if !r.borrow().contains(&student) {
            r.borrow_mut().push(student);
            ic_cdk::println!("Admin {:?} removed student {:?}.", caller(), student);
            "Student removed successfully.".into()
        } else {
            "Student already removed.".into()
        }
    })
}

/// Admin-only: Sets the token reward amount per course.
#[update]
fn set_token_reward(amount: u64) -> String {
    require_admin();
    CONFIG.with(|c| c.borrow_mut().reward_per_course = amount);
    ic_cdk::println!("Admin {:?} set token reward per course to {}.", caller(), amount);
    "Token reward per course set successfully.".into()
}

/// Admin-only: Sets the token cost for a student to enroll in a course.
#[update]
fn set_cost_to_enroll(amount: u64) -> String {
    require_admin();
    CONFIG.with(|c| c.borrow_mut().cost_to_enroll = amount);
    ic_cdk::println!("Admin {:?} set cost to enroll to {}.", caller(), amount);
    "Cost to enroll set successfully.".into()
}

/// Public: Retrieves current platform statistics.
#[query]
fn get_platform_stats() -> PlatformStats {
    PlatformStats {
        total_students: STUDENTS.with(|s| s.borrow().len() as u64),
        total_courses: COURSES.with(|c| c.borrow().len() as u64),
        certificates_issued: ENROLLMENTS.with(|e| e.borrow().iter().filter(|en| en.passed == Some(true)).count() as u64),
    }
}

// Export Candid interface for the canister.
ic_cdk::export_candid!();
