// import React, { useEffect, useState } from 'react';
// import { AuthClient } from '@dfinity/auth-client';
// import { HttpAgent, Actor } from '@dfinity/agent';
// import { idlFactory, canisterId as educhainCanisterId } from '../../declarations/educhain_backend';

// function App() {
//   const [authClient, setAuthClient] = useState(null);
//   const [actor, setActor] = useState(null);
//   const [principal, setPrincipal] = useState(null);

//   const createActor = (identity) => {
//     const agent = new HttpAgent({ identity });
//     if (import.meta.env.VITE_DFX_NETWORK === 'local') {
//       agent.fetchRootKey();
//     }
//     return Actor.createActor(idlFactory, {
//       agent,
//       canisterId: educhainCanisterId,
//     });
//   };

//   useEffect(() => {
//     AuthClient.create().then(async (client) => {
//       setAuthClient(client);
//       if (await client.isAuthenticated()) {
//         const identity = client.getIdentity();
//         setPrincipal(identity.getPrincipal().toText());
//         setActor(createActor(identity));
//       }
//     });
//   }, []);

//   const login = async () => {
//     await authClient.login({
//       identityProvider: import.meta.env.VITE_DFX_NETWORK === 'local'
//         ? `http://${import.meta.env.VITE_INTERNET_IDENTITY_CANISTER_ID}.localhost:4943/#authorize`
//         : 'https://identity.ic0.app/#authorize',
//       onSuccess: async () => {
//         const identity = authClient.getIdentity();
//         setPrincipal(identity.getPrincipal().toText());
//         setActor(createActor(identity));
//       },
//     });
//   };

//   const logout = async () => {
//     await authClient.logout();
//     setActor(null);
//     setPrincipal(null);
//   };

//   return (
//     <div>
//       {principal ? (
//         <>
//           <p>Logged in as {principal}</p>
//           <button onClick={logout}>Logout</button>
//           {/* Add your protected app UI here */}
//         </>
//       ) : (
//         <button onClick={login}>Login with Internet Identity</button>
//       )}
//     </div>
//   );
// }

// export default App;

import React, { useState, useEffect } from "react";
import "./index.scss";
import { Toaster, toast } from "react-hot-toast";
import { AuthClient } from "@dfinity/auth-client";
import { HttpAgent, Actor } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { idlFactory, canisterId as educhainCanisterId } from "../../declarations/educhain_backend";

export default function App() {
  const [role, setRole] = useState(null);
  const [principalText, setPrincipalText] = useState("");
  const [identity, setIdentity] = useState(null);
  const [actor, setActor] = useState(null);
  const [authClient, setAuthClient] = useState(null);

  const [courses, setCourses] = useState([]);
  const [pendingRequests, setPendingRequests] = useState([]);
  const [myCourses, setMyCourses] = useState([]);
  const [daoProposals, setDaoProposals] = useState([]);
  const [stats, setStats] = useState({ total_students: 0, total_courses: 0, certificates_issued: 0 });
  const [studentProfile, setStudentProfile] = useState({ name: "", roll_no: "", email: "" });
  const [newCourse, setNewCourse] = useState({ title: "", description: "", instructor_name: "" });
  const [newProposal, setNewProposal] = useState("");
  const [voteIndex, setVoteIndex] = useState("");
  const [enrolledPopup, setEnrolledPopup] = useState({ show: false, courseId: null, list: [] });
  const [loading, setLoading] = useState(false);
const [users, setUsers] = useState([]);

  const network = import.meta.env.VITE_DFX_NETWORK || "local";
  const iiCanister = import.meta.env.VITE_INTERNET_IDENTITY_CANISTER_ID;

  const createActor = (identity) => {
    const agent = new HttpAgent({ identity });
    if (network === "local") agent.fetchRootKey();
    return Actor.createActor(idlFactory, { agent, canisterId: educhainCanisterId });
  };

  const refreshSession = async () => {
    if (!authClient) return;
    if (!(await authClient.isAuthenticated())) return;

    const ident = authClient.getIdentity();
    setIdentity(ident);
    const principal = ident.getPrincipal();
    setPrincipalText(principal.toText());

    const act = createActor(ident);
    setActor(act);

    try {
      await act.register_user(); // New users get Guest role
    } catch (e) {
      console.log("Already registered or registration failed:", e.message);
    }

    try {
      const res = await act.my_role();
      const detected = res ? Object.keys(res)[0] : "Guest";
      setRole(detected);
      toast.success(`Logged in as ${detected}`);
      loadAll(act);
    } catch (e) {
      console.error("Role fetch failed", e);
    }
  };

  const login = async () => {
    const client = await AuthClient.create();
    setAuthClient(client);
    await client.login({
      identityProvider:
        network === "ic"
          ? "https://identity.ic0.app/#authorize"
          : `http://${iiCanister}.localhost:4943/#authorize`,
      onSuccess: async () => {
        await refreshSession();
      },
    });
  };
const handleLoginAs = async (newRole) => {
  await login();  // perform Internet Identity login
  setRole(newRole); // store role locally

  // Optionally, store this in backend
  try {
    await actor.assign_role(Principal.fromText(principalText), { [newRole]: null });
    toast.success(`Assigned role ${newRole} to self`);
  } catch (e) {
    console.warn(`Failed to persist role in backend:`, e.message);
  }
};
const updateUsers = async () => {
  if (!actor) return;
  try {
    const arr = await actor.list_users();
    setUsers(arr.map(([p, r]) => ({
      principal: p.toText(),
      role: Object.keys(r)[0],
    })));
  } catch {
    toast.error("Failed to load user list");
  }
};
  const logout = async () => {
    await authClient.logout();
    setIdentity(null);
    setActor(null);
    setPrincipalText("");
    setRole(null);
    toast.success("Logged out!");
  };

  const loadAll = async (currentActor = actor) => {
    if (!currentActor) return;
    try {
      setLoading(true);
      const [allCourses, pending, mine, proposals, stats] = await Promise.all([
        currentActor.browse_courses(),
        currentActor.list_pending_requests(),
        currentActor.list_my_courses(),
        currentActor.view_dao_proposals(),
        currentActor.get_platform_stats()
      ]);
      setCourses(allCourses);
      setPendingRequests(pending);
      setMyCourses(mine);
      setDaoProposals(proposals);
      setStats(stats);
    } catch (e) {
      toast.error("Failed to load data");
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const call = async (method, args = []) => {
    if (!actor) return toast.error("Login first!");
    try {
      setLoading(true);
      const res = await actor[method](...args);
      toast.success(typeof res === "string" ? res : "Done");
      if (method === "request_new_course") setNewCourse({ title: "", description: "", instructor_name: "" });
      if (method === "add_dao_proposal") setNewProposal("");
      if (method === "vote_on_proposal") setVoteIndex("");
      await loadAll();
    } catch (e) {
      toast.error("Operation failed");
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const showEnrolled = async (courseId) => {
    try {
      const list = await actor.list_enrolled_students(BigInt(courseId));
      const formatted = list.map(en => ({
        student: en.student.toString(),
        name: en.student_name,
        roll_no: en.roll_no,
        status: en.passed === null ? "Present" : en.passed ? "Passed" : "Failed"
      }));
      setEnrolledPopup({ show: true, courseId, list: formatted });
    } catch (e) {
      toast.error("Failed to load students");
    }
  };

  useEffect(() => {
    AuthClient.create().then(async (client) => {
      setAuthClient(client);
      if (await client.isAuthenticated()) {
        await refreshSession();
      }
    });
  }, []);

  return (
    <div className="app">
      <Toaster position="top-right" />
      <h1>EduChain – Decentralized University</h1>

      <div className="auth-section">
       {!identity ? (
  <div className="roles">
    <button onClick={() => handleLoginAs("Student")}>Login as Student</button>
    <button onClick={() => handleLoginAs("Professor")}>Login as Instructor</button>
    <button onClick={() => handleLoginAs("Admin")}>Login as Admin</button>
  </div>
) : (
  <>
    <p>Logged in as {principalText} ({role})</p>
    <button onClick={logout}>Logout</button>
  </>
)}
      </div>

      {/* Student Panel */}
      {role === "Student" && (
        <div className="panel">
          <h2>Update Profile</h2>
          <input placeholder="Name" value={studentProfile.name}
            onChange={e => setStudentProfile({ ...studentProfile, name: e.target.value })} />
          <input placeholder="Roll No" value={studentProfile.roll_no}
            onChange={e => setStudentProfile({ ...studentProfile, roll_no: e.target.value })} />
          <input placeholder="Email" value={studentProfile.email}
            onChange={e => setStudentProfile({ ...studentProfile, email: e.target.value })} />
          <button disabled={loading}
            onClick={() => call("update_student_profile", [studentProfile.name, studentProfile.roll_no, studentProfile.email])}>
            Save
          </button>

          <h3>Available Courses</h3>
          {courses.map(c => (
            <div key={Number(c.id)} className="card">
              <b>{c.title}</b> – {c.description}
              <div>
                <button disabled={loading} onClick={() => call("enroll_in_course", [BigInt(c.id)])}>Enroll</button>
                <button disabled={loading} onClick={() => call("drop_out_of_course", [BigInt(c.id)])}>Drop Out</button>
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Instructor Panel */}
      {role === "Professor" && (
        <div className="panel">
          <h2>Request New Course</h2>
          <input placeholder="Instructor Name" value={newCourse.instructor_name}
            onChange={e => setNewCourse({ ...newCourse, instructor_name: e.target.value })} />
          <input placeholder="Title" value={newCourse.title}
            onChange={e => setNewCourse({ ...newCourse, title: e.target.value })} />
          <input placeholder="Description" value={newCourse.description}
            onChange={e => setNewCourse({ ...newCourse, description: e.target.value })} />
          <button disabled={loading}
            onClick={() => call("request_new_course", [newCourse.title, newCourse.description, newCourse.instructor_name])}>
            Request
          </button>

          <h3>My Courses</h3>
          {myCourses.map(c => (
            <div key={Number(c.id)} className="card">
              <b>{c.title}</b> – {c.description}
              <button onClick={() => showEnrolled(c.id)}>Show Enrolled Students</button>
            </div>
          ))}
        </div>
      )}

      {/* Admin Panel */}
      {role === "Admin" && (
        <div className="panel">
          <h2>Pending Course Requests</h2>
          {pendingRequests.map(r => (
            <div key={Number(r.id)} className="card">
              <b>{r.title}</b> – {r.description} <br />
              Instructor: {r.instructor_name}
              <button disabled={loading} onClick={() => call("approve_course_request", [BigInt(r.id)])}>Approve</button>
            </div>
          ))}

          <h2>DAO Proposals</h2>
          <ul>
            {daoProposals.map((p, i) => (
              <li key={i}>{p.text} | Yes: {p.yes_votes} No: {p.no_votes}</li>
            ))}
          </ul>
          <input placeholder="New Proposal" value={newProposal}
            onChange={e => setNewProposal(e.target.value)} />
          <button disabled={loading} onClick={() => call("add_dao_proposal", [newProposal])}>Add Proposal</button>

          <h3>Vote on Proposal</h3>
          <input placeholder="Proposal Index" value={voteIndex}
            onChange={e => setVoteIndex(e.target.value)} />
          <button disabled={loading} onClick={() => call("vote_on_proposal", [BigInt(voteIndex), true])}>Vote YES</button>
          <button disabled={loading} onClick={() => call("vote_on_proposal", [BigInt(voteIndex), false])}>Vote NO</button>

          <h3>Platform Stats</h3>
          <p>Students: {stats.total_students}, Courses: {stats.total_courses}, Certificates: {stats.certificates_issued}</p>
        <section>
  <h2>All Users & Roles</h2>
  {users.length > 0 ? (
    <ul>
      {users.map(u => (
        <li key={u.principal}>
          <strong>{u.principal}</strong> — {u.role}
        </li>
      ))}
    </ul>
  ) : (
    <p>No users found.</p>
  )}
</section>
        </div>
      )}

      {enrolledPopup.show && (
        <div className="modal">
          <div className="modal-content">
            <h3>Enrolled Students</h3>
            <ul>
              {enrolledPopup.list.map((s, i) => (
                <li key={i}>{s.name} ({s.roll_no}) – Status: {s.status}
                  <button disabled={loading}
                    onClick={() => call("mark_pass", [BigInt(enrolledPopup.courseId), Principal.fromText(s.student)])}>Pass</button>
                  <button disabled={loading}
                    onClick={() => call("mark_fail", [BigInt(enrolledPopup.courseId), Principal.fromText(s.student)])}>Fail</button>
                </li>
              ))}
            </ul>
            <button onClick={() => setEnrolledPopup({ show: false, courseId: null, list: [] })}>Close</button>
          </div>
        </div>
      )}
    </div>
  );
}

