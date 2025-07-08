import React, { useState, useEffect } from 'react';
import './index.scss';
import { Toaster, toast } from 'react-hot-toast';

function App() {
  const [role, setRole] = useState(null);
  const [courses, setCourses] = useState([]);
  const [pendingRequests, setPendingRequests] = useState([]);
  const [myCourses, setMyCourses] = useState([]);
  const [daoProposals, setDaoProposals] = useState([]);
  const [stats, setStats] = useState({ total_students:0, total_courses:0, certificates_issued:0 });
  const [studentProfile, setStudentProfile] = useState({ name:'', roll_no:'', email:'' });
  const [newCourse, setNewCourse] = useState({ title:'', description:'', instructor_name:'' });
  const [newProposal, setNewProposal] = useState('');
  const [voteIndex, setVoteIndex] = useState('');
  const [enrolledPopup, setEnrolledPopup] = useState({ show: false, courseId: null, list: [] });
  const [loading, setLoading] = useState(false);

  const loadAll = async () => {
    try {
      const { educhain_backend } = await import('../../declarations/educhain_backend');
      setCourses(await educhain_backend.browse_courses());
      setPendingRequests(await educhain_backend.list_pending_requests());
      setMyCourses(await educhain_backend.list_my_courses());
      setDaoProposals(await educhain_backend.view_dao_proposals());
      setStats(await educhain_backend.get_platform_stats());
    } catch (e) {
      console.error(e);
      toast.error('Failed to load data');
    }
  };

  useEffect(() => { loadAll(); }, []);

  const call = async (method, args=[]) => {
    try {
      setLoading(true);
      const { educhain_backend } = await import('../../declarations/educhain_backend');
      const res = await educhain_backend[method](...args);
      toast.success(typeof res === 'string' ? res : 'Done');
      setNewCourse({ title:'', description:'', instructor_name:'' });
      setNewProposal(''); setVoteIndex('');
      loadAll();
    } catch (e) {
      console.error(e);
      toast.error('Operation failed');
    } finally { setLoading(false); }
  };

  const showEnrolled = async (courseId) => {
    try {
      const { educhain_backend } = await import('../../declarations/educhain_backend');
      const list = await educhain_backend.list_enrolled_students(BigInt(courseId));
      // Convert students to string for display
      const formatted = list.map(en => ({
        student: en.student.toString(),
        name: en.student_name,
        roll_no: en.roll_no,
        status: en.passed == null ? 'Present' : en.passed ? 'Passed' : 'Failed'
      }));
      setEnrolledPopup({ show: true, courseId, list: formatted });
    } catch (e) {
      toast.error('Failed to load enrolled students');
    }
  };

  return (
    <div className="app">
      <Toaster position="top-right" />
      <h1>EduChain – Decentralized University</h1>

      <div className="roles">
        <button onClick={()=>setRole('Student')}>Student</button>
        <button onClick={()=>setRole('Instructor')}>Instructor</button>
        <button onClick={()=>setRole('Admin')}>Admin</button>
      </div>

      {/* Student Panel */}
      {role==='Student' && (
        <div className="panel">
          <h2>Update Profile</h2>
          <input placeholder="Name" value={studentProfile.name} onChange={e=>setStudentProfile({...studentProfile,name:e.target.value})}/>
          <input placeholder="Roll No" value={studentProfile.roll_no} onChange={e=>setStudentProfile({...studentProfile,roll_no:e.target.value})}/>
          <input placeholder="Email" value={studentProfile.email} onChange={e=>setStudentProfile({...studentProfile,email:e.target.value})}/>
          <button disabled={loading} onClick={()=>call('update_student_profile',[studentProfile.name, studentProfile.roll_no, studentProfile.email])}>Save</button>

          <h3>Available Courses</h3>
          {courses.map(c=>(
            <div key={Number(c.id)}>
              <b>{c.title}</b> – {c.description}
              <button disabled={loading} onClick={()=>call('enroll_in_course',[BigInt(c.id)])}>Enroll</button>
              <button disabled={loading} onClick={()=>call('drop_out_of_course',[BigInt(c.id)])}>Drop Out</button>
            </div>
          ))}
        </div>
      )}

      {/* Instructor Panel */}
      {role==='Instructor' && (
        <div className="panel">
          <h2>Request New Course</h2>
          <input placeholder="Instructor Name" value={newCourse.instructor_name} onChange={e=>setNewCourse({...newCourse,instructor_name:e.target.value})}/>
          <input placeholder="Title" value={newCourse.title} onChange={e=>setNewCourse({...newCourse,title:e.target.value})}/>
          <input placeholder="Description" value={newCourse.description} onChange={e=>setNewCourse({...newCourse,description:e.target.value})}/>
          <button disabled={loading} onClick={()=>call('request_new_course',[newCourse.title, newCourse.description, newCourse.instructor_name])}>Request</button>

          <h3>My Courses</h3>
          {myCourses.map(c=>(
            <div key={Number(c.id)}>
              <b>{c.title}</b> – {c.description}
              <button onClick={()=>showEnrolled(c.id)}>Show Enrolled Students</button>
            </div>
          ))}
        </div>
      )}

      {/* Admin Panel */}
      {role==='Admin' && (
        <div className="panel">
          <h2>Pending Course Requests</h2>
          {pendingRequests.map(r=>(
            <div key={Number(r.id)} className="card">
              <b>{r.title}</b> – {r.description} <br/>
              Instructor: {r.instructor_name}
              <button disabled={loading} onClick={()=>call('approve_course_request',[BigInt(r.id)])}>Approve</button>
            </div>
          ))}

          <h2>DAO Proposals</h2>
          <ul>
            {daoProposals.map((p,i)=>(
              <li key={i}>{p.text} | Yes: {p.yes_votes} No: {p.no_votes}</li>
            ))}
          </ul>
          <input placeholder="New Proposal" value={newProposal} onChange={e=>setNewProposal(e.target.value)}/>
          <button disabled={loading} onClick={()=>call('add_dao_proposal',[newProposal])}>Add Proposal</button>

          <h3>Vote on Proposal</h3>
          <input placeholder="Proposal Index" value={voteIndex} onChange={e=>setVoteIndex(e.target.value)}/>
          <button disabled={loading} onClick={()=>call('vote_on_proposal',[BigInt(voteIndex),true])}>Vote YES</button>
          <button disabled={loading} onClick={()=>call('vote_on_proposal',[BigInt(voteIndex),false])}>Vote NO</button>

          <h3>Platform Stats</h3>
          <p>Students: {stats.total_students}, Courses: {stats.total_courses}, Certificates: {stats.certificates_issued}</p>
        </div>
      )}

      {/* Enrolled Students Popup */}
      {enrolledPopup.show && (
        <div className="modal">
          <div className="modal-content">
            <h3>Enrolled Students</h3>
            <ul>
              {enrolledPopup.list.map((s,i)=>(
                <li key={i}>{s.name} ({s.roll_no}) – Status: {s.status}
                  <button onClick={()=>call('mark_pass',[BigInt(enrolledPopup.courseId), s.student])}>Pass</button>
                  <button onClick={()=>call('mark_fail',[BigInt(enrolledPopup.courseId), s.student])}>Fail</button>
                </li>
              ))}
            </ul>
            <button onClick={()=>setEnrolledPopup({ show:false, courseId:null, list:[] })}>Close</button>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
